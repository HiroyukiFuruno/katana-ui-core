use super::super::ops;
use super::super::types::{
    ColorPickerAlpha, ColorPickerBlendMode, ColorPickerTriggerSize, InlineColorPicker,
    InlineColorPickerProps, ResolvedInlineColorPicker, RgbaChannel,
};
use super::super::{
    COLOR_PICKER_DEFAULT_PANEL_SCALE, COLOR_PICKER_MAX_PANEL_SCALE, COLOR_PICKER_MIN_PANEL_SCALE,
};
use crate::theme::Theme;
use crate::theme::color::Color;
use std::rc::Rc;

fn noop_change(_: Color) {}
fn noop_pick_color() {}

impl InlineColorPicker {
    #[must_use]
    pub fn new(value: Color, a11y_label: impl Into<String>) -> Self {
        Self {
            props: InlineColorPickerProps {
                value,
                title: None,
                alpha: ColorPickerAlpha::Opaque,
                panel_scale: COLOR_PICKER_DEFAULT_PANEL_SCALE,
                trigger_size: ColorPickerTriggerSize::default(),
                trigger_border: true,
                disabled: false,
                readonly: false,
                a11y_label: a11y_label.into(),
                open: false,
                on_change: Rc::new(noop_change),
                on_pick_color: Rc::new(noop_pick_color),
            },
        }
    }

    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.props.open = open;
        self
    }

    #[must_use]
    pub fn rgba(mut self, is_rgba: bool) -> Self {
        self.props.alpha = if is_rgba {
            ColorPickerAlpha::BlendOrAdditive
        } else {
            ColorPickerAlpha::Opaque
        };
        self.props.value = ops::ColorPickerOps::resolve_value(self.props.value, self.props.alpha);
        self
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.props.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn panel_scale(mut self, scale: f32) -> Self {
        self.props.panel_scale =
            scale.clamp(COLOR_PICKER_MIN_PANEL_SCALE, COLOR_PICKER_MAX_PANEL_SCALE);
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
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    #[must_use]
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.props.readonly = readonly;
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
    pub fn resolve(&self, _theme: &Theme) -> ResolvedInlineColorPicker {
        ResolvedInlineColorPicker {
            value: ops::ColorPickerOps::resolve_value(self.props.value, self.props.alpha),
            title: self.props.title.clone(),
            blending_mode: ColorPickerBlendMode::from_alpha(self.props.alpha),
            open: self.props.open,
            alpha: self.props.alpha,
            panel_scale: self
                .props
                .panel_scale
                .clamp(COLOR_PICKER_MIN_PANEL_SCALE, COLOR_PICKER_MAX_PANEL_SCALE),
            trigger_size: self.props.trigger_size,
            trigger_border: self.props.trigger_border,
            disabled: self.props.disabled,
            readonly: self.props.readonly,
            a11y_label: self.props.a11y_label.clone(),
            on_change: Rc::clone(&self.props.on_change),
            on_pick_color: Rc::clone(&self.props.on_pick_color),
        }
    }

    pub fn set_channel(&self, channel: RgbaChannel, value: u8) -> Option<Color> {
        if self.props.disabled || self.props.readonly {
            return None;
        }

        let next =
            ops::ColorPickerOps::set_channel(self.props.value, channel, value, self.props.alpha);
        (self.props.on_change)(next);
        Some(next)
    }

    pub fn adjust_channel(&self, channel: RgbaChannel, delta: i16) -> Option<Color> {
        if self.props.disabled || self.props.readonly {
            return None;
        }

        let next =
            ops::ColorPickerOps::adjust_channel(self.props.value, channel, delta, self.props.alpha);
        (self.props.on_change)(next);
        Some(next)
    }
}
