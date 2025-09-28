mod type_definition;

mod ast_structures;

use std::collections::{HashMap, HashSet};

use serde_json::{Map, Value};
use regex::Regex;

use crate::error::ValidationError;
use crate::model_manager::ast_structures::{ConceptDeclaration, Property, SuperType};
use crate::model_manager::type_definition::TypeDefinition;

type JsonObject = Map<String, Value>;
type TypeRegistry = HashMap<String, TypeDefinition>;

const CONCERTO_METAMODEL_NAMESPACE: &str = "concerto.metamodel@1.0.0";

/// Loads the system definitions and validates
/// given resource.
/// Currently, there isn't a way to add more metamodels.
pub(crate) struct ModelManager {
    /// Internal look up for all the loaded type definitions.
    /// See [`TypeDefinition`](crate::model_manager::type_definition::TypeDefinition).
    type_registry: TypeRegistry,
    /// Internal look up for string validator regexes.
    /// Regexes are pre-compiled at creation time.
    /// See [`Regex`](regex::Regex).
    regex_cache: HashMap<String, Regex>,
}

/// Public API
impl<'model_manager> ModelManager {
    /// Create a new `ModelManager`.
    pub fn new() -> Result<Self, ValidationError> {
        // Embed the Concerto metamodel from the downloaded JSON file
        let metamodel_json = include_str!("../../metamodel.json");
        let concerto_metamodel: Value = serde_json::from_str(metamodel_json)?;

        let type_registry = Self::build_type_registry(&concerto_metamodel)?;
        let regex_cache = Self::build_regex_cache(&type_registry);

        Ok(Self { type_registry, regex_cache })
    }

    /// Validate a Concerto AST.
    pub fn validate_metamodel(&self, thing: &'model_manager Value) -> Result<(), ValidationError> {
        let obj = self.get_serialized_object(thing)?;
        self.validate_resource(obj)
    }
}

