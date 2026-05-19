use katana_ui_core::atom::Text;
use katana_ui_core::molecule::{
    BulkFixSkipReason, CodeDiff, CodeDiffLine, CodeDiffLineKind, DiagnosticAction,
    DiagnosticFixPreview, DiagnosticItem, DiagnosticKeyboardInput, DiagnosticLocation,
    DiagnosticSeverity, DiagnosticsGroupBy, DiagnosticsList, DiagnosticsListAction,
    DiagnosticsListEvent, DiagnosticsListOptions, DiagnosticsListPlanner, DiagnosticsSortBy,
};
use katana_ui_core::render_model::{UiNodeKind, UiTree};

#[test]
fn groups_filters_and_sorts_deterministically() {
    let options = DiagnosticsListOptions {
        group_by: DiagnosticsGroupBy::Severity,
        sort_by: DiagnosticsSortBy::Severity,
        severity_filter: [DiagnosticSeverity::Error, DiagnosticSeverity::Hint]
            .into_iter()
            .collect(),
        ..DiagnosticsListOptions::default()
    };
    let snapshot = DiagnosticsListPlanner::snapshot(&items(), &options);

    assert_eq!(2, snapshot.total_count);
    assert_eq!("Error", snapshot.groups[0].key);
    assert_eq!("Hint", snapshot.groups[1].key);
    assert_eq!(1, snapshot.groups[0].count);
    assert_eq!(1, snapshot.groups[1].count);
}

#[test]
fn expanded_fix_preview_renders_code_diff_with_distinct_state() {
    let mut list = DiagnosticsList::new("Diagnostics").item(item_with_fix("error-a"));
    list.apply_action(DiagnosticsListAction::ToggleFixPreview(id("error-a")));
    let tree = UiTree::new(list);
    let root = tree.root();

    assert_eq!(UiNodeKind::DiagnosticsList, root.kind());
    let preview = root
        .children()
        .iter()
        .find(|it| it.kind() == UiNodeKind::CodeDiff);
    assert!(preview.is_some());
    assert!(preview.is_some_and(|it| root.props().state_id != it.props().state_id));
}

#[test]
fn apply_fix_and_bulk_apply_emit_typed_events() {
    let mut list = DiagnosticsList::new("Diagnostics")
        .item(item_with_fix("error-a"))
        .item(item_without_fix("warning-a"));
    let fixed = list.apply_action(DiagnosticsListAction::ApplyFix(id("error-a")));
    let bulk = list.apply_action(DiagnosticsListAction::ConfirmBulkApply);

    assert!(matches!(
        fixed.as_slice(),
        [DiagnosticsListEvent::DiagnosticFixApplied { .. }]
    ));
    assert!(matches!(
        bulk.as_slice(),
        [DiagnosticsListEvent::BulkFixApplied { applied_ids, skipped_ids }]
            if applied_ids.len() == 1
                && skipped_ids == &vec![(id("warning-a"), BulkFixSkipReason::NoQuickfix)]
    ));
}

#[test]
fn bulk_apply_respects_current_filter() {
    let options = DiagnosticsListOptions {
        severity_filter: [DiagnosticSeverity::Error].into_iter().collect(),
        ..DiagnosticsListOptions::default()
    };
    let mut list = DiagnosticsList::new("Diagnostics")
        .option(options)
        .item(item_with_fix("error-a"))
        .item(
            item_without_fix("warning-a")
                .quickfix(DiagnosticAction::new("fix-warning", "Fix warning")),
        );
    let events = list.apply_action(DiagnosticsListAction::ConfirmBulkApply);

    assert!(matches!(
        events.as_slice(),
        [DiagnosticsListEvent::BulkFixApplied { applied_ids, skipped_ids }]
            if applied_ids == &vec![id("error-a")]
                && skipped_ids == &vec![(id("warning-a"), BulkFixSkipReason::FilteredOut)]
    ));
}

#[test]
fn empty_and_loading_slots_are_rendered_without_parent_state_conflict() {
    let empty = UiTree::new(
        DiagnosticsList::new("Diagnostics")
            .option(DiagnosticsListOptions {
                severity_filter: [DiagnosticSeverity::Hint].into_iter().collect(),
                ..DiagnosticsListOptions::default()
            })
            .item(item_with_fix("error-a"))
            .empty_slot(Text::new("No diagnostics")),
    );
    let loading = UiTree::new(
        DiagnosticsList::new("Diagnostics")
            .loading(true)
            .loading_slot(Text::new("Loading")),
    );

    assert_eq!("No diagnostics", empty.root().children()[0].props().label);
    assert_eq!("Loading", loading.root().children()[0].props().label);
    assert_ne!(
        empty.root().props().state_id,
        empty.root().children()[0].props().state_id
    );
}

#[test]
fn keyboard_f8_and_space_follow_problems_panel_convention() {
    let mut list = DiagnosticsList::new("Diagnostics")
        .item(item_with_fix("error-a"))
        .item(item_without_fix("warning-a"));
    let selected = list.apply_action(DiagnosticsListAction::Keyboard(DiagnosticKeyboardInput::F8));
    let applied = list.apply_action(DiagnosticsListAction::Keyboard(
        DiagnosticKeyboardInput::Space,
    ));

    assert!(matches!(
        selected.as_slice(),
        [DiagnosticsListEvent::DiagnosticSelected { id }] if id.as_str() == "error-a"
    ));
    assert!(matches!(
        applied.as_slice(),
        [DiagnosticsListEvent::DiagnosticFixApplied { id }] if id.as_str() == "error-a"
    ));
}

fn items() -> Vec<DiagnosticItem> {
    vec![
        item_with_fix("error-a"),
        item_without_fix("warning-a"),
        DiagnosticItem::new(
            "hint-a",
            DiagnosticSeverity::Hint,
            "Unused import",
            DiagnosticLocation::new("src/lib.rs", 1, 1),
        ),
    ]
}

fn item_with_fix(id: &str) -> DiagnosticItem {
    DiagnosticItem::new(
        id,
        DiagnosticSeverity::Error,
        "Syntax error",
        DiagnosticLocation::new("src/lib.rs", 3, 12),
    )
    .source("rustc")
    .quickfix(DiagnosticAction::new("fix", "Apply fix"))
    .fix_preview(DiagnosticFixPreview::new(
        CodeDiff::new("Fix preview").line(CodeDiffLine {
            old_number: Some(3),
            new_number: Some(3),
            kind: CodeDiffLineKind::Added,
            text: "let value = 1;".to_string(),
        }),
    ))
}

fn item_without_fix(id: &str) -> DiagnosticItem {
    DiagnosticItem::new(
        id,
        DiagnosticSeverity::Warning,
        "Unused variable",
        DiagnosticLocation::new("src/main.rs", 7, 4),
    )
    .source("clippy")
}

fn id(value: &str) -> katana_ui_core::molecule::DiagnosticId {
    katana_ui_core::molecule::DiagnosticId::new(value)
}
