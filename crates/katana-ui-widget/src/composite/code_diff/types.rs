use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeDiffSource {
    pub text: String,
    pub first_line_number: usize,
    pub line_count: usize,
}

impl CodeDiffSource {
    #[must_use]
    pub fn new(text: impl Into<String>, first_line_number: usize, line_count: usize) -> Self {
        Self {
            text: text.into(),
            first_line_number,
            line_count,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeDiffMode {
    Split,
    Inline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeDiffSplitOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeDiffLineKind {
    Equal,
    Added,
    Removed,
    Placeholder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeDiffTextRange {
    pub start: usize,
    pub end: usize,
}

impl CodeDiffTextRange {
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    #[must_use]
    pub const fn contains(self, index: usize) -> bool {
        self.start <= index && index < self.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeDiffLine {
    pub kind: CodeDiffLineKind,
    pub line_number: Option<usize>,
    pub text: String,
    pub highlights: Vec<CodeDiffTextRange>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeDiffAlignedRow {
    pub before: CodeDiffLine,
    pub after: CodeDiffLine,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeDiffModel {
    pub rows: Vec<CodeDiffAlignedRow>,
    pub added_count: usize,
    pub removed_count: usize,
    pub changed_block_count: usize,
}

impl CodeDiffModel {
    #[must_use]
    pub fn has_changes(&self) -> bool {
        self.added_count > 0 || self.removed_count > 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeDiffError {
    LineCountMismatch {
        side: CodeDiffSide,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for CodeDiffError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LineCountMismatch {
                side,
                expected,
                actual,
            } => write!(
                f,
                "{side} の行数が一致しません: 指定 {expected} 行 / 本文 {actual} 行"
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeDiffSide {
    Before,
    After,
}

impl fmt::Display for CodeDiffSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Before => write!(f, "変更前"),
            Self::After => write!(f, "変更後"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeDiffCollapseOptions {
    pub enabled: bool,
    pub initially_expanded: bool,
    pub context_lines: usize,
}

impl Default for CodeDiffCollapseOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            initially_expanded: false,
            context_lines: 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CodeDiffProps {
    pub before: CodeDiffSource,
    pub after: CodeDiffSource,
    pub mode: CodeDiffMode,
    pub split_orientation: CodeDiffSplitOrientation,
    pub collapse: CodeDiffCollapseOptions,
    pub show_header: bool,
}
