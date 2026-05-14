use super::types::CodeDiffTextRange;

pub(crate) struct CodeDiffHighlighter;

impl CodeDiffHighlighter {
    pub(crate) fn changed_ranges(
        before: &str,
        after: &str,
    ) -> (Vec<CodeDiffTextRange>, Vec<CodeDiffTextRange>) {
        let before_chars = before.chars().collect::<Vec<_>>();
        let after_chars = after.chars().collect::<Vec<_>>();
        let matches = lcs_matches(&before_chars, &after_chars);
        (
            missing_ranges(before_chars.len(), matches.iter().map(|it| it.0)),
            missing_ranges(after_chars.len(), matches.iter().map(|it| it.1)),
        )
    }
}

fn lcs_matches(before: &[char], after: &[char]) -> Vec<(usize, usize)> {
    let mut table = vec![vec![0_usize; after.len() + 1]; before.len() + 1];
    for before_index in (0..before.len()).rev() {
        for after_index in (0..after.len()).rev() {
            table[before_index][after_index] = if before[before_index] == after[after_index] {
                table[before_index + 1][after_index + 1] + 1
            } else {
                table[before_index + 1][after_index].max(table[before_index][after_index + 1])
            };
        }
    }

    let mut before_index = 0;
    let mut after_index = 0;
    let mut matches = Vec::new();
    while before_index < before.len() && after_index < after.len() {
        if before[before_index] == after[after_index] {
            matches.push((before_index, after_index));
            before_index += 1;
            after_index += 1;
        } else if table[before_index + 1][after_index] >= table[before_index][after_index + 1] {
            before_index += 1;
        } else {
            after_index += 1;
        }
    }
    matches
}

fn missing_ranges(length: usize, kept: impl Iterator<Item = usize>) -> Vec<CodeDiffTextRange> {
    let mut ranges = Vec::new();
    let mut cursor = 0;
    for index in kept {
        if cursor < index {
            ranges.push(CodeDiffTextRange::new(cursor, index));
        }
        cursor = index + 1;
    }
    if cursor < length {
        ranges.push(CodeDiffTextRange::new(cursor, length));
    }
    ranges
}
