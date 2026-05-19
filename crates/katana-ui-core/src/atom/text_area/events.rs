use super::TextAreaKeyChord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAreaCompositionPhase {
    Start,
    Update,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextAreaCompositionState {
    pub phase: TextAreaCompositionPhase,
    pub preedit: String,
    pub caret: usize,
}

impl TextAreaCompositionState {
    #[must_use]
    pub fn new(phase: TextAreaCompositionPhase, preedit: impl Into<String>, caret: usize) -> Self {
        Self {
            phase,
            preedit: preedit.into(),
            caret,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextAreaResizeEvent {
    pub rows: u16,
    pub internal_scroll: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAreaEvent {
    KeyInput(TextAreaKeyChord),
    TextInput(String),
    ImeComposition(TextAreaCompositionState),
    ImeCommit(String),
    EmojiInput { grapheme_count: usize },
    Submit(String),
    Change(String),
    Focus,
    Blur,
    Resize(TextAreaResizeEvent),
    InsertNewline,
    FocusMove,
}
