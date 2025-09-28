use thiserror::Error;

pub type ValidationResult<T> = Result<T, ValidationError>;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Validation failed: {message}")]
    ValidationFailed { message: String },

    #[error("Type mismatch: expected {expected}, found {found}")]
    TypeMismatch { expected: String, found: String },

    #[error("Type mismatch: expected {expected}")]
    UnexpectedType { expected: String },
    
    #[error("Missing required property: {property}")]
    MissingRequiredProperty { property: String },

    #[error("Invalid property value: {property} = {value}")]
    InvalidPropertyValue { property: String, value: String },

    #[error("Unknown class: {class_name}")]
    UnknownClass { class_name: String },

    #[error("Unknown property: {property_name} for type {type_name}")]
    UnknownProperty { property_name: String, type_name: String },

    #[error("Metamodel loading error: {message}")]
    MetamodelError { message: String },

    #[error("Unknown error")]
    UnknownError,

    #[error("Generic: {message}")]
    Generic { message: String },
}
