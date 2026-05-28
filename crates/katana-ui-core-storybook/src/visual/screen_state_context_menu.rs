use super::interaction_spec::StorybookInteractionSpec;
use super::screen_state::StorybookScreenState;

impl StorybookScreenState {
    pub(super) fn register_context_menu(&mut self, page: &str) {
        if page != "tree-view" && page != "context-menu" {
            return;
        }
        self.action_count += 1;
        if page == "tree-view" {
            self.last_action = "tree_context_menu";
            self.last_event = "tree_context_opened";
            self.last_setting = "empty_area_context_menu";
            self.last_setting_value = "visible";
            self.state_label = "context_menu=open";
            return;
        }
        let spec = StorybookInteractionSpec::for_page(page);
        self.last_action = spec.action;
        self.last_event = spec.event;
        self.last_setting = spec.option;
        self.last_setting_value = spec.after;
        self.state_label = spec.state;
    }
}
