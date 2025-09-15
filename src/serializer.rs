use crate::error::ValidationResult;
use crate::metamodel::MetamodelManager;
use serde_json::Value;

/// Serializer that handles validation during JSON deserialization
/// This is the Rust equivalent of the JavaScript Serializer class
pub struct Serializer {
    metamodel_manager: MetamodelManager,
}

impl Serializer {
    /// Create a new serializer with the Concerto metamodel
    pub fn new() -> ValidationResult<Self> {
        let metamodel_manager = MetamodelManager::new()?;
        
        Ok(Self {
            metamodel_manager,
        })
    }

    /// Main validation method - equivalent to fromJSON in JavaScript
    /// This populates and validates the JSON AST against the metamodel
    pub fn from_json(&self, json: &str) -> ValidationResult<Value> {
        let value: Value = serde_json::from_str(json)?;
        self.metamodel_manager.validate_against_metamodel(&value)?;
        Ok(value)
    }

    /// Get a reference to the metamodel manager
    pub fn get_metamodel_manager(&self) -> &MetamodelManager {
        &self.metamodel_manager
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