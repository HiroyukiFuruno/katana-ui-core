use super::super::diagnostics_list_fixture::diagnostics_list;
use super::super::diagnostics_list_option_state::DiagnosticsListOptionState;
use katana_ui_core::molecule::DiagnosticsList;

#[derive(Debug, Clone, PartialEq)]
pub(in crate::visual) struct DiagnosticsListScreenState {
    pub(super) diagnostics: DiagnosticsList,
    pub(super) severity_filter_error_only: bool,
    pub(super) bulk_preview_open: bool,
    pub(super) bulk_applied: bool,
    pub(super) fix_preview_expanded: bool,
    pub(super) selected_item: bool,
    pub(super) keyboard_navigated: bool,
    pub(super) scroll_retained: bool,
    pub(super) callback_action: &'static str,
    pub(super) option_state: DiagnosticsListOptionState,
    pub(in crate::visual) focused: bool,
    pub(in crate::visual) hovered: bool,
}

impl Default for DiagnosticsListScreenState {
    fn default() -> Self {
        Self {
            diagnostics: diagnostics_list(),
            severity_filter_error_only: false,
            bulk_preview_open: false,
            bulk_applied: false,
            fix_preview_expanded: false,
            selected_item: false,
            keyboard_navigated: false,
            scroll_retained: false,
            callback_action: "none",
            option_state: DiagnosticsListOptionState::default(),
            focused: false,
            hovered: false,
        }
    }
}
