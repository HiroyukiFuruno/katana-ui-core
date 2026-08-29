use serde::{Deserialize, Serialize};

/// Opaque identity for one command family mounted in a command-chrome slot.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandChromeFamilyId(String);

impl CommandChromeFamilyId {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl Default for CommandChromeFamilyId {
    fn default() -> Self {
        Self::new("default")
    }
}
