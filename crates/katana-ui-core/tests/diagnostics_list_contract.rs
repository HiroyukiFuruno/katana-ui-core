use katana_ui_core::atom::Text;
use katana_ui_core::molecule::{
    BulkFixSkipReason, CodeDiff, CodeDiffLine, CodeDiffLineKind, DiagnosticAction,
    DiagnosticFixPreview, DiagnosticItem, DiagnosticKeyboardInput, DiagnosticLocation,
    DiagnosticSeverity, DiagnosticsGroupBy, DiagnosticsList, DiagnosticsListAction,
    DiagnosticsListEvent, DiagnosticsListOptions, DiagnosticsListPlanner, DiagnosticsSortBy,
    ModalOverlay,
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
fn group_by_source_and_location_preserves_typed_groups() {
    let source_options = DiagnosticsListOptions {
        group_by: DiagnosticsGroupBy::Source,
        ..DiagnosticsListOptions::default()
    };
    let location_options = DiagnosticsListOptions {
        group_by: DiagnosticsGroupBy::Location,
        ..DiagnosticsListOptions::default()
    };

    let source_snapshot = DiagnosticsListPlanner::snapshot(&items(), &source_options);
    let location_snapshot = DiagnosticsListPlanner::snapshot(&items(), &location_options);

    assert_eq!("rustc", source_snapshot.groups[0].key);
    assert_eq!("clippy", source_snapshot.groups[1].key);
    assert_eq!("src/lib.rs", location_snapshot.groups[0].key);
    assert_eq!("src/main.rs", location_snapshot.groups[1].key);
}

#[test]
fn stable_sort_keeps_input_order_inside_equal_sort_keys() {
    let options = DiagnosticsListOptions {
        sort_by: DiagnosticsSortBy::Severity,
        ..DiagnosticsListOptions::default()
    };
    let ordered = vec![item_without_fix("warning-b"), item_without_fix("warning-a")];
    let snapshot = DiagnosticsListPlanner::snapshot(&ordered, &options);

    assert_eq!(vec![id("warning-b"), id("warning-a")], snapshot.visible_ids);
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
fn open_bulk_preview_renders_modal_overlay_with_typed_event() {
    let mut list = DiagnosticsList::new("Diagnostics")
        .item(item_with_fix("error-a"))
        .bulk_preview(ModalOverlay::new("Bulk fix preview").child(Text::new("Apply safe fixes")));
    let events = list.apply_action(DiagnosticsListAction::OpenBulkPreview);
    let tree = UiTree::new(list);

    assert!(matches!(
        events.as_slice(),
        [DiagnosticsListEvent::BulkFixPreviewOpened]
    ));
    assert!(
        tree.root()
            .children()
            .iter()
            .any(|it| it.kind() == UiNodeKind::ModalOverlay)
    );
}

#[test]
fn severity_filter_renders_chip_row_with_selected_state() {
    let options = DiagnosticsListOptions {
        severity_filter: [DiagnosticSeverity::Error, DiagnosticSeverity::Warning]
            .into_iter()
            .collect(),
        ..DiagnosticsListOptions::default()
    };
    let tree = UiTree::new(
        DiagnosticsList::new("Diagnostics")
            .option(options)
            .item(item_with_fix("error-a")),
    );
    let chips = tree
        .root()
        .children()
        .iter()
        .filter(|it| it.kind() == UiNodeKind::Chip)
        .collect::<Vec<_>>();

    assert_eq!(4, chips.len());
    assert!(
        chips
            .iter()
            .any(|it| { it.props().label == "Error" && it.props().interaction.has_selection })
    );
    assert!(
        chips
            .iter()
            .any(|it| { it.props().label == "Info" && !it.props().interaction.has_selection })
    );
}

#[test]
fn rendered_selection_reports_visible_index_and_missing_selection_fallback() {
    let mut selected = DiagnosticsList::new("Diagnostics")
        .item(item_with_fix("error-a"))
        .item(item_without_fix("warning-a"));
    selected.apply_action(DiagnosticsListAction::Keyboard(
        DiagnosticKeyboardInput::ArrowDown,
    ));
    selected.apply_action(DiagnosticsListAction::Keyboard(
        DiagnosticKeyboardInput::ArrowDown,
    ));
    let selected_tree = UiTree::new(selected);

    let mut missing = DiagnosticsList::new("Diagnostics").item(item_with_fix("error-a"));
    missing.apply_action(DiagnosticsListAction::Select(id("missing")));
    let missing_tree = UiTree::new(missing);

    assert!(selected_tree.root().props().interaction.has_selection);
    assert_eq!(1, selected_tree.root().props().interaction.selected_index);
    assert_eq!(2, selected_tree.root().props().interaction.item_count);
    assert!(missing_tree.root().props().interaction.has_selection);
    assert_eq!(0, missing_tree.root().props().interaction.selected_index);
    assert_eq!(1, missing_tree.root().props().interaction.item_count);
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

#[test]
fn keyboard_enter_requests_navigation_and_arrow_right_toggles_preview() {
    let mut list = DiagnosticsList::new("Diagnostics").item(item_with_fix("error-a"));
    list.apply_action(DiagnosticsListAction::Keyboard(
        DiagnosticKeyboardInput::ArrowDown,
    ));
    let navigate = list.apply_action(DiagnosticsListAction::Keyboard(
        DiagnosticKeyboardInput::Enter,
    ));
    let expand = list.apply_action(DiagnosticsListAction::Keyboard(
        DiagnosticKeyboardInput::ArrowRight,
    ));
    let collapse = list.apply_action(DiagnosticsListAction::Keyboard(
        DiagnosticKeyboardInput::ArrowLeft,
    ));

    assert!(matches!(
        navigate.as_slice(),
        [DiagnosticsListEvent::NavigateRequested { id }] if id.as_str() == "error-a"
    ));
    assert!(matches!(
        expand.as_slice(),
        [DiagnosticsListEvent::DiagnosticFixPreviewToggled { id, expanded }]
            if id.as_str() == "error-a" && *expanded
    ));
    assert!(matches!(
        collapse.as_slice(),
        [DiagnosticsListEvent::DiagnosticFixPreviewToggled { id, expanded }]
            if id.as_str() == "error-a" && !*expanded
    ));
}

#[test]
fn keyboard_arrows_and_shift_f8_move_selection_through_visible_items() {
    let options = DiagnosticsListOptions {
        severity_filter: [DiagnosticSeverity::Error, DiagnosticSeverity::Warning]
            .into_iter()
            .collect(),
        ..DiagnosticsListOptions::default()
    };
    let mut list = DiagnosticsList::new("Diagnostics")
        .option(options)
        .item(item_with_fix("error-a"))
        .item(item_without_fix("warning-a"))
        .item(item_with_fix("error-b"));

    let first = list.apply_action(DiagnosticsListAction::Keyboard(
        DiagnosticKeyboardInput::ArrowDown,
    ));
    let second = list.apply_action(DiagnosticsListAction::Keyboard(
        DiagnosticKeyboardInput::ArrowDown,
    ));
    let previous = list.apply_action(DiagnosticsListAction::Keyboard(
        DiagnosticKeyboardInput::ArrowUp,
    ));
    let next_error =
        list.apply_action(DiagnosticsListAction::Keyboard(DiagnosticKeyboardInput::F8));
    let previous_error = list.apply_action(DiagnosticsListAction::Keyboard(
        DiagnosticKeyboardInput::ShiftF8,
    ));

    assert!(matches!(
        first.as_slice(),
        [DiagnosticsListEvent::DiagnosticSelected { id }] if id.as_str() == "error-a"
    ));
    assert!(matches!(
        second.as_slice(),
        [DiagnosticsListEvent::DiagnosticSelected { id }] if id.as_str() == "warning-a"
    ));
    assert!(matches!(
        previous.as_slice(),
        [DiagnosticsListEvent::DiagnosticSelected { id }] if id.as_str() == "error-a"
    ));
    assert!(matches!(
        next_error.as_slice(),
        [DiagnosticsListEvent::DiagnosticSelected { id }] if id.as_str() == "error-b"
    ));
    assert!(matches!(
        previous_error.as_slice(),
        [DiagnosticsListEvent::DiagnosticSelected { id }] if id.as_str() == "error-a"
    ));
}

#[test]
fn render_snapshot_keeps_generic_visible_state_without_host_specific_values() {
    let mut list = DiagnosticsList::new("診断")
        .item(item_with_fix("error-a"))
        .item(item_without_fix("warning-a"));
    list.apply_action(DiagnosticsListAction::Select(id("warning-a")));

    let snapshot = list.render_snapshot();

    assert_eq!("診断", snapshot.label);
    assert_eq!(2, snapshot.visible.total_count);
    assert_eq!(Some(id("warning-a")), snapshot.state.selected_id);
    assert_eq!(
        vec![id("error-a"), id("warning-a")],
        snapshot.visible.visible_ids
    );
}

#[test]
fn scopes_reconcile_deterministically_and_filter_by_membership() {
    let mut list = DiagnosticsList::new("診断 ⭐️")
        .scope("all", "すべて ⭐️", "すべての診断 ⭐️")
        .scope("active", "現在", "現在の範囲")
        .item(item_with_fix("all").scope("all"))
        .item(item_without_fix("active").scope("active"))
        .item(item_without_fix("both").scopes(["all", "active"]));

    assert_eq!(
        Some("all"),
        list.render_snapshot()
            .state
            .selected_scope_key
            .as_ref()
            .map(|key| key.as_str())
    );
    assert_eq!(
        vec![id("all"), id("both")],
        list.render_snapshot().visible.visible_ids
    );

    let selected = list.apply_action(DiagnosticsListAction::Keyboard(
        DiagnosticKeyboardInput::ScopeNext,
    ));
    assert!(
        matches!(selected.as_slice(), [DiagnosticsListEvent::ScopeSelected { scope_key }] if scope_key.as_str() == "active")
    );
    assert_eq!(
        vec![id("active"), id("both")],
        list.render_snapshot().visible.visible_ids
    );

    list.set_scopes(vec![(
        "all".into(),
        "すべて ⭐️".into(),
        "すべての診断 ⭐️".into(),
    )]);
    assert_eq!(
        Some("all"),
        list.render_snapshot()
            .state
            .selected_scope_key
            .as_ref()
            .map(|key| key.as_str())
    );
    assert!(
        list.apply_action(DiagnosticsListAction::Keyboard(
            DiagnosticKeyboardInput::ScopeNext,
        ))
        .is_empty()
    );
}

#[test]
fn scope_keyboard_selection_uses_current_scopes_only() {
    let mut list = DiagnosticsList::new("診断")
        .scope("one", "一", "一")
        .scope("two", "二", "二");
    let events = list.apply_action(DiagnosticsListAction::Keyboard(
        DiagnosticKeyboardInput::ScopeNext,
    ));
    assert!(
        matches!(events.as_slice(), [DiagnosticsListEvent::ScopeSelected { scope_key }] if scope_key.as_str() == "two")
    );
    list.set_scopes(vec![("one".into(), "一".into(), "一".into())]);
    assert!(
        list.apply_action(DiagnosticsListAction::Keyboard(
            DiagnosticKeyboardInput::ScopePrevious,
        ))
        .is_empty()
    );
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
