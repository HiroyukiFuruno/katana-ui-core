use serde::{Deserialize, Serialize};

use super::dto::UiButtonLayoutDto;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiButtonLayoutPatchDto {
    pub min_width: Option<u16>,
    pub min_height: Option<u16>,
    pub width_mode: Option<String>,
    pub width_value: Option<u16>,
    pub padding_x: Option<u16>,
    pub padding_y: Option<u16>,
    pub border_width: Option<u16>,
    pub radius: Option<u16>,
    pub icon_gap: Option<u16>,
    pub label_align: Option<String>,
}

impl UiButtonLayoutPatchDto {
    #[must_use]
    pub fn with_min_size(mut self, width: u16, height: u16) -> Self {
        self.min_width = Some(width);
        self.min_height = Some(height);
        self
    }

    #[must_use]
    pub fn with_width_auto(mut self) -> Self {
        self.width_mode = Some("auto".to_string());
        self.width_value = Some(0);
        self
    }

    #[must_use]
    pub fn with_width_px(mut self, value: u16) -> Self {
        self.width_mode = Some("px".to_string());
        self.width_value = Some(value);
        self
    }

    #[must_use]
    pub fn with_width_percent(mut self, value: u16) -> Self {
        self.width_mode = Some("percent".to_string());
        self.width_value = Some(value);
        self
    }

    #[must_use]
    pub fn with_width_fill(mut self) -> Self {
        self.width_mode = Some("fill".to_string());
        self.width_value = Some(0);
        self
    }

    #[must_use]
    pub fn with_padding(mut self, x: u16, y: u16) -> Self {
        self.padding_x = Some(x);
        self.padding_y = Some(y);
        self
    }

    #[must_use]
    pub fn with_border(mut self, width: u16, radius: u16) -> Self {
        self.border_width = Some(width);
        self.radius = Some(radius);
        self
    }

    #[must_use]
    pub fn with_icon_gap(mut self, value: u16) -> Self {
        self.icon_gap = Some(value);
        self
    }

    #[must_use]
    pub fn with_label_align(mut self, value: impl Into<String>) -> Self {
        self.label_align = Some(value.into());
        self
    }

    #[must_use]
    pub fn apply_to(self, mut layout: UiButtonLayoutDto) -> UiButtonLayoutDto {
        if let Some(value) = self.min_width {
            layout.min_width = value;
        }
        if let Some(value) = self.min_height {
            layout.min_height = value;
        }
        if let Some(value) = self.width_mode {
            layout.width_mode = value;
        }
        if let Some(value) = self.width_value {
            layout.width_value = value;
        }
        if let Some(value) = self.padding_x {
            layout.padding_x = value;
        }
        if let Some(value) = self.padding_y {
            layout.padding_y = value;
        }
        if let Some(value) = self.border_width {
            layout.border_width = value;
        }
        if let Some(value) = self.radius {
            layout.radius = value;
        }
        if let Some(value) = self.icon_gap {
            layout.icon_gap = value;
        }
        if let Some(value) = self.label_align {
            layout.label_align = value;
        }
        layout
    }
}
