use super::diagnostics_list_fixture::ERROR_ID;
use katana_ui_core::molecule::DiagnosticsListEvent;

pub(in crate::visual) fn assert_preview_event(events: &[DiagnosticsListEvent], expanded: bool) {
    assert!(
        matches!(
            events,
            [DiagnosticsListEvent::DiagnosticFixPreviewToggled { id, expanded: actual }]
            if id.as_str() == ERROR_ID && *actual == expanded
        ),
        "core diagnostics list must toggle fix preview"
    );
}
