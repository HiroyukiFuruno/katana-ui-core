mod builders;
mod hsva;
mod ops;
mod types;
mod view;

pub use hsva::ColorPickerHsva;
pub use types::{
    ColorPickerAlpha, ColorPickerBlendMode, ColorPickerRgba, ColorPickerRgbaProps,
    ColorPickerTriggerSize, ColorPickerValue, InlineColorPicker, InlineColorPickerProps,
    LabeledColorPicker, LabeledColorPickerProps, ResolvedColorPickerRgba,
    ResolvedInlineColorPicker, ResolvedLabeledColorPicker, RgbaChannel,
};

pub const COLOR_LABEL_WIDTH: f32 = 130.0;
pub const COLOR_SPACING: f32 = 16.0;
pub const COLOR_OFFSET_Y: f32 = -2.0;
pub const COLOR_PICKER_DEFAULT_PANEL_SCALE: f32 = 0.75;
pub const COLOR_PICKER_MIN_PANEL_SCALE: f32 = 0.75;
pub const COLOR_PICKER_MAX_PANEL_SCALE: f32 = 1.5;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use crate::theme::color::Color;
    use std::cell::RefCell;
    use std::rc::Rc;

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

    #[test]
    fn state_hue_survives_grayscale_selection() {
        let state = ops::ColorPickerOps::new_value(base(), ColorPickerAlpha::BlendOrAdditive);
        let gray = ops::ColorPickerOps::set_hue_saturation_value(state, 0.72, 0.0, 0.5);

        assert_eq!(gray.color.r, gray.color.g);
        assert_eq!(gray.color.g, gray.color.b);
        assert!((ops::ColorPickerOps::state_hue(gray) - 0.72).abs() < 0.001);
    }

    #[test]
    fn additive_blend_ignores_alpha_like_egui() {
        let state = ops::ColorPickerOps::new_value(base(), ColorPickerAlpha::BlendOrAdditive);
        let next = ops::ColorPickerOps::set_blend_mode(state, ColorPickerBlendMode::Additive);

        assert_eq!(next.color.a, u8::MAX);
    }
}
