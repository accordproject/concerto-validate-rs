use serde_json::Value;
use std::collections::HashMap;

/// Metamodel manager that works with raw JSON instead of hardcoded structs
/// This allows us to validate any metamodel structure against the Concerto metamodel
pub struct MetamodelManager {
    /// The raw Concerto metamodel JSON
    concerto_metamodel: Value,
    /// Registry of type declarations from the metamodel
    type_registry: HashMap<String, Value>,
}

impl MetamodelManager {
    /// Load the Concerto metamodel from the embedded JSON file
    pub fn new() -> Result<Self, crate::error::ValidationError> {
        let metamodel_json = include_str!("../metamodel.json");
        let concerto_metamodel: Value = serde_json::from_str(metamodel_json)?;
        
        let type_registry = Self::build_type_registry(&concerto_metamodel)?;
        
        Ok(Self {
            concerto_metamodel,
            type_registry,
        })
    }
    
    /// Build a type registry from the metamodel declarations
    fn build_type_registry(metamodel: &Value) -> Result<HashMap<String, Value>, crate::error::ValidationError> {
        let mut registry = HashMap::new();
        
        let declarations = metamodel
            .get("declarations")
            .and_then(|v| v.as_array())
            .ok_or_else(|| crate::error::ValidationError::MetamodelError {
                message: "Missing declarations in metamodel".to_string(),
            })?;
        
        let namespace = metamodel
            .get("namespace")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::error::ValidationError::MetamodelError {
                message: "Missing namespace in metamodel".to_string(),
            })?;
            
        for declaration in declarations {
            if let Some(name) = declaration.get("name").and_then(|v| v.as_str()) {
                let full_name = format!("{}.{}", namespace, name);
                registry.insert(full_name, declaration.clone());
            }
        }
        
