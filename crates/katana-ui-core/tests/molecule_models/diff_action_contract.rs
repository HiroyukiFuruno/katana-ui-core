use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{
    CodeDiff, CodeDiffDirection, CodeDiffLine, CodeDiffLineKind, CodeDiffMode, CollapsedBlock,
};

#[test]
fn code_diff_updates_mode_direction_collapse_and_scroll_sync_from_actions() {
    let mut diff = CodeDiff::new("Diff")
        .mode(CodeDiffMode::Inline)
        .direction(CodeDiffDirection::Horizontal)
        .line(CodeDiffLine {
            old_number: Some(1),
            new_number: None,
            kind: CodeDiffLineKind::Removed,
            text: "古い line".to_string(),
        })
        .collapsed_block(CollapsedBlock {
            start_line: 2,
            line_count: 3,
        });

    let mode = diff.apply_action(&UiAction::code_diff_mode(diff.state_id().clone(), "Split"));
    let direction = diff.apply_action(&UiAction::code_diff_direction(
        diff.state_id().clone(),
        "Vertical",
    ));
    let language = diff.apply_action(&UiAction::code_diff_language(
        diff.state_id().clone(),
        "markdown",
    ));
    let expand = diff.apply_action(&UiAction::code_diff_expand(diff.state_id().clone()));
    let sync = diff.apply_action(&UiAction::code_diff_scroll_sync(diff.state_id().clone()));

    assert!(mode.handled);
    assert!(direction.handled);
    assert!(language.handled);
    assert!(expand.handled);
    assert!(sync.handled);
    assert_eq!(CodeDiffMode::Split, diff.mode_model());
    assert_eq!(CodeDiffDirection::Vertical, diff.direction_model());
    assert_eq!("markdown", diff.language_model());
    assert!(diff.collapsed_blocks().is_empty());
    assert!(diff.scroll_sync_enabled());
    assert!(sync.after.active);
}

#[test]
fn code_diff_rejects_wrong_target_invalid_values_and_empty_expand() {
    let mut diff = CodeDiff::new("Diff");
    let other = CodeDiff::new("Other");

    assert!(
        !diff
            .apply_action(&UiAction::code_diff_mode(
                other.state_id().clone(),
                "Inline",
            ))
            .handled
    );
    assert!(
        !diff
            .apply_action(&UiAction::code_diff_mode(
                diff.state_id().clone(),
                "invalid",
            ))
            .handled
    );
    assert!(
        !diff
            .apply_action(&UiAction::code_diff_direction(
                diff.state_id().clone(),
                "invalid",
            ))
            .handled
    );
    assert!(
        !diff
            .apply_action(&UiAction::code_diff_expand(diff.state_id().clone()))
            .handled
    );
    let focus = diff.apply_action(&UiAction::focus(diff.state_id().clone()));
    assert!(focus.handled);
    assert!(focus.after.focused);
}

#[test]
fn code_diff_accepts_lowercase_mode_and_direction_aliases() {
    let mut diff = CodeDiff::new("Diff");

    for mode in ["inline", "split"] {
        assert!(
            diff.apply_action(&UiAction::code_diff_mode(diff.state_id().clone(), mode))
                .handled
        );
    }
    for direction in ["horizontal", "vertical"] {
        assert!(
            diff.apply_action(&UiAction::code_diff_direction(
                diff.state_id().clone(),
                direction,
            ))
            .handled
        );
    }
}
