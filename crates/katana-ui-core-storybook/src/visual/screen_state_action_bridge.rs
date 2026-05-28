use super::screen_state::StorybookScreenState;
use super::search_box_screen_state::SearchBoxScreenAction;
use super::selection_screen_state::SelectionScreenAction;

impl StorybookScreenState {
    pub(super) fn register_selection_action(&mut self, action: SelectionScreenAction) {
        self.action_count += 1;
        let update = self.selection.apply(action);
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }

    pub(super) fn register_search_box_action(&mut self, action: SearchBoxScreenAction) {
        self.action_count += 1;
        let update = self.search_box.apply(action);
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }
}
