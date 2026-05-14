mod blend;
mod channels;
mod color_space;
mod conversion;

use crate::theme::color::Color;

use super::types::ColorPickerValue;

pub(crate) struct ColorPickerOps;

impl ColorPickerOps {
    pub(crate) fn set_color(state: ColorPickerValue, color: Color) -> ColorPickerValue {
        ColorPickerValue::with_modes(color, state.alpha, state.blending_mode)
    }
}
