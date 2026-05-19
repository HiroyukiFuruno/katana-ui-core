use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeDiffSource {
    Unified {
        text: String,
    },
    Split {
        before: String,
        after: String,
    },
    RangedSplit {
        before: CodeDiffTextSource,
        after: CodeDiffTextSource,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeDiffTextSource {
    pub text: String,
    pub first_line: usize,
    pub line_count: usize,
}

impl CodeDiffTextSource {
    #[must_use]
    pub fn new(text: impl Into<String>, first_line: usize, line_count: usize) -> Self {
        Self {
            text: text.into(),
            first_line,
            line_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeDiffBuildError {
    LineCountMismatch {
        side: CodeDiffSide,
        expected: usize,
        actual: usize,
    },
    UnsupportedUnifiedSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeDiffSide {
    Before,
    After,
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
    Placeholder,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeDiffLineHighlight {
    pub line_index: usize,
    pub start_character: usize,
    pub end_character: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CodeDiffSummary {
    pub additions: usize,
    pub removals: usize,
}
