use super::TextRenderer;
use crate::raster_host::canvas::Canvas;
use katana_ui_core::text_raster::PlatformTextRaster;
use unicode_segmentation::UnicodeSegmentation;

const RGBA_ALPHA_COMPONENT_INDEX: usize = 3;
const RGBA_RED_BIT_SHIFT: u32 = 16;
const RGBA_GREEN_BIT_SHIFT: u32 = 8;
const VERTICAL_SCALE_COVERAGE_ROWS_PER_UNIT: f32 = 6.0;

pub(super) fn draw_raster(
    canvas: &mut Canvas,
    raster: &PlatformTextRaster,
    origin_x: isize,
    origin_y: f32,
    scale_factor: f32,
    raster_vertical_scale: f32,
) {
    let scale = TextRenderer::normalized_scale_factor(scale_factor);
    let origin_x = (origin_x as f64 * f64::from(scale)).round() as isize;
    let origin_y = physical_draw_origin(origin_y, scale);
    for (index, pixel) in raster.rgba_pixels.iter().enumerate() {
        let [red, green, blue, alpha] = *pixel;
        if alpha == 0 {
            continue;
        }
        let x = origin_x + (index % raster.width) as isize;
        let y = origin_y + (index / raster.width) as isize;
        for extra_y in 0..=extra_vertical_coverage_rows(raster_vertical_scale) {
            let y = y + extra_y;
            if x >= 0 && y >= 0 {
                canvas.blend_physical(
                    x as usize,
                    canvas.translate_physical_y(y as usize),
                    packed_rgb(red, green, blue),
                    alpha,
                );
            }
        }
    }
}

pub(super) fn physical_draw_origin(origin_y: f32, scale_factor: f32) -> isize {
    (f64::from(origin_y) * f64::from(TextRenderer::normalized_scale_factor(scale_factor))).round()
        as isize
}

pub(super) fn record_runtime_text_run(
    canvas: &mut Canvas,
    text: &str,
    raster: &PlatformTextRaster,
    x: isize,
    y: f32,
) {
    let Some(origin_x) = usize::try_from(x).ok() else {
        return;
    };
    let (glyph_widths, selection_width) = selection_glyph_widths(text, raster);
    canvas.record_text_run_with_glyph_widths(
        text,
        origin_x,
        y.round().max(0.0) as usize,
        selection_width,
        raster
            .grapheme_bounds
            .iter()
            .map(|bounds| bounds.height.ceil().max(1.0) as usize)
            .max()
            .unwrap_or(1),
        &glyph_widths,
    );
}

pub(super) fn selection_glyph_widths(
    text: &str,
    raster: &PlatformTextRaster,
) -> (Vec<usize>, usize) {
    let mut right = 0usize;
    let widths = text
        .grapheme_indices(true)
        .map(|(byte_start, grapheme)| {
            let next_right = raster
                .grapheme_bounds
                .iter()
                .find(|bounds| {
                    bounds.byte_start == byte_start
                        && bounds.byte_end == byte_start + grapheme.len()
                })
                .map(|bounds| logical_position(bounds.x + bounds.width))
                .unwrap_or_else(|| right.saturating_add(1));
            let next_right = next_right.max(right.saturating_add(1));
            let width = next_right.saturating_sub(right);
            right = next_right;
            width
        })
        .collect::<Vec<_>>();
    (widths, right.max(1))
}

fn logical_extent(extent: usize, scale_factor: f32) -> usize {
    (extent as f64 / f64::from(TextRenderer::normalized_scale_factor(scale_factor)))
        .ceil()
        .max(1.0) as usize
}

fn logical_position(position: f32) -> usize {
    position.ceil().max(1.0) as usize
}

pub(super) fn visible_raster_width(raster: &PlatformTextRaster, scale_factor: f32) -> usize {
    raster
        .rgba_pixels
        .iter()
        .enumerate()
        .filter(|(_, pixel)| pixel[RGBA_ALPHA_COMPONENT_INDEX] != 0)
        .map(|(index, _)| index % raster.width + 1)
        .max()
        .map(|extent| logical_extent(extent, scale_factor))
        .unwrap_or(1)
}

fn packed_rgb(red: u8, green: u8, blue: u8) -> u32 {
    (u32::from(red) << RGBA_RED_BIT_SHIFT)
        | (u32::from(green) << RGBA_GREEN_BIT_SHIFT)
        | u32::from(blue)
}

fn extra_vertical_coverage_rows(scale: f32) -> isize {
    ((scale - 1.0).max(0.0) * VERTICAL_SCALE_COVERAGE_ROWS_PER_UNIT).ceil() as isize
}
