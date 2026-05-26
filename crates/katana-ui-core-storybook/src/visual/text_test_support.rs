use super::canvas::Canvas;
use super::text::{TextRenderer, TextVerticalBox};

pub(super) const BACKGROUND: u32 = 0x1e1e1e;
pub(super) const TEXT: u32 = 0xd4d4d4;
pub(super) const CANVAS_WIDTH: usize = 360;
pub(super) const CANVAS_HEIGHT: usize = 80;
pub(super) const TEXT_X: usize = 12;
pub(super) const TEXT_Y: usize = 12;
pub(super) const TEXT_SIZE: f32 = 18.0;
pub(super) const ALIGN_BOX_HEIGHT: f32 = 32.0;
pub(super) const MAX_CENTER_DELTA: f32 = 2.0;
pub(super) const SMALL_CODE_TEXT_SIZE: f32 = 10.0;
pub(super) const SMALL_CODE_BOX_HEIGHT: f32 = 24.0;
pub(super) const MAX_CODE_GLYPH_CENTER_DELTA: f32 = 1.5;
pub(super) const WIDGET_LABEL_TEXT_SIZE: f32 = 14.0;
pub(super) const WIDGET_LABEL_BOX_HEIGHT: f32 = 28.0;

const RED_CHANNEL_SHIFT: u32 = 16;
const GREEN_CHANNEL_SHIFT: u32 = 8;
const COLOR_CHANNEL_MASK: u32 = 0xff;
const RGB_CHANNEL_COUNT: u32 = 3;

pub(super) fn centered_text_delta(renderer: &TextRenderer, sample: &str) -> f32 {
    centered_text_delta_with_size(renderer, sample, TEXT_SIZE, ALIGN_BOX_HEIGHT)
}

pub(super) fn centered_text_delta_with_size(
    renderer: &TextRenderer,
    sample: &str,
    size: f32,
    box_height: f32,
) -> f32 {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);
    renderer.draw_centered(
        &mut canvas,
        sample,
        TEXT_X,
        TextVerticalBox::new(TEXT_Y, box_height),
        size,
        TEXT,
    );
    let bounds = ink_vertical_bounds(&canvas);
    let ink_center = (bounds.top + bounds.bottom) as f32 / 2.0;
    let box_center = TEXT_Y as f32 + box_height / 2.0;
    (ink_center - box_center).abs()
}

pub(super) fn antialias_pixels_for_draw(renderer: &TextRenderer, sample: &str, size: f32) -> usize {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);
    renderer.draw(&mut canvas, sample, TEXT_X, TEXT_Y, size, TEXT);
    antialias_pixel_count(&canvas)
}

pub(super) fn antialias_pixels_for_centered_draw(
    renderer: &TextRenderer,
    sample: &str,
    size: f32,
) -> usize {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT, BACKGROUND);
    renderer.draw_centered(
        &mut canvas,
        sample,
        TEXT_X,
        TextVerticalBox::new(TEXT_Y, WIDGET_LABEL_BOX_HEIGHT),
        size,
        TEXT,
    );
    antialias_pixel_count(&canvas)
}

pub(super) fn antialias_pixel_count_for_colors(
    canvas: &Canvas,
    background: u32,
    text: u32,
) -> usize {
    canvas
        .pixels()
        .iter()
        .filter(|&&pixel| pixel != background && pixel != text)
        .count()
}

pub(super) fn average_alpha_for_antialias_pixels(
    canvas: &Canvas,
    background: u32,
    text: u32,
) -> f32 {
    let mut alpha_sum = 0u32;
    let mut count = 0u32;
    for &pixel in canvas.pixels() {
        if pixel == background || pixel == text {
            continue;
        }
        let red = (pixel >> RED_CHANNEL_SHIFT) & COLOR_CHANNEL_MASK;
        let green = (pixel >> GREEN_CHANNEL_SHIFT) & COLOR_CHANNEL_MASK;
        let blue = pixel & COLOR_CHANNEL_MASK;
        alpha_sum += (red + green + blue) / RGB_CHANNEL_COUNT;
        count += 1;
    }
    if count == 0 {
        return 0.0;
    }
    alpha_sum as f32 / count as f32
}

pub(super) fn antialias_intensity_levels_count(
    canvas: &Canvas,
    background: u32,
    text: u32,
) -> usize {
    let mut levels = std::collections::HashSet::<u32>::new();
    for &pixel in canvas.pixels() {
        if pixel == background || pixel == text {
            continue;
        }
        let red = (pixel >> RED_CHANNEL_SHIFT) & COLOR_CHANNEL_MASK;
        let green = (pixel >> GREEN_CHANNEL_SHIFT) & COLOR_CHANNEL_MASK;
        let blue = pixel & COLOR_CHANNEL_MASK;
        let intensity = (red + green + blue) / RGB_CHANNEL_COUNT;
        levels.insert(intensity);
    }
    levels.len()
}

pub(super) fn scale_nearest(canvas: &Canvas, scale: usize) -> Canvas {
    let mut output = Canvas::new(canvas.width() * scale, canvas.height() * scale, BACKGROUND);
    for y in 0..canvas.height() {
        for x in 0..canvas.width() {
            let color = canvas.pixels()[y * canvas.width() + x];
            for offset_y in 0..scale {
                for offset_x in 0..scale {
                    output.set(x * scale + offset_x, y * scale + offset_y, color);
                }
            }
        }
    }
    output
}

fn antialias_pixel_count(canvas: &Canvas) -> usize {
    canvas
        .pixels()
        .iter()
        .filter(|&&pixel| pixel != BACKGROUND && pixel != TEXT)
        .count()
}

struct VerticalBounds {
    top: usize,
    bottom: usize,
}

fn ink_vertical_bounds(canvas: &Canvas) -> VerticalBounds {
    let mut top = canvas.height();
    let mut bottom = 0;
    for (index, pixel) in canvas.pixels().iter().enumerate() {
        if *pixel == BACKGROUND {
            continue;
        }
        let y = index / canvas.width();
        top = top.min(y);
        bottom = bottom.max(y);
    }
    VerticalBounds { top, bottom }
}
