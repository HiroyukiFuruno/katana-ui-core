use crate::render_model::UiSearchReplaceMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchOptions {
    pub match_case: bool,
    pub whole_word: bool,
    pub use_regex: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchOptionKind {
    MatchCase,
    WholeWord,
    UseRegex,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplaceMode {
    #[default]
    Hidden,
    Visible,
    Disabled,
}

impl SearchOptions {
    pub(super) fn toggle(&mut self, value: SearchOptionKind) -> bool {
        match value {
            SearchOptionKind::MatchCase => {
                self.match_case = !self.match_case;
                self.match_case
            }
            SearchOptionKind::WholeWord => {
                self.whole_word = !self.whole_word;
                self.whole_word
            }
            SearchOptionKind::UseRegex => {
                self.use_regex = !self.use_regex;
                self.use_regex
            }
        }
    }
}

impl From<ReplaceMode> for UiSearchReplaceMode {
    fn from(value: ReplaceMode) -> Self {
        match value {
            ReplaceMode::Hidden => Self::Hidden,
            ReplaceMode::Visible => Self::Visible,
            ReplaceMode::Disabled => Self::Disabled,
        }
    }
}

pub(super) fn result_summary(count: Option<usize>, active_index: Option<usize>) -> String {
    match (count, active_index) {
        (Some(0), _) => "0 results".to_string(),
        (Some(1), Some(0)) | (Some(1), None) => "1 / 1".to_string(),
        (Some(count), Some(index)) => format!("{} / {count}", index.saturating_add(1).min(count)),
        (Some(count), None) => format!("{count} results"),
        (None, _) => String::new(),
    }
}
