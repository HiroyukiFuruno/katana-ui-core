use super::diagnostics_list_event_assertions::assert_preview_event;
use super::diagnostics_list_fixture::error_id;
use super::diagnostics_list_operation::DiagnosticsListStoryAction;
#[cfg(test)]
use super::diagnostics_list_option_state::DiagnosticsListOptionState;
use super::diagnostics_list_update::DiagnosticsListUpdate;
use katana_ui_core::molecule::{
    DiagnosticKeyboardInput, DiagnosticSeverity, DiagnosticsGroupBy, DiagnosticsListAction,
    DiagnosticsListEvent, DiagnosticsSortBy,
};

#[path = "diagnostics_list_state_types.rs"]
mod state_types;

pub(in crate::visual) use state_types::DiagnosticsListScreenState;

impl DiagnosticsListScreenState {
    pub(in crate::visual) fn apply_action(
        &mut self,
        action: DiagnosticsListStoryAction,
    ) -> DiagnosticsListUpdate {
        match action {
            DiagnosticsListStoryAction::ToggleFixPreview => self.toggle_fix_preview(),
            DiagnosticsListStoryAction::OpenBulkPreview => self.open_bulk_preview(),
            DiagnosticsListStoryAction::SelectItem => self.select_item(),
            DiagnosticsListStoryAction::FocusList => self.focus_list(),
            DiagnosticsListStoryAction::HoverItem => self.hover_item(),
            DiagnosticsListStoryAction::KeyboardNavigate => self.keyboard_navigate(),
            DiagnosticsListStoryAction::ScrollRetention => self.scroll_retention(),
        }
    }

    pub(in crate::visual) fn apply_option(&mut self, setting: &str) {
        self.option_state.apply(setting);
        match setting {
            "diagnostics.group_by" => self.apply_filter_action(
                DiagnosticsListAction::SetGroupBy(DiagnosticsGroupBy::Source),
                "diagnostic_group_by_source",
            ),
            "diagnostics.sort_by" => self.apply_filter_action(
                DiagnosticsListAction::SetSortBy(DiagnosticsSortBy::Location),
                "diagnostic_sort_by_location",
            ),
            "diagnostics.severity_filter" => self.apply_severity_filter(),
            "diagnostics.wrap_error_navigation" => {
                self.callback_action = "diagnostic_wrap_navigation_disabled";
            }
            "diagnostics.virtualization" => {
                self.callback_action = "diagnostic_virtualization_windowed";
            }
            "diagnostics.bulk_action" => {
                let _ = self.apply_bulk_fix();
            }
            "diagnostics.fix_preview" => {
                let _ = self.collapse_fix_preview();
            }
            _ => {}
        }
    }

