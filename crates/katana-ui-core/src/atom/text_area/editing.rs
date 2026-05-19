use super::caret;
use super::events::{TextAreaCompositionState, TextAreaResizeEvent};
use super::{TextArea, TextAreaAction, TextAreaCaretMove, TextAreaEvent, TextAreaSelection};

impl TextArea {
    pub(super) fn suppresses(&self, action: &TextAreaAction) -> bool {
        self.state.disabled
            || (self.state.readonly
                && matches!(
                    action,
                    TextAreaAction::Type(_)
                        | TextAreaAction::Submit
                        | TextAreaAction::InsertNewline
                        | TextAreaAction::Clear
                        | TextAreaAction::ImeComposition(_)
                        | TextAreaAction::ImeCommit(_)
                        | TextAreaAction::DeleteBackward
                ))
    }

    pub(super) fn insert_text(&mut self, value: String, events: &mut Vec<TextAreaEvent>) -> bool {
        self.replace_selection_or_insert(&value);
        self.options.value = self.state.value.clone();
        events.push(TextAreaEvent::TextInput(value.clone()));
        if contains_emoji(&value) {
            events.push(TextAreaEvent::EmojiInput {
                grapheme_count: caret::count_graphemes(&value),
            });
        }
        events.push(TextAreaEvent::Change(self.state.value.clone()));
        true
    }

    pub(super) fn submit(&mut self, events: &mut Vec<TextAreaEvent>) -> bool {
        events.push(TextAreaEvent::Submit(self.state.value.clone()));
        true
    }

    pub(super) fn insert_newline(&mut self, events: &mut Vec<TextAreaEvent>) -> bool {
        self.replace_selection_or_insert("\n");
        self.options.value = self.state.value.clone();
        events.push(TextAreaEvent::InsertNewline);
        events.push(TextAreaEvent::Change(self.state.value.clone()));
        true
    }

    pub(super) fn clear_value(&mut self, events: &mut Vec<TextAreaEvent>) -> bool {
        self.state.set_value(String::new());
        self.options.value.clear();
        events.push(TextAreaEvent::Change(String::new()));
        true
    }

    pub(super) fn move_caret(&mut self, value: TextAreaCaretMove) -> bool {
        let caret = match value {
            TextAreaCaretMove::PreviousGrapheme => {
                caret::previous_grapheme_start(&self.state.value, self.state.caret)
            }
            TextAreaCaretMove::NextGrapheme => {
                caret::next_grapheme_end(&self.state.value, self.state.caret)
            }
            TextAreaCaretMove::Start => 0,
            TextAreaCaretMove::End => self.state.value.len(),
            TextAreaCaretMove::To(value) => value,
        };
        self.state.set_caret(caret);
        true
    }

    pub(super) fn select(&mut self, value: TextAreaSelection) -> bool {
        self.state.set_selection(value);
        true
    }

    pub(super) fn compose(
        &mut self,
        value: TextAreaCompositionState,
        events: &mut Vec<TextAreaEvent>,
    ) -> bool {
        if !self.options.ime_enabled {
            return false;
        }
        self.state.composition = Some(value.clone());
        events.push(TextAreaEvent::ImeComposition(value));
        true
    }

    pub(super) fn commit_ime(&mut self, value: String, events: &mut Vec<TextAreaEvent>) -> bool {
        if !self.options.ime_enabled {
            return false;
        }
        self.state.composition = None;
        self.replace_selection_or_insert(&value);
        self.options.value = self.state.value.clone();
        events.push(TextAreaEvent::ImeCommit(value));
        events.push(TextAreaEvent::Change(self.state.value.clone()));
        true
    }

    pub(super) fn delete_backward(&mut self, events: &mut Vec<TextAreaEvent>) -> bool {
        let previous_value = self.state.value.clone();
        self.state.caret = caret::delete_previous_grapheme(&mut self.state.value, self.state.caret);
        self.state.selection = TextAreaSelection::collapsed(self.state.caret);
        if self.state.value == previous_value {
            return false;
        }
        self.options.value = self.state.value.clone();
        events.push(TextAreaEvent::Change(self.state.value.clone()));
        true
    }

    fn replace_selection_or_insert(&mut self, value: &str) {
        let (start, end) = self.state.selection.ordered();
        let start = caret::clamp_to_char_boundary(&self.state.value, start);
        let end = caret::clamp_to_char_boundary(&self.state.value, end);
        self.state.value.replace_range(start..end, value);
        self.state.caret = start + value.len();
        self.state.selection = TextAreaSelection::collapsed(self.state.caret);
    }

    pub(super) fn push_resize_event(
        &mut self,
        before_rows: u16,
        before_scroll: bool,
        events: &mut Vec<TextAreaEvent>,
    ) {
        self.remeasure();
        if self.state.measured_rows != before_rows || self.state.internal_scroll != before_scroll {
            events.push(TextAreaEvent::Resize(TextAreaResizeEvent {
                rows: self.state.measured_rows,
                internal_scroll: self.state.internal_scroll,
            }));
        }
    }
}

fn contains_emoji(value: &str) -> bool {
    value
        .chars()
        .any(|character| matches!(character as u32, 0x1f300..=0x1faff))
}
