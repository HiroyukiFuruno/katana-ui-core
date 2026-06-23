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
    pub const fn alpha_control_visible(&self) -> bool {
        self.rgba_mode
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
    pub const fn uses_rgba_mode(&self) -> bool {
        self.rgba_mode
    }

    #[must_use]
    pub const fn has_trigger_border(&self) -> bool {
        self.trigger_border
    }

    #[must_use]
    pub fn eyedropper_callback_model(&self) -> &str {
        &self.eyedropper_callback
    }

    #[must_use]
    pub fn shows_eyedropper_control(&self) -> bool {
        !self.eyedropper_callback.is_empty()
    }

    #[must_use]
    pub const fn trigger_shows_numeric_value(&self) -> bool {
        false
    }

    #[must_use]
    pub fn trigger_transparent_preview(&self) -> String {
        self.value.css_rgba()
    }

    #[must_use]
    pub fn trigger_opaque_preview(&self) -> String {
        self.value.opaque().css_rgba()
    }

    #[must_use]
    pub const fn trigger_uses_checker_background(&self) -> bool {
        self.rgba_mode && self.value.alpha < u8::MAX
    }

    #[must_use]
    pub const fn panel_scale_percent_model(&self) -> u16 {
        self.panel_scale_percent
    }

    #[must_use]
    pub const fn panel_exposes_alpha(&self) -> bool {
        self.rgba_mode
    }

    #[must_use]
    pub fn panel_shows_eyedropper(&self) -> bool {
        self.shows_eyedropper_control()
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state.state_id
    }
}
