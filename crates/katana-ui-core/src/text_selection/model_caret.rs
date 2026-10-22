use super::super::model_bidi::visual_caret_glyphs;
use super::super::{UiTextCaretHit, UiTextCaretPosition, UiTextGlyphBox, UiTextSelectionRange};
use super::UiTextSelectionModel;
use crate::render_model::UiRect;

impl UiTextSelectionModel {
    #[must_use]
    pub(crate) fn point_to_caret_position(&self, x: i32, y: i32) -> UiTextCaretPosition {
        self.visual_caret_candidate_for_point(x, y)
            .map_or_else(UiTextCaretPosition::default, |(_, candidate)| candidate)
    }

    #[must_use]
    pub(crate) fn caret_hit_for_point(&self, x: i32, y: i32) -> UiTextCaretHit {
        self.visual_caret_candidate_for_point(x, y).map_or_else(
            || UiTextCaretHit::from_position(UiTextCaretPosition::default()),
            |(_, candidate)| UiTextCaretHit::from_position(candidate),
        )
    }

    fn visual_caret_candidate_for_point(
        &self,
        x: i32,
        y: i32,
    ) -> Option<(&UiTextGlyphBox, UiTextCaretPosition)> {
        let line = self.line_for_y(y).or_else(|| self.lines.first())?;
        let candidates = visual_caret_glyphs(line, &self.text, self.resolved_bidi_rtl);
        let first = *candidates.first()?;
        if x <= first.1.x {
            return Some(first);
        }
        for candidates in candidates.windows(2) {
            let left = candidates[0];
            let right = candidates[1];
            let midpoint = left.1.x + (right.1.x - left.1.x) / 2;
            if x < midpoint {
                return Some(left);
            }
        }
        candidates.last().copied().or(Some(first))
    }

    #[must_use]
    pub(crate) fn caret_rect_for_hit(&self, hit: UiTextCaretHit) -> UiRect {
        if let Some(glyph_index) = hit.glyph_index()
            && let Some(glyph) = self
                .all_glyphs()
                .find(|glyph| glyph.grapheme_index == glyph_index)
        {
            return UiRect::new(hit.position().x, glyph.bounds.y, 1, glyph.bounds.height);
        }
        self.caret_rect_with_affinity(
            UiTextSelectionRange::caret(hit.caret_position()),
            Some(hit.position()),
        )
    }

    #[must_use]
    pub fn caret_rect(&self, selection: UiTextSelectionRange) -> UiRect {
        self.caret_rect_with_affinity(selection, None)
    }

    #[must_use]
    pub(crate) fn caret_rect_with_affinity(
        &self,
        selection: UiTextSelectionRange,
        affinity: Option<UiTextCaretPosition>,
    ) -> UiRect {
        let caret = selection.caret_position();
        if let Some(affinity) = affinity.filter(|value| value.position == caret)
            && let Some(line) = self.lines.iter().find(|line| {
                line.glyphs
                    .first()
                    .is_some_and(|glyph| glyph.bounds.y == affinity.line_y)
            })
            && let Some((glyph, _)) = visual_caret_glyphs(line, &self.text, self.resolved_bidi_rtl)
                .into_iter()
                .find(|(_, candidate)| *candidate == affinity)
        {
            return UiRect::new(affinity.x, glyph.bounds.y, 1, glyph.bounds.height);
        }
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
}
