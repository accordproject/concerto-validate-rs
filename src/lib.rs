//! Concerto Validator RS
//!
//! A Rust library that validates Accord Project Concerto data models in their JSON AST format
//! against the [Concerto Metamodel](https://models.accordproject.org/concerto/metamodel@1.0.0.html)
//! 
//! Currently, the library only exposes a single function to validate Concerto ASTs,
//! although internally there are primitive implementations of structures that would sound familiar
//! to the JS classes, like [`ModelManager`](crate::model_manager::ModelManager). But they are not
//! ready for public consumption yet.

pub mod error;
mod model_manager;
mod validator;

use std::sync::OnceLock;


pub use error::{ValidationError, ValidationResult};
use validator::Validator;

// Reference to hold singleton instance of Validator
static GLOBAL_VALIDATOR: OnceLock<Option<Validator>> = OnceLock::new();

/// Validates a Concerto model JSON AST against the system metamodel
pub fn validate_metamodel(json_ast: &str) -> ValidationResult<()> {
    let validator = GLOBAL_VALIDATOR.get_or_init( ||  {
        Validator::new().ok()
    }).as_ref().ok_or(ValidationError::ValidatorInitializationError)?;
    validator.validate(json_ast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn validate_value(json_ast: Value) -> ValidationResult<()> {
        validate_metamodel(&json_ast.to_string())
    }

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

    #[test]
    fn test_extra_properties() {
        let simple_model = r#"{
            "$class": "concerto.metamodel@1.0.0.Model",
            "namespace": "test.namespace@1.0.0",
            "imports": [],
            "declarations": [
                {
                    "$class": "concerto.metamodel@1.0.0.ConceptDeclaration",
                    "name": "TestConcept",
                    "isAbstract": false,
                    "isOptional": false,
                    "properties": [
                        {
                            "$class": "concerto.metamodel@1.0.0.StringProperty",
                            "name": "testField",
                            "isArray": false,
                            "isOptional": false,
                            "propertyType": "String"
                        }
                    ]
                }
            ]
        }"#;

        let result = validate_metamodel(simple_model);
        assert!(
            result.is_err(),
            "Extra properties should fail validation: {:?}",
            result
        );
    }

    #[test]
    fn test_boolean_property_type_mismatch() {
        let concept_declaration = json!({
            "$class": "concerto.metamodel@1.0.0.ConceptDeclaration",
            "name": "TestConcept",
            "isAbstract": "false",
            "properties": []
        });

        let result = validate_value(concept_declaration);
        let error = result.expect_err("String passed to BooleanProperty should fail validation");
        let error_message = error.to_string();

        assert!(
            error_message.contains("UnexpectedType") && error_message.contains("Boolean"),
            "Boolean type mismatch should mention the expected type: {error_message}"
        );
    }

    #[test]
    fn test_integer_property_type_mismatch() {
        let position = json!({
            "$class": "concerto.metamodel@1.0.0.Position",
            "line": "1",
            "column": 1,
            "offset": 0
        });

        let result = validate_value(position);
        let error = result.expect_err("String passed to IntegerProperty should fail validation");
        let error_message = error.to_string();

        assert!(
            error_message.contains("UnexpectedType") && error_message.contains("Integer"),
            "Integer type mismatch should mention the expected type: {error_message}"
        );
    }

    #[test]
    fn test_array_property_expects_array() {
        let model = json!({
            "$class": "concerto.metamodel@1.0.0.Model",
            "namespace": "test.namespace@1.0.0",
            "declarations": {
                "$class": "concerto.metamodel@1.0.0.ConceptDeclaration",
                "name": "TestConcept",
                "isAbstract": false,
                "properties": []
            }
        });

        let result = validate_value(model);
        let error = result.expect_err("Non-array value for an array property should fail validation");
        let error_message = error.to_string();

        assert!(
            error_message.contains("Expected an array"),
            "Array mismatch should mention the array expectation: {error_message}"
        );
    }

    #[test]
    fn test_optional_property_can_be_missing() {
        let model = json!({
            "$class": "concerto.metamodel@1.0.0.Model",
            "namespace": "test.namespace@1.0.0"
        });

        let result = validate_value(model);

        assert!(
            result.is_ok(),
            "Missing optional properties should not fail validation: {:?}",
            result
        );
    }

    #[test]
    fn test_boolean_property_accepts_boolean() {
        let concept_declaration = json!({
            "$class": "concerto.metamodel@1.0.0.ConceptDeclaration",
            "name": "TestConcept",
            "isAbstract": false,
            "properties": []
        });

        let result = validate_value(concept_declaration);

        assert!(
            result.is_ok(),
            "BooleanProperty should accept boolean values: {:?}",
            result
        );
    }

    #[test]
    fn test_integer_property_accepts_integer() {
        let position = json!({
            "$class": "concerto.metamodel@1.0.0.Position",
            "line": 1,
            "column": 2,
            "offset": 3
        });

        let result = validate_value(position);

        assert!(
            result.is_ok(),
            "IntegerProperty should accept integer values: {:?}",
            result
        );
    }
}
