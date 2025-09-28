mod type_definition;

use crate::error::ValidationError;
use type_definition::{ConceptDeclaration, Property, TypeDefinition};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use serde_json::Map;

type JsonObject = Map<String, Value>;

const CONCERTO_METAMODEL_NAMESPACE: &str = "concerto.metamodel@1.0.0";

/// MetamodelManager loads the system definitions and validates
/// Given Concerto AST
pub struct MetamodelManager {
    /// Registry of type declarations from the metamodel
    type_registry: HashMap<String, TypeDefinition>,
}

// Public API
impl<'model_manager> MetamodelManager {
    pub fn new() -> Result<Self, ValidationError> {
        // Embed the Concerto metamodel from the downloaded JSON file
        let metamodel_json = include_str!("../../metamodel.json");
        let concerto_metamodel: Value = serde_json::from_str(metamodel_json)?;

        let type_registry = Self::build_type_registry(&concerto_metamodel)?;

        Ok(Self { type_registry })
    }

    pub fn validate_metamodel(&self, thing: &'model_manager Value) -> Result<(), ValidationError> {
        let obj = self.get_serialized_object(thing)?;
        self.validate_resource(obj)
    }
}

// Validate resource
impl<'model_manager> MetamodelManager {
    // Validates a resource
    fn validate_resource(&self, thing: &'model_manager JsonObject) -> Result<(), ValidationError> {
        let class_name = self.get_class_name(thing)?;

        let type_def = self.get_type_definition(&class_name)?;

        self.validate_expected_properties(thing, type_def)?;
        self.validate_required_properties(thing, type_def)?;
        self.validate_property_structure(thing, type_def)?;
        Ok(())
    }

    fn validate_expected_properties(&self, thing: &'model_manager JsonObject, type_def: &TypeDefinition) -> Result<(), ValidationError> {
        let expected_properties = type_def.expected_properties();

        let invalid_properties = thing
            .keys()
            .map(|x| x.clone())
            .filter(|x| !expected_properties.contains_key(x) && x != "$class")
            .collect::<Vec<String>>();

        if invalid_properties.len() > 0 {
            return Err(ValidationError::UnknownProperty {
                property_name: invalid_properties.get(0).unwrap().clone(),
                type_name: type_def.class_name(),
            });
        }
        Ok(())
    }

    fn validate_required_properties(&self, thing: &'model_manager JsonObject, type_def: &TypeDefinition) -> Result<(), ValidationError> {
        let required_properties = type_def.required_properties();

        let existing_properties = thing.keys()
            .map(|x| x.clone())
            .collect::<HashSet<String>>();

        let missing_properties = required_properties.keys()
            .filter(|x| !existing_properties.contains(x.as_str()))
            .map(|x| x.clone())
            .collect::<Vec<String>>();

        if required_properties.len() > 0 && missing_properties.len() > 0
        {
            return Err(ValidationError::MissingRequiredProperty {
                property: missing_properties.get(0).unwrap().clone(),
            });
        }
        Ok(())
    }

    fn validate_property_structure(&self, thing: &'model_manager JsonObject, type_def: &TypeDefinition) -> Result<(), ValidationError> {
        let properties = type_def.expected_properties();
        let validations = thing
            .iter()
            .map(|(prop_name, prop_value)| {
                if prop_name == "$class" {
                    return Ok(());
                }
                if let Some(property_type) = properties.get(prop_name) {
                    if property_type.is_array {
                        if prop_value.is_array() {
                            let validation_errors = prop_value.as_array().unwrap()
                                .iter()
                                .map(|x| {
                                    self.validate_property(property_type, x)
                                })
                                .filter(|x| x.is_err())
                                .collect::<Vec<Result<(), ValidationError>>>();

                            if validation_errors.len() > 0 {
                                match validation_errors.get(0).unwrap() {
                                    Err(e) => Err(ValidationError::Generic {
                                        message: e.to_string(),
                                    }),
                                    _ => unreachable!()
                                }
                            } else {
                                Ok(())
                            }
                        } else {
                            return Err(ValidationError::Generic {
                                message: format!("Error validating property {:}. Expected an array.", prop_name),
                            })
                        }

                    } else {
                        self.validate_property(property_type, prop_value)
                    }
                } else {
                    Err(ValidationError::Generic {
                        message: format!("Error validating property {:}", prop_name),
                    })
                }
            })
            .collect::<Vec<_>>();

        let validation_errors = validations
            .iter()
            .filter(|x| x.is_err())
            .collect::<Vec<_>>();

        if validation_errors.len() > 0 {
            let first_err = validation_errors.get(0);
            return Err(ValidationError::Generic {
                message: format!("{:?}", first_err),
            });
        };

        Ok(())
    }
}

