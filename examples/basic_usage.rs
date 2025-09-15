use concerto_validator_rs::{validate_metamodel, ConcertoValidator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Using the convenience function
    println!("=== Example 1: Using convenience function ===");
    
    let sample_model = r#"{
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
                    },
                    {
                        "$class": "concerto.metamodel@1.0.0.StringProperty",
                        "name": "lastName",
                        "isArray": false,
                        "isOptional": false
                    },
                    {
                        "$class": "concerto.metamodel@1.0.0.IntegerProperty",
                        "name": "age",
                        "isArray": false,
                        "isOptional": true
                    }
                ]
            }
        ]
    }"#;

    match validate_metamodel(sample_model) {
        Ok(()) => println!("✅ Model validation successful!"),
        Err(e) => println!("❌ Model validation failed: {}", e),
    }

    // Example 2: Using the validator directly
    println!("\n=== Example 2: Using validator directly ===");
    
    let validator = ConcertoValidator::new()?;
    
    match validator.validate(sample_model) {
        Ok(()) => println!("✅ Direct validation successful!"),
        Err(e) => println!("❌ Direct validation failed: {}", e),
    }

    // Example 3: Validating an invalid model
    println!("\n=== Example 3: Validating invalid model ===");
    
    let invalid_model = r#"{
        "$class": "concerto.metamodel@1.0.0.Model",
        "namespace": "org.example@1.0.0",
        "imports": [],
        "declarations": [
            {
                "$class": "concerto.metamodel@1.0.0.ConceptDeclaration",
                "name": "",
                "isAbstract": false,
                "properties": []
            }
        ]
    }"#;

    match validate_metamodel(invalid_model) {
        Ok(()) => println!("✅ Model validation successful!"),
        Err(e) => println!("❌ Model validation failed (as expected): {}", e),
    }

    // Example 4: Validating the Concerto metamodel itself
    println!("\n=== Example 4: Validating Concerto metamodel itself ===");
    
    let metamodel_json = include_str!("../metamodel.json");
    match validate_metamodel(metamodel_json) {
        Ok(()) => println!("✅ Concerto metamodel validation successful!"),
        Err(e) => println!("❌ Concerto metamodel validation failed: {}", e),
    }

    Ok(())
}
