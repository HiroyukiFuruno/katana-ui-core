use super::{ColorBlendingMode, ColorPicker, RgbaColor};
use crate::render_model::{UiSize, UiStateId};

impl ColorPicker {
    #[must_use]
    pub fn color_value(&self) -> RgbaColor {
        self.value
    }

    #[must_use]
    pub fn blending_mode(&self) -> ColorBlendingMode {
        self.blending
    }

    #[must_use]
    pub fn hue_value(&self) -> u16 {
        self.hue
    }

    #[must_use]
    pub fn alpha_value(&self) -> u8 {
        self.alpha
    }

    #[must_use]
    pub fn previews_color(&self) -> bool {
        self.preview
    }

    #[must_use]
    pub fn color_area_model(&self) -> &str {
        &self.color_area
    }

    #[must_use]
    pub fn trigger_size_model(&self) -> UiSize {
        self.trigger_size
    }

    #[must_use]
    pub fn title_model(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state.state_id
    }
}
