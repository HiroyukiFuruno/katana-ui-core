use super::*;
use crate::molecule::structured::diagnostics_list::{
    DiagnosticAction, DiagnosticLocation, DiagnosticSeverity,
};

#[path = "state_preview_tests.rs"]
mod preview_tests;

fn scope(key: &str) -> DiagnosticScopeInput {
    DiagnosticScopeInput::new(key, format!("label-{key}"), format!("a11y-{key}"))
}

fn item(id: &str, key: &str) -> DiagnosticItem {
    DiagnosticItem::new(
        id,
        DiagnosticSeverity::Error,
        format!("message-{id}"),
        DiagnosticLocation::new("file.rs", 1, 1),
    )
    .scope(key)
}

#[test]
fn reconcile_scope_selection_fails_closed_and_recovers_to_first_scope() {
    let mut state = DiagnosticsListState {
        selected_scope_key: Some(DiagnosticScopeKey::new("stale")),
        ..DiagnosticsListState::default()
    };

    state.reconcile_scope_selection(&[]);
    assert_eq!(state.selected_scope_key, None);

    let first = scope("first");
    state.reconcile_scope_selection(&[first.clone(), scope("second")]);
    assert_eq!(state.selected_scope_key, Some(first.key));
}

#[test]
fn select_scope_returns_no_event_if_scope_is_invalid_or_unchanged() {
    let mut state = DiagnosticsListState::default();
    let first = scope("a");

    assert!(
        state
            .select_scope(
                DiagnosticScopeKey::new("missing"),
                std::slice::from_ref(&first),
            )
            .is_empty()
    );
    assert!(
        state
            .select_scope(
                DiagnosticScopeKey::new("missing"),
                std::slice::from_ref(&first),
            )
            .is_empty()
    );

    state = DiagnosticsListState {
        selected_scope_key: Some(DiagnosticScopeKey::new("a")),
        ..DiagnosticsListState::default()
    };
    assert!(
        state
            .select_scope(DiagnosticScopeKey::new("a"), &[first, scope("second")],)
            .is_empty()
    );
}

#[test]
fn keyboard_scope_next_routes_through_relative_scope_selection() {
    let first = scope("a");
    let second = scope("b");
    let mut state = DiagnosticsListState {
        selected_scope_key: Some(first.key.clone()),
        ..DiagnosticsListState::default()
    };

    assert_eq!(
        state.apply_keyboard(
            DiagnosticKeyboardInput::ScopeNext,
            &[],
            &[first, second.clone()],
            &DiagnosticsListOptions::default(),
        ),
        vec![DiagnosticsListEvent::ScopeSelected {
            scope_key: second.key,
        }]
    );
}

#[test]
fn select_scope_returns_event_when_scope_changes() {
    let mut state = DiagnosticsListState::default();
    let first = scope("a");
    let second = scope("b");
    let output = state.select_scope(
        DiagnosticScopeKey::new("b"),
        &[first.clone(), second.clone()],
    );
    assert_eq!(
        output,
        vec![DiagnosticsListEvent::ScopeSelected {
            scope_key: second.key.clone()
        }]
    );
    assert_eq!(state.selected_scope_key, Some(second.key));
}

#[test]
fn select_scope_relative_drops_event_when_single_scope_only() {
    let mut state = DiagnosticsListState::default();
    let only = scope("single");
    assert_eq!(state.select_scope_relative(&[only], true), Vec::new());
}

#[test]
fn select_scope_relative_wraps_when_moving_around_list() {
    let first = scope("a");
    let second = scope("b");
    let third = scope("c");
    let scopes = vec![first.clone(), second.clone(), third];

    let mut state = DiagnosticsListState {
        selected_scope_key: Some(first.key.clone()),
        ..DiagnosticsListState::default()
    };
    assert_eq!(
        state.select_scope_relative(&scopes, true),
        vec![DiagnosticsListEvent::ScopeSelected {
            scope_key: second.key.clone()
        }]
    );

    let mut state = DiagnosticsListState {
        selected_scope_key: Some(first.key.clone()),
        ..DiagnosticsListState::default()
    };
    assert_eq!(
        state.select_scope_relative(&scopes, false),
        vec![DiagnosticsListEvent::ScopeSelected {
            scope_key: scopes[2].key.clone()
        }]
    );
}

