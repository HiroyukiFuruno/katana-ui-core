use super::engine_builder::DiffBuilder;
use super::types::{
    CodeDiffBuildError, CodeDiffLine, CodeDiffLineHighlight, CodeDiffSide, CodeDiffSource,
    CodeDiffSummary, CodeDiffTextSource, CodeDiffWhitespace, CollapsedBlock,
};

pub(super) struct BuiltCodeDiff {
    pub lines: Vec<CodeDiffLine>,
    pub local_highlights: Vec<CodeDiffLineHighlight>,
    pub collapsed_blocks: Vec<CollapsedBlock>,
    pub summary: CodeDiffSummary,
    pub trailing_newline_difference: bool,
}

pub(super) fn build(
    source: &CodeDiffSource,
    whitespace: Option<&CodeDiffWhitespace>,
) -> Result<BuiltCodeDiff, CodeDiffBuildError> {
    let (before, after) = match source {
        CodeDiffSource::RangedSplit { before, after } => (before.clone(), after.clone()),
        CodeDiffSource::Split { before, after } => (
            CodeDiffTextSource::new(before.clone(), 1, display_lines(before).len()),
            CodeDiffTextSource::new(after.clone(), 1, display_lines(after).len()),
        ),
        CodeDiffSource::Unified { .. } => return Err(CodeDiffBuildError::UnsupportedUnifiedSource),
    };
    validate_line_count(CodeDiffSide::Before, &before)?;
    validate_line_count(CodeDiffSide::After, &after)?;

    let before_lines = display_lines(&before.text);
    let after_lines = display_lines(&after.text);
    let pairs = lcs_pairs(&before_lines, &after_lines);
    let mut builder = DiffBuilder::new(&before, &after, whitespace);
    builder.push_from_pairs(&pairs, &before_lines, &after_lines);
    Ok(builder.finish(before.text.ends_with('\n') != after.text.ends_with('\n')))
}

fn validate_line_count(
    side: CodeDiffSide,
    source: &CodeDiffTextSource,
) -> Result<(), CodeDiffBuildError> {
    let actual = display_lines(&source.text).len();
    if source.line_count == actual {
        return Ok(());
    }
    Err(CodeDiffBuildError::LineCountMismatch {
        side,
        expected: source.line_count,
        actual,
    })
}

fn display_lines(text: &str) -> Vec<&str> {
    text.split('\n').collect()
}

fn lcs_pairs(before: &[&str], after: &[&str]) -> Vec<(usize, usize)> {
    let mut lengths = vec![vec![0; after.len() + 1]; before.len() + 1];
    for before_index in (0..before.len()).rev() {
        for after_index in (0..after.len()).rev() {
            lengths[before_index][after_index] = if before[before_index] == after[after_index] {
                lengths[before_index + 1][after_index + 1] + 1
            } else {
                lengths[before_index + 1][after_index].max(lengths[before_index][after_index + 1])
            };
        }
    }
    collect_pairs(before, after, &lengths)
}

fn collect_pairs(before: &[&str], after: &[&str], lengths: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    let mut before_index = 0;
    let mut after_index = 0;
    while before_index < before.len() && after_index < after.len() {
        if before[before_index] == after[after_index] {
            pairs.push((before_index, after_index));
            before_index += 1;
            after_index += 1;
        } else if lengths[before_index + 1][after_index] >= lengths[before_index][after_index + 1] {
            before_index += 1;
        } else {
            after_index += 1;
        }
    }
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_and_unified_sources_cover_engine_boundary() {
        let split = build(
            &CodeDiffSource::Split {
                before: "before".to_string(),
                after: "after".to_string(),
            },
            None,
        );
        assert!(matches!(
            split,
            Ok(split) if split.summary.additions == 1 && split.summary.removals == 1
        ));

        let error = build(
            &CodeDiffSource::Unified {
                text: "@@".to_string(),
            },
            None,
        )
        .err();
        assert_eq!(Some(CodeDiffBuildError::UnsupportedUnifiedSource), error);
    }
}
