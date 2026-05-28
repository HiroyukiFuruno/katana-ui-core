use super::typed_icon::UiIconProps;
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
    pub reserve_space: bool,
    pub icon: Option<UiIconProps>,
    pub action: Option<UiSlotActionSpec>,
}

impl UiSlotSpec {
    #[must_use]
    pub fn new(placement: UiSlotPlacement, label: impl Into<String>) -> Self {
        Self {
            placement,
            label: label.into(),
            reserve_space: true,
            icon: None,
            action: None,
        }
    }

    #[must_use]
    pub fn icon(placement: UiSlotPlacement, label: impl Into<String>, icon: UiIconProps) -> Self {
        Self {
            placement,
            label: label.into(),
            reserve_space: true,
            icon: Some(icon),
            action: None,
        }
    }

    #[must_use]
    pub fn icon_button(
        placement: UiSlotPlacement,
        label: impl Into<String>,
        icon: UiIconProps,
        callback: impl Into<String>,
    ) -> Self {
        let label = label.into();
        Self {
            placement,
            action: Some(UiSlotActionSpec::new(label.clone(), callback)),
            label,
            reserve_space: true,
            icon: Some(icon),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSlotActionSpec {
    pub label: String,
    pub callback: String,
}

impl UiSlotActionSpec {
    #[must_use]
    pub fn new(label: impl Into<String>, callback: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            callback: callback.into(),
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
    pub trailing_icon_buttons: Vec<UiSlotSpec>,
    pub clear_action: Option<UiClearActionSpec>,
    pub submit_on_enter: bool,
    pub ime_enabled: bool,
    pub emoji_enabled: bool,
}
