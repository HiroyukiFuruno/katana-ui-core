use super::layout_model::TextSurfaceLayout;
use crate::render_model::UiRect;
use crate::text_selection::{UiTextCaretHit, UiTextSelectionRange};

impl TextSurfaceLayout {
    #[must_use]
    pub fn hit_test(&self, point_x: i32, point_y: i32) -> UiTextSelectionRange {
        UiTextSelectionRange::caret(self.caret_position_for_point(point_x, point_y).position)
    }

    #[must_use]
    pub fn hit_test_with_affinity(&self, point_x: i32, point_y: i32) -> UiTextCaretHit {
        UiTextCaretHit::from_position(self.caret_position_for_point(point_x, point_y))
    }

    #[must_use]
    pub fn caret_rect_for_hit(&self, hit: UiTextCaretHit) -> UiRect {
        self.caret_rect_with_affinity(
            UiTextSelectionRange::caret(hit.caret_position()),
            Some(hit.position()),
        )
    }
}
