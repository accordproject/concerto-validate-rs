use crate::error::{ValidationError, ValidationResult};
use crate::factory::Factory;
use crate::serializer::Serializer;
use serde_json::Value;

/// Main validator class - equivalent to the validateMetaModel function in JavaScript
/// This brings together the Factory and Serializer to validate Concerto models
pub struct ConcertoValidator {
    factory: Factory,
    serializer: Serializer,
}

impl ConcertoValidator {
    /// Create a new validator with the Concerto metamodel
    pub fn new() -> ValidationResult<Self> {
        let factory = Factory::new_with_metamodel()?;
        let serializer = Serializer::new()?;
        
        Ok(Self {
            factory,
            serializer,
        })
    }

    /// Main validation method - equivalent to validateMetaModel in JavaScript
    /// This validates a JSON AST representing a Concerto model against the metamodel
    pub fn validate(&self, json_ast: &str) -> ValidationResult<()> {
        // First, validate that the JSON is well-formed and matches the metamodel structure
        let validated_ast = self.serializer.from_json(json_ast)?;
        
        // Then use the factory to further validate the structure
        self.validate_with_factory(&validated_ast)?;
        
        Ok(())
    }

    /// Validate using the factory (additional semantic validation)
    fn validate_with_factory(&self, ast: &Value) -> ValidationResult<()> {
        // Validate that this is a Model
        let obj = ast.as_object()
            .ok_or_else(|| ValidationError::TypeMismatch {
                expected: "object".to_string(),
                found: "non-object".to_string(),
            })?;

        let class_name = obj.get("$class")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "$class".to_string(),
            })?;

        // Check that this is indeed a Model
        if !class_name.ends_with(".Model") {
            return Err(ValidationError::TypeMismatch {
                expected: "Model".to_string(),
                found: class_name.to_string(),
            });
        }

        // Validate the namespace
        let namespace = obj.get("namespace")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "namespace".to_string(),
            })?;

        if namespace.is_empty() {
            return Err(ValidationError::InvalidPropertyValue {
                property: "namespace".to_string(),
                value: "empty string".to_string(),
            });
        }

        // Validate declarations exist
        let declarations = obj.get("declarations")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "declarations".to_string(),
            })?;

        // Validate each declaration
        for declaration in declarations {
            self.validate_declaration(declaration)?;
        }

        // Validate imports if present
        if let Some(imports) = obj.get("imports") {
            let imports_array = imports.as_array()
                .ok_or_else(|| ValidationError::TypeMismatch {
                    expected: "array".to_string(),
                    found: "non-array".to_string(),
                })?;

            for import in imports_array {
                self.validate_import(import)?;
            }
        }

        Ok(())
    }

    /// Validate a single declaration
    fn validate_declaration(&self, declaration: &Value) -> ValidationResult<()> {
        let obj = declaration.as_object()
            .ok_or_else(|| ValidationError::TypeMismatch {
                expected: "object".to_string(),
                found: "non-object".to_string(),
            })?;

        let class_name = obj.get("$class")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "$class".to_string(),
            })?;

        // Validate that it's a known declaration type
        match class_name {
            name if name.ends_with(".ConceptDeclaration") => self.validate_concept_declaration(obj),
            name if name.ends_with(".AssetDeclaration") => self.validate_asset_declaration(obj),
            name if name.ends_with(".ParticipantDeclaration") => self.validate_participant_declaration(obj),
            name if name.ends_with(".TransactionDeclaration") => self.validate_transaction_declaration(obj),
            name if name.ends_with(".EventDeclaration") => self.validate_event_declaration(obj),
            name if name.ends_with(".EnumDeclaration") => self.validate_enum_declaration(obj),
            name if name.ends_with(".MapDeclaration") => self.validate_map_declaration(obj),
            name if name.ends_with(".ScalarDeclaration") => self.validate_scalar_declaration(obj),
            _ => Err(ValidationError::UnknownClass {
                class_name: class_name.to_string(),
            }),
        }
    }

    /// Validate a concept declaration
    fn validate_concept_declaration(&self, obj: &serde_json::Map<String, Value>) -> ValidationResult<()> {
        self.validate_basic_declaration(obj)?;
        self.validate_properties_array(obj)?;
        Ok(())
    }

    /// Validate an asset declaration
    fn validate_asset_declaration(&self, obj: &serde_json::Map<String, Value>) -> ValidationResult<()> {
        self.validate_basic_declaration(obj)?;
        self.validate_properties_array(obj)?;
        Ok(())
    }

    /// Validate a participant declaration
    fn validate_participant_declaration(&self, obj: &serde_json::Map<String, Value>) -> ValidationResult<()> {
        self.validate_basic_declaration(obj)?;
        self.validate_properties_array(obj)?;
        Ok(())
    }

    /// Validate a transaction declaration
    fn validate_transaction_declaration(&self, obj: &serde_json::Map<String, Value>) -> ValidationResult<()> {
        self.validate_basic_declaration(obj)?;
        self.validate_properties_array(obj)?;
        Ok(())
    }

    /// Validate an event declaration
    fn validate_event_declaration(&self, obj: &serde_json::Map<String, Value>) -> ValidationResult<()> {
        self.validate_basic_declaration(obj)?;
        self.validate_properties_array(obj)?;
        Ok(())
    }

    /// Validate an enum declaration
    fn validate_enum_declaration(&self, obj: &serde_json::Map<String, Value>) -> ValidationResult<()> {
        // Validate name
        let name = obj.get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "name".to_string(),
            })?;

        if name.is_empty() {
            return Err(ValidationError::InvalidPropertyValue {
                property: "name".to_string(),
                value: "empty string".to_string(),
            });
        }

        // Validate properties (enum values)
        let properties = obj.get("properties")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "properties".to_string(),
            })?;

        if properties.is_empty() {
            return Err(ValidationError::InvalidPropertyValue {
                property: "properties".to_string(),
                value: "empty array".to_string(),
            });
        }

        Ok(())
    }

    /// Validate a map declaration
    fn validate_map_declaration(&self, obj: &serde_json::Map<String, Value>) -> ValidationResult<()> {
        // Validate name
        let name = obj.get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "name".to_string(),
            })?;

        if name.is_empty() {
            return Err(ValidationError::InvalidPropertyValue {
                property: "name".to_string(),
                value: "empty string".to_string(),
            });
        }

        // Validate key and value types
        obj.get("key")
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "key".to_string(),
            })?;

        obj.get("value")
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "value".to_string(),
            })?;

        Ok(())
    }

    /// Validate a scalar declaration
    fn validate_scalar_declaration(&self, obj: &serde_json::Map<String, Value>) -> ValidationResult<()> {
        // Validate name
        let name = obj.get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "name".to_string(),
            })?;

        if name.is_empty() {
            return Err(ValidationError::InvalidPropertyValue {
                property: "name".to_string(),
                value: "empty string".to_string(),
            });
        }

        // Validate type
        let scalar_type = obj.get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "type".to_string(),
            })?;

        // Check that it's a valid scalar type
        match scalar_type {
            "String" | "Double" | "Integer" | "Long" | "DateTime" | "Boolean" => Ok(()),
            _ => Err(ValidationError::InvalidPropertyValue {
                property: "type".to_string(),
                value: scalar_type.to_string(),
            }),
        }
    }

    /// Validate basic declaration properties (name, isAbstract)
    fn validate_basic_declaration(&self, obj: &serde_json::Map<String, Value>) -> ValidationResult<()> {
        // Validate name
        let name = obj.get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "name".to_string(),
            })?;

        if name.is_empty() {
            return Err(ValidationError::InvalidPropertyValue {
                property: "name".to_string(),
                value: "empty string".to_string(),
            });
        }

        // Validate isAbstract if present
        if let Some(is_abstract) = obj.get("isAbstract") {
            if !is_abstract.is_boolean() {
                return Err(ValidationError::TypeMismatch {
                    expected: "boolean".to_string(),
                    found: "non-boolean".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate properties array
    fn validate_properties_array(&self, obj: &serde_json::Map<String, Value>) -> ValidationResult<()> {
        let properties = obj.get("properties")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "properties".to_string(),
            })?;

        for property in properties {
            self.validate_property(property)?;
        }

        Ok(())
    }

    /// Validate a single property
    fn validate_property(&self, property: &Value) -> ValidationResult<()> {
        let obj = property.as_object()
            .ok_or_else(|| ValidationError::TypeMismatch {
                expected: "object".to_string(),
                found: "non-object".to_string(),
            })?;

        // Validate property has required fields
        let name = obj.get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "name".to_string(),
            })?;

        if name.is_empty() {
            return Err(ValidationError::InvalidPropertyValue {
                property: "name".to_string(),
                value: "empty string".to_string(),
            });
        }

        // Validate isArray
        if let Some(is_array) = obj.get("isArray") {
            if !is_array.is_boolean() {
                return Err(ValidationError::TypeMismatch {
                    expected: "boolean".to_string(),
                    found: "non-boolean".to_string(),
                });
            }
        }

        // Validate isOptional
        if let Some(is_optional) = obj.get("isOptional") {
            if !is_optional.is_boolean() {
                return Err(ValidationError::TypeMismatch {
                    expected: "boolean".to_string(),
                    found: "non-boolean".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate an import
    fn validate_import(&self, import: &Value) -> ValidationResult<()> {
        let obj = import.as_object()
            .ok_or_else(|| ValidationError::TypeMismatch {
                expected: "object".to_string(),
                found: "non-object".to_string(),
            })?;

        // Validate namespace
        let namespace = obj.get("namespace")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "namespace".to_string(),
            })?;

        if namespace.is_empty() {
            return Err(ValidationError::InvalidPropertyValue {
                property: "namespace".to_string(),
                value: "empty string".to_string(),
            });
        }

        Ok(())
    }

    /// Get a reference to the factory
    pub fn get_factory(&self) -> &Factory {
        &self.factory
    }

    /// Get a reference to the serializer
    pub fn get_serializer(&self) -> &Serializer {
        &self.serializer
    }
}

impl Default for ConcertoValidator {
    fn default() -> Self {
        Self::new().expect("Failed to create default ConcertoValidator")
    }
}
