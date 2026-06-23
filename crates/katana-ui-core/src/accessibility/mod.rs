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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReducedMotionPreference {
    NoPreference,
    Reduce,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedMotionQuery {
    preference: ReducedMotionPreference,
}

impl ReducedMotionQuery {
    #[must_use]
    pub const fn new(preference: ReducedMotionPreference) -> Self {
        Self { preference }
    }

    #[must_use]
    pub const fn prefers_reduced_motion(self) -> bool {
        matches!(self.preference, ReducedMotionPreference::Reduce)
    }
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
