use super::{UiTextCaretPosition, UiTextGlyphBox, UiTextLineBox};
use unicode_bidi::BidiInfo;

pub(super) fn visual_caret_glyphs<'a>(
    line: &'a UiTextLineBox,
    text: &str,
    resolved_bidi_rtl: Option<bool>,
) -> Vec<(&'a UiTextGlyphBox, UiTextCaretPosition)> {
    let bidi = BidiInfo::new(text, None);
    let mut glyphs = line.glyphs.iter().collect::<Vec<_>>();
    glyphs.sort_by_key(|glyph| glyph.grapheme_index);
    let mut positions = Vec::with_capacity(glyphs.len().saturating_mul(2));
    for glyph in glyphs {
        let glyph_is_rtl = resolved_bidi_rtl.unwrap_or_else(|| {
            bidi.levels
                .get(glyph.byte_range.start)
                .is_some_and(unicode_bidi::Level::is_rtl)
        });
        let (before_x, after_x) = if glyph_is_rtl {
            (glyph.caret_x_after(), glyph.caret_x_before())
        } else {
            (glyph.caret_x_before(), glyph.caret_x_after())
        };
        positions.push((
            glyph,
            UiTextCaretPosition {
                position: glyph.grapheme_index,
                x: before_x,
                line_y: glyph.bounds.y,
                glyph_index: Some(glyph.grapheme_index),
            },
        ));
        positions.push((
            glyph,
            UiTextCaretPosition {
                position: glyph.grapheme_index.saturating_add(1),
                x: after_x,
                line_y: glyph.bounds.y,
                glyph_index: Some(glyph.grapheme_index),
            },
        ));
    }
    positions.sort_by_key(|(_, candidate)| candidate.x);
    positions
}
