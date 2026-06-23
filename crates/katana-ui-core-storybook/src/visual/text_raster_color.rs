use cosmic_text::Color;

const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const CHANNEL_MASK: u32 = 0xff;
const OPAQUE_ALPHA: u8 = 0xff;

pub(super) const RGB_MASK: u32 = 0x00ff_ffff;
pub(super) const OPAQUE_TEXT_ALPHA: u8 = OPAQUE_ALPHA;

pub(super) fn text_color(color: u32) -> Color {
    Color::rgba(red(color), green(color), blue(color), OPAQUE_ALPHA)
}

pub(super) fn packed_color(color: Color) -> u32 {
    pack_rgb(color.r(), color.g(), color.b())
}

pub(super) fn pack_rgb(red: u8, green: u8, blue: u8) -> u32 {
    (u32::from(red) << RED_SHIFT) | (u32::from(green) << GREEN_SHIFT) | u32::from(blue)
}

pub(super) fn red(color: u32) -> u8 {
    ((color >> RED_SHIFT) & CHANNEL_MASK) as u8
}

pub(super) fn green(color: u32) -> u8 {
    ((color >> GREEN_SHIFT) & CHANNEL_MASK) as u8
}

pub(super) fn blue(color: u32) -> u8 {
    (color & CHANNEL_MASK) as u8
}
