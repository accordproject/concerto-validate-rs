use crate::error::ValidationResult;
use crate::metamodel_manager::MetamodelManager;

pub struct Validator {
    metamodel_manager: MetamodelManager,
}

impl Validator {
    pub fn new() -> Result<Self, crate::error::ValidationError> {
        let metamodel_manager = MetamodelManager::new()?;

        Ok(Self { metamodel_manager })
    }

    pub fn validate(&self, json_ast: &str) -> ValidationResult<()> {
        match serde_json::from_str(json_ast) {
            Ok(ast) => {
                self.metamodel_manager.validate_metamodel(&ast)?;
                Ok(())
            }
            Err(err) => Err(crate::ValidationError::JsonError(err)),
        }
    }
}
