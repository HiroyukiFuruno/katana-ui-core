use super::props::TextSurfacePresentation;
use super::surface_model::TextSurface;
use crate::render_model::UiStateId;

impl TextSurface {
    /// Synchronizes a controlled identity without replacing KUC-owned interaction state.
    pub fn synchronize_state_id(&mut self, value: impl Into<UiStateId>) -> bool {
        if !self.props.text_area.synchronize_state_id(value) {
            return false;
        }
        self.state.text_area.state_id = self.props.text_area.state_id().clone();
        true
    }

    #[must_use]
    pub fn has_controlled_automatic_gutter(&self) -> bool {
        self.props
            .gutter
            .as_ref()
            .is_some_and(super::gutter::TextSurfaceGutter::is_controlled_automatic)
    }

    /// Applies host-controlled presentation without fabricating an interaction action/event.
    /// KUC-owned focus, drag, scroll, IME/preedit, layout and texture state stay intact.
    pub fn synchronize_presentation(&mut self, value: TextSurfacePresentation) -> bool {
        let value_changed = self.props.text_area.synchronize_value(value.value);
        let mut changed = value_changed;
        changed |= self
            .props
            .text_area
            .synchronize_selection(crate::atom::TextAreaSelection {
                start: value.selection_start,
                end: value.selection_end,
            });
        changed |= self.props.text_area.synchronize_input_policy(
            value.readonly,
            value.disabled,
            value.ime_enabled,
        );
        let presentation_changed = self.props.spans != value.spans
            || self.props.annotations != value.annotations
            || self
                .props
                .gutter
                .as_ref()
                .and_then(|gutter| gutter.controlled_automatic.as_ref())
                != value.automatic_gutter.as_ref()
            || self.props.accessibility_label != value.accessibility_label
            || self.props.accessibility_actions != value.accessibility_actions
            || self.props.context_target_label != value.context_target_label
            || self.props.disabled_reason != value.disabled_reason;
        let request_changed = self.props.scroll_request != value.scroll_request
            || self.props.focus_request != value.focus_request;
        self.props.spans = value.spans;
        self.props.annotations = value.annotations;
        self.props.gutter = value
            .automatic_gutter
            .map(super::gutter::TextSurfaceGutter::from_controlled_automatic);
        self.props.accessibility_label = value.accessibility_label;
        self.props.accessibility_actions = value.accessibility_actions;
        self.props.context_target_label = value.context_target_label;
        self.props.disabled_reason = value.disabled_reason;
        self.props.scroll_request = value.scroll_request;
        self.props.focus_request = value.focus_request;
        self.state.text_area = self.props.text_area.state().clone();
        if value_changed {
            self.state.scroll_bounds = None;
        }
        changed || presentation_changed || request_changed
    }
}
