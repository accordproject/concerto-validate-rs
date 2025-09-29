# Concerto Validator RS

A Rust library that validates Accord Project Concerto data models in their JSON AST (Abstract Syntax Tree) format.

## Overview

This validator implements the core functionality of the JavaScript `validateMetaModel` function by:

1. Loading the raw [Concerto Metamodel JSON](https://github.com/accordproject/concerto-metamodel/blob/main/lib/metamodel.json) directly as the validation schema
2. Using a `ModelManager` that works with raw JSON instead of hardcoded Rust structs. Except for a few structs needed to replicate some functionality of the `introspect` classes.

## Features

- ✅ **Auto-Updating Metamodel**: Build script automatically checks and updates the metamodel from the official repository
- ✅ **Dynamic Metamodel Loading**: Loads metamodel from raw JSON instead of hardcoded structures
- ✅ **Complete Metamodel Support**: Validates against the full Concerto metamodel specification
- ✅ **Structural Validation**: Ensures JSON structure matches expected Concerto AST format
- ✅ **Type Validation**: Validates property types, arrays, optionality, and nested objects
- ✅ **Error Reporting**: Provides detailed error messages for debugging
- ✅ **Self-Validation**: Can validate the Concerto metamodel itself
- ✅ **Performance**: Fast validation using native Rust performance
- ✅ **Command Line Interface**: Extensible CLI tool for validating JSON files

## Installation
Currently the library is in early development phase, the package is not published to [crates.io](crates.io) yet.

## Automatic Metamodel Updates

The library includes a build script that automatically ensures you're always using the latest version of the [Concerto Metamodel](https://github.com/accordproject/concerto-metamodel/blob/main/lib/metamodel.json) as the source of truth.

**How it works:**
- On every build, the script checks the local `metamodel.json` against the official version
- If the local file is missing, outdated, or corrupted, it automatically downloads the latest version
- The comparison uses SHA256 hashing of normalized JSON to detect changes
- Build warnings inform you when updates occur

**Features:**
- ✅ Automatic detection of metamodel updates
- ✅ Graceful fallback to local version if network is unavailable
- ✅ JSON validation to ensure downloaded content is valid
- ✅ Build-time integration with no runtime overhead

This ensures that your validator is always using the official, up-to-date metamodel definition without manual intervention.

## Usage

### Command Line Interface

The project includes a command-line tool for validating Concerto model JSON files:

#### Building the CLI
```bash
cargo build
```

#### Basic Validation
```bash
# Validate a single file
./target/debug/concerto-validator validate --input model.json

# Validate multiple files
./target/debug/concerto-validator validate --input model1.json --input model2.json --input model3.json

# Stop at the first error (fail-early mode)
./target/debug/concerto-validator validate --input model1.json --input model2.json --fail-early
```

#### CLI Help
```bash
# General help
./target/debug/concerto-validator --help

# Help for the validate command
./target/debug/concerto-validator validate --help

# Show version
./target/debug/concerto-validator --version
```

### Library Usage

#### Basic Usage

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

## Related Projects

- [Accord Project Concerto](https://github.com/accordproject/concerto) - The original JavaScript implementation
- [Concerto Metamodel](https://github.com/accordproject/concerto-metamodel) - The metamodel definition used for validation

## License
Accord Project source code files are made available under the [Apache License, Version 2.0][apache].
Accord Project documentation files are made available under the [Creative Commons Attribution 4.0 International License][creativecommons] (CC-BY-4.0).

Copyright 2018-2019 Clause, Inc. All trademarks are the property of their respective owners. See [LF Projects Trademark Policy](https://lfprojects.org/policies/trademark-policy/).

[linuxfound]: https://www.linuxfoundation.org
[charter]: https://github.com/accordproject/governance/blob/master/accord-project-technical-charter.md
[apmain]: https://accordproject.org/ 
[apblog]: https://medium.com/@accordhq
[apdoc]: https://docs.accordproject.org/
[apdiscord]: https://discord.com/invite/Zm99SKhhtA

[contributing]: https://github.com/accordproject/concerto/blob/master/CONTRIBUTING.md
[developers]: https://github.com/accordproject/concerto/blob/master/DEVELOPERS.md

[apache]: https://github.com/accordproject/concerto/blob/master/LICENSE
[creativecommons]: http://creativecommons.org/licenses/by/4.0/
