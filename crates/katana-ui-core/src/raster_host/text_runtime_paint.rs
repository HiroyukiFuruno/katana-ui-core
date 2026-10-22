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
    _scale_factor: f32,
    raster_vertical_scale: f32,
) {
    let origin_x = canvas.physical_text_origin_x(origin_x);
    let origin_y = canvas.physical_text_origin_y(origin_y);
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

pub(super) fn record_runtime_text_run(
    canvas: &mut Canvas,
    text: &str,
    raster: &PlatformTextRaster,
    x: isize,
    y: f32,
) {
    record_runtime_text_run_with_raster_prefix(canvas, text, raster, x, y, 0, None);
}

pub(super) fn record_runtime_text_run_with_raster_prefix(
    canvas: &mut Canvas,
    text: &str,
    raster: &PlatformTextRaster,
    x: isize,
    y: f32,
    raster_prefix_len: usize,
    resolved_rtl: Option<bool>,
) {
    let Some(origin_x) = usize::try_from(x).ok() else {
        return;
    };
    let (glyph_positions, selection_width) =
        selection_glyph_positions_with_raster_prefix(text, raster, raster_prefix_len);
    canvas.record_text_run_with_glyph_positions_with_bidi_direction(
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
        &glyph_positions,
        resolved_rtl,
    );
}

pub(super) fn text_run_selection_width(
    text: &str,
    raster: &PlatformTextRaster,
    raster_prefix_len: usize,
) -> usize {
    selection_glyph_positions_with_raster_prefix(text, raster, raster_prefix_len).1
}

#[cfg(test)]
pub(super) fn selection_glyph_widths(
    text: &str,
    raster: &PlatformTextRaster,
) -> (Vec<usize>, usize) {
    let (glyph_positions, selection_width) =
        selection_glyph_positions_with_raster_prefix(text, raster, 0);
    (
        glyph_positions
            .into_iter()
            .map(|(_, width)| width)
            .collect(),
        selection_width,
    )
}

#[cfg(test)]
pub(super) fn selection_glyph_positions(
    text: &str,
    raster: &PlatformTextRaster,
) -> (Vec<(usize, usize)>, usize) {
    selection_glyph_positions_with_raster_prefix(text, raster, 0)
}

fn selection_glyph_positions_with_raster_prefix(
    text: &str,
    raster: &PlatformTextRaster,
    raster_prefix_len: usize,
) -> (Vec<(usize, usize)>, usize) {
    let mut fallback_right = 0usize;
    let mut selection_right = 0usize;
    let positions = text
        .grapheme_indices(true)
        .map(|(byte_start, grapheme)| {
            let (x, width) = raster
                .grapheme_bounds
                .iter()
                .find(|bounds| {
                    bounds.byte_start == raster_prefix_len + byte_start
                        && bounds.byte_end == raster_prefix_len + byte_start + grapheme.len()
                })
                .map(|bounds| (logical_origin(bounds.x), logical_bound_extent(bounds.width)))
                .unwrap_or((fallback_right, 1));
            let right = x.saturating_add(width);
            fallback_right = fallback_right.max(right);
            selection_right = selection_right.max(right);
            (x, width)
        })
        .collect::<Vec<_>>();
    (positions, selection_right.max(1))
}

fn logical_extent(extent: usize, scale_factor: f32) -> usize {
    (extent as f64 / f64::from(TextRenderer::normalized_scale_factor(scale_factor)))
        .ceil()
        .max(1.0) as usize
}

fn logical_origin(position: f32) -> usize {
    position.ceil().max(0.0) as usize
}

fn logical_bound_extent(extent: f32) -> usize {
    extent.ceil().max(1.0) as usize
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
