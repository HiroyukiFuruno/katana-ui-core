use crate::render_model::UiStateId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiActionSource {
    Generic,
    Button,
    Input,
    Checkbox,
    Radio,
    Toggle,
    Progress,
    ColorPicker,
}

impl UiActionSource {
    pub(crate) fn press_name(self) -> &'static str {
        match self {
            Self::Button => "button_press",
            _ => "press",
        }
    }

    pub(crate) fn selection_name(self) -> &'static str {
        match self {
            Self::Checkbox => "checkbox_checked",
            Self::Radio => "radio_selected",
            Self::Toggle => "toggle_checked",
            _ => "set_selected_index",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RgbaActionValue {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl RgbaActionValue {
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    #[must_use]
    pub fn css_rgba(self) -> String {
        format!(
            "rgba({}, {}, {}, {})",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorDragAction {
    pub target: UiStateId,
    pub value: RgbaActionValue,
    pub hue: u16,
    pub preview: bool,
}

impl ColorDragAction {
    #[must_use]
    pub fn new(target: UiStateId, value: RgbaActionValue, hue: u16, preview: bool) -> Self {
        Self {
            target,
            value,
            hue,
            preview,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressAction {
    pub target: UiStateId,
    pub determinate: bool,
    pub percent: u8,
}

impl ProgressAction {
    #[must_use]
    pub fn new(target: UiStateId, determinate: bool, percent: u8) -> Self {
        Self {
            target,
            determinate,
            percent,
        }
    }
}

macro_rules! target_action {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            pub target: UiStateId,
        }

        impl $name {
            #[must_use]
            pub fn new(target: UiStateId) -> Self {
                Self { target }
            }
        }
    };
}

target_action!(ButtonAction);
target_action!(RadioAction);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputAction {
    pub target: UiStateId,
    pub value: String,
}

impl InputAction {
    #[must_use]
    pub fn new(target: UiStateId, value: impl Into<String>) -> Self {
        Self {
            target,
            value: value.into(),
        }
    }
}

macro_rules! checked_action {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            pub target: UiStateId,
            pub checked: bool,
        }

        impl $name {
            #[must_use]
            pub fn new(target: UiStateId, checked: bool) -> Self {
                Self { target, checked }
            }
        }
    };
}

checked_action!(CheckboxAction);
checked_action!(ToggleAction);
