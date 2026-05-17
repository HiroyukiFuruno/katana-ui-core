use super::conversion::OPAQUE_ALPHA;
use super::{ColorPickerOps, ColorPickerValue};
use crate::composite::selector::color_picker::types::ColorPickerBlendMode;
use crate::theme::color::Color;

impl ColorPickerOps {
    pub(crate) fn set_alpha(state: ColorPickerValue, alpha: u8) -> ColorPickerValue {
        Self::set_color(
            state,
            Color {
                a: alpha,
                ..state.color
            },
        )
    }

    pub(crate) fn set_blend_mode(
        state: ColorPickerValue,
        mode: ColorPickerBlendMode,
    ) -> ColorPickerValue {
        let color = if mode.allows_alpha() {
            state.color
        } else {
            Color {
                a: OPAQUE_ALPHA,
                ..state.color
            }
        };
        ColorPickerValue::with_modes(color, state.alpha, mode)
    }
}
