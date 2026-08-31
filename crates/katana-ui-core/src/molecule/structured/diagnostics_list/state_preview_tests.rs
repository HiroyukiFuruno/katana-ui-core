use super::*;

#[test]
fn preview_state_transitions_open_bulk_and_toggle_one_diagnostic() {
    let mut state = DiagnosticsListState::default();
    let id = DiagnosticId::new("diagnostic");

    assert_eq!(
        state.toggle_fix_preview(id.clone()),
        vec![DiagnosticsListEvent::DiagnosticFixPreviewToggled {
            id: id.clone(),
            expanded: true,
        }]
    );
    assert!(state.expanded_ids.contains(&id));

    assert_eq!(
        state.toggle_fix_preview(id.clone()),
        vec![DiagnosticsListEvent::DiagnosticFixPreviewToggled {
            id: id.clone(),
            expanded: false,
        }]
    );
    assert!(!state.expanded_ids.contains(&id));

    assert_eq!(
        state.open_bulk_preview(),
        vec![DiagnosticsListEvent::BulkFixPreviewOpened]
    );
    assert!(state.bulk_preview_open);
}
