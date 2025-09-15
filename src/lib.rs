//! Concerto Validator RS
//! 
//! A Rust library that validates Accord Project Concerto data models in their JSON AST format.
//! This library replicates the validation logic from the JavaScript implementation.

pub mod metamodel;
pub mod validator;
pub mod factory;
pub mod serializer;
pub mod error;

pub use validator::ConcertoValidator;
pub use error::{ValidationError, ValidationResult};

/// Validates a Concerto model JSON AST against the metamodel
pub fn validate_metamodel(json_ast: &str) -> ValidationResult<()> {
    let validator = ConcertoValidator::new()?;
    validator.validate(json_ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_metamodel_validation() {
        // Test with the actual Concerto metamodel
        let metamodel_json = include_str!("../metamodel.json");
        let result = validate_metamodel(metamodel_json);
        assert!(result.is_ok(), "Metamodel validation should succeed: {:?}", result);
    }

    #[test]
    fn test_invalid_json() {
        let invalid_json = r#"{ invalid json }"#;
        let result = validate_metamodel(invalid_json);
        assert!(result.is_err(), "Invalid JSON should fail validation");
    }

    #[test]
    fn test_missing_class_property() {
        let json_without_class = r#"{
            "namespace": "test.namespace",
            "imports": [],
            "declarations": []
        }"#;
        let result = validate_metamodel(json_without_class);
        assert!(result.is_err(), "JSON without $class should fail validation");
    }

    #[test]
    fn test_simple_model_validation() {
        let simple_model = r#"{
            "$class": "concerto.metamodel@1.0.0.Model",
            "namespace": "test.namespace@1.0.0",
            "imports": [],
            "declarations": [
                {
                    "$class": "concerto.metamodel@1.0.0.ConceptDeclaration",
                    "name": "TestConcept",
                    "isAbstract": false,
                    "properties": [
                        {
                            "$class": "concerto.metamodel@1.0.0.StringProperty",
                            "name": "testField",
                            "isArray": false,
                            "isOptional": false
                        }
                    ]
                }
            ]
        }"#;
        
        let result = validate_metamodel(simple_model);
        assert!(result.is_ok(), "Simple valid model should pass validation: {:?}", result);
    }

    #[test]
    fn test_validator_creation() {
        let validator = ConcertoValidator::new();
        assert!(validator.is_ok(), "Validator creation should succeed");
    }

    #[test]
    fn test_factory_creation() {
        use crate::factory::Factory;
        let factory = Factory::new_with_metamodel();
        assert!(factory.is_ok(), "Factory creation should succeed");
    }

    #[test]
    fn test_serializer_creation() {
        use crate::serializer::Serializer;
        let serializer = Serializer::new();
        assert!(serializer.is_ok(), "Serializer creation should succeed");
    }
}
