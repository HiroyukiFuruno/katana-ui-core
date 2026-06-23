use super::Input;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputValidationError {
    MultilineValueRequiresTextArea,
}

impl Input {
    pub fn validate(&self) -> Result<(), InputValidationError> {
        if self.state.interaction.value.contains(['\n', '\r']) {
            return Err(InputValidationError::MultilineValueRequiresTextArea);
        }
        Ok(())
    }
}
