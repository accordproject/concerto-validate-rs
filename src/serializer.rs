use crate::error::{ValidationError, ValidationResult};
use crate::metamodel::{Model, Declaration, Property, TypeIdentifier};
use serde_json::{Value, Map};
use std::collections::HashMap;

/// Serializer that handles validation during JSON deserialization
/// This is the Rust equivalent of the JavaScript Serializer class
pub struct Serializer {
    type_registry: HashMap<String, Declaration>,
}

impl Serializer {
    /// Create a new serializer with the Concerto metamodel
    pub fn new() -> ValidationResult<Self> {
        let metamodel = Model::load_concerto_metamodel()?;
        let type_registry = metamodel.create_type_registry()
            .into_iter()
            .map(|(k, v)| (k, v.clone()))
            .collect();
        
        Ok(Self {
            type_registry,
        })
    }

    /// Main validation method - equivalent to fromJSON in JavaScript
    /// This populates and validates the JSON AST against the metamodel
    pub fn from_json(&self, json: &str) -> ValidationResult<Value> {
        let value: Value = serde_json::from_str(json)?;
        self.validate_value(&value)?;
        Ok(value)
    }

    /// Validate a JSON value against the metamodel
    fn validate_value(&self, value: &Value) -> ValidationResult<()> {
        match value {
            Value::Object(obj) => self.validate_object(obj),
            Value::Array(arr) => {
                for item in arr {
                    self.validate_value(item)?;
                }
                Ok(())
            }
            _ => Ok(()), // Primitive values are generally valid
        }
    }

