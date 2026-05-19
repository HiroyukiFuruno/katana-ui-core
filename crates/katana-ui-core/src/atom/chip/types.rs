use crate::render_model::UiStateId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChipTone {
    Neutral,
    Accent,
    Success,
    Warning,
    Danger,
    Muted,
}

impl ChipTone {
    #[must_use]
    pub const fn token_name(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Accent => "accent",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Muted => "muted",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChipVariant {
    Solid,
    Soft,
    Outline,
    Ghost,
}

impl ChipVariant {
    #[must_use]
    pub const fn token_name(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Soft => "soft",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChipSize {
    Compact,
    Default,
    Large,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChipKeyboardInput {
    Backspace,
    Delete,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChipAction {
    Press,
    Dismiss,
    Keyboard(ChipKeyboardInput),
    Focus,
    Blur,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChipEvent {
    ChipPressed { id: UiStateId },
    ChipDismissed { id: UiStateId },
    Focus { id: UiStateId },
    Blur { id: UiStateId },
}
