use super::action::{
    TextSurfaceAction, TextSurfaceActionOutcome, TextSurfaceClipboardOperation, TextSurfaceEvent,
    TextSurfaceHistoryOperation,
};
use super::frame::TextSurfaceFrameInput;
use super::frame_record::TextSurfaceFrameRecord;
use super::layout_model::TextSurfaceLayout;
use super::props::TextSurfaceProps;
use super::state::TextSurfaceState;
use super::surface_model::TextSurface;
use crate::atom::{TextAreaAction, TextAreaActionOutcome, TextAreaKeyChord};
use crate::component::ComponentAction;
use crate::interaction::UiAction;

impl TextSurface {
    #[must_use]
    pub fn new(props: TextSurfaceProps) -> Self {
        let state = TextSurfaceState::new(
            props.text_area.state().clone(),
            props.viewport.scroll_x,
            props.viewport.scroll_y,
        );
        Self { props, state }
    }

    #[must_use]
    pub fn props(&self) -> &TextSurfaceProps {
        &self.props
    }

    #[must_use]
    pub fn state(&self) -> &TextSurfaceState {
        &self.state
    }

    /// Makes the adapter-owned layout the source of the viewport dimensions.
    pub fn use_adapter_measured_viewport(&mut self) {
        self.props.viewport_sizing = super::props::TextSurfaceViewportSizing::AdapterMeasured;
    }

    /// Synchronizes host-controlled text without emitting a user-input event.
    pub fn synchronize_value(&mut self, value: impl Into<String>) -> bool {
        if !self.props.text_area.synchronize_value(value) {
            return false;
        }
        self.state.text_area = self.props.text_area.state().clone();
        true
    }

    /// Synchronizes adapter-measured viewport size without changing interaction-owned state.
    #[must_use]
    pub fn synchronize_measured_viewport_size(&mut self, width: u32, height: u32) -> bool {
        let width = width.max(1);
        let height = height.max(1);
        if self.props.viewport.width == width && self.props.viewport.height == height {
            return false;
        }
        self.props.viewport.width = width;
        self.props.viewport.height = height;
        true
    }

    #[must_use]
    pub fn frame(&self, layout: &TextSurfaceLayout) -> TextSurfaceFrameRecord {
        let gutter_width = self
            .props
            .gutter
            .as_ref()
            .map_or(0, |gutter| gutter.layout_derived_width(layout));
        let surface_bounds = crate::render_model::UiRect::new(
            layout.content_bounds.x.saturating_sub(gutter_width as i32),
            layout.content_bounds.y,
            layout.content_bounds.width.saturating_add(gutter_width),
            layout.content_bounds.height,
        );
        self.frame_with_bounds(layout, surface_bounds, layout.content_bounds)
    }

    #[must_use]
    pub fn frame_with_bounds(
        &self,
        layout: &TextSurfaceLayout,
        surface_bounds: crate::render_model::UiRect,
        viewport_bounds: crate::render_model::UiRect,
    ) -> TextSurfaceFrameRecord {
        let viewport = self
            .props
            .viewport
            .scroll_offset(self.state.scroll_x, self.state.scroll_y);
        let gutter_width =
            u32::try_from(viewport_bounds.x.saturating_sub(surface_bounds.x)).unwrap_or_default();
        TextSurfaceFrameRecord::new(TextSurfaceFrameInput {
            layout,
            viewport,
            state: &self.state,
            label: self.props.accessibility_label.clone(),
            accessibility_actions: &self.props.accessibility_actions,
            context_target_label: self.props.context_target_label.as_deref(),
            disabled_reason: self.props.disabled_reason.clone(),
            annotations: &self.props.annotations,
            gutter: self.props.gutter.as_ref(),
            gutter_width,
            surface_bounds,
            viewport_bounds,
        })
    }

    #[must_use]
    pub fn apply_action(&mut self, action: TextSurfaceAction) -> TextSurfaceActionOutcome {
        match action {
            TextSurfaceAction::TextArea(action) => self.apply_text_area_action(action),
            TextSurfaceAction::Key(key) => self.apply_text_area_key(key),
            TextSurfaceAction::SetFocus(focused) => self.apply_focus(focused),
            TextSurfaceAction::ClipboardRequest(operation) => self.request_clipboard(operation),
            TextSurfaceAction::HistoryRequest(operation) => self.request_history(operation),
            TextSurfaceAction::ScrollBy { delta_x, delta_y } => self.scroll_by(delta_x, delta_y),
            TextSurfaceAction::RequestContextTarget { selection } => {
                self.request_context_target(selection)
            }
            TextSurfaceAction::CancelComposition => self.cancel_composition(),
            TextSurfaceAction::ActivateGutterRow { logical_row } => {
                self.activate_gutter_row(logical_row)
            }
            TextSurfaceAction::ActivateGutterMarker {
                logical_row,
                marker_id,
            } => self.activate_gutter_marker(logical_row, marker_id),
        }
    }

