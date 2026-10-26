use crate::text_raster::model::{
    PlatformTextGraphemeBounds, PlatformTextRasterError, RGBA_CHANNEL_COUNT, TRANSPARENT_RGBA,
};
use cosmic_text::{BorrowedWithFontSystem, Buffer, Color, SwashCache};
use std::collections::BTreeMap;
use unicode_segmentation::UnicodeSegmentation;

use super::{
    MAX_RASTER_DIMENSION, MAX_RASTER_PIXELS, MIN_GRAPHEME_COUNT, MIN_GRAPHEME_EXTENT_PX,
    MIN_RASTER_DIMENSION, OPAQUE_COLOR_CHANNEL, metrics::line_offsets,
};

pub(super) fn collect_grapheme_bounds(
    buffer: &mut BorrowedWithFontSystem<'_, Buffer>,
    source_text: &str,
    scale: f32,
) -> Vec<PlatformTextGraphemeBounds> {
    let line_offsets = line_offsets(source_text);
    let line_height = buffer.metrics().line_height / scale;
    let mut line_metrics = BTreeMap::<usize, (f32, f32, f32, usize)>::new();
    let mut bounds = BTreeMap::<(usize, usize), PlatformTextGraphemeBounds>::new();
    for run in buffer.layout_runs() {
        let line_offset = line_offsets.get(run.line_i).copied().unwrap_or_default();
        if let Some(terminal) = run.glyphs.iter().max_by_key(|glyph| glyph.end) {
            let terminal_end = line_offset + terminal.end;
            let terminal_x = if terminal.level.is_rtl() {
                terminal.x / scale
            } else {
                (terminal.x + terminal.w) / scale
            };
            let entry = line_metrics.entry(run.line_i).or_insert((
                terminal_x,
                run.line_top / scale,
                run.line_height / scale,
                0,
            ));
            if terminal_end >= entry.3 {
                entry.0 = terminal_x;
                entry.1 = run.line_top / scale;
                entry.2 = run.line_height / scale;
                entry.3 = terminal_end;
            }
        }
        for glyph in run.glyphs {
            let cluster = &run.text[glyph.start..glyph.end];
            let graphemes = cluster.grapheme_indices(true).collect::<Vec<_>>();
            let grapheme_width = glyph.w / graphemes.len().max(MIN_GRAPHEME_COUNT) as f32 / scale;
            for (index, grapheme) in graphemes {
                let byte_start = line_offset + glyph.start + index;
                let byte_end = byte_start + grapheme.len();
                let candidate = PlatformTextGraphemeBounds {
                    byte_start,
                    byte_end,
                    x: glyph.x / scale + grapheme_width * grapheme_index(cluster, index) as f32,
                    y: run.line_top / scale,
                    width: grapheme_width.max(MIN_GRAPHEME_EXTENT_PX),
                    height: (run.line_height / scale).max(MIN_GRAPHEME_EXTENT_PX),
                };
                merge_bounds(&mut bounds, candidate);
            }
        }
    }
    add_hard_break_bounds(source_text, &mut bounds, &line_metrics, line_height);
    bounds.into_values().collect()
}

fn add_hard_break_bounds(
    text: &str,
    bounds: &mut BTreeMap<(usize, usize), PlatformTextGraphemeBounds>,
    lines: &BTreeMap<usize, (f32, f32, f32, usize)>,
    line_height: f32,
) {
    let mut logical_row = 0;
    let mut next_y = 0.0;
    for (start, grapheme) in text.grapheme_indices(true) {
        if !grapheme
            .chars()
            .all(|character| matches!(character, '\n' | '\r' | '\u{2028}' | '\u{2029}'))
        {
            continue;
        }
        let (x, y, height, _) =
            lines
                .get(&logical_row)
                .copied()
                .unwrap_or((0.0, next_y, line_height, 0));
        let end = start + grapheme.len();
        bounds.insert(
            (start, end),
            PlatformTextGraphemeBounds {
                byte_start: start,
                byte_end: end,
                x,
                y,
                width: MIN_GRAPHEME_EXTENT_PX,
                height: height.max(MIN_GRAPHEME_EXTENT_PX),
            },
        );
        logical_row += 1;
        next_y = y + height;
    }
}

