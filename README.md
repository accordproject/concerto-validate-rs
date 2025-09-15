# Concerto Validator RS

A Rust library that validates Accord Project Concerto data models in their JSON AST (Abstract Syntax Tree) format. This library replicates the validation logic from the JavaScript implementation found in the [Accord Project Concerto repository](https://github.com/accordproject/concerto).

## Overview

This validator implements the core functionality of the JavaScript `validateMetaModel` function by:

1. Loading the raw [Concerto Metamodel JSON](https://github.com/accordproject/concerto-metamodel/blob/main/lib/metamodel.json) directly as the validation schema
2. Using a `MetamodelManager` that works with raw JSON instead of hardcoded Rust structs
3. Using a `Serializer` (equivalent to the JavaScript Serializer class) that validates JSON structure against the metamodel
4. Using a `Factory` (equivalent to the JavaScript Factory class) for semantic validation
5. Providing comprehensive error reporting for validation failures

## Features

- ✅ **Dynamic Metamodel Loading**: Loads metamodel from raw JSON instead of hardcoded structures
- ✅ **Complete Metamodel Support**: Validates against the full Concerto metamodel specification
- ✅ **Structural Validation**: Ensures JSON structure matches expected Concerto AST format
- ✅ **Type Validation**: Validates property types, arrays, optionality, and nested objects
- ✅ **Semantic Validation**: Checks business rules and constraints
- ✅ **Error Reporting**: Provides detailed error messages for debugging
- ✅ **Self-Validation**: Can validate the Concerto metamodel itself
- ✅ **Performance**: Fast validation using native Rust performance

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
concerto-validator-rs = "0.1.0"
```

## Usage

### Basic Usage

```rust
use concerto_validator_rs::validate_metamodel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model_json = r#"{
        "$class": "concerto.metamodel@1.0.0.Model",
        "namespace": "org.example@1.0.0",
        "imports": [],
        "declarations": [
            {
                "$class": "concerto.metamodel@1.0.0.ConceptDeclaration",
                "name": "Person",
                "isAbstract": false,
                "properties": [
                    {
                        "$class": "concerto.metamodel@1.0.0.StringProperty",
                        "name": "firstName",
                        "isArray": false,
                        "isOptional": false
                    }
                ]
            }
        ]
    }"#;

    match validate_metamodel(model_json) {
        Ok(()) => println!("✅ Model is valid!"),
        Err(e) => println!("❌ Validation error: {}", e),
    }

    Ok(())
}
```

### Advanced Usage

```rust
use concerto_validator_rs::{ConcertoValidator, ValidationError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a validator instance
    let validator = ConcertoValidator::new()?;
    
    // Validate a model
    let result = validator.validate(model_json);
    
    // Handle specific error types
    match result {
        Ok(()) => println!("Validation successful"),
        Err(ValidationError::MissingProperty { property }) => {
            println!("Missing required property: {}", property);
        },
        Err(ValidationError::TypeMismatch { expected, found }) => {
            println!("Type mismatch: expected {}, found {}", expected, found);
        },
        Err(e) => println!("Other validation error: {}", e),
    }

    Ok(())
}
```

## Error Types

The library provides detailed error information through the `ValidationError` enum:

- `JsonError`: Invalid JSON syntax
- `IoError`: File system errors
- `ValidationFailed`: General validation failure
- `TypeMismatch`: Type doesn't match expected type
- `MissingProperty`: Required property is missing
- `InvalidPropertyValue`: Property value is invalid
- `UnknownClass`: Referenced class is not defined
- `MetamodelError`: Error loading the metamodel

## Supported Declaration Types

The validator supports all Concerto declaration types:

- **ConceptDeclaration**: Business concepts
- **AssetDeclaration**: Asset definitions
- **ParticipantDeclaration**: Participant definitions
- **TransactionDeclaration**: Transaction definitions
- **EventDeclaration**: Event definitions
- **EnumDeclaration**: Enumeration definitions
- **MapDeclaration**: Map/dictionary definitions
- **ScalarDeclaration**: Scalar type definitions

## Supported Property Types

- **StringProperty**: String values with optional validation
- **IntegerProperty**: Integer values with optional range validation
- **LongProperty**: Long integer values
- **DoubleProperty**: Floating-point values
- **BooleanProperty**: Boolean values
- **DateTimeProperty**: Date/time values
- **ObjectProperty**: References to other concepts

## Architecture

The library follows the same architectural patterns as the JavaScript implementation:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Validator     │────│   Serializer    │────│ MetamodelManager│
│                 │    │                 │    │                 │
│ Main validation │    │ JSON validation │    │ Raw JSON schema │
│ orchestration   │    │ orchestration   │    │ validation      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │
         │              ┌─────────────────┐
         └──────────────│     Factory     │
                        │                 │
                        │ Semantic        │
                        │ validation      │
                        └─────────────────┘
```

## Examples

Run the included examples:

```bash
cargo run --example basic_usage
```

## Testing

Run the test suite:

```bash
cargo test
```

The tests include:
- Validation of the Concerto metamodel itself
- Validation of various valid model structures
- Error handling for invalid models
- Component integration testing

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project follows the same license as the Accord Project Concerto repository.

## Related Projects

- [Accord Project Concerto](https://github.com/accordproject/concerto) - The original JavaScript implementation
- [Concerto Metamodel](https://github.com/accordproject/concerto-metamodel) - The metamodel definition used for validation
