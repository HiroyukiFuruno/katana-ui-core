use super::{
    CodeDiff, CodeDiffError, CodeDiffLineKind, CodeDiffMode, CodeDiffSource,
    CodeDiffSplitOrientation,
};

fn source(text: &str, first_line_number: usize) -> CodeDiffSource {
    CodeDiffSource::new(text, first_line_number, line_count(text))
}

fn line_count(text: &str) -> usize {
    if text.is_empty() {
        0
    } else {
        text.split('\n').count()
    }
}

fn build_model(
    before: &str,
    after: &str,
    first_line_number: usize,
) -> Result<super::types::CodeDiffModel, String> {
    CodeDiff::new(
        source(before, first_line_number),
        source(after, first_line_number),
    )
    .model()
    .map_err(|error| error.to_string())
}

fn build_model_error(
    before: CodeDiffSource,
    after: CodeDiffSource,
) -> Result<CodeDiffError, String> {
    match CodeDiff::new(before, after).model() {
        Ok(_) => Err("expected error, but model build succeeded".to_string()),
        Err(error) => Ok(error),
    }
}

#[test]
fn code_diff_default_is_horizontal_split() {
    let diff = CodeDiff::new(source("a", 1), source("a", 1));

    assert_eq!(diff.props.mode, CodeDiffMode::Split);
    assert_eq!(
        diff.props.split_orientation,
        CodeDiffSplitOrientation::Horizontal
    );
}

#[test]
fn code_diff_reports_no_changes() -> Result<(), String> {
    let model = build_model("a\nb", "a\nb", 10)?;

    assert!(!model.has_changes());
    assert_eq!(model.added_count, 0);
    assert_eq!(model.removed_count, 0);
    assert_eq!(model.rows[0].before.line_number, Some(10));
    Ok(())
}

#[test]
fn code_diff_detects_added_and_removed_lines() -> Result<(), String> {
    let model = build_model("a\nb", "a\nc\nd", 1)?;

    assert_eq!(model.removed_count, 1);
    assert_eq!(model.added_count, 2);
    assert!(
        model
            .rows
            .iter()
            .any(|row| row.before.kind == CodeDiffLineKind::Removed)
    );
    assert!(
        model
            .rows
            .iter()
            .any(|row| row.after.kind == CodeDiffLineKind::Added)
    );
    Ok(())
}

#[test]
fn code_diff_pairs_replaced_lines_and_highlights_characters() -> Result<(), String> {
    let model = build_model("let name = \"old\";", "let name = \"new\";", 1)?;

    assert_eq!(model.changed_block_count, 1);
    assert_eq!(model.rows[0].before.kind, CodeDiffLineKind::Removed);
    assert_eq!(model.rows[0].after.kind, CodeDiffLineKind::Added);
    assert!(!model.rows[0].before.highlights.is_empty());
    assert!(!model.rows[0].after.highlights.is_empty());
    Ok(())
}

#[test]
fn code_diff_keeps_trailing_newline_as_display_line() -> Result<(), String> {
    let model = build_model("a", "a\n", 1)?;

    assert_eq!(model.added_count, 1);
    let last_row = model
        .rows
        .last()
        .ok_or_else(|| "expected trailing display row, but no rows were built".to_string())?;
    assert_eq!(last_row.after.text, "");
    Ok(())
}

#[test]
fn code_diff_accepts_empty_source_as_zero_lines() -> Result<(), String> {
    let model = build_model("", "a\nb", 1)?;

    assert_eq!(model.added_count, 2);
    assert_eq!(model.removed_count, 0);
    Ok(())
}

#[test]
fn code_diff_rejects_line_count_mismatch() -> Result<(), String> {
    let error = build_model_error(CodeDiffSource::new("a\n", 1, 1), source("a", 1))?;

    assert!(matches!(error, CodeDiffError::LineCountMismatch { .. }));
    Ok(())
}

#[test]
fn code_diff_highlight_uses_character_indices_for_multibyte_text() -> Result<(), String> {
    let model = build_model("名前 = \"太郎\"", "名前 = \"花子\"", 1)?;

    assert!(
        model.rows[0]
            .before
            .highlights
            .iter()
            .all(|range| range.end <= 9)
    );
    assert!(
        model.rows[0]
            .after
            .highlights
            .iter()
            .all(|range| range.end <= 9)
    );
    Ok(())
}

#[test]
fn code_diff_tracks_space_and_tab_changes_as_highlight_ranges() -> Result<(), String> {
    let model = build_model("let\tname = value", "let name  = value", 1)?;

    assert!(!model.rows[0].before.highlights.is_empty());
    assert!(!model.rows[0].after.highlights.is_empty());
    Ok(())
}

#[test]
fn code_diff_pairs_similar_lines_across_blank_anchor() -> Result<(), String> {
    let before = "fn main() {\n\n    old_call();\n}";
    let after = "fn main() {\n\n    new_call();\n}";
    let model = build_model(before, after, 1)?;

    let changed_rows = model
        .rows
        .iter()
        .filter(|row| row.before.kind == CodeDiffLineKind::Removed)
        .count();
    assert_eq!(changed_rows, 1);
    assert_eq!(model.changed_block_count, 1);
    Ok(())
}