    /// Validate a JSON object
    fn validate_object(&self, obj: &Map<String, Value>) -> ValidationResult<()> {
        // Check if object has a $class property
        let class_name = obj.get("$class")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "$class".to_string(),
            })?;

        // Find the declaration for this class
        // For the metamodel itself, we need to handle classes that are part of the concerto.metamodel namespace
        let declaration = if class_name.starts_with("concerto.metamodel@") {
            // For metamodel classes, we validate structure directly without looking them up
            return self.validate_metamodel_object(obj, class_name);
        } else {
            self.type_registry.get(class_name)
                .ok_or_else(|| ValidationError::UnknownClass {
                    class_name: class_name.to_string(),
                })?
        };

        // Validate properties based on the declaration
        self.validate_properties(obj, declaration)?;

        // Recursively validate nested objects
        for (key, value) in obj {
            if key != "$class" {
                self.validate_value(value)?;
            }
        }

        Ok(())
    }

    /// Validate properties of an object against its declaration
    fn validate_properties(&self, obj: &Map<String, Value>, declaration: &Declaration) -> ValidationResult<()> {
        let properties = match declaration {
            Declaration::ConceptDeclaration { properties, .. } |
            Declaration::AssetDeclaration { properties, .. } |
            Declaration::ParticipantDeclaration { properties, .. } |
            Declaration::TransactionDeclaration { properties, .. } |
            Declaration::EventDeclaration { properties, .. } => properties,
            Declaration::EnumDeclaration { .. } => {
                // Enum validation is simpler - just check that the value is valid
                return self.validate_enum_value(obj, declaration);
            }
            Declaration::MapDeclaration { .. } => {
                // Map validation
                return self.validate_map_value(obj, declaration);
            }
            Declaration::ScalarDeclaration { .. } => {
                // Scalar validation
                return self.validate_scalar_value(obj, declaration);
            }
        };

        for property in properties {
            self.validate_property(obj, property)?;
        }

        Ok(())
    }

    /// Validate a single property
    fn validate_property(&self, obj: &Map<String, Value>, property: &Property) -> ValidationResult<()> {
        let property_name = self.get_property_name(property);
        let is_optional = self.is_property_optional(property);
        let is_array = self.is_property_array(property);

        let value = obj.get(property_name);

        // Check if required property is missing
        if !is_optional && value.is_none() {
            return Err(ValidationError::MissingProperty {
                property: property_name.to_string(),
            });
        }

        if let Some(value) = value {
            // Validate array vs non-array
            if is_array && !value.is_array() {
                return Err(ValidationError::TypeMismatch {
                    expected: "array".to_string(),
                    found: self.get_value_type_name(value),
                });
            }

            if !is_array && value.is_array() {
                return Err(ValidationError::TypeMismatch {
                    expected: "non-array".to_string(),
                    found: "array".to_string(),
                });
            }

            // Validate the property type
            self.validate_property_type(value, property)?;
        }

        Ok(())
    }

    /// Validate the type of a property value
    fn validate_property_type(&self, value: &Value, property: &Property) -> ValidationResult<()> {
        match property {
            Property::StringProperty { .. } => {
                if value.is_array() {
                    for item in value.as_array().unwrap() {
                        if !item.is_string() {
                            return Err(ValidationError::TypeMismatch {
                                expected: "string".to_string(),
                                found: self.get_value_type_name(item),
                            });
                        }
                    }
                } else if !value.is_string() {
                    return Err(ValidationError::TypeMismatch {
                        expected: "string".to_string(),
                        found: self.get_value_type_name(value),
                    });
                }
            }
            Property::IntegerProperty { .. } | Property::LongProperty { .. } => {
                if value.is_array() {
                    for item in value.as_array().unwrap() {
                        if !item.is_i64() && !item.is_u64() {
                            return Err(ValidationError::TypeMismatch {
                                expected: "integer".to_string(),
                                found: self.get_value_type_name(item),
                            });
                        }
                    }
                } else if !value.is_i64() && !value.is_u64() {
                    return Err(ValidationError::TypeMismatch {
                        expected: "integer".to_string(),
                        found: self.get_value_type_name(value),
                    });
                }
            }
            Property::DoubleProperty { .. } => {
                if value.is_array() {
                    for item in value.as_array().unwrap() {
                        if !item.is_f64() && !item.is_i64() && !item.is_u64() {
                            return Err(ValidationError::TypeMismatch {
                                expected: "number".to_string(),
                                found: self.get_value_type_name(item),
                            });
                        }
                    }
                } else if !value.is_f64() && !value.is_i64() && !value.is_u64() {
                    return Err(ValidationError::TypeMismatch {
                        expected: "number".to_string(),
                        found: self.get_value_type_name(value),
                    });
                }
            }
            Property::BooleanProperty { .. } => {
                if value.is_array() {
                    for item in value.as_array().unwrap() {
                        if !item.is_boolean() {
                            return Err(ValidationError::TypeMismatch {
                                expected: "boolean".to_string(),
                                found: self.get_value_type_name(item),
                            });
                        }
                    }
                } else if !value.is_boolean() {
                    return Err(ValidationError::TypeMismatch {
                        expected: "boolean".to_string(),
                        found: self.get_value_type_name(value),
                    });
                }
            }
            Property::DateTimeProperty { .. } => {
                // DateTime should be represented as string in ISO format
                if value.is_array() {
                    for item in value.as_array().unwrap() {
                        if !item.is_string() {
                            return Err(ValidationError::TypeMismatch {
                                expected: "datetime string".to_string(),
                                found: self.get_value_type_name(item),
                            });
                        }
                    }
                } else if !value.is_string() {
                    return Err(ValidationError::TypeMismatch {
                        expected: "datetime string".to_string(),
                        found: self.get_value_type_name(value),
                    });
                }
            }
            Property::ObjectProperty { object_type, .. } => {
                // Validate that the object has the correct $class
                if value.is_array() {
                    for item in value.as_array().unwrap() {
                        self.validate_object_type(item, object_type)?;
                    }
                } else {
                    self.validate_object_type(value, object_type)?;
                }
            }
        }

        Ok(())
    }

    /// Validate that an object has the correct type
    fn validate_object_type(&self, value: &Value, expected_type: &TypeIdentifier) -> ValidationResult<()> {
        if !value.is_object() {
            return Err(ValidationError::TypeMismatch {
                expected: "object".to_string(),
                found: self.get_value_type_name(value),
            });
        }

        let obj = value.as_object().unwrap();
        let class_name = obj.get("$class")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingProperty {
                property: "$class".to_string(),
            })?;

        let expected_class = if let Some(namespace) = &expected_type.namespace {
            format!("{}.{}", namespace, expected_type.name)
        } else {
            expected_type.name.clone()
        };

        if class_name != expected_class {
            return Err(ValidationError::TypeMismatch {
                expected: expected_class,
                found: class_name.to_string(),
            });
        }

        Ok(())
    }

    /// Validate enum values
    fn validate_enum_value(&self, _obj: &Map<String, Value>, _declaration: &Declaration) -> ValidationResult<()> {
        // TODO: Implement enum validation
        Ok(())
    }

    /// Validate map values
    fn validate_map_value(&self, _obj: &Map<String, Value>, _declaration: &Declaration) -> ValidationResult<()> {
        // TODO: Implement map validation
        Ok(())
    }

    /// Validate scalar values
    fn validate_scalar_value(&self, _obj: &Map<String, Value>, _declaration: &Declaration) -> ValidationResult<()> {
        // TODO: Implement scalar validation
        Ok(())
    }

    /// Validate metamodel objects (objects that are part of the concerto.metamodel namespace)
    fn validate_metamodel_object(&self, obj: &Map<String, Value>, class_name: &str) -> ValidationResult<()> {
        // Basic validation for metamodel objects - they should at least have the required structure
        match class_name {
            "concerto.metamodel@1.0.0.Model" => {
                // Validate Model structure
                if !obj.contains_key("namespace") {
                    return Err(ValidationError::MissingProperty {
                        property: "namespace".to_string(),
                    });
                }
                if !obj.contains_key("declarations") {
                    return Err(ValidationError::MissingProperty {
                        property: "declarations".to_string(),
                    });
                }
                if !obj.contains_key("imports") {
                    return Err(ValidationError::MissingProperty {
                        property: "imports".to_string(),
                    });
                }
            }
            "concerto.metamodel@1.0.0.ConceptDeclaration" |
            "concerto.metamodel@1.0.0.AssetDeclaration" |
            "concerto.metamodel@1.0.0.ParticipantDeclaration" |
            "concerto.metamodel@1.0.0.TransactionDeclaration" |
            "concerto.metamodel@1.0.0.EventDeclaration" => {
                // Validate basic declaration structure
                if !obj.contains_key("name") {
                    return Err(ValidationError::MissingProperty {
                        property: "name".to_string(),
                    });
                }
                if !obj.contains_key("properties") {
                    return Err(ValidationError::MissingProperty {
                        property: "properties".to_string(),
                    });
                }
            }
            "concerto.metamodel@1.0.0.StringProperty" |
            "concerto.metamodel@1.0.0.IntegerProperty" |
            "concerto.metamodel@1.0.0.LongProperty" |
            "concerto.metamodel@1.0.0.DoubleProperty" |
            "concerto.metamodel@1.0.0.BooleanProperty" |
            "concerto.metamodel@1.0.0.DateTimeProperty" |
            "concerto.metamodel@1.0.0.ObjectProperty" => {
                // Validate property structure
                if !obj.contains_key("name") {
                    return Err(ValidationError::MissingProperty {
                        property: "name".to_string(),
                    });
                }
                if !obj.contains_key("isArray") {
                    return Err(ValidationError::MissingProperty {
                        property: "isArray".to_string(),
                    });
                }
                if !obj.contains_key("isOptional") {
                    return Err(ValidationError::MissingProperty {
                        property: "isOptional".to_string(),
                    });
                }
            }
            "concerto.metamodel@1.0.0.TypeIdentifier" => {
                // Validate TypeIdentifier structure
                if !obj.contains_key("name") {
                    return Err(ValidationError::MissingProperty {
                        property: "name".to_string(),
                    });
                }
            }
            "concerto.metamodel@1.0.0.Decorator" => {
                // Validate Decorator structure
                if !obj.contains_key("name") {
                    return Err(ValidationError::MissingProperty {
                        property: "name".to_string(),
                    });
                }
            }
            "concerto.metamodel@1.0.0.DecoratorString" => {
                // Validate DecoratorString structure
                if !obj.contains_key("value") {
                    return Err(ValidationError::MissingProperty {
                        property: "value".to_string(),
                    });
                }
            }
            "concerto.metamodel@1.0.0.StringRegexValidator" => {
                // Validate StringRegexValidator structure
                if !obj.contains_key("pattern") {
                    return Err(ValidationError::MissingProperty {
                        property: "pattern".to_string(),
                    });
                }
                if !obj.contains_key("flags") {
                    return Err(ValidationError::MissingProperty {
                        property: "flags".to_string(),
                    });
                }
            }
            _ => {
                // For other metamodel classes, perform basic validation
                // We accept them if they have the $class property
            }
        }

        // Recursively validate nested objects
        for (key, value) in obj {
            if key != "$class" {
                self.validate_value(value)?;
            }
        }

        Ok(())
    }

    /// Get property name from Property enum
    fn get_property_name<'a>(&self, property: &'a Property) -> &'a str {
        match property {
            Property::StringProperty { name, .. } |
            Property::IntegerProperty { name, .. } |
            Property::LongProperty { name, .. } |
            Property::DoubleProperty { name, .. } |
            Property::BooleanProperty { name, .. } |
            Property::DateTimeProperty { name, .. } |
            Property::ObjectProperty { name, .. } => name,
        }
    }

    /// Check if property is optional
    fn is_property_optional(&self, property: &Property) -> bool {
        match property {
            Property::StringProperty { is_optional, .. } |
            Property::IntegerProperty { is_optional, .. } |
            Property::LongProperty { is_optional, .. } |
            Property::DoubleProperty { is_optional, .. } |
            Property::BooleanProperty { is_optional, .. } |
            Property::DateTimeProperty { is_optional, .. } |
            Property::ObjectProperty { is_optional, .. } => *is_optional,
        }
    }

    /// Check if property is an array
    fn is_property_array(&self, property: &Property) -> bool {
        match property {
            Property::StringProperty { is_array, .. } |
            Property::IntegerProperty { is_array, .. } |
            Property::LongProperty { is_array, .. } |
            Property::DoubleProperty { is_array, .. } |
            Property::BooleanProperty { is_array, .. } |
            Property::DateTimeProperty { is_array, .. } |
            Property::ObjectProperty { is_array, .. } => *is_array,
        }
    }

    /// Get a human-readable type name for a JSON value
    fn get_value_type_name(&self, value: &Value) -> String {
        match value {
            Value::Null => "null".to_string(),
            Value::Bool(_) => "boolean".to_string(),
            Value::Number(n) => {
                if n.is_i64() || n.is_u64() {
                    "integer".to_string()
                } else {
                    "number".to_string()
                }
            }
            Value::String(_) => "string".to_string(),
            Value::Array(_) => "array".to_string(),
            Value::Object(_) => "object".to_string(),
        }
    }
}

/// JSON Populator equivalent - handles the actual population of objects from JSON
/// This is embedded within the Serializer in our Rust implementation
pub struct JsonPopulator {
    serializer: Serializer,
}

impl JsonPopulator {
    pub fn new() -> ValidationResult<Self> {
        Ok(Self {
            serializer: Serializer::new()?,
        })
    }

    /// Populate an object from JSON with validation
    pub fn populate(&self, json: &str) -> ValidationResult<Value> {
        self.serializer.from_json(json)
    }
}