    pub(super) fn apply_text_area_action(
        &mut self,
        action: TextAreaAction,
    ) -> TextSurfaceActionOutcome {
        let selection_requested = matches!(&action, TextAreaAction::Select(_));
        let outcome = self.props.text_area.apply_text_area_action(action);
        self.apply_text_area_outcome(outcome, selection_requested)
    }

    pub(super) fn clamp_scroll_offset(&mut self) {
        let Some(bounds) = self.state.scroll_bounds else {
            return;
        };
        self.state.scroll_x = self.state.scroll_x.clamp(0, bounds.max_x);
        self.state.scroll_y = self.state.scroll_y.clamp(0, bounds.max_y);
    }

    fn apply_text_area_key(&mut self, key: TextAreaKeyChord) -> TextSurfaceActionOutcome {
        match self.props.text_area.handle_key(key) {
            Ok(outcome) => self.apply_text_area_outcome(outcome, false),
            Err(error) => TextSurfaceActionOutcome {
                handled: false,
                events: vec![TextSurfaceEvent::KeyValidationFailed { key, error }],
                state: self.state.clone(),
            },
        }
    }

    fn apply_text_area_outcome(
        &mut self,
        outcome: TextAreaActionOutcome,
        selection_requested: bool,
    ) -> TextSurfaceActionOutcome {
        self.state.text_area = outcome.state.clone();
        let mut events = outcome
            .events
            .into_iter()
            .map(TextSurfaceEvent::TextArea)
            .collect::<Vec<_>>();
        if selection_requested && outcome.handled {
            events.push(TextSurfaceEvent::SelectionChanged {
                selection_start: outcome.state.selection.start,
                selection_end: outcome.state.selection.end,
            });
        }
        TextSurfaceActionOutcome {
            handled: outcome.handled,
            events,
            state: self.state.clone(),
        }
    }

    fn apply_focus(&mut self, focused: bool) -> TextSurfaceActionOutcome {
        let target = self.props.text_area.state_id().clone();
        let action = if focused {
            UiAction::focus(target)
        } else {
            UiAction::blur(target)
        };
        let result = ComponentAction::apply_action(&mut self.props.text_area, &action);
        self.state.text_area = self.props.text_area.state().clone();
        TextSurfaceActionOutcome {
            handled: result.handled,
            events: result
                .handled
                .then_some(TextSurfaceEvent::FocusChanged(focused))
                .into_iter()
                .collect(),
            state: self.state.clone(),
        }
    }

    fn request_clipboard(
        &self,
        operation: TextSurfaceClipboardOperation,
    ) -> TextSurfaceActionOutcome {
        let state = &self.state.text_area;
        let has_selection = state.selection.start != state.selection.end;
        let handled = match operation {
            TextSurfaceClipboardOperation::Copy => !state.disabled && has_selection,
            TextSurfaceClipboardOperation::Cut => {
                !state.disabled && !state.readonly && has_selection
            }
            TextSurfaceClipboardOperation::Paste => !state.disabled && !state.readonly,
        };
        let events = handled
            .then_some(TextSurfaceEvent::ClipboardRequested {
                operation,
                selection_start: state.selection.start,
                selection_end: state.selection.end,
            })
            .into_iter()
            .collect();
        TextSurfaceActionOutcome {
            handled,
            events,
            state: self.state.clone(),
        }
    }

    fn request_history(&self, operation: TextSurfaceHistoryOperation) -> TextSurfaceActionOutcome {
        let state = &self.state.text_area;
        let handled = !state.disabled && !state.readonly;
        let events = handled
            .then_some(TextSurfaceEvent::HistoryRequested(operation))
            .into_iter()
            .collect();
        TextSurfaceActionOutcome {
            handled,
            events,
            state: self.state.clone(),
        }
    }

    fn activate_gutter_row(&self, logical_row: usize) -> TextSurfaceActionOutcome {
        let handled = !self.state.text_area.disabled
            && self.props.gutter.as_ref().is_some_and(|gutter| {
                gutter.is_controlled_automatic()
                    || gutter.rows.iter().any(|row| row.logical_row == logical_row)
            });
        self.gutter_outcome(
            handled,
            TextSurfaceEvent::GutterRowActivated { logical_row },
        )
    }

    fn activate_gutter_marker(
        &self,
        logical_row: usize,
        marker_id: String,
    ) -> TextSurfaceActionOutcome {
        let handled = !self.state.text_area.disabled
            && self.props.gutter.as_ref().is_some_and(|gutter| {
                gutter.is_controlled_automatic()
                    || gutter.rows.iter().any(|row| {
                        row.logical_row == logical_row
                            && row.marker_id.as_deref() == Some(&marker_id)
                    })
            });
        self.gutter_outcome(
            handled,
            TextSurfaceEvent::GutterMarkerActivated {
                logical_row,
                marker_id,
            },
        )
    }

    pub(super) fn gutter_outcome(
        &self,
        handled: bool,
        event: TextSurfaceEvent,
    ) -> TextSurfaceActionOutcome {
        TextSurfaceActionOutcome {
            handled,
            events: handled.then_some(event).into_iter().collect(),
            state: self.state.clone(),
        }
    }
}
