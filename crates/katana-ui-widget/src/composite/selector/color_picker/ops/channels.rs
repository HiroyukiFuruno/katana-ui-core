use super::super::types::{ColorPickerAlpha, RgbaChannel};
use super::{ColorPickerOps, ColorPickerValue};
use crate::theme::color::Color;

const CHANNEL_MIN: i16 = 0;
const CHANNEL_MAX: i16 = 255;
const CHANNEL_SIZE: i16 = 1;

fn clamp_channel(value: i16) -> u8 {
    value.clamp(CHANNEL_MIN, CHANNEL_MAX) as u8
}

impl ColorPickerOps {
    pub(crate) fn set_channel(
        color: Color,
        channel: RgbaChannel,
        value: u8,
        alpha: ColorPickerAlpha,
    ) -> Color {
        let state = Self::new_value(color, alpha);
        Self::set_channel_state(state, channel, value).color
    }

    pub(crate) fn set_channel_state(
        state: ColorPickerValue,
        channel: RgbaChannel,
        value: u8,
    ) -> ColorPickerValue {
        let current = state.color;
        let next = match channel {
            RgbaChannel::Red => Color {
                r: value,
                ..current
            },
            RgbaChannel::Green => Color {
                g: value,
                ..current
            },
            RgbaChannel::Blue => Color {
                b: value,
                ..current
            },
            RgbaChannel::Alpha => Color {
                a: value,
                ..current
            },
        };
        Self::set_color(state, next)
    }

    pub(crate) fn adjust_channel(
        color: Color,
        channel: RgbaChannel,
        delta: i16,
        alpha: ColorPickerAlpha,
    ) -> Color {
        let current = match channel {
            RgbaChannel::Red => color.r,
            RgbaChannel::Green => color.g,
            RgbaChannel::Blue => color.b,
            RgbaChannel::Alpha => color.a,
        };
        Self::set_channel(
            color,
            channel,
            clamp_channel(i16::from(current) + delta),
            alpha,
        )
    }

    pub(crate) fn adjust_channel_state(
        state: ColorPickerValue,
        channel: RgbaChannel,
        delta: i16,
    ) -> ColorPickerValue {
        let current = match channel {
            RgbaChannel::Red => state.color.r,
            RgbaChannel::Green => state.color.g,
            RgbaChannel::Blue => state.color.b,
            RgbaChannel::Alpha => state.color.a,
        };
        Self::set_channel_state(
            state,
            channel,
            clamp_channel(i16::from(current) + delta * CHANNEL_SIZE),
        )
    }
}
