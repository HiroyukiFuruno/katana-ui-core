use crate::render_model::{UiSize, UiTone};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmptyStateAction {
    pub id: String,
    pub label: String,
}

impl EmptyStateAction {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmptyStateActionId {
    Primary,
    Secondary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmptyStateEvent {
    Actioned {
        id: EmptyStateActionId,
        action_id: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmptyStateContractViolation {
    MissingHeading,
    IconAndIllustrationConflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmptyStateTone {
    Neutral,
    Subtle,
    Accent,
    Warning,
    Danger,
}

impl EmptyStateTone {
    #[must_use]
    pub const fn announce_label(self) -> &'static str {
        match self {
            Self::Neutral => "Neutral",
            Self::Subtle => "Subtle",
            Self::Accent => "Accent",
            Self::Warning => "Warning",
            Self::Danger => "Danger",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmptyStateSize {
    Compact,
    Default,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmptyStateAlignment {
    Center,
    Leading,
}

impl From<EmptyStateTone> for UiTone {
    fn from(value: EmptyStateTone) -> Self {
        match value {
            EmptyStateTone::Neutral => Self::Neutral,
            EmptyStateTone::Subtle | EmptyStateTone::Accent => Self::Accent,
            EmptyStateTone::Warning => Self::Warning,
            EmptyStateTone::Danger => Self::Danger,
        }
    }
}

impl From<EmptyStateSize> for UiSize {
    fn from(value: EmptyStateSize) -> Self {
        match value {
            EmptyStateSize::Compact => Self::Small,
            EmptyStateSize::Default => Self::Medium,
            EmptyStateSize::Large => Self::Large,
        }
    }
}
