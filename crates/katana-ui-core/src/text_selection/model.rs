use super::{UiTextGlyphBox, UiTextLineBox, UiTextPasteResult, UiTextSelectionRange};
use crate::render_model::UiRect;
use serde::{Deserialize, Serialize};
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiTextSelectionModel {
    text: String,
    lines: Vec<UiTextLineBox>,
    #[serde(skip)]
    resolved_bidi_rtl: Option<bool>,
}

impl PartialEq for UiTextSelectionModel {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
            && self.lines == other.lines
            && self.resolved_bidi_rtl == other.resolved_bidi_rtl
    }
}

impl Eq for UiTextSelectionModel {}

#[path = "model_caret.rs"]
mod model_caret;

impl UiTextSelectionModel {
    #[must_use]
    pub fn new(text: impl Into<String>, lines: Vec<UiTextLineBox>) -> Self {
        Self {
            text: text.into(),
            lines,
            resolved_bidi_rtl: None,
        }
    }

    #[must_use]
    #[cfg(feature = "raster-host")]
    pub(crate) fn with_resolved_bidi_direction(
        text: impl Into<String>,
        lines: Vec<UiTextLineBox>,
        resolved_bidi_rtl: bool,
    ) -> Self {
        Self {
            text: text.into(),
            lines,
            resolved_bidi_rtl: Some(resolved_bidi_rtl),
        }
    }

