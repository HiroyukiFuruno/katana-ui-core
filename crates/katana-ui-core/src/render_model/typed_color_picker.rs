use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiColorPickerTriggerKind {
    ColorButton,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiColorBlendingMode {
    Normal,
    Additive,
    Replace,
    Multiply,
    Screen,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiColorPickerProps {
    pub trigger_kind: UiColorPickerTriggerKind,
    pub rgba_css: String,
    pub opaque_preview_css: String,
    pub checker_background: bool,
    pub rgba_mode: bool,
    pub alpha_slider_visible: bool,
    pub eyedropper_visible: bool,
    pub hue_degrees: u16,
    pub alpha: u8,
    pub blending: UiColorBlendingMode,
    pub color_plane: String,
    pub eyedropper_action: String,
    pub panel_scale_percent: u16,
}

impl Default for UiColorPickerProps {
    fn default() -> Self {
        Self {
            trigger_kind: UiColorPickerTriggerKind::ColorButton,
            rgba_css: String::new(),
            opaque_preview_css: String::new(),
            checker_background: false,
            rgba_mode: true,
            alpha_slider_visible: true,
            eyedropper_visible: false,
            hue_degrees: 0,
            alpha: 255,
            blending: UiColorBlendingMode::Normal,
            color_plane: String::new(),
            eyedropper_action: String::new(),
            panel_scale_percent: 75,
        }
    }
}
