use super::screen_state::StorybookScreenState;
use super::screen_state_tabs::{TabsScreenAction, TabsScreenUpdate};

impl StorybookScreenState {
    pub(super) fn register_tabs_preview_action(&mut self) {
        self.register_tabs_action(TabsScreenAction::AddTab);
    }

    pub(super) fn register_tabs_setting_change(&mut self) {
        self.settings_revision += 1;
        let update = self.tabs.apply(TabsScreenAction::AddTab);
        self.apply_tabs_update(update);
    }

    pub(super) fn register_tabs_action(&mut self, action: TabsScreenAction) {
        self.action_count += 1;
        let update = self.tabs.apply(action);
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
