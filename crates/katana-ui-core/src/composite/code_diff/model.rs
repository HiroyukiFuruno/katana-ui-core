use super::highlight::CodeDiffHighlighter;
use super::types::{
    CodeDiffAlignedRow, CodeDiffError, CodeDiffLine, CodeDiffLineKind, CodeDiffModel, CodeDiffSide,
    CodeDiffSource,
};

const SIMILARITY_MATCH_THRESHOLD: f32 = 0.35;

#[derive(Clone)]
struct SourceLine {
    number: usize,
    text: String,
}

pub(crate) struct CodeDiffModelBuilder;

impl CodeDiffModelBuilder {
    pub(crate) fn build_model(
        before: &CodeDiffSource,
        after: &CodeDiffSource,
    ) -> Result<CodeDiffModel, CodeDiffError> {
        validate_line_count(before, CodeDiffSide::Before)?;
        validate_line_count(after, CodeDiffSide::After)?;

        let before_lines = source_lines(before);
        let after_lines = source_lines(after);
        let pairs = line_lcs(&before_lines, &after_lines);
        let mut rows = Vec::new();
        let mut before_cursor = 0;
        let mut after_cursor = 0;

        for (before_index, after_index) in pairs {
            append_changed_rows(
                &before_lines[before_cursor..before_index],
                &after_lines[after_cursor..after_index],
                &mut rows,
            );
            rows.push(equal_row(
                &before_lines[before_index],
                &after_lines[after_index],
            ));
            before_cursor = before_index + 1;
            after_cursor = after_index + 1;
        }
        append_changed_rows(
            &before_lines[before_cursor..],
            &after_lines[after_cursor..],
            &mut rows,
        );

        Ok(model_from_rows(rows))
    }
}

fn validate_line_count(source: &CodeDiffSource, side: CodeDiffSide) -> Result<(), CodeDiffError> {
    let actual = display_lines(&source.text).len();
    if actual == source.line_count {
        Ok(())
    } else {
        Err(CodeDiffError::LineCountMismatch {
            side,
            expected: source.line_count,
            actual,
        })
    }
}

fn source_lines(source: &CodeDiffSource) -> Vec<SourceLine> {
    display_lines(&source.text)
        .into_iter()
        .enumerate()
        .map(|(index, text)| SourceLine {
            number: source.first_line_number + index,
            text,
        })
        .collect()
}

fn display_lines(text: &str) -> Vec<String> {
    if text.is_empty() {
        Vec::new()
    } else {
        text.split('\n').map(ToString::to_string).collect()
    }
}

fn line_lcs(before: &[SourceLine], after: &[SourceLine]) -> Vec<(usize, usize)> {
    let mut table = vec![vec![0_usize; after.len() + 1]; before.len() + 1];
    for before_index in (0..before.len()).rev() {
        for after_index in (0..after.len()).rev() {
            table[before_index][after_index] =
                if before[before_index].text == after[after_index].text {
                    table[before_index + 1][after_index + 1] + 1
                } else {
                    table[before_index + 1][after_index].max(table[before_index][after_index + 1])
                };
        }
    }

    let mut pairs = Vec::new();
    let mut before_index = 0;
    let mut after_index = 0;
    while before_index < before.len() && after_index < after.len() {
        if before[before_index].text == after[after_index].text {
            pairs.push((before_index, after_index));
            before_index += 1;
            after_index += 1;
        } else if table[before_index + 1][after_index] >= table[before_index][after_index + 1] {
            before_index += 1;
        } else {
            after_index += 1;
        }
    }
    pairs
}

fn append_changed_rows(
    before: &[SourceLine],
    after: &[SourceLine],
    rows: &mut Vec<CodeDiffAlignedRow>,
) {
    let mut used_after = vec![false; after.len()];
    for before_line in before {
        if let Some(after_index) = best_match(before_line, after, &used_after) {
            used_after[after_index] = true;
            rows.push(changed_row(before_line, &after[after_index]));
        } else {
            rows.push(remove_row(before_line));
        }
    }
    for (after_index, after_line) in after.iter().enumerate() {
        if !used_after[after_index] {
            rows.push(add_row(after_line));
        }
    }
}

