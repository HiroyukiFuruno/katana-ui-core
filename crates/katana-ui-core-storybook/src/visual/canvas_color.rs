const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const CHANNEL_MASK: u32 = 0xff;
const ALPHA_MAX: u32 = 255;

pub(super) fn blend_color(destination: u32, source: u32, alpha: u8) -> u32 {
    let alpha = u32::from(alpha);
    let inverse = ALPHA_MAX - alpha;
    let red = blend_channel(destination, source, alpha, inverse, RED_SHIFT);
    let green = blend_channel(destination, source, alpha, inverse, GREEN_SHIFT);
    let blue = blend_channel(destination, source, alpha, inverse, 0);
    (red << RED_SHIFT) | (green << GREEN_SHIFT) | blue
}

fn blend_channel(destination: u32, source: u32, alpha: u32, inverse: u32, shift: u32) -> u32 {
    let destination_channel = (destination >> shift) & CHANNEL_MASK;
    let source_channel = (source >> shift) & CHANNEL_MASK;
    (source_channel * alpha + destination_channel * inverse) / ALPHA_MAX
}