        Ok(registry)
    }
    
    /// Get the raw metamodel JSON
    pub fn get_metamodel(&self) -> &Value {
        &self.concerto_metamodel
    }
    
    /// Get a type declaration by its full name
    pub fn get_type_declaration(&self, full_name: &str) -> Option<&Value> {
        self.type_registry.get(full_name)
    }
    
    /// Get all type declarations
    pub fn get_type_registry(&self) -> &HashMap<String, Value> {
        &self.type_registry
    }
    
    /// Check if a class exists in the metamodel
    pub fn has_class(&self, class_name: &str) -> bool {
        self.type_registry.contains_key(class_name)
    }
    
    /// Get the class type from a declaration (e.g., "ConceptDeclaration", "AssetDeclaration")
    pub fn get_declaration_type(&self, full_name: &str) -> Option<String> {
        self.type_registry.get(full_name)
            .and_then(|decl| decl.get("$class"))
            .and_then(|class| class.as_str())
            .map(|s| s.to_string())
    }
    
    /// Validate that a given JSON structure matches the metamodel schema
    pub fn validate_against_metamodel(&self, json_to_validate: &Value) -> Result<(), crate::error::ValidationError> {
        // The input should be a Model that follows the structure defined in the metamodel
        self.validate_model_structure(json_to_validate)
    }
    
    /// Validate the structure of a model JSON against the metamodel
    fn validate_model_structure(&self, model: &Value) -> Result<(), crate::error::ValidationError> {
        let obj = model.as_object()
            .ok_or_else(|| crate::error::ValidationError::TypeMismatch {
                expected: "object".to_string(),
                found: "non-object".to_string(),
            })?;

        // Check $class
        let class_name = obj.get("$class")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::error::ValidationError::MissingProperty {
                property: "$class".to_string(),
            })?;

        if !class_name.ends_with(".Model") {
            return Err(crate::error::ValidationError::TypeMismatch {
                expected: "Model".to_string(),
                found: class_name.to_string(),
            });
        }

        // Check required properties for Model
        self.validate_required_property(obj, "namespace", "string")?;
        
        // Validate declarations array if present
        if let Some(declarations) = obj.get("declarations") {
            let decl_array = declarations.as_array()
                .ok_or_else(|| crate::error::ValidationError::TypeMismatch {
                    expected: "array".to_string(),
                    found: "non-array".to_string(),
                })?;
                
            for declaration in decl_array {
                self.validate_declaration_structure(declaration)?;
            }
        }
        
        // Validate imports array if present
        if let Some(imports) = obj.get("imports") {
            let imports_array = imports.as_array()
                .ok_or_else(|| crate::error::ValidationError::TypeMismatch {
                    expected: "array".to_string(),
                    found: "non-array".to_string(),
                })?;
                
            for import in imports_array {
                self.validate_import_structure(import)?;
            }
        }
        
        Ok(())
    }
    
    /// Validate a declaration structure
    fn validate_declaration_structure(&self, declaration: &Value) -> Result<(), crate::error::ValidationError> {
        let obj = declaration.as_object()
            .ok_or_else(|| crate::error::ValidationError::TypeMismatch {
                expected: "object".to_string(),
                found: "non-object".to_string(),
            })?;

        // Check $class
        let class_name = obj.get("$class")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::error::ValidationError::MissingProperty {
                property: "$class".to_string(),
            })?;

        // Validate based on declaration type
        match class_name {
            name if name.ends_with("ConceptDeclaration") ||
                    name.ends_with("AssetDeclaration") ||
                    name.ends_with("ParticipantDeclaration") ||
                    name.ends_with("TransactionDeclaration") ||
                    name.ends_with("EventDeclaration") => {
                self.validate_required_property(obj, "name", "string")?;
                
                if let Some(properties) = obj.get("properties") {
                    let props_array = properties.as_array()
                        .ok_or_else(|| crate::error::ValidationError::TypeMismatch {
                            expected: "array".to_string(),
                            found: "non-array".to_string(),
                        })?;
                        
                    for property in props_array {
                        self.validate_property_structure(property)?;
                    }
                }
            }
            name if name.ends_with("EnumDeclaration") => {
                self.validate_required_property(obj, "name", "string")?;
            }
            name if name.ends_with("MapDeclaration") => {
                self.validate_required_property(obj, "name", "string")?;
            }
            name if name.ends_with("ScalarDeclaration") => {
                self.validate_required_property(obj, "name", "string")?;
                self.validate_required_property(obj, "type", "string")?;
            }
            _ => {
                return Err(crate::error::ValidationError::UnknownClass {
                    class_name: class_name.to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    /// Validate a property structure
    fn validate_property_structure(&self, property: &Value) -> Result<(), crate::error::ValidationError> {
        let obj = property.as_object()
            .ok_or_else(|| crate::error::ValidationError::TypeMismatch {
                expected: "object".to_string(),
                found: "non-object".to_string(),
            })?;

        // Check $class
        obj.get("$class")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::error::ValidationError::MissingProperty {
                property: "$class".to_string(),
            })?;

        // All properties must have these fields
        self.validate_required_property(obj, "name", "string")?;
        self.validate_required_property(obj, "isArray", "boolean")?;
        self.validate_required_property(obj, "isOptional", "boolean")?;
        
        Ok(())
    }
    
    /// Validate an import structure
    fn validate_import_structure(&self, import: &Value) -> Result<(), crate::error::ValidationError> {
        let obj = import.as_object()
            .ok_or_else(|| crate::error::ValidationError::TypeMismatch {
                expected: "object".to_string(),
                found: "non-object".to_string(),
            })?;

        self.validate_required_property(obj, "namespace", "string")?;
        
        Ok(())
    }
    
    /// Helper to validate that a required property exists and has the correct type
    fn validate_required_property(
        &self, 
        obj: &serde_json::Map<String, Value>, 
        property_name: &str, 
        expected_type: &str
    ) -> Result<(), crate::error::ValidationError> {
        let value = obj.get(property_name)
            .ok_or_else(|| crate::error::ValidationError::MissingProperty {
                property: property_name.to_string(),
            })?;
            
        let is_correct_type = match expected_type {
            "string" => value.is_string(),
            "boolean" => value.is_boolean(),
            "number" => value.is_number(),
            "array" => value.is_array(),
            "object" => value.is_object(),
            _ => true, // Unknown types are accepted
        };
        
        if !is_correct_type {
            return Err(crate::error::ValidationError::TypeMismatch {
                expected: expected_type.to_string(),
                found: match value {
                    Value::String(_) => "string",
                    Value::Bool(_) => "boolean", 
                    Value::Number(_) => "number",
                    Value::Array(_) => "array",
                    Value::Object(_) => "object",
                    Value::Null => "null",
                }.to_string(),
            });
        }
        
        Ok(())
    }
}
