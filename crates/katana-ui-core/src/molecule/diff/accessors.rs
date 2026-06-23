use super::{
    CodeDiff, CodeDiffDirection, CodeDiffLine, CodeDiffLineHighlight, CodeDiffMode, CodeDiffSource,
    CodeDiffSummary, CodeDiffWhitespace, CollapsedBlock, HighlightRange,
};
use crate::render_model::UiStateId;

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
    pub fn language_model(&self) -> &str {
        &self.language
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
    pub fn local_highlights(&self) -> &[CodeDiffLineHighlight] {
        &self.local_highlights
    }

    #[must_use]
    pub fn local_highlight_ranges(&self) -> Vec<(usize, usize, usize)> {
        self.local_highlights
            .iter()
            .map(|range| (range.line_index, range.start_character, range.end_character))
            .collect()
    }

    #[must_use]
    pub fn collapsed_blocks(&self) -> &[CollapsedBlock] {
        &self.collapsed_blocks
    }

    #[must_use]
    pub fn expanded_blocks(&self) -> &[CollapsedBlock] {
        &self.expanded_blocks
    }

    #[must_use]
    pub fn summary(&self) -> CodeDiffSummary {
        self.summary
    }

    #[must_use]
    pub fn addition_count(&self) -> usize {
        self.summary.additions
    }

    #[must_use]
    pub fn removal_count(&self) -> usize {
        self.summary.removals
    }

    #[must_use]
    pub fn effective_direction_model(&self) -> Option<CodeDiffDirection> {
        match self.mode {
            CodeDiffMode::Inline => None,
            CodeDiffMode::Split => Some(self.direction),
        }
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
