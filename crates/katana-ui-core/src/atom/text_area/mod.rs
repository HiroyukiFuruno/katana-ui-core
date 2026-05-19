mod actions;
mod autogrow;
mod builders;
mod caret;
mod editing;
mod events;
mod options;
mod state;

use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult, UiActionSource};
use crate::render_model::{UiCommonProps, UiNode, UiNodeKind, UiStateId, UiVisualRole};
use serde::{Deserialize, Serialize};

pub use actions::{
    TextAreaAction, TextAreaActionOutcome, TextAreaCaretMove, TextAreaKey, TextAreaKeyChord,
};
pub use events::{
    TextAreaCompositionPhase, TextAreaCompositionState, TextAreaEvent, TextAreaResizeEvent,
};
pub use options::{
    TextAreaNewlineKey, TextAreaOptions, TextAreaSubmitKey, TextAreaTabBehavior,
    TextAreaValidationError, TextAreaWrapPolicy,
};
pub use state::{TextAreaSelection, TextAreaState};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextArea {
    label: String,
    common: UiCommonProps,
    visual_role: UiVisualRole,
    options: TextAreaOptions,
    state: TextAreaState,
    events: Vec<TextAreaEvent>,
}

impl TextArea {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        let options = TextAreaOptions::default();
        Self {
            label: label.into(),
            common: UiCommonProps::default(),
            visual_role: UiVisualRole::Input,
            state: TextAreaState::from_options(&options),
            options,
            events: Vec::new(),
        }
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state.state_id
    }

    #[must_use]
    pub fn options(&self) -> &TextAreaOptions {
        &self.options
    }

    #[must_use]
    pub fn state(&self) -> &TextAreaState {
        &self.state
    }

    #[must_use]
    pub fn events(&self) -> &[TextAreaEvent] {
        &self.events
    }

    pub(super) fn set_value(&mut self, value: String) {
        self.options.value = value.clone();
        self.state.set_value(value);
        self.remeasure();
    }

    pub(super) fn remeasure(&mut self) {
        self.state.measure(
            self.options.min_rows,
            self.options.max_rows,
            self.options.auto_grow,
        );
    }

    fn sync_focus(&mut self, focused: bool) {
        self.state.focused = focused;
        self.events.push(if focused {
            TextAreaEvent::Focus
        } else {
            TextAreaEvent::Blur
        });
    }
}

impl ComponentAction for TextArea {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = self.state.interaction_state();
        if action.target() != self.state_id() || self.state.disabled {
            return UiActionResult::ignored(self.state_id().clone(), before);
        }

        let handled = match action {
            UiAction::SetValue { value, .. } if !self.state.readonly => {
                self.set_value(value.clone());
                true
            }
            UiAction::ClearValue { .. } if !self.state.readonly => {
                self.set_value(String::new());
                true
            }
            UiAction::SetCursorSelection {
                cursor,
                selection_start,
                selection_end,
                ..
            } if !self.state.readonly => {
                self.state.set_caret(*cursor);
                self.state.set_selection(TextAreaSelection {
                    start: *selection_start,
                    end: *selection_end,
                });
                true
            }
            UiAction::Press {
                source: UiActionSource::InputSubmit,
                ..
            } if !self.state.readonly => true,
            UiAction::SetFocus { focused, .. } => {
                self.sync_focus(*focused);
                true
            }
            _ => false,
        };

        if !handled {
            return UiActionResult::ignored(self.state_id().clone(), before);
        }
        UiActionResult::handled(
            self.state_id().clone(),
            action,
            before,
            self.state.interaction_state(),
        )
    }
}

impl From<TextArea> for UiNode {
    fn from(value: TextArea) -> Self {
        let interaction = value.state.interaction_state();
        let state_id = value.state.state_id.clone();
        let text_entry = value.options.text_entry_props();
        let text_area = value
            .options
            .text_area_props(value.state.measured_rows, value.state.internal_scroll);

        UiNode::from_state(UiNodeKind::TextArea, value.label, state_id)
            .common(value.common)
            .disabled(value.options.disabled)
            .focusable(true)
            .interaction(interaction)
            .font_role(value.options.font_role.clone())
            .visual_role(value.visual_role)
            .readonly(value.options.readonly)
            .invalid(value.options.invalid)
            .placeholder(value.options.placeholder.clone())
            .text_entry(text_entry)
            .text_area(text_area)
    }
}
