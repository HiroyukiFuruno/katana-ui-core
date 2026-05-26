use super::screen_state::StorybookScreenState;
use super::text_input_screen_state::{
    apply_text_input_focus_state, apply_text_input_submit_state, apply_text_input_value_state,
    text_input_value,
};

impl StorybookScreenState {
    pub(super) fn register_text_input_focus(&mut self) {
        self.action_count += 1;
        self.text_input_uses_live_value = true;
        self.text_input_state = apply_text_input_focus_state(&self.text_input_state, true);
        self.last_action = "text_input_focus";
        self.last_event = "text_input_focused";
        self.last_setting = "interaction.value";
        self.last_setting_value = "focus";
        self.state_label = "focused=true";
    }

    pub(super) fn register_text_input_character(&mut self, value: char) -> bool {
        if !self.text_input_focused() {
            return false;
        }
        let mut next = self.text_input_value().to_string();
        next.push(value);
        self.apply_text_input_value(next.as_str(), "text_input_type");
        true
    }

    pub(super) fn register_text_input_backspace(&mut self) -> bool {
        if !self.text_input_focused() {
            return false;
        }
        let mut next = self.text_input_value().to_string();
        if next.pop().is_none() {
            return false;
        }
        self.apply_text_input_value(next.as_str(), "text_input_delete_backward");
        true
    }

    pub(super) fn register_text_input_submit(&mut self) -> bool {
        if !self.text_input_focused() {
            return false;
        }
        self.action_count += 1;
        self.text_input_uses_live_value = true;
        self.text_input_state = apply_text_input_submit_state(&self.text_input_state);
        self.last_action = "input_commit";
        self.last_event = "text_committed";
        self.last_setting = "interaction.value";
        self.last_setting_value = "keyboard";
        self.state_label = "value=typed";
        true
    }

    pub(super) fn text_input_value(&self) -> &str {
        text_input_value(&self.text_input_state)
    }

    pub(super) fn text_input_focused(&self) -> bool {
        self.text_input_state.interaction.focused
    }

    pub(super) const fn text_input_uses_live_value(&self) -> bool {
        self.text_input_uses_live_value
    }

    fn apply_text_input_value(&mut self, next: &str, action: &'static str) {
        self.action_count += 1;
        self.text_input_uses_live_value = true;
        self.text_input_state = apply_text_input_value_state(&self.text_input_state, next);
        self.last_action = action;
        self.last_event = "text_input_changed";
        self.last_setting = "interaction.value";
        self.last_setting_value = "keyboard";
        self.state_label = "value=typing";
    }
}
