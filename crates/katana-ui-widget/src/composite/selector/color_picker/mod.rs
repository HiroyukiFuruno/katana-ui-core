mod ops;
mod types;
mod view;

pub use types::{
    ColorPickerAlpha, ColorPickerRgba, ColorPickerRgbaProps, InlineColorPicker,
    InlineColorPickerProps, LabeledColorPicker, LabeledColorPickerProps, ResolvedColorPickerRgba,
    ResolvedInlineColorPicker, ResolvedLabeledColorPicker, RgbaChannel,
};

use crate::theme::Theme;
use crate::theme::color::Color;
use std::rc::Rc;

pub const COLOR_LABEL_WIDTH: f32 = 130.0;
pub const COLOR_SPACING: f32 = 16.0;
pub const COLOR_OFFSET_Y: f32 = -2.0;

fn noop_change(_: Color) {}

impl InlineColorPicker {
    #[must_use]
    pub fn new(value: Color, a11y_label: impl Into<String>) -> Self {
        Self {
            props: InlineColorPickerProps {
                value,
                alpha: ColorPickerAlpha::Opaque,
                disabled: false,
                readonly: false,
                a11y_label: a11y_label.into(),
                on_change: Rc::new(noop_change),
            },
        }
    }

    #[must_use]
    pub fn rgba(mut self, is_rgba: bool) -> Self {
        self.props.alpha = if is_rgba {
            ColorPickerAlpha::BlendOrAdditive
        } else {
            ColorPickerAlpha::Opaque
        };
        self.props.value = ops::resolve_value(self.props.value, self.props.alpha);
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
    pub fn resolve(&self, _theme: &Theme) -> ResolvedInlineColorPicker {
        ResolvedInlineColorPicker {
            value: ops::resolve_value(self.props.value, self.props.alpha),
            alpha: self.props.alpha,
            disabled: self.props.disabled,
            readonly: self.props.readonly,
            a11y_label: self.props.a11y_label.clone(),
            on_change: Rc::clone(&self.props.on_change),
        }
    }

    pub fn set_channel(&self, channel: RgbaChannel, value: u8) -> Option<Color> {
        if self.props.disabled || self.props.readonly {
            return None;
        }

        let next = ops::set_channel(self.props.value, channel, value, self.props.alpha);
        (self.props.on_change)(next);
        Some(next)
    }

    pub fn adjust_channel(&self, channel: RgbaChannel, delta: i16) -> Option<Color> {
        if self.props.disabled || self.props.readonly {
            return None;
        }

        let next = ops::adjust_channel(self.props.value, channel, delta, self.props.alpha);
        (self.props.on_change)(next);
        Some(next)
    }
}

impl LabeledColorPicker {
    #[must_use]
    pub fn new(label: impl Into<String>, value: Color) -> Self {
        let label = label.into();
        Self {
            props: LabeledColorPickerProps {
                label: label.clone(),
                label_width: COLOR_LABEL_WIDTH,
                spacing: COLOR_SPACING,
                offset_y: COLOR_OFFSET_Y,
                picker: InlineColorPicker::new(value, label).props,
            },
        }
    }

    #[must_use]
    pub fn rgba(mut self, is_rgba: bool) -> Self {
        self.props.picker.alpha = if is_rgba {
            ColorPickerAlpha::BlendOrAdditive
        } else {
            ColorPickerAlpha::Opaque
        };
        self.props.picker.value =
            ops::resolve_value(self.props.picker.value, self.props.picker.alpha);
        self
    }

    #[must_use]
    pub fn label_width(mut self, width: f32) -> Self {
        self.props.label_width = width;
        self
    }

    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.props.spacing = spacing;
        self
    }

    #[must_use]
    pub fn offset_y(mut self, offset: f32) -> Self {
        self.props.offset_y = offset;
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.picker.disabled = disabled;
        self
    }

    #[must_use]
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.props.picker.readonly = readonly;
        self
    }

    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(Color) + 'static) -> Self {
        self.props.picker.on_change = Rc::new(on_change);
        self
    }

    #[must_use]
    pub fn resolve(&self, theme: &Theme) -> ResolvedLabeledColorPicker {
        let picker = InlineColorPicker {
            props: self.props.picker.clone(),
        }
        .resolve(theme);
        ResolvedLabeledColorPicker {
            label: self.props.label.clone(),
            label_width: self.props.label_width,
            spacing: self.props.spacing,
            offset_y: self.props.offset_y,
            picker,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use std::cell::RefCell;

    fn base() -> Color {
        Color {
            r: 10,
            g: 20,
            b: 30,
            a: 40,
        }
    }

    #[test]
    fn inline_picker_defaults_to_opaque_alpha() {
        let resolved = InlineColorPicker::new(base(), "Color").resolve(&Theme::default_light());
        assert_eq!(resolved.alpha, ColorPickerAlpha::Opaque);
        assert_eq!(resolved.value.a, u8::MAX);
    }

    #[test]
    fn labeled_picker_defaults_match_katana_layout() {
        let resolved = LabeledColorPicker::new("Accent", base()).resolve(&Theme::default_light());
        assert_eq!(resolved.label, "Accent");
        assert_eq!(resolved.label_width, COLOR_LABEL_WIDTH);
        assert_eq!(resolved.spacing, COLOR_SPACING);
        assert_eq!(resolved.offset_y, COLOR_OFFSET_Y);
    }

    #[test]
    fn channel_update_calls_on_change() {
        let called = Rc::new(RefCell::new(None));
        let called_ref = Rc::clone(&called);
        let picker = ColorPickerRgba::new(base(), "Color").on_change(move |color| {
            *called_ref.borrow_mut() = Some(color);
        });

        let next = picker.set_channel(RgbaChannel::Red, 200);
        assert_eq!(next.map(|color| color.r), Some(200));
        assert_eq!(called.borrow().map(|color| color.r), Some(200));
    }

    #[test]
    fn rgb_mode_keeps_alpha_opaque() {
        let picker = InlineColorPicker::new(base(), "Color");
        let next = picker.set_channel(RgbaChannel::Red, 200);
        assert_eq!(next.map(|color| color.a), Some(u8::MAX));
    }

    #[test]
    fn disabled_does_not_call_on_change() {
        let called = Rc::new(RefCell::new(false));
        let called_ref = Rc::clone(&called);
        let picker = ColorPickerRgba::new(base(), "Color")
            .disabled(true)
            .on_change(move |_| {
                *called_ref.borrow_mut() = true;
            });

        assert!(picker.set_channel(RgbaChannel::Alpha, 128).is_none());
        assert!(!*called.borrow());
    }

    #[test]
    fn resolve_preserves_alpha() {
        let resolved = ColorPickerRgba::new(base(), "Color").resolve(&Theme::default_light());
        assert_eq!(resolved.value.a, 40);
    }
}
