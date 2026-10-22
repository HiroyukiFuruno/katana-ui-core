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
    let mut offsets = vec![0];
    let mut characters = text.char_indices().peekable();
    while let Some((index, character)) = characters.next() {
        if !matches!(character, '\n' | '\r' | '\u{2028}' | '\u{2029}') {
            continue;
        }
        if character == '\r'
            && matches!(characters.peek().copied(), Some((_, '\n')))
            && let Some((next_index, next)) = characters.next()
        {
            offsets.push(next_index + next.len_utf8());
            continue;
        }
        offsets.push(index + character.len_utf8());
    }
    offsets
}

#[cfg(test)]
mod tests {
    use super::line_offsets;

    #[test]
    fn line_offsets_preserve_utf8_byte_offsets_for_all_line_separators() {
        assert_eq!(
            line_offsets("a\nb\rc\u{2028}d\u{2029}e"),
            vec![0, 2, 4, 8, 12]
        );
    }

    #[test]
    fn line_offsets_treat_crlf_as_one_source_separator() {
        assert_eq!(line_offsets("a\r\nb"), vec![0, 3]);
    }
}