fn best_match(before: &SourceLine, after: &[SourceLine], used_after: &[bool]) -> Option<usize> {
    after
        .iter()
        .enumerate()
        .filter(|(index, _)| !used_after[*index])
        .map(|(index, after_line)| (index, similarity(&before.text, &after_line.text)))
        .filter(|(_, score)| *score >= SIMILARITY_MATCH_THRESHOLD)
        .max_by(|left, right| left.1.total_cmp(&right.1))
        .map(|(index, _)| index)
}

fn similarity(before: &str, after: &str) -> f32 {
    let before_chars = before.chars().collect::<Vec<_>>();
    let after_chars = after.chars().collect::<Vec<_>>();
    if before_chars.is_empty() || after_chars.is_empty() {
        return if before_chars.is_empty() && after_chars.is_empty() {
            1.0
        } else {
            0.0
        };
    }
    let mut table = vec![vec![0_usize; after_chars.len() + 1]; before_chars.len() + 1];
    for before_index in (0..before_chars.len()).rev() {
        for after_index in (0..after_chars.len()).rev() {
            table[before_index][after_index] =
                if before_chars[before_index] == after_chars[after_index] {
                    table[before_index + 1][after_index + 1] + 1
                } else {
                    table[before_index + 1][after_index].max(table[before_index][after_index + 1])
                };
        }
    }
    table[0][0] as f32 / before_chars.len().max(after_chars.len()) as f32
}

fn equal_row(before: &SourceLine, after: &SourceLine) -> CodeDiffAlignedRow {
    CodeDiffAlignedRow {
        before: line(
            CodeDiffLineKind::Equal,
            Some(before.number),
            before.text.clone(),
            Vec::new(),
        ),
        after: line(
            CodeDiffLineKind::Equal,
            Some(after.number),
            after.text.clone(),
            Vec::new(),
        ),
    }
}

fn changed_row(before: &SourceLine, after: &SourceLine) -> CodeDiffAlignedRow {
    let (before_ranges, after_ranges) =
        CodeDiffHighlighter::changed_ranges(&before.text, &after.text);
    CodeDiffAlignedRow {
        before: line(
            CodeDiffLineKind::Removed,
            Some(before.number),
            before.text.clone(),
            before_ranges,
        ),
        after: line(
            CodeDiffLineKind::Added,
            Some(after.number),
            after.text.clone(),
            after_ranges,
        ),
    }
}

fn remove_row(before: &SourceLine) -> CodeDiffAlignedRow {
    CodeDiffAlignedRow {
        before: line(
            CodeDiffLineKind::Removed,
            Some(before.number),
            before.text.clone(),
            full_range(&before.text),
        ),
        after: placeholder(),
    }
}

fn add_row(after: &SourceLine) -> CodeDiffAlignedRow {
    CodeDiffAlignedRow {
        before: placeholder(),
        after: line(
            CodeDiffLineKind::Added,
            Some(after.number),
            after.text.clone(),
            full_range(&after.text),
        ),
    }
}

fn line(
    kind: CodeDiffLineKind,
    line_number: Option<usize>,
    text: String,
    highlights: Vec<super::types::CodeDiffTextRange>,
) -> CodeDiffLine {
    CodeDiffLine {
        kind,
        line_number,
        text,
        highlights,
    }
}

fn placeholder() -> CodeDiffLine {
    line(
        CodeDiffLineKind::Placeholder,
        None,
        String::new(),
        Vec::new(),
    )
}

fn full_range(text: &str) -> Vec<super::types::CodeDiffTextRange> {
    vec![super::types::CodeDiffTextRange::new(
        0,
        text.chars().count().max(1),
    )]
}

fn model_from_rows(rows: Vec<CodeDiffAlignedRow>) -> CodeDiffModel {
    let added_count = rows
        .iter()
        .filter(|row| row.after.kind == CodeDiffLineKind::Added)
        .count();
    let removed_count = rows
        .iter()
        .filter(|row| row.before.kind == CodeDiffLineKind::Removed)
        .count();
    let changed_block_count = rows
        .iter()
        .fold((0_usize, false), |(count, active), row| {
            let changed = row.before.kind == CodeDiffLineKind::Removed
                || row.after.kind == CodeDiffLineKind::Added;
            if changed && !active {
                (count + 1, true)
            } else {
                (count, changed)
            }
        })
        .0;
    CodeDiffModel {
        rows,
        added_count,
        removed_count,
        changed_block_count,
    }
}
