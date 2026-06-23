#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::visual) struct DiagnosticsListOptionState {
    pub(in crate::visual) group_by_source: bool,
    pub(in crate::visual) sort_by_location: bool,
    pub(in crate::visual) severity_filter_error_only: bool,
    pub(in crate::visual) wrap_error_navigation_disabled: bool,
    pub(in crate::visual) virtualization_windowed: bool,
    pub(in crate::visual) bulk_action_apply: bool,
    pub(in crate::visual) fix_preview_collapsed: bool,
}

impl DiagnosticsListOptionState {
    pub(in crate::visual) fn apply(&mut self, setting: &str) {
        match setting {
            "diagnostics.group_by" => self.group_by_source = true,
            "diagnostics.sort_by" => self.sort_by_location = true,
            "diagnostics.severity_filter" => self.severity_filter_error_only = true,
            "diagnostics.wrap_error_navigation" => {
                self.wrap_error_navigation_disabled = true;
            }
            "diagnostics.virtualization" => self.virtualization_windowed = true,
            "diagnostics.bulk_action" => self.bulk_action_apply = true,
            "diagnostics.fix_preview" => self.fix_preview_collapsed = true,
            _ => {}
        }
    }
}
