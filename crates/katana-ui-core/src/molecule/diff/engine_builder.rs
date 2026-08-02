use super::engine::BuiltCodeDiff;
use super::engine_text::{changed_character_range, collapsed_blocks, render_text};
use super::types::{
    CodeDiffLine, CodeDiffLineHighlight, CodeDiffLineKind, CodeDiffSummary, CodeDiffTextSource,
    CodeDiffWhitespace,
};

pub(super) struct DiffBuilder<'a> {
    before: &'a CodeDiffTextSource,
    after: &'a CodeDiffTextSource,
    whitespace: Option<&'a CodeDiffWhitespace>,
    lines: Vec<CodeDiffLine>,
    local_highlights: Vec<CodeDiffLineHighlight>,
    summary: CodeDiffSummary,
}

impl<'a> DiffBuilder<'a> {
    pub fn new(
        before: &'a CodeDiffTextSource,
        after: &'a CodeDiffTextSource,
        whitespace: Option<&'a CodeDiffWhitespace>,
    ) -> Self {
        Self {
            before,
            after,
            whitespace,
            lines: Vec::new(),
            local_highlights: Vec::new(),
            summary: CodeDiffSummary::default(),
        }
    }

    pub fn push_from_pairs(
        &mut self,
        pairs: &[(usize, usize)],
        before_lines: &[&str],
        after_lines: &[&str],
    ) {
        let mut before_cursor = 0;
        let mut after_cursor = 0;
        for &(before_match, after_match) in pairs {
            self.push_changed_blocks(
                &before_lines[before_cursor..before_match],
                before_cursor,
                &after_lines[after_cursor..after_match],
                after_cursor,
            );
            self.push_context(before_match, after_match, before_lines[before_match]);
            before_cursor = before_match + 1;
            after_cursor = after_match + 1;
        }
        self.push_changed_blocks(
            &before_lines[before_cursor..],
            before_cursor,
            &after_lines[after_cursor..],
            after_cursor,
        );
    }

    fn push_context(&mut self, before_index: usize, after_index: usize, text: &str) {
        self.lines.push(CodeDiffLine {
            old_number: Some(self.before.first_line + before_index),
            new_number: Some(self.after.first_line + after_index),
            kind: CodeDiffLineKind::Context,
            text: render_text(text, self.whitespace),
        });
    }

    fn push_changed_blocks(
        &mut self,
        removed: &[&str],
        removed_start: usize,
        added: &[&str],
        added_start: usize,
    ) {
        let paired_count = removed.len().min(added.len());
        for pair_index in 0..paired_count {
            self.push_removed(
                removed[pair_index],
                removed_start + pair_index,
                Some(added[pair_index]),
            );
            self.push_added(
                added[pair_index],
                added_start + pair_index,
                Some(removed[pair_index]),
            );
        }
        for (index, text) in removed[paired_count..].iter().enumerate() {
            self.push_removed(text, removed_start + paired_count + index, None);
            self.push_placeholder();
        }
        for (index, text) in added[paired_count..].iter().enumerate() {
            self.push_placeholder();
            self.push_added(text, added_start + paired_count + index, None);
        }
    }

    fn push_removed(&mut self, text: &str, line_index: usize, paired: Option<&str>) {
        let line_index_in_result = self.lines.len();
        self.lines.push(CodeDiffLine {
            old_number: Some(self.before.first_line + line_index),
            new_number: None,
            kind: CodeDiffLineKind::Removed,
            text: render_text(text, self.whitespace),
        });
        self.summary.removals += 1;
        self.push_highlight(line_index_in_result, text, paired);
    }

    fn push_added(&mut self, text: &str, line_index: usize, paired: Option<&str>) {
        let line_index_in_result = self.lines.len();
        self.lines.push(CodeDiffLine {
            old_number: None,
            new_number: Some(self.after.first_line + line_index),
            kind: CodeDiffLineKind::Added,
            text: render_text(text, self.whitespace),
        });
        self.summary.additions += 1;
        self.push_highlight(line_index_in_result, text, paired);
    }

    fn push_placeholder(&mut self) {
        self.lines.push(CodeDiffLine {
            old_number: None,
            new_number: None,
            kind: CodeDiffLineKind::Placeholder,
            text: String::new(),
        });
    }

    fn push_highlight(&mut self, line_index: usize, text: &str, paired: Option<&str>) {
        let Some(paired_text) = paired else {
            return;
        };
        let (start_character, end_character) = changed_character_range(text, paired_text);
        self.local_highlights.push(CodeDiffLineHighlight {
            line_index,
            start_character,
            end_character,
        });
    }

    pub fn finish(self, trailing_newline_difference: bool) -> BuiltCodeDiff {
        BuiltCodeDiff {
            collapsed_blocks: collapsed_blocks(&self.lines),
            lines: self.lines,
            local_highlights: self.local_highlights,
            summary: self.summary,
            trailing_newline_difference,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_paired_text_creates_an_empty_local_highlight_range() {
        let before = CodeDiffTextSource::new("same", 1, 1);
        let after = CodeDiffTextSource::new("same", 1, 1);
        let mut builder = DiffBuilder::new(&before, &after, None);

        builder.push_highlight(0, "same", Some("same"));

        assert_eq!(
            vec![CodeDiffLineHighlight {
                line_index: 0,
                start_character: 4,
                end_character: 4,
            }],
            builder.local_highlights
        );
    }
}
