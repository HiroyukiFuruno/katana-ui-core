use super::screen_state::StorybookScreenState;
use super::window_interaction::theme_tokens_operation::ThemeTokensStoryUpdate;

impl StorybookScreenState {
    pub(in crate::visual) fn apply_theme_tokens_update(&mut self, update: ThemeTokensStoryUpdate) {
        self.last_action = update.action;
        self.last_event = update.event;
        self.state_label = update.state;
    }
}
