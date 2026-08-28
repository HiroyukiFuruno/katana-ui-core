use super::action::{TextSurfaceActionOutcome, TextSurfaceEvent, TextSurfaceLayoutAction};
use super::layout_model::TextSurfaceLayout;
use super::props::TextSurfacePoint;
use super::surface_model::TextSurface;
use crate::atom::{TextAreaAction, TextAreaSelection};

impl TextSurface {
    #[must_use]
    pub fn apply_layout_action(
        &mut self,
        layout: &TextSurfaceLayout,
        action: TextSurfaceLayoutAction,
    ) -> TextSurfaceActionOutcome {
        match action {
            TextSurfaceLayoutAction::PointerPress {
                point,
                extend_selection,
            } => self.pointer_press(layout, point, extend_selection),
            TextSurfaceLayoutAction::PointerDrag { point } => self.pointer_drag(layout, point),
            TextSurfaceLayoutAction::PointerRelease => self.pointer_release(),
        }
    }

    pub(super) fn scroll_by(&mut self, delta_x: i32, delta_y: i32) -> TextSurfaceActionOutcome {
        if self.state.text_area.disabled {
            return self.gutter_outcome(
                false,
                TextSurfaceEvent::Scrolled {
                    scroll_x: self.state.scroll_x,
                    scroll_y: self.state.scroll_y,
                },
            );
        }
        self.state.scroll_x = self.state.scroll_x.saturating_add(delta_x);
        self.state.scroll_y = self.state.scroll_y.saturating_add(delta_y);
        self.clamp_scroll_offset();
        self.gutter_outcome(
            true,
            TextSurfaceEvent::Scrolled {
                scroll_x: self.state.scroll_x,
                scroll_y: self.state.scroll_y,
            },
        )
    }

    pub(super) fn request_context_target(
        &self,
        selection: crate::text_selection::UiTextSelectionRange,
    ) -> TextSurfaceActionOutcome {
        self.gutter_outcome(
            !self.state.text_area.disabled,
            TextSurfaceEvent::ContextTargetRequested { selection },
        )
    }

    pub(super) fn cancel_composition(&mut self) -> TextSurfaceActionOutcome {
        let outcome = self.props.text_area.cancel_ime_composition();
        self.state.text_area = outcome.state;
        self.gutter_outcome(outcome.handled, TextSurfaceEvent::CompositionCancelled)
    }

    fn pointer_press(
        &mut self,
        layout: &TextSurfaceLayout,
        point: TextSurfacePoint,
        extend_selection: bool,
    ) -> TextSurfaceActionOutcome {
        if self.state.text_area.disabled {
            return self.gutter_outcome(
                false,
                TextSurfaceEvent::SelectionChanged {
                    selection_start: self.state.text_area.selection.start,
                    selection_end: self.state.text_area.selection.end,
                },
            );
        }
        let hit = layout.hit_test(point.x, point.y).caret_position();
        self.state.pointer_anchor = Some(point);
        let start = if extend_selection {
            layout
                .grapheme_range_for_byte_offsets(
                    self.state.text_area.selection.start,
                    self.state.text_area.selection.start,
                )
                .caret_position()
        } else {
            hit
        };
        self.select_grapheme_range(layout, start, hit)
    }

    fn pointer_drag(
        &mut self,
        layout: &TextSurfaceLayout,
        point: TextSurfacePoint,
    ) -> TextSurfaceActionOutcome {
        let Some(anchor) = self.state.pointer_anchor else {
            return self.gutter_outcome(
                false,
                TextSurfaceEvent::SelectionChanged {
                    selection_start: self.state.text_area.selection.start,
                    selection_end: self.state.text_area.selection.end,
                },
            );
        };
        let start = layout.hit_test(anchor.x, anchor.y).caret_position();
        let end = layout.hit_test(point.x, point.y).caret_position();
        self.select_grapheme_range(layout, start, end)
    }

    fn pointer_release(&mut self) -> TextSurfaceActionOutcome {
        let handled = self.state.pointer_anchor.take().is_some();
        self.gutter_outcome(
            handled,
            TextSurfaceEvent::SelectionChanged {
                selection_start: self.state.text_area.selection.start,
                selection_end: self.state.text_area.selection.end,
            },
        )
    }

    fn select_grapheme_range(
        &mut self,
        layout: &TextSurfaceLayout,
        start: usize,
        end: usize,
    ) -> TextSurfaceActionOutcome {
        let (selection_start, selection_end) = layout.byte_offsets_for_grapheme_range(
            crate::text_selection::UiTextSelectionRange::new(start, end),
        );
        self.apply_text_area_action(TextAreaAction::Select(TextAreaSelection {
            start: selection_start,
            end: selection_end,
        }))
    }
}
