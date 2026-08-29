use super::layout_model::{
    TextSurfaceCompositionLayout, TextSurfaceGraphemeBox, TextSurfaceLayout, TextSurfaceLineBox,
};
use crate::render_model::UiRect;
use crate::text_selection::{
    UiTextGlyphBox, UiTextLineBox, UiTextSelectionModel, UiTextSelectionRange,
};
use std::collections::BTreeMap;
use unicode_segmentation::UnicodeSegmentation;

impl TextSurfaceLayout {
    #[must_use]
    pub fn new(identity: impl Into<String>, content_bounds: UiRect) -> Self {
        Self {
            identity: identity.into(),
            content_bounds,
            graphemes: Vec::new(),
            lines: Vec::new(),
            text: String::new(),
            composition: None,
            selection_model: UiTextSelectionModel::new(String::new(), Vec::new()),
        }
    }

    #[must_use]
    pub fn from_grapheme_boxes(
        identity: impl Into<String>,
        content_bounds: UiRect,
        text: impl Into<String>,
        graphemes: Vec<TextSurfaceGraphemeBox>,
    ) -> Self {
        let text = text.into();
        let lines = Self::line_boxes(&graphemes);
        let selection_model = Self::selection_model(&text, &graphemes);
        Self {
            identity: identity.into(),
            content_bounds,
            graphemes,
            lines,
            text,
            composition: None,
            selection_model,
        }
    }

    #[must_use]
    pub fn with_composition(
        mut self,
        source_start: usize,
        source_end: usize,
        preedit: impl Into<String>,
        caret_byte: usize,
    ) -> Self {
        let preedit = preedit.into();
        self.composition = composition_layout(
            &self.text,
            &self.graphemes,
            source_start,
            source_end,
            preedit,
            caret_byte,
        );
        self
    }

