use super::super::types::{ColorPickerAlpha, ColorPickerValue};
use super::ColorPickerOps;
use crate::theme::color::Color;

pub(crate) const OPAQUE_ALPHA: u8 = u8::MAX;

impl ColorPickerOps {
    pub(crate) fn clamp01(value: f64) -> f64 {
        value.clamp(0.0, 1.0)
    }

    pub(crate) fn apply_alpha_mode(color: Color, alpha: ColorPickerAlpha) -> Color {
        if alpha.allows_alpha() {
            color
        } else {
            Color {
                a: OPAQUE_ALPHA,
                ..color
            }
        }
    }

    pub(crate) fn new_value(color: Color, alpha: ColorPickerAlpha) -> ColorPickerValue {
        let color = Self::apply_alpha_mode(color, alpha);
        ColorPickerValue::with_alpha_mode(color, alpha)
    }

    pub(crate) fn resolve_value(color: Color, alpha: ColorPickerAlpha) -> Color {
        Self::apply_alpha_mode(color, alpha)
    }
}
