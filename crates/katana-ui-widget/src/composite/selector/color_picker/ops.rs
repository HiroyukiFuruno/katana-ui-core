use super::types::{ColorPickerAlpha, RgbaChannel};
use crate::theme::color::Color;

const CHANNEL_MIN: i16 = 0;
const CHANNEL_MAX: i16 = 255;
const OPAQUE_ALPHA: u8 = u8::MAX;

fn clamp_channel(value: i16) -> u8 {
    value.clamp(CHANNEL_MIN, CHANNEL_MAX) as u8
}

fn apply_alpha_mode(mut color: Color, alpha: ColorPickerAlpha) -> Color {
    if !alpha.allows_alpha() {
        color.a = OPAQUE_ALPHA;
    }
    color
}

pub(super) fn set_channel(
    mut color: Color,
    channel: RgbaChannel,
    value: u8,
    alpha: ColorPickerAlpha,
) -> Color {
    match channel {
        RgbaChannel::Red => color.r = value,
        RgbaChannel::Green => color.g = value,
        RgbaChannel::Blue => color.b = value,
        RgbaChannel::Alpha => color.a = value,
    }
    apply_alpha_mode(color, alpha)
}

pub(super) fn adjust_channel(
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
    set_channel(
        color,
        channel,
        clamp_channel(i16::from(current) + delta),
        alpha,
    )
}

pub(super) fn resolve_value(color: Color, alpha: ColorPickerAlpha) -> Color {
    apply_alpha_mode(color, alpha)
}

pub(super) fn color_text(color: Color, alpha: ColorPickerAlpha) -> String {
    if alpha.allows_alpha() {
        return format!("rgba({}, {}, {}, {})", color.r, color.g, color.b, color.a);
    }

    format!("rgb({}, {}, {})", color.r, color.g, color.b)
}

pub(super) fn hex_text(color: Color, alpha: ColorPickerAlpha) -> String {
    if alpha.allows_alpha() {
        return format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            color.r, color.g, color.b, color.a
        );
    }

    format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b)
}
