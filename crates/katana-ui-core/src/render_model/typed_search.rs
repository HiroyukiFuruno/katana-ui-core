use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiSearchReplaceMode {
    #[default]
    Hidden,
    Visible,
    Disabled,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSearchControlProps {
    pub query: String,
    pub match_case: bool,
    pub whole_word: bool,
    pub use_regex: bool,
    pub result_count: Option<usize>,
    pub active_index: Option<usize>,
    pub result_summary: String,
    pub replace_mode: UiSearchReplaceMode,
    pub replace_value: String,
}
