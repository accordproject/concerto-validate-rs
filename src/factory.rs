use crate::error::{ValidationError, ValidationResult};
use crate::metamodel::{Model, Declaration};
use serde_json::Value;
use std::collections::HashMap;

/// Factory class equivalent - manages model creation and validation
/// This replicates the functionality of the JavaScript Factory class
pub struct Factory {
    model_manager: ModelManager,
}

impl Factory {
    /// Create a new factory with the given model manager
    pub fn new(model_manager: ModelManager) -> Self {
        Self { model_manager }
    }

    /// Create a new factory with the Concerto metamodel
    pub fn new_with_metamodel() -> ValidationResult<Self> {
        let metamodel = Model::load_concerto_metamodel()?;
        let model_manager = ModelManager::new(metamodel);
        Ok(Self { model_manager })
    }

    /// Create a new resource from JSON data
    /// This is equivalent to the newResource method in JavaScript Factory
    pub fn new_resource(&self, json: &str) -> ValidationResult<Value> {
        let value: Value = serde_json::from_str(json)?;
        
        // Validate that the JSON represents a valid resource
        self.validate_resource(&value)?;
        
        Ok(value)
    }

    /// Create a new concept from JSON data
    /// This is equivalent to the newConcept method in JavaScript Factory
    pub fn new_concept(&self, namespace: &str, type_name: &str, json: &str) -> ValidationResult<Value> {
        let value: Value = serde_json::from_str(json)?;
        
        // Validate that the JSON represents a valid concept
        self.validate_concept(namespace, type_name, &value)?;
        
        Ok(value)
    }

    /// Create a new transaction from JSON data
    pub fn new_transaction(&self, namespace: &str, type_name: &str, json: &str) -> ValidationResult<Value> {
        let value: Value = serde_json::from_str(json)?;
        
        // Validate that the JSON represents a valid transaction
        self.validate_transaction(namespace, type_name, &value)?;
        
        Ok(value)
    }

    /// Create a new event from JSON data
    pub fn new_event(&self, namespace: &str, type_name: &str, json: &str) -> ValidationResult<Value> {
        let value: Value = serde_json::from_str(json)?;
        
        // Validate that the JSON represents a valid event
        self.validate_event(namespace, type_name, &value)?;
        
        Ok(value)
    }

    /// Validate a resource
    fn validate_resource(&self, value: &Value) -> ValidationResult<()> {
        let obj = value.as_object()
            .ok_or_else(|| ValidationError::TypeMismatch {
                expected: "object".to_string(),
                found: "non-object".to_string(),
            })?;

        let class_name = obj.get("$class")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "$class".to_string(),
            })?;

        // Check if the class exists in the model
        self.model_manager.validate_class(class_name)?;

        Ok(())
    }

    /// Validate a concept
    fn validate_concept(&self, namespace: &str, type_name: &str, value: &Value) -> ValidationResult<()> {
        let full_name = format!("{}.{}", namespace, type_name);
        
        let obj = value.as_object()
            .ok_or_else(|| ValidationError::TypeMismatch {
                expected: "object".to_string(),
                found: "non-object".to_string(),
            })?;

        // Check if $class matches expected type
        if let Some(class_value) = obj.get("$class") {
            let class_name = class_value.as_str()
                .ok_or_else(|| ValidationError::TypeMismatch {
                    expected: "string".to_string(),
                    found: "non-string".to_string(),
                })?;

            if class_name != full_name {
                return Err(ValidationError::TypeMismatch {
                    expected: full_name,
                    found: class_name.to_string(),
                });
            }
        }

        // Validate the type exists and is a concept
        self.model_manager.validate_concept_class(&full_name)?;

        Ok(())
    }

    /// Validate a transaction
    fn validate_transaction(&self, namespace: &str, type_name: &str, value: &Value) -> ValidationResult<()> {
        let full_name = format!("{}.{}", namespace, type_name);
        
        let obj = value.as_object()
            .ok_or_else(|| ValidationError::TypeMismatch {
                expected: "object".to_string(),
                found: "non-object".to_string(),
            })?;

        // Check if $class matches expected type
        if let Some(class_value) = obj.get("$class") {
            let class_name = class_value.as_str()
                .ok_or_else(|| ValidationError::TypeMismatch {
                    expected: "string".to_string(),
                    found: "non-string".to_string(),
                })?;

            if class_name != full_name {
                return Err(ValidationError::TypeMismatch {
                    expected: full_name,
                    found: class_name.to_string(),
                });
            }
        }

        // Validate the type exists and is a transaction
        self.model_manager.validate_transaction_class(&full_name)?;

        Ok(())
    }

    /// Validate an event
    fn validate_event(&self, namespace: &str, type_name: &str, value: &Value) -> ValidationResult<()> {
        let full_name = format!("{}.{}", namespace, type_name);
        
        let obj = value.as_object()
            .ok_or_else(|| ValidationError::TypeMismatch {
                expected: "object".to_string(),
                found: "non-object".to_string(),
            })?;

        // Check if $class matches expected type
        if let Some(class_value) = obj.get("$class") {
            let class_name = class_value.as_str()
                .ok_or_else(|| ValidationError::TypeMismatch {
                    expected: "string".to_string(),
                    found: "non-string".to_string(),
                })?;

            if class_name != full_name {
                return Err(ValidationError::TypeMismatch {
                    expected: full_name,
                    found: class_name.to_string(),
                });
            }
        }

        // Validate the type exists and is an event
        self.model_manager.validate_event_class(&full_name)?;

        Ok(())
    }

    /// Get the model manager
    pub fn get_model_manager(&self) -> &ModelManager {
        &self.model_manager
    }
}