    pub(in crate::visual) const fn callback_action(&self) -> &'static str {
        self.callback_action
    }

    pub(in crate::visual) const fn selected_item(&self) -> bool {
        self.selected_item
    }

    pub(in crate::visual) const fn keyboard_navigated(&self) -> bool {
        self.keyboard_navigated
    }

    pub(in crate::visual) const fn scroll_retained(&self) -> bool {
        self.scroll_retained
    }

    #[cfg(test)]
    pub(in crate::visual) const fn option_state(&self) -> DiagnosticsListOptionState {
        self.option_state
    }

    #[cfg(test)]
    pub(in crate::visual) const fn has_fix_preview(&self) -> bool {
        self.fix_preview_expanded
    }

    #[cfg(test)]
    pub(in crate::visual) const fn has_bulk_preview_open(&self) -> bool {
        self.bulk_preview_open
    }

    #[cfg(test)]
    pub(in crate::visual) const fn has_bulk_applied(&self) -> bool {
        self.bulk_applied
    }

    #[cfg(test)]
    pub(in crate::visual) const fn has_error_filter(&self) -> bool {
        self.severity_filter_error_only
    }

    fn toggle_fix_preview(&mut self) -> DiagnosticsListUpdate {
        let events = self
            .diagnostics
            .apply_action(DiagnosticsListAction::ToggleFixPreview(error_id()));
        assert_preview_event(&events, true);
        self.fix_preview_expanded = true;
        self.callback_action = "diagnostic_fix_preview";
        DiagnosticsListUpdate::new(
            "diagnostic_fix_preview",
            "diagnostic_fix_preview_toggled",
            "preview=true",
            "diagnostics.fix_preview",
        )
    }

    fn collapse_fix_preview(&mut self) -> DiagnosticsListUpdate {
        if !self.fix_preview_expanded {
            let events = self
                .diagnostics
                .apply_action(DiagnosticsListAction::ToggleFixPreview(error_id()));
            assert_preview_event(&events, true);
        }
        let events = self
            .diagnostics
            .apply_action(DiagnosticsListAction::ToggleFixPreview(error_id()));
        assert_preview_event(&events, false);
        self.fix_preview_expanded = false;
        self.callback_action = "diagnostic_fix_preview";
        DiagnosticsListUpdate::new(
            "diagnostic_fix_preview",
            "diagnostic_fix_preview_toggled",
            "diagnostics.fix_preview=Collapsed",
            "diagnostics.fix_preview",
        )
    }

    fn open_bulk_preview(&mut self) -> DiagnosticsListUpdate {
        let events = self
            .diagnostics
            .apply_action(DiagnosticsListAction::OpenBulkPreview);
        assert!(
            matches!(&events[..], [DiagnosticsListEvent::BulkFixPreviewOpened]),
            "core diagnostics list must open bulk preview"
        );
        self.bulk_preview_open = true;
        self.callback_action = "diagnostic_bulk_preview";
        DiagnosticsListUpdate::new(
            "diagnostic_bulk_preview",
            "diagnostic_bulk_preview_opened",
            "diagnostics.bulk_preview=open",
            "diagnostics.bulk_action",
        )
    }

    fn select_item(&mut self) -> DiagnosticsListUpdate {
        let events = self
            .diagnostics
            .apply_action(DiagnosticsListAction::Select(error_id()));
        assert!(
            matches!(&events[..], [DiagnosticsListEvent::DiagnosticSelected { id }] if id == &error_id()),
            "core diagnostics list must select a diagnostic item"
        );
        self.selected_item = true;
        self.callback_action = "diagnostic_select_item";
        DiagnosticsListUpdate::new(
            "diagnostic_select_item",
            "diagnostic_selected",
            "selected=syntax-error",
            "diagnostics.selection",
        )
    }

    fn focus_list(&mut self) -> DiagnosticsListUpdate {
        let events = self
            .diagnostics
            .apply_action(DiagnosticsListAction::Keyboard(DiagnosticKeyboardInput::F8));
        assert!(
            matches!(&events[..], [DiagnosticsListEvent::DiagnosticSelected { id }] if id == &error_id()),
            "core diagnostics list focus must be backed by keyboard diagnostic selection"
        );
        self.focused = true;
        self.selected_item = true;
        self.callback_action = "diagnostic_focus_list";
        DiagnosticsListUpdate::new(
            "diagnostic_focus_list",
            "diagnostic_selected",
            "focus=syntax-error",
            "diagnostics.focus",
        )
    }

    fn hover_item(&mut self) -> DiagnosticsListUpdate {
        self.hovered = true;
        self.callback_action = "diagnostic_hover_item";
        DiagnosticsListUpdate::new(
            "diagnostic_hover_item",
            "hover_start",
            "hover=syntax-error",
            "diagnostics.hover",
        )
    }

    fn keyboard_navigate(&mut self) -> DiagnosticsListUpdate {
        let _ = self
            .diagnostics
            .apply_action(DiagnosticsListAction::Keyboard(DiagnosticKeyboardInput::F8));
        let events = self
            .diagnostics
            .apply_action(DiagnosticsListAction::Keyboard(
                DiagnosticKeyboardInput::Enter,
            ));
        assert!(
            matches!(&events[..], [DiagnosticsListEvent::NavigateRequested { id }] if id == &error_id()),
            "core diagnostics list keyboard enter must request navigation for selected diagnostic"
        );
        self.keyboard_navigated = true;
        self.selected_item = true;
        self.callback_action = "diagnostic_keyboard_navigate";
        DiagnosticsListUpdate::new(
            "diagnostic_keyboard_navigate",
            "diagnostic_jump_requested",
            "jump=syntax-error",
            "diagnostics.keyboard",
        )
    }

    fn scroll_retention(&mut self) -> DiagnosticsListUpdate {
        let _ = self
            .diagnostics
            .apply_action(DiagnosticsListAction::Keyboard(
                DiagnosticKeyboardInput::ArrowDown,
            ));
        self.scroll_retained = true;
        self.callback_action = "diagnostic_scroll_retained";
        DiagnosticsListUpdate::new(
            "diagnostic_scroll_retained",
            "diagnostic_visible_range_kept",
            "scroll=selection-retained",
            "diagnostics.virtualization",
        )
    }

    fn apply_bulk_fix(&mut self) -> DiagnosticsListUpdate {
        let events = self
            .diagnostics
            .apply_action(DiagnosticsListAction::ConfirmBulkApply);
        assert!(
            matches!(&events[..], [DiagnosticsListEvent::BulkFixApplied { .. }]),
            "core diagnostics list must apply visible quick fixes"
        );
        self.bulk_applied = true;
        self.callback_action = "diagnostic_bulk_apply";
        DiagnosticsListUpdate::new(
            "diagnostic_bulk_apply",
            "diagnostic_bulk_fix_applied",
            "diagnostics.bulk_action=Apply",
            "diagnostics.bulk_action",
        )
    }

    fn apply_severity_filter(&mut self) {
        self.apply_filter_action(
            DiagnosticsListAction::SetSeverityFilter([DiagnosticSeverity::Error].into()),
            "diagnostic_filter_error",
        );
        self.severity_filter_error_only = true;
    }

    fn apply_filter_action(&mut self, action: DiagnosticsListAction, callback: &'static str) {
        let events = self.diagnostics.apply_action(action);
        assert!(
            matches!(&events[..], [DiagnosticsListEvent::FilterChanged]),
            "core diagnostics list must emit filter change event"
        );
        self.callback_action = callback;
    }
}