    #[must_use]
    pub const fn composition_model(&self) -> Option<&TextSurfaceCompositionLayout> {
        self.composition.as_ref()
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub fn grapheme_range_for_byte_offsets(
        &self,
        byte_start: usize,
        byte_end: usize,
    ) -> UiTextSelectionRange {
        if byte_start == byte_end {
            return UiTextSelectionRange::caret(self.grapheme_index_for_byte_offset(byte_start));
        }
        let start = self
            .graphemes
            .iter()
            .position(|value| value.byte_end > byte_start)
            .unwrap_or(self.graphemes.len());
        let end = self
            .graphemes
            .iter()
            .position(|value| value.byte_end >= byte_end)
            .map_or(self.graphemes.len(), |index| index.saturating_add(1));
        UiTextSelectionRange::new(start, end.max(start))
    }

    #[must_use]
    pub fn byte_offsets_for_grapheme_range(&self, range: UiTextSelectionRange) -> (usize, usize) {
        let anchor = self.byte_offset_for_grapheme_index(range.anchor);
        let focus = self.byte_offset_for_grapheme_index(range.focus);
        (anchor, focus)
    }

    #[must_use]
    pub fn hit_test(&self, point_x: i32, point_y: i32) -> UiTextSelectionRange {
        UiTextSelectionRange::caret(self.selection_model.point_to_caret(point_x, point_y))
    }

    #[must_use]
    pub fn selection_rects(&self, selection: UiTextSelectionRange) -> Vec<UiRect> {
        self.selection_model.highlight_rects(selection)
    }

    #[must_use]
    pub fn caret_rect(&self, selection: UiTextSelectionRange) -> UiRect {
        self.selection_model.caret_rect(selection)
    }

    #[must_use]
    pub fn visible_graphemes(&self, viewport: UiRect) -> Vec<&TextSurfaceGraphemeBox> {
        self.graphemes
            .iter()
            .filter(|value| Self::intersects(value.bounds, viewport))
            .collect()
    }

    fn grapheme_index_for_byte_offset(&self, byte_offset: usize) -> usize {
        self.graphemes
            .iter()
            .position(|value| value.byte_start >= byte_offset)
            .or_else(|| {
                self.graphemes
                    .iter()
                    .position(|value| value.byte_end > byte_offset)
            })
            .unwrap_or(self.graphemes.len())
    }

    fn byte_offset_for_grapheme_index(&self, grapheme_index: usize) -> usize {
        self.graphemes
            .get(grapheme_index)
            .map_or(self.text.len(), |value| value.byte_start)
    }

    fn selection_model(text: &str, graphemes: &[TextSurfaceGraphemeBox]) -> UiTextSelectionModel {
        let mut rows = BTreeMap::<i32, Vec<UiTextGlyphBox>>::new();
        for value in graphemes {
            let glyph = UiTextGlyphBox::new(
                value.grapheme_index,
                value.byte_start..value.byte_end,
                value.bounds,
                value.bounds.y.saturating_add(value.bounds.height as i32),
            )
            .with_text(
                text.get(value.byte_start..value.byte_end)
                    .unwrap_or_default(),
            );
            rows.entry(value.bounds.y).or_default().push(glyph);
        }
        let lines = rows
            .into_values()
            .map(|glyphs| {
                let byte_start = glyphs.first().map_or(0, |glyph| glyph.byte_range.start);
                let byte_end = glyphs
                    .last()
                    .map_or(byte_start, |glyph| glyph.byte_range.end);
                UiTextLineBox::new(byte_start..byte_end, glyphs)
            })
            .collect();
        UiTextSelectionModel::new(text, lines)
    }

    fn line_boxes(graphemes: &[TextSurfaceGraphemeBox]) -> Vec<TextSurfaceLineBox> {
        let mut rows = BTreeMap::<i32, UiRect>::new();
        for value in graphemes {
            rows.entry(value.bounds.y)
                .and_modify(|bounds| *bounds = Self::union(*bounds, value.bounds))
                .or_insert(value.bounds);
        }
        rows.into_values()
            .enumerate()
            .map(|(logical_row, bounds)| TextSurfaceLineBox {
                logical_row,
                bounds,
            })
            .collect()
    }

    pub(super) fn intersects(left: UiRect, right: UiRect) -> bool {
        let left_right = left.x.saturating_add(left.width as i32);
        let right_right = right.x.saturating_add(right.width as i32);
        let left_bottom = left.y.saturating_add(left.height as i32);
        let right_bottom = right.y.saturating_add(right.height as i32);
        left.x < right_right
            && right.x < left_right
            && left.y < right_bottom
            && right.y < left_bottom
    }

    pub(super) fn union(left: UiRect, right: UiRect) -> UiRect {
        let x = left.x.min(right.x);
        let y = left.y.min(right.y);
        let right_edge = left
            .x
            .saturating_add(left.width as i32)
            .max(right.x.saturating_add(right.width as i32));
        let bottom_edge = left
            .y
            .saturating_add(left.height as i32)
            .max(right.y.saturating_add(right.height as i32));
        UiRect::new(
            x,
            y,
            u32::try_from(right_edge.saturating_sub(x)).unwrap_or_default(),
            u32::try_from(bottom_edge.saturating_sub(y)).unwrap_or_default(),
        )
    }

    #[must_use]
    pub(super) fn has_logical_row(&self, logical_row: usize) -> bool {
        self.lines
            .iter()
            .any(|line| line.logical_row == logical_row)
    }
}

fn composition_layout(
    text: &str,
    graphemes: &[TextSurfaceGraphemeBox],
    source_start: usize,
    source_end: usize,
    preedit: String,
    caret_byte: usize,
) -> Option<TextSurfaceCompositionLayout> {
    if preedit.is_empty() || source_start > source_end || source_start > text.len() {
        return None;
    }
    let preedit_end = source_start.checked_add(preedit.len())?;
    if text.get(source_start..preedit_end)? != preedit {
        return None;
    }
    let preedit_start = graphemes
        .iter()
        .position(|grapheme| grapheme.byte_start == source_start)?;
    let preedit_end_index = graphemes
        .iter()
        .position(|grapheme| grapheme.byte_end == preedit_end)
        .map(|index| index.saturating_add(1))?;
    let caret_offset = clamp_grapheme_boundary(&preedit, caret_byte);
    let caret_byte = source_start.checked_add(caret_offset)?;
    let caret_index = graphemes
        .iter()
        .position(|grapheme| grapheme.byte_start >= caret_byte)
        .unwrap_or(preedit_end_index);
    Some(TextSurfaceCompositionLayout {
        source_start,
        source_end,
        preedit,
        preedit_range: UiTextSelectionRange::new(preedit_start, preedit_end_index),
        caret_range: UiTextSelectionRange::caret(caret_index),
    })
}

fn clamp_grapheme_boundary(text: &str, byte_offset: usize) -> usize {
    text.grapheme_indices(true)
        .map(|(index, _)| index)
        .chain(std::iter::once(text.len()))
        .take_while(|index| *index <= byte_offset.min(text.len()))
        .last()
        .unwrap_or_default()
}
