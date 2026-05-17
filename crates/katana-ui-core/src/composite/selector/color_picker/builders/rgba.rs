use super::super::types::{
    ColorPickerRgba, ColorPickerTriggerSize, InlineColorPicker, ResolvedColorPickerRgba,
    RgbaChannel,
};
use super::super::{COLOR_PICKER_MAX_PANEL_SCALE, COLOR_PICKER_MIN_PANEL_SCALE};
use crate::theme::Theme;
use crate::theme::color::Color;
use std::rc::Rc;

impl ColorPickerRgba {
    #[must_use]
    pub fn new(value: Color, a11y_label: impl Into<String>) -> Self {
        Self {
            props: InlineColorPicker::new(value, a11y_label).rgba(true).props,
        }
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.props.open = open;
        self
    }

    #[must_use]
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.props.readonly = readonly;
        self
    }

    #[must_use]
    pub fn panel_scale(mut self, scale: f32) -> Self {
        self.props.panel_scale =
            scale.clamp(COLOR_PICKER_MIN_PANEL_SCALE, COLOR_PICKER_MAX_PANEL_SCALE);
        self
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.props.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn trigger_size(mut self, size: ColorPickerTriggerSize) -> Self {
        self.props.trigger_size = size;
        self
    }

    #[must_use]
    pub fn trigger_border(mut self, border: bool) -> Self {
        self.props.trigger_border = border;
        self
    }

    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(Color) + 'static) -> Self {
        self.props.on_change = Rc::new(on_change);
        self
    }

    #[must_use]
    pub fn on_pick_color(mut self, on_pick_color: impl Fn() + 'static) -> Self {
        self.props.on_pick_color = Rc::new(on_pick_color);
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedColorPickerRgba {
        InlineColorPicker {
            props: self.props.clone(),
        }
        .resolve(theme)
    }

    pub fn set_channel(&self, channel: RgbaChannel, value: u8) -> Option<Color> {
        InlineColorPicker {
            props: self.props.clone(),
        }
        .set_channel(channel, value)
    }

    pub fn adjust_channel(&self, channel: RgbaChannel, delta: i16) -> Option<Color> {
        InlineColorPicker {
            props: self.props.clone(),
        }
        .adjust_channel(channel, delta)
    }
}
