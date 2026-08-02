use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{
    CodeDiff, CodeDiffBuildError, CodeDiffDirection, CodeDiffLineHighlight, CodeDiffLineKind,
    CodeDiffMode, CodeDiffSource, CodeDiffSummary, CodeDiffWhitespace,
};

#[test]
fn code_diff_builds_from_before_after_first_line_and_line_count_contracts()
-> Result<(), CodeDiffBuildError> {
    let diff = CodeDiff::from_sources("Diff", "alpha\nbeta", 10, 2, "alpha\ngamma", 20, 2)?
        .language("rust");

    assert_eq!(CodeDiffMode::Split, diff.mode_model());
    assert_eq!("rust", diff.language_model());
    assert_eq!(
        Some(CodeDiffDirection::Horizontal),
        diff.effective_direction_model()
    );
    assert_eq!(Some(10), diff.lines()[0].old_number);
    assert_eq!(Some(20), diff.lines()[0].new_number);
    assert_eq!(1, diff.addition_count());
    assert_eq!(1, diff.removal_count());
    assert_eq!(
        CodeDiffSummary {
            additions: 1,
            removals: 1,
        },
        diff.summary()
    );
    Ok(())
}

#[test]
fn code_diff_reports_invalid_line_count_without_ambiguous_diff() {
    let result = CodeDiff::from_sources("Diff", "one\ntwo", 1, 1, "one\ntwo", 1, 2);
    let invalid_after = CodeDiff::from_sources("Diff", "one\ntwo", 1, 2, "one\ntwo", 1, 1);

    assert!(result.is_err());
    assert!(invalid_after.is_err());
}

#[test]
fn code_diff_returns_zero_summary_for_no_diff() -> Result<(), CodeDiffBuildError> {
    let diff = CodeDiff::from_sources("Diff", "same\ntext", 1, 2, "same\ntext", 1, 2)?;

    assert!(
        diff.lines()
            .iter()
            .all(|line| line.kind == CodeDiffLineKind::Context)
    );
    assert_eq!(0, diff.addition_count());
    assert_eq!(0, diff.removal_count());
    Ok(())
}

#[test]
fn code_diff_uses_lcs_classification_and_split_placeholders() -> Result<(), CodeDiffBuildError> {
    let diff = CodeDiff::from_sources("Diff", "keep\nremove\nstay", 1, 3, "keep\nadd\nstay", 1, 3)?;
    let kinds: Vec<CodeDiffLineKind> = diff.lines().iter().map(|line| line.kind).collect();

    assert_eq!(
        vec![
            CodeDiffLineKind::Context,
            CodeDiffLineKind::Removed,
            CodeDiffLineKind::Added,
            CodeDiffLineKind::Context,
        ],
        kinds
    );
    Ok(())
}

#[test]
fn code_diff_uses_character_positions_for_japanese_local_highlights()
-> Result<(), CodeDiffBuildError> {
    let diff = CodeDiff::from_sources("Diff", "名前:太郎", 1, 1, "名前:花子", 1, 1)?
        .local_highlight(CodeDiffLineHighlight {
            line_index: 2,
            start_character: 1,
            end_character: 2,
        });

    assert_eq!(3, diff.local_highlights().len());
    assert_eq!(
        vec![(0, 3, 5), (1, 3, 5), (2, 1, 2)],
        diff.local_highlight_ranges()
    );
    Ok(())
}

#[test]
fn code_diff_represents_empty_lines_trailing_newline_and_visible_whitespace()
-> Result<(), CodeDiffBuildError> {
    let diff = CodeDiff::new("Diff")
        .whitespace(CodeDiffWhitespace::visible("·", "→"))
        .source_texts("let value\t= 1\n", 1, 2, "let value = 1", 1, 1)?;

    assert!(diff.has_trailing_newline_difference());
    assert!(diff.lines().iter().any(|line| line.text == "↵"));
    assert!(diff.lines().iter().any(|line| line.text.contains('·')));
    assert!(diff.lines().iter().any(|line| line.text.contains('→')));
    Ok(())
}

#[test]
fn code_diff_collapses_and_recollapses_unchanged_blocks() -> Result<(), CodeDiffBuildError> {
    let mut diff = CodeDiff::from_sources(
        "Diff",
        "a\nb\nc\nd\ne\nold",
        1,
        6,
        "a\nb\nc\nd\ne\nnew",
        1,
        6,
    )?;

    assert_eq!(1, diff.collapsed_blocks().len());
    let expand = diff.apply_action(&UiAction::code_diff_expand(diff.state_id().clone()));
    assert!(expand.handled);
    assert!(diff.collapsed_blocks().is_empty());
    assert_eq!(1, diff.expanded_blocks().len());

    let collapse = diff.apply_action(&UiAction::code_diff_expand(diff.state_id().clone()));
    assert!(collapse.handled);
    assert_eq!(1, diff.collapsed_blocks().len());
    assert!(diff.expanded_blocks().is_empty());
    Ok(())
}

#[test]
fn code_diff_inline_ignores_direction_and_scroll_sync_toggles_state()
-> Result<(), CodeDiffBuildError> {
    let mut diff = CodeDiff::from_sources("Diff", "old", 1, 1, "new", 1, 1)?
        .mode(CodeDiffMode::Inline)
        .direction(CodeDiffDirection::Vertical);

    assert_eq!(None, diff.effective_direction_model());
    let sync = diff.apply_action(&UiAction::code_diff_scroll_sync(diff.state_id().clone()));
    assert!(sync.handled);
    assert!(diff.scroll_sync_enabled());
    assert!(sync.after.active);
    Ok(())
}

#[test]
fn code_diff_split_source_hidden_whitespace_and_extra_added_line_are_built() {
    let split = CodeDiffSource::Split {
        before: "old".to_string(),
        after: "new\nextra".to_string(),
    };
    let hidden_whitespace = CodeDiffWhitespace {
        visible: false,
        space_symbol: "·".to_string(),
        tab_symbol: "→".to_string(),
    };
    let diff = CodeDiff::new("Diff")
        .whitespace(hidden_whitespace)
        .source_texts("old value", 1, 1, "new value\nextra", 1, 2);
    assert!(diff.is_ok(), "valid split source");
    let Ok(diff) = diff else {
        return;
    };

    assert!(
        diff.lines()
            .iter()
            .any(|line| line.kind == CodeDiffLineKind::Placeholder)
    );
    assert!(diff.lines().iter().any(|line| line.text == "new value"));
    assert!(matches!(split, CodeDiffSource::Split { .. }));
}

#[test]
fn code_diff_builds_split_source_and_rejects_unified_source() -> Result<(), CodeDiffBuildError> {
    let split = CodeDiff::from_source(
        "Split",
        CodeDiffSource::Split {
            before: "before".to_string(),
            after: "after".to_string(),
        },
    )?;
    assert_eq!(1, split.summary().additions);
    assert_eq!(1, split.summary().removals);

    assert_eq!(
        Err(CodeDiffBuildError::UnsupportedUnifiedSource),
        CodeDiff::from_source(
            "Unified",
            CodeDiffSource::Unified {
                text: "@@".to_string(),
            },
        )
    );
    Ok(())
}
