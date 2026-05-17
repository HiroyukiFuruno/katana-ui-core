use crate::render_model::UiNodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityLabel(String);

impl AccessibilityLabel {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessibilityRole {
    Text,
    Button,
    Input,
    Checkbox,
    Radio,
    List,
    Dialog,
    Toolbar,
    Window,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityNode {
    pub target: UiNodeId,
    pub role: AccessibilityRole,
    pub label: AccessibilityLabel,
}

impl AccessibilityNode {
    #[must_use]
    pub fn new(target: UiNodeId, role: AccessibilityRole, label: AccessibilityLabel) -> Self {
        Self {
            target,
            role,
            label,
        }
    }
}
