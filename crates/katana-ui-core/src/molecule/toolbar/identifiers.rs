use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ToolbarActionId(String);

impl ToolbarActionId {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ToolbarActionId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ToolbarActionId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ToolbarGroupId(String);

impl ToolbarGroupId {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ToolbarGroupId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ToolbarGroupId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ToolbarPriority(i32);

impl ToolbarPriority {
    #[must_use]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn value(self) -> i32 {
        self.0
    }
}
