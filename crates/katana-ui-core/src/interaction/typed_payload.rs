use crate::render_model::UiStateId;
use serde::{Deserialize, Serialize};

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
    pub const fn new(target: UiStateId, determinate: bool, percent: u8) -> Self {
        Self {
            target,
            determinate,
            percent,
        }
    }
}
