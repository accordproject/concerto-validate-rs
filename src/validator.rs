use crate::error::ValidationResult;
use crate::model_manager::ModelManager;

pub struct Validator {
    metamodel_manager: ModelManager,
}

impl Validator {
    pub fn new() -> Result<Self, crate::error::ValidationError> {
        let metamodel_manager = ModelManager::new()?;

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
