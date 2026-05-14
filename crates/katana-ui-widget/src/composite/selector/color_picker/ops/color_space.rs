use super::super::hsva::ColorPickerHsva;
use super::super::types::ColorPickerValue;
use super::ColorPickerOps;
use crate::theme::color::Color;

const CHANNEL_MAX_FLOAT: f64 = 255.0;

impl ColorPickerOps {
    #[cfg(test)]
    pub(crate) fn state_hue(state: ColorPickerValue) -> f64 {
        state.hsva.hue
    }

    pub(crate) fn set_hue(state: ColorPickerValue, hue: f64) -> ColorPickerValue {
        let hsva = ColorPickerHsva {
            hue: hue.rem_euclid(1.0),
            ..state.hsva
        };
        Self::set_hsva(state, hsva)
    }

    pub(crate) fn set_hue_saturation_value(
        state: ColorPickerValue,
        hue: f64,
        saturation: f64,
        value: f64,
    ) -> ColorPickerValue {
        let hsva = ColorPickerHsva {
            hue: hue.rem_euclid(1.0),
            saturation: Self::clamp01(saturation),
            value: Self::clamp01(value),
            alpha: state.hsva.alpha,
        };
        Self::set_hsva(state, hsva)
    }

    pub(crate) fn color_grid_color(hue: f64, saturation: f64, value: f64, alpha: u8) -> Color {
        ColorPickerHsva {
            hue: hue.rem_euclid(1.0),
            saturation: Self::clamp01(saturation),
            value: Self::clamp01(value),
            alpha: f64::from(alpha) / CHANNEL_MAX_FLOAT,
        }
        .to_color()
    }

    fn set_hsva(state: ColorPickerValue, hsva: ColorPickerHsva) -> ColorPickerValue {
        let color = hsva.to_color();
        let mut next = ColorPickerValue::with_modes(color, state.alpha, state.blending_mode);
        next.hsva = ColorPickerHsva {
            alpha: next.hsva.alpha,
            ..hsva
        };
        next
    }
}
