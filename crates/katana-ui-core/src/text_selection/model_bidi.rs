use super::{UiTextCaretPosition, UiTextLineBox};
use unicode_bidi::BidiInfo;

pub(super) fn visual_caret_positions(line: &UiTextLineBox, text: &str) -> Vec<UiTextCaretPosition> {
    let bidi = BidiInfo::new(text, None);
    let mut glyphs = line.glyphs.iter().collect::<Vec<_>>();
    glyphs.sort_by_key(|glyph| glyph.grapheme_index);
    let mut positions = Vec::with_capacity(glyphs.len().saturating_mul(2));
    for glyph in glyphs {
        let (before_x, after_x) = if bidi
            .levels
            .get(glyph.byte_range.start)
            .is_some_and(unicode_bidi::Level::is_rtl)
        {
            (glyph.caret_x_after(), glyph.caret_x_before())
        } else {
            (glyph.caret_x_before(), glyph.caret_x_after())
        };
        positions.push(UiTextCaretPosition {
            position: glyph.grapheme_index,
            x: before_x,
            line_y: glyph.bounds.y,
        });
        positions.push(UiTextCaretPosition {
            position: glyph.grapheme_index.saturating_add(1),
            x: after_x,
            line_y: glyph.bounds.y,
        });
    }
    positions.sort_by_key(|candidate| candidate.x);
    positions
}
