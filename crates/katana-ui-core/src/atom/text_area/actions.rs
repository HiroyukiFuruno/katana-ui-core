use super::events::TextAreaCompositionState;
use super::options::{newline_chord, submit_chord};
use super::{
    TextArea, TextAreaCompositionPhase, TextAreaEvent, TextAreaSelection, TextAreaState,
    TextAreaTabBehavior, TextAreaValidationError,
};
use crate::render_model::UiStateId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAreaKey {
    Enter,
    Tab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextAreaKeyChord {
    pub key: TextAreaKey,
    pub shift: bool,
    pub primary_modifier: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAreaCaretMove {
    PreviousGrapheme,
    NextGrapheme,
    Start,
    End,
    To(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextAreaResizeDelta {
    pub width_delta: u16,
    pub height_delta: u16,
}

impl TextAreaResizeDelta {
    #[must_use]
    pub const fn new(width_delta: u16, height_delta: u16) -> Self {
        Self {
            width_delta,
            height_delta,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAreaAction {
    Type(String),
    Submit,
    InsertNewline,
    Clear,
    MoveCaret(TextAreaCaretMove),
    Select(TextAreaSelection),
    ImeComposition(TextAreaCompositionState),
    ImeCommit(String),
    DeleteBackward,
    Resize(TextAreaResizeDelta),
}

impl TextAreaAction {
    #[must_use]
    pub fn composition(
        phase: TextAreaCompositionPhase,
        preedit: impl Into<String>,
        caret: usize,
    ) -> Self {
        Self::ImeComposition(TextAreaCompositionState::new(phase, preedit, caret))
    }

    #[must_use]
    pub fn ime_commit(value: impl Into<String>) -> Self {
        Self::ImeCommit(value.into())
    }

    #[must_use]
    pub const fn resize(width_delta: u16, height_delta: u16) -> Self {
        Self::Resize(TextAreaResizeDelta::new(width_delta, height_delta))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextAreaActionOutcome {
    pub handled: bool,
    pub events: Vec<TextAreaEvent>,
    pub state: TextAreaState,
}

impl TextArea {
    /// Synchronizes a controlled identity without replacing interaction-owned state.
    pub fn synchronize_state_id(&mut self, value: impl Into<UiStateId>) -> bool {
        let value = value.into();
        if self.state.state_id == value {
            return false;
        }
        self.state.state_id = value;
        true
    }

    /// Updates a controlled value without treating it as user input.
    pub fn synchronize_value(&mut self, value: impl Into<String>) -> bool {
        let value = value.into();
        if self.state.value == value {
            return false;
        }
        self.set_value(value);
        true
    }

    /// Synchronizes host-controlled selection without emitting an input event.
    ///
    /// This intentionally bypasses `ComponentAction`: a controlled render update is not a
    /// user interaction and must not be observable as one.
    pub fn synchronize_selection(&mut self, selection: TextAreaSelection) -> bool {
        let before = self.state.selection;
        self.state.set_selection(selection);
        before != self.state.selection
    }

    /// Synchronizes the input policy while preserving focus and IME composition state.
    pub fn synchronize_input_policy(
        &mut self,
        readonly: bool,
        disabled: bool,
        ime_enabled: bool,
    ) -> bool {
        let changed = self.options.readonly != readonly
            || self.options.disabled != disabled
            || self.options.ime_enabled != ime_enabled;
        if !changed {
            return false;
        }
        self.options.readonly = readonly;
        self.options.disabled = disabled;
        self.options.ime_enabled = ime_enabled;
        self.state.readonly = readonly;
        self.state.disabled = disabled;
        changed
    }

    #[must_use]
    pub fn cancel_ime_composition(&mut self) -> TextAreaActionOutcome {
        if self.state.disabled || self.state.composition.take().is_none() {
            return TextAreaActionOutcome::ignored(self.state.clone());
        }
        TextAreaActionOutcome {
            handled: true,
            events: Vec::new(),
            state: self.state.clone(),
        }
    }

    pub fn handle_key(
        &mut self,
        key: TextAreaKeyChord,
    ) -> Result<TextAreaActionOutcome, TextAreaValidationError> {
        self.validate()?;
        let event_start = self.events.len();
        let mut result = if submit_chord(self.options.submit_key) == Some(key) {
            self.apply_text_area_action(TextAreaAction::Submit)
        } else if newline_chord(self.options.newline_key) == Some(key) {
            self.apply_text_area_action(TextAreaAction::InsertNewline)
        } else if key == TextAreaKeyChord::tab() {
            self.apply_tab_key()
        } else {
            TextAreaActionOutcome::ignored(self.state.clone())
        };

        if result.handled {
            let event = TextAreaEvent::KeyInput(key);
            result.events.insert(0, event.clone());
            self.events.insert(event_start, event);
        }
        Ok(result)
    }

    #[must_use]
    pub fn apply_text_area_action(&mut self, action: TextAreaAction) -> TextAreaActionOutcome {
        if self.suppresses(&action) {
            return TextAreaActionOutcome::ignored(self.state.clone());
        }

        let before_rows = self.state.measured_rows;
        let before_scroll = self.state.internal_scroll;
        let mut events = Vec::new();
        let handled = match action {
            TextAreaAction::Type(value) => self.insert_text(value, &mut events),
            TextAreaAction::Submit => self.submit(&mut events),
            TextAreaAction::InsertNewline => self.insert_newline(&mut events),
            TextAreaAction::Clear => self.clear_value(&mut events),
            TextAreaAction::MoveCaret(value) => self.move_caret(value),
            TextAreaAction::Select(value) => self.select(value),
            TextAreaAction::ImeComposition(value) => self.compose(value, &mut events),
            TextAreaAction::ImeCommit(value) => self.commit_ime(value, &mut events),
            TextAreaAction::DeleteBackward => self.delete_backward(&mut events),
            TextAreaAction::Resize(value) => self.resize(value, &mut events),
        };

        if handled {
            self.push_resize_event(before_rows, before_scroll, &mut events);
            self.events.extend(events.clone());
        }
        TextAreaActionOutcome {
            handled,
            events,
            state: self.state.clone(),
        }
    }

    fn apply_tab_key(&mut self) -> TextAreaActionOutcome {
        match self.options.tab_behavior {
            TextAreaTabBehavior::MoveFocus => {
                let events = vec![TextAreaEvent::FocusMove];
                self.events.extend(events.clone());
                TextAreaActionOutcome {
                    handled: true,
                    events,
                    state: self.state.clone(),
                }
            }
            TextAreaTabBehavior::InsertTab => {
                self.apply_text_area_action(TextAreaAction::Type("\t".to_string()))
            }
        }
    }
}

impl TextAreaActionOutcome {
    fn ignored(state: TextAreaState) -> Self {
        Self {
            handled: false,
            events: Vec::new(),
            state,
        }
    }
}

#[cfg(test)]
mod coverage_tests {
    use super::*;

    #[test]
    fn cancelling_without_an_ime_composition_is_ignored() {
        let mut area = TextArea::new("area");
        assert!(!area.cancel_ime_composition().handled);
    }
}