/// Model Manager - manages the loaded models and their declarations
/// This is a simplified version of the JavaScript ModelManager
pub struct ModelManager {
    model: Model,
    type_registry: HashMap<String, Declaration>,
}

impl ModelManager {
    /// Create a new model manager with the given model
    pub fn new(model: Model) -> Self {
        let type_registry = model.create_type_registry()
            .into_iter()
            .map(|(k, v)| (k, v.clone()))
            .collect();

        Self {
            model,
            type_registry,
        }
    }

    /// Validate that a class exists in the model
    pub fn validate_class(&self, class_name: &str) -> ValidationResult<()> {
        if !self.type_registry.contains_key(class_name) {
            return Err(ValidationError::UnknownClass {
                class_name: class_name.to_string(),
            });
        }
        Ok(())
    }

    /// Validate that a class is a concept declaration
    pub fn validate_concept_class(&self, class_name: &str) -> ValidationResult<()> {
        match self.type_registry.get(class_name) {
            Some(Declaration::ConceptDeclaration { .. }) => Ok(()),
            Some(_) => Err(ValidationError::TypeMismatch {
                expected: "ConceptDeclaration".to_string(),
                found: "other declaration type".to_string(),
            }),
            None => Err(ValidationError::UnknownClass {
                class_name: class_name.to_string(),
            }),
        }
    }

    /// Validate that a class is a transaction declaration
    pub fn validate_transaction_class(&self, class_name: &str) -> ValidationResult<()> {
        match self.type_registry.get(class_name) {
            Some(Declaration::TransactionDeclaration { .. }) => Ok(()),
            Some(_) => Err(ValidationError::TypeMismatch {
                expected: "TransactionDeclaration".to_string(),
                found: "other declaration type".to_string(),
            }),
            None => Err(ValidationError::UnknownClass {
                class_name: class_name.to_string(),
            }),
        }
    }

    /// Validate that a class is an event declaration
    pub fn validate_event_class(&self, class_name: &str) -> ValidationResult<()> {
        match self.type_registry.get(class_name) {
            Some(Declaration::EventDeclaration { .. }) => Ok(()),
            Some(_) => Err(ValidationError::TypeMismatch {
                expected: "EventDeclaration".to_string(),
                found: "other declaration type".to_string(),
            }),
            None => Err(ValidationError::UnknownClass {
                class_name: class_name.to_string(),
            }),
        }
    }

    /// Get a declaration by name
    pub fn get_declaration(&self, class_name: &str) -> Option<&Declaration> {
        self.type_registry.get(class_name)
    }

    /// Get the underlying model
    pub fn get_model(&self) -> &Model {
        &self.model
    }

    /// Get all declarations
    pub fn get_declarations(&self) -> &HashMap<String, Declaration> {
        &self.type_registry
    }
}
