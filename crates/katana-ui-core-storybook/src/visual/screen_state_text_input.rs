use super::screen_state::StorybookScreenState;
use super::text_input_screen_state::DEFAULT_TEXT_INPUT_INSTANCE;

const TEXT_INPUT_CARET_BLINK_FRAMES: usize = 30;

impl StorybookScreenState {
    pub(super) fn register_text_input_focus(&mut self, initial_value: &str, readonly: bool) {
        self.register_text_input_focus_for(DEFAULT_TEXT_INPUT_INSTANCE, initial_value, readonly);
    }

    pub(super) fn register_text_input_focus_for(
        &mut self,
        instance: &'static str,
        initial_value: &str,
        readonly: bool,
    ) {
        self.action_count += 1;
        self.text_inputs.focus(instance, initial_value, readonly);
        self.last_action = "text_input_focus";
        self.last_event = "text_input_focused";
        self.last_setting = "interaction.value";
        self.last_setting_value = "focus";
        self.state_label = if readonly {
            "focused=true readonly=true"
        } else {
            "focused=true"
        };
    }

    pub(super) fn register_text_input_icon_button(&mut self) {
        self.action_count += 1;
        self.last_action = "text_input_icon_button";
        self.last_event = "text_input_icon_button_clicked";
        self.last_setting = "text_entry.trailing_icon_buttons.action";
        self.last_setting_value = "input.trailing_icon";
        self.state_label = "icon_button=clicked";
    }

    pub(super) fn register_text_input_character(&mut self, value: char, readonly: bool) -> bool {
        self.register_text_input_character_for(DEFAULT_TEXT_INPUT_INSTANCE, value, readonly)
    }

    pub(super) fn register_text_input_character_for(
        &mut self,
        instance: &'static str,
        value: char,
        readonly: bool,
    ) -> bool {
        if !self.text_input_focused_for(instance) {
            return false;
        }
        if readonly {
            self.register_text_input_readonly_block();
            return true;
        }
        let mut next = self.text_input_value_for(instance).to_string();
        next.push(value);
        self.apply_text_input_value(instance, next.as_str(), "text_input_type");
        true
    }

    pub(super) fn register_text_input_backspace(&mut self, readonly: bool) -> bool {
        self.register_text_input_backspace_for(DEFAULT_TEXT_INPUT_INSTANCE, readonly)
    }

    pub(super) fn register_text_input_backspace_for(
        &mut self,
        instance: &'static str,
        readonly: bool,
    ) -> bool {
        if !self.text_input_focused_for(instance) {
            return false;
        }
        if readonly {
            self.register_text_input_readonly_block();
            return true;
        }
        let mut next = self.text_input_value_for(instance).to_string();
        if next.pop().is_none() {
            return false;
        }
        self.apply_text_input_value(instance, next.as_str(), "text_input_delete_backward");
        true
    }

    pub(super) fn register_text_input_submit(&mut self) -> bool {
        if !self.text_input_focused() {
            return false;
        }
        self.action_count += 1;
        self.text_inputs.submit(DEFAULT_TEXT_INPUT_INSTANCE);
        self.last_action = "input_commit";
        self.last_event = "text_committed";
        self.last_setting = "interaction.value";
        self.last_setting_value = "keyboard";
        self.state_label = "value=typed";
        true
    }

    pub(super) fn text_input_value(&self) -> &str {
        self.text_input_value_for(DEFAULT_TEXT_INPUT_INSTANCE)
    }

    pub(super) fn text_input_value_for(&self, instance: &'static str) -> &str {
        self.text_inputs.value(instance)
    }

    pub(super) fn text_input_focused(&self) -> bool {
        self.text_input_focused_for(DEFAULT_TEXT_INPUT_INSTANCE)
    }

    pub(super) fn text_input_focused_for(&self, instance: &'static str) -> bool {
        self.text_inputs.focused(instance)
    }

    pub(super) fn text_input_uses_live_value(&self) -> bool {
        self.text_inputs
            .uses_live_value(DEFAULT_TEXT_INPUT_INSTANCE)
    }

    pub(super) fn text_input_caret_visible(&self) -> bool {
        self.text_inputs.caret_visible(DEFAULT_TEXT_INPUT_INSTANCE)
    }

    pub(super) fn show_text_input_caret(&mut self) -> bool {
        self.set_text_input_caret_visibility(true)
    }

    pub(super) fn update_text_input_caret_visibility(&mut self, elapsed_frames: usize) -> bool {
        if !self.text_input_focused() {
            return self.set_text_input_caret_visibility(false);
        }
        let blink_index = elapsed_frames / TEXT_INPUT_CARET_BLINK_FRAMES;
        self.set_text_input_caret_visibility(blink_index.is_multiple_of(2))
    }

    fn apply_text_input_value(&mut self, instance: &'static str, next: &str, action: &'static str) {
        self.action_count += 1;
        self.text_inputs.apply_value(instance, next);
        self.last_action = action;
        self.last_event = "text_input_changed";
        self.last_setting = "interaction.value";
        self.last_setting_value = "keyboard";
        self.state_label = "value=typing";
    }

    fn register_text_input_readonly_block(&mut self) {
        self.action_count += 1;
        self.last_action = "text_input_readonly_blocked";
        self.last_event = "text_input_readonly_ignored";
        self.last_setting = "readonly";
        self.last_setting_value = "true";
        self.state_label = "readonly=true";
    }

    fn set_text_input_caret_visibility(&mut self, visible: bool) -> bool {
        self.text_inputs
            .set_caret_visibility(DEFAULT_TEXT_INPUT_INSTANCE, visible)
    }
}