#[test]
fn keyboard_navigation_respects_selected_scope_and_avoids_hidden_diagnostics() {
    let first = scope("a");
    let second = scope("b");
    let hidden = item("hidden", "a").quickfix(DiagnosticAction::new("fix-a", "fix"));
    let visible = item("visible-a", "b").quickfix(DiagnosticAction::new("fix-b", "fix"));
    let visible_without_fix = item("visible-b", "b");

    let mut state = DiagnosticsListState {
        selected_scope_key: Some(second.key.clone()),
        selected_id: Some(hidden.id.clone()),
        ..DiagnosticsListState::default()
    };
    let items = vec![hidden.clone(), visible.clone(), visible_without_fix.clone()];
    let visible_id = visible.id.clone();

    assert_eq!(
        state.apply_keyboard(
            DiagnosticKeyboardInput::ArrowDown,
            &items,
            &[first.clone(), second.clone()],
            &DiagnosticsListOptions::default(),
        ),
        vec![DiagnosticsListEvent::DiagnosticSelected {
            id: visible_id.clone(),
        }]
    );
    assert_eq!(state.selected_id, Some(visible_id.clone()));

    state.selected_id = Some(hidden.id.clone());
    assert_eq!(
        state.apply_keyboard(
            DiagnosticKeyboardInput::F8,
            &items,
            &[first.clone(), second.clone()],
            &DiagnosticsListOptions::default(),
        ),
        vec![DiagnosticsListEvent::DiagnosticSelected {
            id: visible_id.clone(),
        }]
    );

    assert_eq!(
        state.apply_keyboard(
            DiagnosticKeyboardInput::Space,
            &items,
            &[first, second],
            &DiagnosticsListOptions::default(),
        ),
        vec![DiagnosticsListEvent::DiagnosticFixApplied { id: visible_id }]
    );
}

#[test]
fn keyboard_activation_rejects_a_retained_selection_outside_the_selected_scope() {
    let first = scope("a");
    let second = scope("b");
    let hidden = item("hidden", "a").quickfix(DiagnosticAction::new("fix-a", "fix"));
    let visible = item("visible", "b").quickfix(DiagnosticAction::new("fix-b", "fix"));
    let items = vec![hidden.clone(), visible];
    let scopes = vec![first.clone(), second.clone()];
    let options = DiagnosticsListOptions::default();
    let mut state = DiagnosticsListState {
        selected_scope_key: Some(first.key),
        selected_id: Some(hidden.id.clone()),
        expanded_ids: BTreeSet::from([hidden.id.clone()]),
        ..DiagnosticsListState::default()
    };

    assert_eq!(
        state.apply_action(
            DiagnosticsListAction::SelectScope(second.key.clone()),
            &items,
            &scopes,
            &options,
        ),
        vec![DiagnosticsListEvent::ScopeSelected {
            scope_key: second.key,
        }]
    );
    for input in [
        DiagnosticKeyboardInput::Space,
        DiagnosticKeyboardInput::Enter,
        DiagnosticKeyboardInput::ArrowRight,
        DiagnosticKeyboardInput::ArrowLeft,
    ] {
        assert!(
            state
                .apply_action(
                    DiagnosticsListAction::Keyboard(input),
                    &items,
                    &scopes,
                    &options,
                )
                .is_empty(),
            "hidden retained selection must not activate from {input:?}"
        );
    }
    assert_eq!(state.selected_id, Some(hidden.id.clone()));
    assert!(state.expanded_ids.contains(&hidden.id));
}

#[test]
fn confirm_bulk_apply_filters_items_outside_selected_scope() {
    let first = scope("a");
    let second = scope("b");
    let in_scope_with_fix =
        item("scope-a-with-fix", "a").quickfix(DiagnosticAction::new("a-with-fix", "fix"));
    let in_scope_without_fix = item("scope-a-without-fix", "a");
    let out_of_scope_with_fix =
        item("scope-b-with-fix", "b").quickfix(DiagnosticAction::new("b-with-fix", "fix"));

    let items = vec![
        in_scope_with_fix.clone(),
        in_scope_without_fix.clone(),
        out_of_scope_with_fix.clone(),
    ];

    let mut state = DiagnosticsListState {
        selected_scope_key: Some(first.key.clone()),
        ..DiagnosticsListState::default()
    };

    assert!(matches!(
        state
            .apply_action(
                DiagnosticsListAction::ConfirmBulkApply,
                &items,
                &[first, second],
                &DiagnosticsListOptions::default(),
            )
            .as_slice(),
        [DiagnosticsListEvent::BulkFixApplied { applied_ids, skipped_ids }]
            if applied_ids == &vec![in_scope_with_fix.id.clone()]
                && skipped_ids
                    == &vec![
                        (in_scope_without_fix.id.clone(), BulkFixSkipReason::NoQuickfix),
                        (out_of_scope_with_fix.id.clone(), BulkFixSkipReason::FilteredOut)
                    ]
    ));
}
