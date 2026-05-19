use super::events::TextAreaCompositionState;
use super::options::{newline_chord, submit_chord};
use super::{
    TextArea, TextAreaCompositionPhase, TextAreaEvent, TextAreaSelection, TextAreaState,
    TextAreaTabBehavior, TextAreaValidationError,
};
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextAreaActionOutcome {
    pub handled: bool,
    pub events: Vec<TextAreaEvent>,
    pub state: TextAreaState,
}

impl TextArea {
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