/// Internal validation functions.
impl<'model_manager> ModelManager {
    // Validates a resource
    fn validate_resource(&self, thing: &'model_manager JsonObject) -> Result<(), ValidationError> {
        let class_name = self.get_class_name(thing)?;

        let type_def = self.get_type_definition(class_name)?;

        let mut expected_properties = type_def.expected_properties();
        let mut required_properties = type_def.required_properties();

        if type_def.has_supertype() {
            let super_type = type_def.get_supertype().ok_or(ValidationError::MissingSuperTypeDefinition {
                name: class_name.to_string(),
            })?;
            let super_type_definition = self.get_supertype_definition(super_type)?;
            super_type_definition.expected_properties().iter().for_each(|(k, v)| {
               expected_properties.insert(k.to_string(), v);
            });
            super_type_definition.required_properties().iter().for_each(|(k, v)| {
                required_properties.insert(k.to_string(), v);
            });
        }

        self.validate_expected_properties(thing, &expected_properties)?;
        self.validate_required_properties(thing, &required_properties)?;
        self.validate_property_structure(thing, &expected_properties)?;
        Ok(())
    }

    fn validate_expected_properties(&self, thing: &'model_manager JsonObject, expected_properties: &HashMap<String, &Property>) -> Result<(), ValidationError> {
        let invalid_properties = thing
            .keys().filter(|&x| !expected_properties.contains_key(x) && x != "$class").cloned()
            .collect::<Vec<String>>();

        if !invalid_properties.is_empty() {
            return Err(ValidationError::UnknownProperty {
                property_name: invalid_properties.first().unwrap().clone(),
            });
        }
        Ok(())
    }

    fn validate_required_properties(&self, thing: &'model_manager JsonObject, required_properties: &HashMap<String, &Property>) -> Result<(), ValidationError> {

        let existing_properties = thing.keys().cloned()
            .collect::<HashSet<String>>();

        let missing_properties = required_properties.keys()
            .filter(|x| !existing_properties.contains(x.as_str())).cloned()
            .collect::<Vec<String>>();

        if !required_properties.is_empty() && !missing_properties.is_empty()
        {
            return Err(ValidationError::MissingRequiredProperty {
                property: missing_properties.first().unwrap().clone(),
            });
        }
        Ok(())
    }

    fn validate_property_structure(&self, thing: &'model_manager JsonObject, properties: &HashMap<String, &Property>) -> Result<(), ValidationError> {
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

                            if !validation_errors.is_empty() {
                                match validation_errors.first().unwrap() {
                                    Err(e) => Err(ValidationError::Generic {
                                        message: e.to_string(),
                                    }),
                                    _ => unreachable!()
                                }
                            } else {
                                Ok(())
                            }
                        } else {
                            Err(ValidationError::Generic {
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

        if !validation_errors.is_empty() {
            let first_err = validation_errors.first();
            return Err(ValidationError::Generic {
                message: format!("{:?}", first_err),
            });
        };

        Ok(())
    }
}

/// Functions related to property validations.
impl<'model_manager> ModelManager {
    fn validate_property(
        &self,
        type_def: &Property,
        thing: &'model_manager Value,
    ) -> Result<(), ValidationError> {
        match type_def.class.as_str() {
            "concerto.metamodel@1.0.0.ObjectProperty" => {
                self.validate_object_property(thing)
            },
            "concerto.metamodel@1.0.0.StringProperty" => self.validate_string_property(thing, type_def),
            "concerto.metamodel@1.0.0.BooleanProperty" => self.validate_boolean_property(thing),
            "concerto.metamodel@1.0.0.DoubleProperty" => self.validate_double_property(thing),
            "concerto.metamodel@1.0.0.IntegerProperty" => self.validate_integer_property(thing),
            _ => Err(ValidationError::ValidationFailed {
                message: "Unknown property type".to_string(),
            }),
        }
    }

    fn validate_string_property(&self, thing: &'model_manager Value, type_def: &Property) -> Result<(), ValidationError> {
        let str = thing.as_str().ok_or(ValidationError::UnexpectedType {
            expected: "String".to_string(),
        })?;
        if let Some(validator) = &type_def.validator {
            let pattern = &validator.pattern;
            let re = self.regex_cache.get(pattern).ok_or( ValidationError::StringValidationError {
                message: format!("Cannot compile pattern {}", pattern)
            })?;
            if !re.is_match(str) {
                return Err(ValidationError::StringValidationError {
                    message: format!("Invalid string property: {}", str)
                })
            }
        }
        Ok(())
    }

    fn validate_boolean_property(&self, thing: &'model_manager Value) -> Result<(), ValidationError> {
        thing
            .as_bool()
            .ok_or(ValidationError::UnexpectedType {
                expected: "Boolean".to_string(),
            }).map(|_| ())
    }

    fn validate_integer_property(&self, thing: &'model_manager Value) -> Result<(), ValidationError> {
        thing
            .as_i64()
            .ok_or(ValidationError::UnexpectedType {
                expected: "Integer".to_string(),
            }).map(|_| ())
    }

    fn validate_double_property(&self, thing: &'model_manager Value) -> Result<(), ValidationError> {
        thing
            .as_f64()
            .ok_or(ValidationError::UnexpectedType {
                expected: "Double".to_string(),
            }).map(|_| ())
    }

    fn validate_object_property(&self, thing: &'model_manager Value) -> Result<(), ValidationError> {
        let obj = self.get_serialized_object(thing)?;
        self.validate_resource(obj)?;
        Ok(())
    }
}

/// Ancillary functions that still needs to be part of `ModelManager`.
impl<'model_manager> ModelManager {
    fn build_type_registry(
        metamodel: &'model_manager Value,
    ) -> Result<TypeRegistry, ValidationError> {
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

    fn build_regex_cache(type_registry: &TypeRegistry) -> HashMap<String, Regex> {
        let mut cache = HashMap::<String, Regex>::new();
        type_registry.values().for_each(|type_def| {

            type_def.get_string_validator_patterns().iter().for_each(|pattern| {
                let pattern = &pattern;
                if let Ok(re) = Regex::new(pattern) {
                    cache.insert(pattern.to_string(), re);
                }
            })
        });

        cache
    }

    fn get_type_definition(&self, full_name: &str) -> Result<&TypeDefinition, ValidationError> {
        self.type_registry
            .get(full_name)
            .ok_or(ValidationError::Generic {
                message: format!("Error getting type def {}", full_name),
            })
    }

    fn get_supertype_definition(&self, super_type: &SuperType) -> Result<&TypeDefinition, ValidationError> {
        let class_name = if let Some(ns) = &super_type.namespace {
            format!{"{}.{}", ns, super_type.name}
        } else {
            format!{"{}.{}", CONCERTO_METAMODEL_NAMESPACE, super_type.name}
        };
        self.get_type_definition(&class_name)
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

