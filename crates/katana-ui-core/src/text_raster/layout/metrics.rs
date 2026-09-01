use crate::text_raster::model::PlatformTextGraphemeAdvance;
use cosmic_text::{BorrowedWithFontSystem, Buffer};
use std::collections::BTreeMap;
use unicode_segmentation::UnicodeSegmentation;

use super::MIN_GRAPHEME_COUNT;

pub(super) fn collect_grapheme_advances(
    buffer: &mut BorrowedWithFontSystem<'_, Buffer>,
    source_text: &str,
) -> Vec<PlatformTextGraphemeAdvance> {
    let line_offsets = line_offsets(source_text);
    let mut advances = BTreeMap::<(usize, usize), f32>::new();
    for run in buffer.layout_runs() {
        let line_offset = line_offsets.get(run.line_i).copied().unwrap_or_default();
        for glyph in run.glyphs {
            let cluster = &run.text[glyph.start..glyph.end];
            let graphemes = cluster.grapheme_indices(true).collect::<Vec<_>>();
            let advance = glyph.w / graphemes.len().max(MIN_GRAPHEME_COUNT) as f32;
            for (index, grapheme) in graphemes {
                let byte_start = line_offset + glyph.start + index;
                let byte_end = byte_start + grapheme.len();
                *advances.entry((byte_start, byte_end)).or_default() += advance;
            }
        }
    }
    advances
        .into_iter()
        .map(
            |((byte_start, byte_end), advance_px)| PlatformTextGraphemeAdvance {
                byte_start,
                byte_end,
                advance_px,
            },
        )
        .collect()
}

pub(super) fn line_offsets(text: &str) -> Vec<usize> {
    std::iter::once(0)
        .chain(text.char_indices().filter_map(|(index, character)| {
            (character == '\n').then_some(index + MIN_GRAPHEME_COUNT)
        }))
        .collect()
}
