use super::{UiTone, UiVariant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiSlotPlacement {
    Leading,
    Trailing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSlotSpec {
    pub placement: UiSlotPlacement,
    pub label: String,
}

impl UiSlotSpec {
    #[must_use]
    pub fn new(placement: UiSlotPlacement, label: impl Into<String>) -> Self {
        Self {
            placement,
            label: label.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiClearActionSpec {
    pub label: String,
}

impl UiClearActionSpec {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTextEntryProps {
    pub leading_slot: Option<UiSlotSpec>,
    pub trailing_slot: Option<UiSlotSpec>,
    pub clear_action: Option<UiClearActionSpec>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiDismissAction {
    None,
    Available,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiStatusProps {
    pub severity: UiTone,
    pub variant: UiVariant,
    pub dismiss_action: UiDismissAction,
}

impl Default for UiStatusProps {
    fn default() -> Self {
        Self {
            severity: UiTone::Neutral,
            variant: UiVariant::Plain,
            dismiss_action: UiDismissAction::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiProgressMode {
    Determinate,
    Indeterminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiAnimationState {
    Idle,
    Running,
    Paused,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiLoadingProps {
    pub mode: UiProgressMode,
    pub label: String,
    pub animation_state: UiAnimationState,
}

impl Default for UiLoadingProps {
    fn default() -> Self {
        Self {
            mode: UiProgressMode::Indeterminate,
            label: String::new(),
            animation_state: UiAnimationState::Idle,
        }
    }
}
