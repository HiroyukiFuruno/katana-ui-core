use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub depth: usize,
    pub expanded: bool,
    pub selected: bool,
    pub active: bool,
}

impl TreeNode {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>, depth: usize) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            depth,
            expanded: false,
            selected: false,
            active: false,
        }
    }

    #[must_use]
    pub fn active(mut self, value: bool) -> Self {
        self.active = value;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandItem {
    pub id: String,
    pub title: String,
    pub shortcut: String,
    pub disabled: bool,
}

impl CommandItem {
    #[must_use]
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            shortcut: String::new(),
            disabled: false,
        }
    }

    #[must_use]
    pub fn shortcut(mut self, value: impl Into<String>) -> Self {
        self.shortcut = value.into();
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArrayEditorItem {
    pub id: String,
    pub label: String,
    pub value: String,
    pub removable: bool,
}

impl ArrayEditorItem {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value: String::new(),
            removable: true,
        }
    }
}
