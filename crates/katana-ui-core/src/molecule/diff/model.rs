use super::{
    CodeDiffDirection, CodeDiffLine, CodeDiffMode, CodeDiffSource, CodeDiffWhitespace,
    CollapsedBlock, HighlightRange,
};
use crate::molecule::state::MoleculeState;
use crate::render_model::{UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeDiff {
    label: String,
    pub(super) state: MoleculeState,
    pub(super) source: Option<CodeDiffSource>,
    pub(super) mode: CodeDiffMode,
    pub(super) direction: CodeDiffDirection,
    pub(super) lines: Vec<CodeDiffLine>,
    pub(super) highlights: Vec<HighlightRange>,
    pub(super) collapsed_blocks: Vec<CollapsedBlock>,
    pub(super) whitespace: Option<CodeDiffWhitespace>,
    pub(super) long_line_column: Option<usize>,
    pub(super) trailing_newline_difference: bool,
    pub(super) scroll_sync_enabled: bool,
    pub(super) children: Vec<UiNode>,
}

impl CodeDiff {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state: MoleculeState::new(UiNodeKind::CodeDiff),
            source: None,
            mode: CodeDiffMode::Inline,
            direction: CodeDiffDirection::Horizontal,
            lines: Vec::new(),
            highlights: Vec::new(),
            collapsed_blocks: Vec::new(),
            whitespace: None,
            long_line_column: None,
            trailing_newline_difference: false,
            scroll_sync_enabled: false,
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn source(mut self, value: CodeDiffSource) -> Self {
        self.source = Some(value);
        self
    }

    #[must_use]
    pub fn mode(mut self, value: CodeDiffMode) -> Self {
        self.mode = value;
        self
    }

    #[must_use]
    pub fn direction(mut self, value: CodeDiffDirection) -> Self {
        self.direction = value;
        self
    }

    #[must_use]
    pub fn line(mut self, line: CodeDiffLine) -> Self {
        self.lines.push(line);
        self.state.item_count = self.lines.len();
        self
    }

    #[must_use]
    pub fn highlight(mut self, value: HighlightRange) -> Self {
        self.highlights.push(value);
        self
    }

    #[must_use]
    pub fn collapsed_block(mut self, value: CollapsedBlock) -> Self {
        self.collapsed_blocks.push(value);
        self
    }

    #[must_use]
    pub fn whitespace(mut self, value: CodeDiffWhitespace) -> Self {
        self.whitespace = Some(value);
        self
    }

    #[must_use]
    pub fn long_line_column(mut self, value: usize) -> Self {
        self.long_line_column = Some(value);
        self
    }

    #[must_use]
    pub fn trailing_newline_difference(mut self, value: bool) -> Self {
        self.trailing_newline_difference = value;
        self
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    #[must_use]
    pub fn item_count(mut self, value: usize) -> Self {
        self.state.item_count = value;
        self
    }
}

impl CodeDiff {
    #[must_use]
    pub fn source_model(&self) -> Option<&CodeDiffSource> {
        self.source.as_ref()
    }

    #[must_use]
    pub fn mode_model(&self) -> CodeDiffMode {
        self.mode
    }

    #[must_use]
    pub fn direction_model(&self) -> CodeDiffDirection {
        self.direction
    }

    #[must_use]
    pub fn lines(&self) -> &[CodeDiffLine] {
        &self.lines
    }

    #[must_use]
    pub fn highlights(&self) -> &[HighlightRange] {
        &self.highlights
    }

    #[must_use]
    pub fn collapsed_blocks(&self) -> &[CollapsedBlock] {
        &self.collapsed_blocks
    }

    #[must_use]
    pub fn whitespace_model(&self) -> Option<&CodeDiffWhitespace> {
        self.whitespace.as_ref()
    }

    #[must_use]
    pub fn long_line_column_model(&self) -> Option<usize> {
        self.long_line_column
    }

    #[must_use]
    pub fn has_trailing_newline_difference(&self) -> bool {
        self.trailing_newline_difference
    }

    #[must_use]
    pub fn scroll_sync_enabled(&self) -> bool {
        self.scroll_sync_enabled
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state.state_id
    }
}

impl From<CodeDiff> for UiNode {
    fn from(value: CodeDiff) -> Self {
        let mut node = value.state.node(UiNodeKind::CodeDiff, value.label);
        for child in value.children {
            node = node.child(child);
        }
        node
    }
}
