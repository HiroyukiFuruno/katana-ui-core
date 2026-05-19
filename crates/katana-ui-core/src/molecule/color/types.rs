use crate::interaction::RgbaActionValue;
use serde::{Deserialize, Serialize};

const OPAQUE_ALPHA: u8 = 255;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RgbaColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl RgbaColor {
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

    #[must_use]
    pub const fn opaque(self) -> Self {
        Self {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: OPAQUE_ALPHA,
        }
    }
}

impl From<RgbaActionValue> for RgbaColor {
    fn from(value: RgbaActionValue) -> Self {
        Self::new(value.red, value.green, value.blue, value.alpha)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorBlendingMode {
    Replace,
    Multiply,
    Screen,
}