// Validate property
impl<'model_manager> MetamodelManager {
    fn validate_property(
        &self,
        type_def: &Property,
        thing: &'model_manager Value,
    ) -> Result<(), ValidationError> {
        match type_def.class.as_str() {
            "concerto.metamodel@1.0.0.ObjectProperty" => {
                self.validate_object_property(thing)
            },
            "concerto.metamodel@1.0.0.StringProperty" => self.validate_string_property(thing),
            "concerto.metamodel@1.0.0.BooleanProperty" => self.validate_boolean_property(thing),
            "concerto.metamodel@1.0.0.DoubleProperty" => self.validate_double_property(thing),
            "concerto.metamodel@1.0.0.IntegerProperty" => self.validate_integer_property(thing),
            _ => Err(ValidationError::ValidationFailed {
                message: "Unknown property type".to_string(),
            }),
        }
    }

    fn validate_string_property(&self, thing: &'model_manager Value) -> Result<(), ValidationError> {
        thing.as_str().ok_or(ValidationError::UnexpectedType {
            expected: "String".to_string(),
        })?;
        Ok(())
    }

    fn validate_boolean_property(&self, thing: &'model_manager Value) -> Result<(), ValidationError> {
        thing
            .as_bool()
            .ok_or(ValidationError::UnexpectedType {
                expected: "Boolean".to_string(),
            })
            .and_then(|_| Ok(()))
    }

    fn validate_integer_property(&self, thing: &'model_manager Value) -> Result<(), ValidationError> {
        thing
            .as_i64()
            .ok_or(ValidationError::UnexpectedType {
                expected: "Integer".to_string(),
            })
            .and_then(|_| Ok(()))
    }

    fn validate_double_property(&self, thing: &'model_manager Value) -> Result<(), ValidationError> {
        thing
            .as_f64()
            .ok_or(ValidationError::UnexpectedType {
                expected: "Double".to_string(),
            })
            .and_then(|_| Ok(()))
    }

    fn validate_object_property(&self, thing: &'model_manager Value) -> Result<(), ValidationError> {
        let obj = self.get_serialized_object(thing)?;
        self.validate_resource(obj)?;
        Ok(())
    }
}

// Ancillary implementations
impl<'model_manager> MetamodelManager {
    fn build_type_registry(
        metamodel: &'model_manager Value,
    ) -> Result<HashMap<String, TypeDefinition>, ValidationError> {
        let declarations = metamodel
            .get("declarations")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ValidationError::MetamodelError {
                message: "Missing declarations in in system AST".to_string(),
            })?;

        let namespace = metamodel
            .get("namespace")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MetamodelError {
                message: "Missing namespace in system AST".to_string(),
            })?;

        let parsed_definitions = declarations
            .iter()
            .map(|declaration| {
                serde_json::from_value::<ConceptDeclaration>(declaration.clone()).map_err(|_| {
                    ValidationError::Generic {
                        message: "Error serializing system AST to ConceptDeclaration".to_string(),
                    }
                })
            })
            .collect::<Vec<Result<ConceptDeclaration, ValidationError>>>();

        if parsed_definitions.iter().any(|x| x.is_err()) {
            return Err(ValidationError::Generic {
                message: "Error parsing type definitions from system AST".to_string(),
            });
        }

        let type_map = parsed_definitions
            .iter()
            .map(|def| def.as_ref().ok().unwrap())
            .map(|def| (format!("{}.{}", namespace, def.name), TypeDefinition::new(def.clone())))
            .collect::<HashMap<String, TypeDefinition>>();

        Ok(type_map)
    }

    fn get_type_definition(&self, full_name: &str) -> Result<&TypeDefinition, ValidationError> {
        self.type_registry
            .get(full_name)
            .ok_or(ValidationError::Generic {
                message: format!("Error getting type def {}", full_name),
            })
    }

    fn get_class_name(&self, thing: &'model_manager JsonObject) -> Result<&'model_manager str, ValidationError> {
        thing.get("$class").ok_or(ValidationError::MissingRequiredProperty {
            property: "$class".to_string(),
        })?.as_str().ok_or(ValidationError::Generic {
            message: "Cannot convert $class to string".to_string(),
        })
    }

    fn get_serialized_object(&self, thing: &'model_manager Value) -> Result<&'model_manager JsonObject, ValidationError> {
        thing
            .as_object()
            .ok_or_else(|| ValidationError::TypeMismatch {
                expected: "object".to_string(),
                found: "non-object".to_string(),
            })
    }
}

