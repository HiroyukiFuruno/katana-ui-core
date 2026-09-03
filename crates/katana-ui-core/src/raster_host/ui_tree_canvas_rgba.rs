use super::ui_tree_canvas_types::RgbaBlitRequest;

const RED_CHANNEL_INDEX: usize = 0;
const GREEN_CHANNEL_INDEX: usize = 1;
const BLUE_CHANNEL_INDEX: usize = 2;
const ALPHA_CHANNEL_INDEX: usize = 3;
const RGBA_CHANNEL_COUNT: u32 = 4;
const RGBA_SAMPLE_CHANNEL_COUNT: usize = 4;
const PREMULTIPLIED_SAMPLE_CHANNEL_COUNT: usize = 4;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const ALPHA_MAX: f32 = 255.0;
type RgbaSample = [u8; RGBA_SAMPLE_CHANNEL_COUNT];
type PremultipliedSample = [f32; PREMULTIPLIED_SAMPLE_CHANNEL_COUNT];

pub(super) fn rgba_alpha(sample: RgbaSample) -> u8 {
    sample[ALPHA_CHANNEL_INDEX]
}

pub(super) fn rgba_sample(source: &RgbaBlitRequest<'_>, x: f32, y: f32) -> RgbaSample {
    let max_x = source.width.saturating_sub(1);
    let max_y = source.height.saturating_sub(1);
    let left = x.floor().max(0.0) as u32;
    let top = y.floor().max(0.0) as u32;
    let right = left.saturating_add(1).min(max_x);
    let bottom = top.saturating_add(1).min(max_y);
    let tx = (x - left as f32).clamp(0.0, 1.0);
    let ty = (y - top as f32).clamp(0.0, 1.0);
    let top_color = mix_rgba(
        premultiply(pixel(source, left, top)),
        premultiply(pixel(source, right, top)),
        tx,
    );
    let bottom_color = mix_rgba(
        premultiply(pixel(source, left, bottom)),
        premultiply(pixel(source, right, bottom)),
        tx,
    );
    unpremultiply(mix_rgba(top_color, bottom_color, ty))
}

pub(super) fn packed_rgb(sample: RgbaSample) -> u32 {
    u32::from(sample[RED_CHANNEL_INDEX]) << RED_SHIFT
        | u32::from(sample[GREEN_CHANNEL_INDEX]) << GREEN_SHIFT
        | u32::from(sample[BLUE_CHANNEL_INDEX])
}

fn pixel(source: &RgbaBlitRequest<'_>, x: u32, y: u32) -> RgbaSample {
    let offset = ((y * source.width + x) * RGBA_CHANNEL_COUNT) as usize;
    [
        source.rgba[offset + RED_CHANNEL_INDEX],
        source.rgba[offset + GREEN_CHANNEL_INDEX],
        source.rgba[offset + BLUE_CHANNEL_INDEX],
        source.rgba[offset + ALPHA_CHANNEL_INDEX],
    ]
}

fn premultiply(sample: RgbaSample) -> PremultipliedSample {
    let alpha = f32::from(sample[ALPHA_CHANNEL_INDEX]) / ALPHA_MAX;
    [
        f32::from(sample[RED_CHANNEL_INDEX]) * alpha,
        f32::from(sample[GREEN_CHANNEL_INDEX]) * alpha,
        f32::from(sample[BLUE_CHANNEL_INDEX]) * alpha,
        f32::from(sample[ALPHA_CHANNEL_INDEX]),
    ]
}

fn mix_rgba(left: PremultipliedSample, right: PremultipliedSample, t: f32) -> PremultipliedSample {
    [
        mix_channel(left[RED_CHANNEL_INDEX], right[RED_CHANNEL_INDEX], t),
        mix_channel(left[GREEN_CHANNEL_INDEX], right[GREEN_CHANNEL_INDEX], t),
        mix_channel(left[BLUE_CHANNEL_INDEX], right[BLUE_CHANNEL_INDEX], t),
        mix_channel(left[ALPHA_CHANNEL_INDEX], right[ALPHA_CHANNEL_INDEX], t),
    ]
}

fn unpremultiply(sample: PremultipliedSample) -> RgbaSample {
    let alpha = sample[ALPHA_CHANNEL_INDEX].round().clamp(0.0, ALPHA_MAX);
    if alpha == 0.0 {
        return [0, 0, 0, 0];
    }
    let scale = ALPHA_MAX / alpha;
    [
        unpremultiplied_channel(sample[RED_CHANNEL_INDEX], scale),
        unpremultiplied_channel(sample[GREEN_CHANNEL_INDEX], scale),
        unpremultiplied_channel(sample[BLUE_CHANNEL_INDEX], scale),
        alpha as u8,
    ]
}

fn unpremultiplied_channel(channel: f32, scale: f32) -> u8 {
    (channel * scale).round().clamp(0.0, ALPHA_MAX) as u8
}

fn mix_channel(left: f32, right: f32, t: f32) -> f32 {
    left + (right - left) * t
}
