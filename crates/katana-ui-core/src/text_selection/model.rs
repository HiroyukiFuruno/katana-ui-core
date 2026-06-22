use super::{UiTextGlyphBox, UiTextLineBox, UiTextPasteResult, UiTextSelectionRange};
use crate::render_model::UiRect;
use serde::{Deserialize, Serialize};
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTextSelectionModel {
    text: String,
    lines: Vec<UiTextLineBox>,
}

impl UiTextSelectionModel {
    #[must_use]
    pub fn new(text: impl Into<String>, lines: Vec<UiTextLineBox>) -> Self {
        Self {
            text: text.into(),
            lines,
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
        Self { text, lines }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub fn point_to_caret(&self, x: i32, y: i32) -> usize {
        let Some(line) = self.line_for_y(y).or_else(|| self.lines.first()) else {
            return 0;
        };
        let Some(first) = line.glyphs.first() else {
            return 0;
        };
        if x <= first.caret_x_before() {
            return first.grapheme_index;
        }
        for glyph in &line.glyphs {
            let midpoint = glyph.bounds.x + (glyph.bounds.width as i32 / 2);
            if x < midpoint {
                return glyph.grapheme_index;
            }
            if x <= glyph.caret_x_after() {
                return glyph.grapheme_index + 1;
            }
        }
        line.glyphs
            .last()
            .map_or(first.grapheme_index, |glyph| glyph.grapheme_index + 1)
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
    pub fn caret_rect(&self, selection: UiTextSelectionRange) -> UiRect {
        let caret = selection.caret_position();
        if let Some(glyph) = self
            .all_glyphs()
            .find(|glyph| glyph.grapheme_index == caret)
        {
            return UiRect::new(
                glyph.caret_x_before(),
                glyph.bounds.y,
                1,
                glyph.bounds.height,
            );
        }
        if let Some(glyph) = self
            .all_glyphs()
            .find(|glyph| glyph.grapheme_index + 1 == caret)
        {
            return UiRect::new(
                glyph.caret_x_after(),
                glyph.bounds.y,
                1,
                glyph.bounds.height,
            );
        }
        UiRect::default()
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
