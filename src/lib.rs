//! Concerto Validator RS
//!
//! A Rust library that validates Accord Project Concerto data models in their JSON AST format.
//! This library replicates the validation logic from the JavaScript implementation.

pub mod error;
pub mod metamodel_manager;
pub mod validator;

pub use error::{ValidationError, ValidationResult};
pub use validator::Validator;

/// Validates a Concerto model JSON AST against the metamodel
pub fn validate_metamodel(json_ast: &str) -> ValidationResult<()> {
    let validator = Validator::new()?;
    validator.validate(json_ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_metamodel_validation() {
        let metamodel_json = include_str!("../metamodel.json");
        let result = validate_metamodel(metamodel_json);
        assert!(
            result.is_ok(),
            "Metamodel validation should succeed: {:?}",
            result
        );
    }

    #[test]
    fn test_invalid_json() {
        let invalid_json = r#"{ invalid json }"#;
        let result = validate_metamodel(invalid_json);
        assert!(result.is_err(), "Invalid JSON should fail validation");
    }

    #[test]
    fn test_invalid_namespace() {
        let invalid_json = r#"{ 
            "$class": "concerto.metamodel@1.0.0.Model",
            "namespace": 123
        }"#;
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
        assert!(
            result.is_err(),
            "JSON without $class should fail validation"
        );
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
        assert!(
            result.is_ok(),
            "Simple valid model should pass validation: {:?}",
            result
        );
    }
}
