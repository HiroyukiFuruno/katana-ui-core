use super::screen_state::StorybookScreenState;
use super::screen_state_tabs::{TabsContextMenuCommand, TabsScreenAction, TabsScreenUpdate};
use super::screen_state_tabs_core::core_event_name;
use super::screen_state_tabs_types::{TabsScreenTab, tabs_update};
use super::storybook_ui_option_contract::StorybookUiOptionContract;
use katana_ui_core::widget::molecules::{
    CloseableTabId, CloseableTabKeyboardInput, CloseableTabStripAction,
};

impl StorybookScreenState {
    pub(in crate::visual) fn register_tabs_preview_action(&mut self) {
        self.register_tabs_action(TabsScreenAction::AddTab);
    }

    pub(in crate::visual) fn register_tabs_contract_setting(
        &mut self,
        option: StorybookUiOptionContract,
    ) -> bool {
        if let Some(update) = self.tabs.apply_contract_option(option.setting) {
            self.settings_revision += 1;
            self.apply_tabs_update(TabsScreenUpdate {
                value: option.after,
                ..update
            });
            return true;
        }
        let Some(action) = tabs_action_for_setting(option.setting) else {
            return false;
        };
        self.settings_revision += 1;
        let update = self.tabs.apply(action);
        self.apply_tabs_update(update);
        self.last_setting = option.setting;
        self.last_setting_value = option.after;
        true
    }

    pub(in crate::visual) fn register_closeable_tab_strip_contract_setting(
        &mut self,
        option: StorybookUiOptionContract,
    ) -> bool {
        if option.setting != "active_tab_id" {
            return self.register_tabs_contract_setting(option);
        }
        self.settings_revision += 1;
        ensure_closeable_contract_tab(&mut self.tabs, option.after);
        let events = self
            .tabs
            .apply_core_tab_action(CloseableTabStripAction::SelectTab {
                tab_id: CloseableTabId::new(option.after),
            });
        let update = tabs_update(
            "select_tab",
            core_event_name(&events, "closeable_tab_select_missing"),
            option.setting,
            option.after,
            "tabs.active=settings",
        );
        self.apply_tabs_update(update);
        true
    }

    pub(in crate::visual) fn register_closeable_tab_strip_select(&mut self, tab_id: &str) {
        self.action_count += 1;
        let events = self
            .tabs
            .apply_core_tab_action(CloseableTabStripAction::SelectTab {
                tab_id: CloseableTabId::new(tab_id),
            });
        let state = if events.is_empty() {
            "tabs.active=missing"
        } else {
            "tabs.active=component"
        };
        let update = tabs_update(
            "select_tab",
            core_event_name(&events, "closeable_tab_select_missing"),
            "active_tab_id",
            "component",
            state,
        );
        self.apply_tabs_update(update);
    }

    pub(in crate::visual) fn register_tabs_action(&mut self, action: TabsScreenAction) {
        self.action_count += 1;
        let update = self.tabs.apply(action);
        self.apply_tabs_update(update);
    }

    pub(in crate::visual) fn register_tabs_horizontal_scroll(&mut self, delta_x: f32) {
        self.action_count += 1;
        let update = self.tabs.scroll_horizontal(delta_x);
        self.apply_tabs_update(update);
    }

    pub(in crate::visual) fn register_tabs_focus(&mut self, tab_id: &str) {
        self.action_count += 1;
        let update = self.tabs.focus_tab(tab_id);
        self.apply_tabs_update(update);
    }

    pub(in crate::visual) fn register_tabs_context_menu(
        &mut self,
        tab_id: &str,
        x: usize,
        y: usize,
    ) {
        self.action_count += 1;
        let update = self.tabs.open_context_menu_for_tab(tab_id, x, y);
        self.apply_tabs_update(update);
    }

    pub(in crate::visual) fn register_tabs_group_context_menu(
        &mut self,
        group_id: &str,
        x: usize,
        y: usize,
    ) {
        self.action_count += 1;
        let update = self.tabs.open_context_menu_for_group(group_id, x, y);
        self.apply_tabs_update(update);
    }

    pub(in crate::visual) fn register_tabs_context_command(
        &mut self,
        command: TabsContextMenuCommand,
    ) {
        self.action_count += 1;
        let update = self.tabs.apply_context_command(command);
        self.apply_tabs_update(update);
    }

    pub(in crate::visual) fn register_tabs_pin_icon_unpin(&mut self, tab_id: &str) {
        self.action_count += 1;
        let update = self.tabs.unpin_tab_by_icon(tab_id);
        self.apply_tabs_update(update);
    }

    pub(in crate::visual) fn register_tabs_drag_start(&mut self, tab_id: &str) {
        self.action_count += 1;
        let update = self.tabs.start_drag_tab(tab_id);
        self.apply_tabs_update(update);
    }

    pub(in crate::visual) fn register_tabs_drag_move(
        &mut self,
        tab_id: &str,
        to_visual_index: usize,
    ) {
        self.action_count += 1;
        let update = self.tabs.drag_tab_to_visual_index(tab_id, to_visual_index);
        self.apply_tabs_update(update);
    }

    pub(in crate::visual) fn register_tabs_drag_end(&mut self, tab_id: &str, committed: bool) {
        self.action_count += 1;
        let update = self.tabs.end_drag_tab(tab_id, committed);
        self.apply_tabs_update(update);
    }

    pub(in crate::visual) fn register_tabs_keyboard_input(
        &mut self,
        input: CloseableTabKeyboardInput,
    ) {
        self.action_count += 1;
        let update = self.tabs.apply_keyboard_input(input);
        self.apply_tabs_update(update);
    }

    fn apply_tabs_update(&mut self, update: TabsScreenUpdate) {
        self.last_action = update.action;
        self.last_event = update.event;
        self.last_setting = update.setting;
        self.last_setting_value = update.value;
        self.state_label = update.state;
    }
}

fn ensure_closeable_contract_tab(tabs: &mut super::screen_state_tabs::TabsScreenState, id: &str) {
    if tabs.tabs.iter().any(|tab| tab.id == id) {
        return;
    }
    tabs.tabs.push(TabsScreenTab::new(id, id).dirty(true));
}

fn tabs_action_for_setting(setting: &str) -> Option<TabsScreenAction> {
    match setting {
        "tabs.add" => Some(TabsScreenAction::AddTab),
        "tabs.close" => Some(TabsScreenAction::CloseActive),
        "tabs.pin" => Some(TabsScreenAction::TogglePinActive),
        "tabs.move" => Some(TabsScreenAction::MoveActiveRight),
        "tabs.group" => Some(TabsScreenAction::GroupActive),
        "tabs.overflow" | "tabs.active_scroll" => Some(TabsScreenAction::ToggleOverflow),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tabs_bridge_reports_unknown_settings_missing_tabs_and_existing_contract_tabs() {
        let mut screen = StorybookScreenState::default();
        assert!(
            !screen.register_tabs_contract_setting(StorybookUiOptionContract::new(
                "tabs.unknown",
                "before",
                "after"
            ))
        );

        screen.register_closeable_tab_strip_select("missing");
        assert_eq!("tabs.active=missing", screen.state_label);

        let mut tabs = super::super::screen_state_tabs::TabsScreenState::default();
        let existing_id = tabs.tabs[0].id.clone();
        let initial_len = tabs.tabs.len();
        ensure_closeable_contract_tab(&mut tabs, &existing_id);
        assert_eq!(initial_len, tabs.tabs.len());
        assert_eq!(None, tabs_action_for_setting("tabs.unknown"));
    }
}
