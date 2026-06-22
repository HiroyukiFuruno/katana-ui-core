use super::{
    CodeDiffBuildError, CodeDiffDirection, CodeDiffLine, CodeDiffLineHighlight, CodeDiffMode,
    CodeDiffSource, CodeDiffSummary, CodeDiffTextSource, CodeDiffWhitespace, CollapsedBlock,
    HighlightRange, engine,
};
use crate::molecule::state::MoleculeState;
use crate::render_model::{UiNode, UiNodeKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeDiff {
    label: String,
    pub(super) state: MoleculeState,
    pub(super) source: Option<CodeDiffSource>,
    pub(super) mode: CodeDiffMode,
    pub(super) direction: CodeDiffDirection,
    pub(super) language: String,
    pub(super) lines: Vec<CodeDiffLine>,
    pub(super) highlights: Vec<HighlightRange>,
    pub(super) local_highlights: Vec<CodeDiffLineHighlight>,
    pub(super) collapsed_blocks: Vec<CollapsedBlock>,
    pub(super) expanded_blocks: Vec<CollapsedBlock>,
    pub(super) whitespace: Option<CodeDiffWhitespace>,
    pub(super) summary: CodeDiffSummary,
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
            mode: CodeDiffMode::Split,
            direction: CodeDiffDirection::Horizontal,
            language: String::new(),
            lines: Vec::new(),
            highlights: Vec::new(),
            local_highlights: Vec::new(),
            collapsed_blocks: Vec::new(),
            expanded_blocks: Vec::new(),
            whitespace: None,
            summary: CodeDiffSummary::default(),
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

    pub fn from_sources(
        label: impl Into<String>,
        before_text: impl Into<String>,
        before_first_line: usize,
        before_line_count: usize,
        after_text: impl Into<String>,
        after_first_line: usize,
        after_line_count: usize,
    ) -> Result<Self, CodeDiffBuildError> {
        Self::new(label).source_texts(
            before_text,
            before_first_line,
            before_line_count,
            after_text,
            after_first_line,
            after_line_count,
        )
    }

    pub fn source_contract(
        mut self,
        before: CodeDiffTextSource,
        after: CodeDiffTextSource,
    ) -> Result<Self, CodeDiffBuildError> {
        let source = CodeDiffSource::RangedSplit { before, after };
        self.rebuild_from_source(source)?;
        Ok(self)
    }

    pub fn source_texts(
        self,
        before_text: impl Into<String>,
        before_first_line: usize,
        before_line_count: usize,
        after_text: impl Into<String>,
        after_first_line: usize,
        after_line_count: usize,
    ) -> Result<Self, CodeDiffBuildError> {
        self.source_contract(
            CodeDiffTextSource::new(before_text, before_first_line, before_line_count),
            CodeDiffTextSource::new(after_text, after_first_line, after_line_count),
        )
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
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = value.into();
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
    pub fn local_highlight(mut self, value: CodeDiffLineHighlight) -> Self {
        self.local_highlights.push(value);
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

    fn rebuild_from_source(&mut self, source: CodeDiffSource) -> Result<(), CodeDiffBuildError> {
        let built = engine::build(&source, self.whitespace.as_ref())?;
        self.state.item_count = built.lines.len();
        self.source = Some(source);
        self.lines = built.lines;
        self.local_highlights = built.local_highlights;
        self.collapsed_blocks = built.collapsed_blocks;
        self.expanded_blocks = Vec::new();
        self.summary = built.summary;
        self.trailing_newline_difference = built.trailing_newline_difference;
        Ok(())
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
