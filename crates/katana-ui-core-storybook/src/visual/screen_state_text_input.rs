use super::screen_state::StorybookScreenState;
use super::storybook_ui_option_contract::StorybookUiOptionContract;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::widget::atoms::Input;

const TEXT_INPUT_CARET_BLINK_FRAMES: usize = 30;

impl StorybookScreenState {
    pub(in crate::visual) fn register_text_input_focus_for(
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

    pub(in crate::visual) fn register_text_input_icon_button(&mut self) {
        self.action_count += 1;
        self.last_action = "text_input_icon_button";
        self.last_event = "text_input_icon_button_clicked";
        self.last_setting = "text_entry.trailing_icon_buttons.action";
        self.last_setting_value = "input.trailing_icon";
        self.state_label = "icon_button=clicked";
    }

    pub(in crate::visual) fn register_text_input_clear_action_for(
        &mut self,
        instance: &'static str,
        initial_value: &str,
        readonly: bool,
    ) {
        self.text_inputs.focus(instance, initial_value, readonly);
        let Some(value) = self.apply_core_text_input_clear_value(instance, readonly) else {
            self.register_text_input_readonly_block();
            return;
        };
        self.action_count += 1;
        self.text_inputs.apply_value(instance, value.as_str());
        self.last_action = "text_input_clear_action";
        self.last_event = "text_input_changed";
        self.last_setting = "text_entry.clear_action";
        self.last_setting_value = "cleared";
        self.state_label = "value=cleared";
    }

    pub(in crate::visual) fn register_text_input_character_for(
        &mut self,
        instance: &'static str,
        value: char,
        readonly: bool,
    ) -> bool {
        if !self.text_input_focused_for(instance) {
            return false;
        }
        let mut next = self.text_input_value_for(instance).to_string();
        next.push(value);
        let Some(value) = self.apply_core_text_input_value(instance, next.as_str(), readonly)
        else {
            self.register_text_input_readonly_block();
            return true;
        };
        self.apply_text_input_value(instance, value.as_str(), "text_input_type");
        true
    }

    pub(in crate::visual) fn register_text_input_backspace_for(
        &mut self,
        instance: &'static str,
        readonly: bool,
    ) -> bool {
        if !self.text_input_focused_for(instance) {
            return false;
        }
        let mut next = self.text_input_value_for(instance).to_string();
        if next.pop().is_none() {
            return false;
        }
        let Some(value) = self.apply_core_text_input_value(instance, next.as_str(), readonly)
        else {
            self.register_text_input_readonly_block();
            return true;
        };
        self.apply_text_input_value(instance, value.as_str(), "text_input_delete_backward");
        true
    }

    pub(in crate::visual) fn register_text_input_paste_for(
        &mut self,
        instance: &'static str,
        text: &str,
        readonly: bool,
    ) -> bool {
        if !self.text_input_focused_for(instance) {
            return false;
        }
        let Some(state) = self.apply_core_text_input_paste(instance, text, readonly) else {
            self.register_text_input_readonly_block();
            return true;
        };
        self.action_count += 1;
        self.text_inputs.apply_interaction(instance, state);
        self.last_action = "text_input_paste";
        self.last_event = "clipboard_paste";
        self.last_setting = "interaction.value";
        self.last_setting_value = "clipboard";
        self.state_label = "value=pasted";
        true
    }

    pub(in crate::visual) fn register_text_input_submit_for(
        &mut self,
        instance: &'static str,
    ) -> bool {
        if !self.text_input_focused_for(instance) {
            return false;
        }
        self.action_count += 1;
        self.text_inputs.submit(instance);
        self.last_action = "input_commit";
        self.last_event = "text_committed";
        self.last_setting = "interaction.value";
        self.last_setting_value = "keyboard";
        self.state_label = "value=typed";
        true
    }

    pub(in crate::visual) fn text_input_value(&self) -> &str {
        self.text_input_value_for(super::text_input_screen_state::DEFAULT_TEXT_INPUT_INSTANCE)
    }

    pub(in crate::visual) fn text_input_value_for(&self, instance: &'static str) -> &str {
        self.text_inputs.value(instance)
    }

    #[cfg(test)]
    pub(in crate::visual) fn text_input_focused(&self) -> bool {
        self.text_input_focused_for(super::text_input_screen_state::DEFAULT_TEXT_INPUT_INSTANCE)
    }

    pub(in crate::visual) fn text_input_focused_for(&self, instance: &'static str) -> bool {
        self.text_inputs.focused(instance)
    }

    pub(in crate::visual) fn text_input_uses_live_value_for(&self, instance: &'static str) -> bool {
        self.text_inputs.uses_live_value(instance)
    }

    #[cfg(test)]
    pub(in crate::visual) fn text_input_caret_visible(&self) -> bool {
        self.text_input_caret_visible_for(
            super::text_input_screen_state::DEFAULT_TEXT_INPUT_INSTANCE,
        )
    }

    pub(in crate::visual) fn text_input_caret_visible_for(&self, instance: &'static str) -> bool {
        self.text_inputs.caret_visible(instance)
    }

    pub(in crate::visual) fn show_text_input_caret_for(&mut self, instance: &'static str) -> bool {
        self.set_text_input_caret_visibility_for(instance, true)
    }

    #[cfg(test)]
    pub(in crate::visual) fn update_text_input_caret_visibility(
        &mut self,
        elapsed_frames: usize,
    ) -> bool {
        self.update_text_input_caret_visibility_for(
            super::text_input_screen_state::DEFAULT_TEXT_INPUT_INSTANCE,
            elapsed_frames,
        )
    }

    pub(in crate::visual) fn update_text_input_caret_visibility_for(
        &mut self,
        instance: &'static str,
        elapsed_frames: usize,
    ) -> bool {
        if !self.text_input_focused_for(instance) {
            return self.set_text_input_caret_visibility_for(instance, false);
        }
        let blink_index = elapsed_frames / TEXT_INPUT_CARET_BLINK_FRAMES;
        self.set_text_input_caret_visibility_for(instance, blink_index.is_multiple_of(2))
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

    fn apply_core_text_input_value(
        &self,
        instance: &'static str,
        next: &str,
        readonly: bool,
    ) -> Option<String> {
        let mut input = Input::new("Storybook text input")
            .value(self.text_input_value_for(instance))
            .readonly(readonly);
        let action = UiAction::input_value(input.state_id().clone(), next);
        let result = input.apply_action(&action);
        result.handled.then_some(result.after.value)
    }

    fn apply_core_text_input_clear_value(
        &self,
        instance: &'static str,
        readonly: bool,
    ) -> Option<String> {
        let mut input = Input::new("Storybook text input")
            .value(self.text_input_value_for(instance))
            .readonly(readonly);
        let action = UiAction::clear_value(input.state_id().clone());
        let result = input.apply_action(&action);
        result.handled.then_some(result.after.value)
    }

    pub(in crate::visual) fn apply_text_input_contract_option_for(
        &mut self,
        instance: &'static str,
        option: StorybookUiOptionContract,
    ) {
        if option.setting == "interaction.value" {
            self.text_inputs.apply_value(instance, option.after);
        }
    }

    fn set_text_input_caret_visibility_for(
        &mut self,
        instance: &'static str,
        visible: bool,
    ) -> bool {
        self.text_inputs.set_caret_visibility(instance, visible)
    }
}

#[cfg(test)]
mod tests {
    use super::StorybookScreenState;

    const INSTANCE: &str = "text-input.boundary";

    #[test]
    fn text_input_rejects_unfocused_empty_and_readonly_keyboard_operations() {
        let mut state = StorybookScreenState::default();
        assert!(!state.register_text_input_backspace_for(INSTANCE, false));
        assert!(!state.register_text_input_paste_for(INSTANCE, "paste", false));
        assert!(!state.register_text_input_submit_for(INSTANCE));

        state.register_text_input_focus_for(INSTANCE, "", false);
        assert!(!state.register_text_input_backspace_for(INSTANCE, false));

        let mut readonly = StorybookScreenState::default();
        readonly.register_text_input_focus_for(INSTANCE, "value", true);
        assert!(readonly.register_text_input_paste_for(INSTANCE, "paste", true));
        assert_eq!("value", readonly.text_input_value_for(INSTANCE));
        assert_eq!("text_input_readonly_blocked", readonly.last_action);
        assert_eq!("text_input_readonly_ignored", readonly.last_event);
    }
}