fn grapheme_index(cluster: &str, byte_index: usize) -> usize {
    cluster
        .grapheme_indices(true)
        .position(|(index, _)| index == byte_index)
        .unwrap_or_default()
}

fn merge_bounds(
    bounds: &mut BTreeMap<(usize, usize), PlatformTextGraphemeBounds>,
    candidate: PlatformTextGraphemeBounds,
) {
    let key = (candidate.byte_start, candidate.byte_end);
    let Some(current) = bounds.get_mut(&key) else {
        bounds.insert(key, candidate);
        return;
    };
    let right = (current.x + current.width).max(candidate.x + candidate.width);
    let bottom = (current.y + current.height).max(candidate.y + candidate.height);
    current.x = current.x.min(candidate.x);
    current.y = current.y.min(candidate.y);
    current.width = (right - current.x).max(MIN_GRAPHEME_EXTENT_PX);
    current.height = (bottom - current.y).max(MIN_GRAPHEME_EXTENT_PX);
}

pub(super) fn raster_extent(
    bounds: &[PlatformTextGraphemeBounds],
    scale: f32,
) -> Result<(usize, usize), PlatformTextRasterError> {
    let width = bounds
        .iter()
        .map(|bounds| bounds.x + bounds.width)
        .fold(MIN_RASTER_DIMENSION, f32::max);
    let height = bounds
        .iter()
        .map(|bounds| bounds.y + bounds.height)
        .fold(MIN_RASTER_DIMENSION, f32::max);
    let width = raster_dimension(width * scale)?;
    let height = raster_dimension(height * scale)?;
    /* WHY: Both dimensions were bounded above before this multiplication, so
     * it is representable on every supported desktop target. */
    let pixels = width * height;
    if pixels > MAX_RASTER_PIXELS {
        return Err(PlatformTextRasterError::RasterTooLarge {
            width,
            height,
            max_pixels: MAX_RASTER_PIXELS,
        });
    }
    Ok((width, height))
}

fn raster_dimension(value: f32) -> Result<usize, PlatformTextRasterError> {
    if !value.is_finite() {
        return Err(PlatformTextRasterError::NonFiniteLayoutExtent);
    }
    let dimension = value.ceil().max(MIN_RASTER_DIMENSION) as usize;
    if dimension > MAX_RASTER_DIMENSION {
        return Err(PlatformTextRasterError::RasterTooLarge {
            width: dimension,
            height: dimension,
            max_pixels: MAX_RASTER_PIXELS,
        });
    }
    Ok(dimension)
}

pub(super) fn collect_pixels(
    buffer: &mut BorrowedWithFontSystem<'_, Buffer>,
    swash_cache: &mut SwashCache,
    width: usize,
    height: usize,
) -> Vec<[u8; RGBA_CHANNEL_COUNT]> {
    let mut pixels = vec![TRANSPARENT_RGBA; width * height];
    buffer.draw(
        swash_cache,
        Color::rgba(
            OPAQUE_COLOR_CHANNEL,
            OPAQUE_COLOR_CHANNEL,
            OPAQUE_COLOR_CHANNEL,
            OPAQUE_COLOR_CHANNEL,
        ),
        |left, top, width_in_pixels, height_in_pixels, color| {
            for y in 0..height_in_pixels {
                for x in 0..width_in_pixels {
                    let x = left + x as i32;
                    let y = top + y as i32;
                    if x < 0 || y < 0 || x as usize >= width || y as usize >= height {
                        continue;
                    }
                    pixels[y as usize * width + x as usize] =
                        [color.r(), color.g(), color.b(), color.a()];
                }
            }
        },
    );
    pixels
}

#[cfg(test)]
#[path = "raster_tests.rs"]
mod tests;
