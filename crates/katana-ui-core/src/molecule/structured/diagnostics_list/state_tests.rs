use super::*;

fn scope(key: &str) -> DiagnosticScopeInput {
    DiagnosticScopeInput::new(key, format!("label-{key}"), format!("a11y-{key}"))
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
