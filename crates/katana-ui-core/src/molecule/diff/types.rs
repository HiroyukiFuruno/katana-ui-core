use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeDiffSource {
    Unified { text: String },
    Split { before: String, after: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeDiffMode {
    Inline,
    Split,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeDiffDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeDiffWhitespace {
    pub visible: bool,
    pub space_symbol: String,
    pub tab_symbol: String,
}

impl CodeDiffWhitespace {
    #[must_use]
    pub fn visible(space_symbol: impl Into<String>, tab_symbol: impl Into<String>) -> Self {
        Self {
            visible: true,
            space_symbol: space_symbol.into(),
            tab_symbol: tab_symbol.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeDiffLine {
    pub old_number: Option<usize>,
    pub new_number: Option<usize>,
    pub kind: CodeDiffLineKind,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeDiffLineKind {
    Context,
    Added,
    Removed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HighlightRange {
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollapsedBlock {
    pub start_line: usize,
    pub line_count: usize,
}
