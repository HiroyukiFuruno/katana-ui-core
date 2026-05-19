use serde::{Deserialize, Serialize};

use super::preset::UiButtonLayoutPreset;

const LABEL_ALIGN_CENTER: &str = "center";
const WIDTH_MODE_AUTO: &str = "auto";
const WIDTH_MODE_PX: &str = "px";
const WIDTH_MODE_PERCENT: &str = "percent";
const WIDTH_MODE_FILL: &str = "fill";
const WIDTH_VALUE_EMPTY: u16 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiButtonLayoutDto {
    pub min_width: u16,
    pub min_height: u16,
    pub width_mode: String,
    pub width_value: u16,
    pub padding_x: u16,
    pub padding_y: u16,
    pub border_width: u16,
    pub radius: u16,
    pub icon_gap: u16,
    pub label_align: String,
}

impl UiButtonLayoutDto {
    #[must_use]
    pub fn from_preset(preset: UiButtonLayoutPreset) -> Self {
        preset.to_dto()
    }

    #[must_use]
    pub fn with_min_size(mut self, width: u16, height: u16) -> Self {
        self.min_width = width;
        self.min_height = height;
        self
    }

    #[must_use]
    pub fn with_width_auto(mut self) -> Self {
        self.width_mode = WIDTH_MODE_AUTO.to_string();
        self.width_value = WIDTH_VALUE_EMPTY;
        self
    }

    #[must_use]
    pub fn with_width_px(mut self, value: u16) -> Self {
        self.width_mode = WIDTH_MODE_PX.to_string();
        self.width_value = value;
        self
    }

    #[must_use]
    pub fn with_width_percent(mut self, value: u16) -> Self {
        self.width_mode = WIDTH_MODE_PERCENT.to_string();
        self.width_value = value;
        self
    }

    #[must_use]
    pub fn with_width_fill(mut self) -> Self {
        self.width_mode = WIDTH_MODE_FILL.to_string();
        self.width_value = WIDTH_VALUE_EMPTY;
        self
    }

    #[must_use]
    pub fn with_padding(mut self, x: u16, y: u16) -> Self {
        self.padding_x = x;
        self.padding_y = y;
        self
    }

    #[must_use]
    pub fn with_border(mut self, width: u16, radius: u16) -> Self {
        self.border_width = width;
        self.radius = radius;
        self
    }

    #[must_use]
    pub fn with_icon_gap(mut self, value: u16) -> Self {
        self.icon_gap = value;
        self
    }

    #[must_use]
    pub fn with_label_align(mut self, value: impl Into<String>) -> Self {
        self.label_align = value.into();
        self
    }

    #[must_use]
    pub fn new(
        min_width: u16,
        min_height: u16,
        padding_x: u16,
        padding_y: u16,
        border_width: u16,
        radius: u16,
        icon_gap: u16,
    ) -> Self {
        Self {
            min_width,
            min_height,
            width_mode: WIDTH_MODE_AUTO.to_string(),
            width_value: WIDTH_VALUE_EMPTY,
            padding_x,
            padding_y,
            border_width,
            radius,
            icon_gap,
            label_align: LABEL_ALIGN_CENTER.to_string(),
        }
    }
}

impl Default for UiButtonLayoutDto {
    fn default() -> Self {
        UiButtonLayoutPreset::Modern.to_dto()
    }
}