    #[must_use]
    pub fn from_monospace_text(
        text: impl Into<String>,
        origin_x: i32,
        origin_y: i32,
        char_width: u32,
        line_height: u32,
    ) -> Self {
        let text = text.into();
        let mut byte_offset = 0usize;
        let mut grapheme_index = 0usize;
        let mut lines = Vec::new();
        for (line_index, line) in text.split('\n').enumerate() {
            let line_start = byte_offset;
            let mut glyphs = Vec::new();
            let mut x = origin_x;
            for grapheme in line.graphemes(true) {
                let byte_start = byte_offset;
                byte_offset += grapheme.len();
                glyphs.push(
                    UiTextGlyphBox::new(
                        grapheme_index,
                        byte_start..byte_offset,
                        UiRect::new(
                            x,
                            origin_y + (line_index as i32 * line_height as i32),
                            char_width,
                            line_height,
                        ),
                        origin_y + ((line_index + 1) as i32 * line_height as i32),
                    )
                    .with_text(grapheme),
                );
                grapheme_index += 1;
                x += char_width as i32;
            }
            lines.push(UiTextLineBox::new(line_start..byte_offset, glyphs));
            byte_offset = byte_offset.saturating_add(1);
        }
        Self::new(text, lines)
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub fn point_to_caret(&self, x: i32, y: i32) -> usize {
        self.point_to_caret_position(x, y).position
    }

    #[must_use]
    pub fn drag_range(&self, start: (i32, i32), end: (i32, i32)) -> UiTextSelectionRange {
        UiTextSelectionRange::new(
            self.point_to_caret(start.0, start.1),
            self.point_to_caret(end.0, end.1),
        )
    }

    #[must_use]
    pub fn highlight_rects(&self, selection: UiTextSelectionRange) -> Vec<UiRect> {
        if selection.is_collapsed() {
            return Vec::new();
        }
        let range = selection.ordered();
        self.glyphs_in_range(range)
            .map(|glyph| glyph.bounds)
            .collect()
    }

    #[must_use]
    pub fn selected_text(&self, selection: UiTextSelectionRange) -> String {
        if selection.is_collapsed() {
            return String::new();
        }
        let range = selection.ordered();
        self.glyphs_in_range(range)
            .map(|glyph| {
                if glyph.text.is_empty() {
                    self.text
                        .get(glyph.byte_range.clone())
                        .unwrap_or_default()
                        .to_string()
                } else {
                    glyph.text.clone()
                }
            })
            .collect()
    }

    #[must_use]
    pub fn replace_selection(
        &self,
        selection: UiTextSelectionRange,
        replacement: &str,
    ) -> UiTextPasteResult {
        let range = selection.ordered();
        let (byte_start, byte_end) = self.byte_range_for_grapheme_range(range.clone());
        let mut text = self.text.clone();
        text.replace_range(byte_start..byte_end, replacement);
        let inserted_graphemes = replacement.graphemes(true).count();
        let caret = range.start.saturating_add(inserted_graphemes);
        UiTextPasteResult {
            text,
            selection: UiTextSelectionRange::caret(caret),
        }
    }

    #[must_use]
    pub(crate) fn replace_grapheme_range(
        value: &str,
        selection: UiTextSelectionRange,
        replacement: &str,
    ) -> UiTextPasteResult {
        Self::from_monospace_text(value, 0, 0, 1, 1).replace_selection(selection, replacement)
    }

    fn line_for_y(&self, y: i32) -> Option<&UiTextLineBox> {
        self.lines.iter().find(|line| {
            line.glyphs.first().is_some_and(|first| {
                y >= first.bounds.y && y <= first.bounds.y + first.bounds.height as i32
            })
        })
    }

    fn all_glyphs(&self) -> impl Iterator<Item = &UiTextGlyphBox> {
        self.lines.iter().flat_map(|line| line.glyphs.iter())
    }

    fn glyphs_in_range(&self, range: Range<usize>) -> impl Iterator<Item = &UiTextGlyphBox> {
        self.all_glyphs()
            .filter(move |glyph| range.contains(&glyph.grapheme_index))
    }

    fn byte_range_for_grapheme_range(&self, range: Range<usize>) -> (usize, usize) {
        let mut byte_start = self.text.len();
        let mut byte_end = self.text.len();
        for glyph in self.all_glyphs() {
            if glyph.grapheme_index == range.start {
                byte_start = glyph.byte_range.start;
            }
            if glyph.grapheme_index + 1 == range.end {
                byte_end = glyph.byte_range.end;
            }
        }
        if range.start == range.end {
            byte_start = self
                .all_glyphs()
                .find(|glyph| glyph.grapheme_index == range.start)
                .map_or(self.text.len(), |glyph| glyph.byte_range.start);
            byte_end = byte_start;
        }
        (
            byte_start.min(self.text.len()),
            byte_end.min(self.text.len()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_selection::{UiTextCaretHit, UiTextCaretPosition};

    #[test]
    fn bidi_run_boundaries_keep_both_visual_caret_candidates() {
        let text = "a\u{05d0}\u{05d1}b";
        let model = UiTextSelectionModel::new(
            text,
            vec![UiTextLineBox::new(
                0..text.len(),
                vec![
                    UiTextGlyphBox::new(0, 0..1, UiRect::new(0, 0, 10, 10), 10),
                    UiTextGlyphBox::new(1, 1..3, UiRect::new(20, 0, 10, 10), 10),
                    UiTextGlyphBox::new(2, 3..5, UiRect::new(10, 0, 10, 10), 10),
                    UiTextGlyphBox::new(3, 5..6, UiRect::new(30, 0, 10, 10), 10),
                ],
            )],
        );

        assert_eq!(3, model.point_to_caret(11, 5));
        assert_eq!(2, model.point_to_caret(19, 5));
        assert_eq!(2, model.point_to_caret(21, 5));
        assert_eq!(1, model.point_to_caret(29, 5));

        let selection = model.drag_range((11, 5), (19, 5));
        assert_eq!(UiTextSelectionRange::new(3, 2), selection);
        assert_eq!(
            vec![UiRect::new(10, 0, 10, 10)],
            model.highlight_rects(selection)
        );
    }

    #[test]
    fn selected_visual_caret_affinity_is_available_for_rendering() {
        let text = "a\u{05d0}\u{05d1}b";
        let model = UiTextSelectionModel::new(
            text,
            vec![UiTextLineBox::new(
                0..text.len(),
                vec![
                    UiTextGlyphBox::new(0, 0..1, UiRect::new(0, 0, 10, 10), 10),
                    UiTextGlyphBox::new(1, 1..3, UiRect::new(20, 0, 10, 10), 10),
                    UiTextGlyphBox::new(2, 3..5, UiRect::new(10, 0, 10, 10), 10),
                    UiTextGlyphBox::new(3, 5..6, UiRect::new(30, 0, 10, 10), 10),
                ],
            )],
        );

        let affinity = model.point_to_caret_position(11, 5);

        assert_eq!(
            UiTextCaretPosition {
                position: 3,
                x: 10,
                line_y: 0,
                glyph_index: Some(2),
            },
            affinity
        );
        assert_eq!(
            UiRect::new(10, 0, 1, 10),
            model.caret_rect_with_affinity(UiTextSelectionRange::caret(3), Some(affinity))
        );
    }

    #[test]
    fn affinity_caret_uses_the_hit_glyph_height_in_mixed_bidi_line() {
        let text = "a\u{05d0}";
        let model = UiTextSelectionModel::new(
            text,
            vec![UiTextLineBox::new(
                0..text.len(),
                vec![
                    UiTextGlyphBox::new(0, 0..1, UiRect::new(0, 0, 10, 8), 8),
                    UiTextGlyphBox::new(1, 1..3, UiRect::new(10, 0, 10, 24), 24),
                ],
            )],
        );

        let affinity = model.point_to_caret_position(19, 5);

        assert_eq!(
            UiTextCaretPosition {
                position: 1,
                x: 20,
                line_y: 0,
                glyph_index: Some(1),
            },
            affinity,
            "the hit resolves to the RTL glyph's visual candidate"
        );
        assert_eq!(
            UiRect::new(20, 0, 1, 24),
            model.caret_rect_with_affinity(UiTextSelectionRange::caret(1), Some(affinity)),
            "the caret must use the glyph that emitted the hit affinity"
        );
    }

    #[test]
    fn caret_hits_with_different_glyphs_are_not_equal_at_the_same_position() {
        let position = UiTextCaretPosition {
            position: 1,
            x: 20,
            line_y: 0,
            glyph_index: None,
        };
        let first = UiTextCaretHit::from_position_and_glyph(position, Some(0));
        let second = UiTextCaretHit::from_position_and_glyph(position, Some(1));

        assert_ne!(first, second);
    }

    #[test]
    fn resolved_bidi_direction_does_not_change_the_serialized_model_shape() -> serde_json::Result<()>
    {
        let model = UiTextSelectionModel::new(
            "\u{05d0}",
            vec![UiTextLineBox::new(
                0..2,
                vec![UiTextGlyphBox::new(0, 0..2, UiRect::new(10, 0, 10, 10), 10)],
            )],
        );

        let serialized = serde_json::to_value(&model)?;
        assert_eq!(Some(2), serialized.as_object().map(serde_json::Map::len));
        assert!(
            serialized
                .as_object()
                .is_some_and(|object| object.contains_key("text"))
        );
        assert!(
            serialized
                .as_object()
                .is_some_and(|object| object.contains_key("lines"))
        );
        let restored: UiTextSelectionModel = serde_json::from_value(serialized)?;
        assert_eq!(1, restored.point_to_caret(11, 5));
        assert_eq!(0, restored.point_to_caret(19, 5));
        Ok(())
    }

    #[test]
    fn resolved_bidi_direction_participates_in_model_equality() {
        let ltr = UiTextSelectionModel::new(
            "a",
            vec![UiTextLineBox::new(
                0..1,
                vec![UiTextGlyphBox::new(0, 0..1, UiRect::new(0, 0, 10, 10), 10)],
            )],
        );
        let mut rtl = ltr.clone();
        rtl.resolved_bidi_rtl = Some(true);

        assert_ne!(ltr, rtl);
    }

    #[test]
    fn a_single_rtl_grapheme_uses_its_resolved_direction() {
        let model = UiTextSelectionModel::new(
            "\u{05d0}",
            vec![UiTextLineBox::new(
                0..2,
                vec![UiTextGlyphBox::new(0, 0..2, UiRect::new(10, 0, 10, 10), 10)],
            )],
        );

        assert_eq!(1, model.point_to_caret(11, 5));
        assert_eq!(0, model.point_to_caret(19, 5));
    }
}
