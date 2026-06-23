use super::screen_state_tabs_context::context_update;
use super::screen_state_tabs_core::core_event_name;
use super::screen_state_tabs_types::{TabsScreenState, TabsScreenTab, TabsScreenUpdate};
use katana_ui_core::widget::molecules::{CloseableTabId, CloseableTabStripAction};

impl TabsScreenState {
    pub(in crate::visual) fn close_menu_tab(&mut self, tab_id: &str) -> TabsScreenUpdate {
        if self.tab_by_id(tab_id).is_some_and(|tab| tab.pinned) {
            return context_update(
                "tab_context_close",
                "closeable_tab_context_close_blocked",
                "blocked",
                "tabs.pinned=true close=blocked",
            );
        }
        self.close_with_core_action(
            CloseableTabStripAction::CloseTab {
                tab_id: CloseableTabId::new(tab_id),
            },
            "tab_context_close",
            "removed",
        )
    }

    pub(in crate::visual) fn close_other_menu_tabs(&mut self, tab_id: &str) -> TabsScreenUpdate {
        self.close_with_core_action(
            CloseableTabStripAction::CloseOthers {
                tab_id: CloseableTabId::new(tab_id),
            },
            "tab_context_close_others",
            "others",
        )
    }

    pub(in crate::visual) fn close_all_menu_tabs(&mut self) -> TabsScreenUpdate {
        self.close_with_core_action(
            CloseableTabStripAction::CloseAll,
            "tab_context_close_all",
            "all",
        )
    }

    pub(in crate::visual) fn close_menu_tabs_to_right(&mut self, tab_id: &str) -> TabsScreenUpdate {
        let visual_ids = self.visual_context_tab_ids();
        if !visual_ids.iter().any(|candidate| candidate == tab_id) {
            return context_update(
                "tab_context_close_right",
                "closeable_tab_context_command_missing",
                "none",
                "tabs.tab=missing",
            );
        }
        self.close_with_core_action(
            CloseableTabStripAction::CloseToRight {
                tab_id: CloseableTabId::new(tab_id),
            },
            "tab_context_close_right",
            "right",
        )
    }

    pub(in crate::visual) fn close_menu_tabs_to_left(&mut self, tab_id: &str) -> TabsScreenUpdate {
        let visual_ids = self.visual_context_tab_ids();
        if !visual_ids.iter().any(|candidate| candidate == tab_id) {
            return context_update(
                "tab_context_close_left",
                "closeable_tab_context_command_missing",
                "none",
                "tabs.tab=missing",
            );
        }
        self.close_with_core_action(
            CloseableTabStripAction::CloseToLeft {
                tab_id: CloseableTabId::new(tab_id),
            },
            "tab_context_close_left",
            "left",
        )
    }

    fn close_with_core_action(
        &mut self,
        tab_action: CloseableTabStripAction,
        action_name: &'static str,
        value: &'static str,
    ) -> TabsScreenUpdate {
        let events = self.apply_core_tab_action_confirming_dirty(tab_action);
        self.remove_empty_context_groups();
        context_update(
            action_name,
            core_event_name(&events, "closeable_tab_context_command_missing"),
            value,
            "tabs.context=applied",
        )
    }

    fn remove_empty_context_groups(&mut self) {
        self.groups.retain(|group| {
            self.tabs
                .iter()
                .any(|tab| tab.group_id.as_deref() == Some(group.id.as_str()))
        });
    }

    fn tab_by_id(&self, tab_id: &str) -> Option<&TabsScreenTab> {
        self.tabs.iter().find(|tab| tab.id == tab_id)
    }

    fn visual_context_tab_ids(&self) -> Vec<String> {
        self.core_visual_tab_ids()
    }
}
