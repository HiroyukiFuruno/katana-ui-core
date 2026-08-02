use super::button_options::{StorybookButtonOptionControl, StorybookButtonOptions};
use super::interaction_spec::StorybookInteractionSpec;
use super::screen_state::StorybookScreenState;

impl StorybookScreenState {
    pub(in crate::visual) fn register_button_click(&mut self, page: &str) {
        if self.button_options.disabled {
            self.last_action = "button_press_blocked";
            self.last_event = "button_disabled_ignored";
            self.state_label = "disabled=true";
            return;
        }
        self.action_count += 1;
        self.button_pressed = true;
        let spec = StorybookInteractionSpec::for_page(page);
        self.last_action = spec.action;
        self.last_event = spec.event;
        self.state_label = spec.state;
    }

    pub(in crate::visual) fn register_button_focus(&mut self) {
        if self.button_options.disabled || !self.button_options.focusable {
            self.last_action = "button_focus_blocked";
            self.last_event = "button_focus_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "button_focus";
        self.last_event = "button_focused";
        self.last_setting = "button.focusable";
        self.last_setting_value = "focus";
        self.state_label = "focused=true";
    }

    pub(in crate::visual) fn register_button_keyboard_activation(&mut self, page: &str) {
        if self.button_options.disabled || !self.button_options.keyboard_activation {
            self.last_action = "button_keyboard_blocked";
            self.last_event = "button_keyboard_ignored";
            self.state_label = "keyboard=false";
            return;
        }
        if !self.button_focused {
            self.last_action = "button_keyboard_without_focus";
            self.last_event = "button_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        self.button_pressed = true;
        let spec = StorybookInteractionSpec::for_page(page);
        self.last_action = spec.action;
        self.last_event = spec.event;
        self.last_setting = "button.keyboard_activation";
        self.last_setting_value = "Enter";
        self.state_label = spec.state;
    }

    pub(in crate::visual) fn register_card_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "card_focus";
        self.last_event = "card_focused";
        self.last_setting = "card.clickable";
        self.last_setting_value = "focus";
        self.state_label = "focused=true";
    }

    pub(in crate::visual) fn register_card_keyboard_activation(&mut self) {
        if !self.button_focused {
            self.last_action = "card_keyboard_without_focus";
            self.last_event = "card_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        self.button_pressed = true;
        let spec = StorybookInteractionSpec::for_page("card");
        self.last_action = spec.action;
        self.last_event = spec.event;
        self.last_setting = "card.clickable";
        self.last_setting_value = "Enter";
        self.state_label = spec.state;
    }

    pub(in crate::visual) fn register_chip_focus(&mut self, disabled: bool) {
        if disabled {
            self.last_action = "chip_focus_blocked";
            self.last_event = "chip_focus_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "chip_focus";
        self.last_event = "chip_focused";
        self.last_setting = "chip.focused";
        self.last_setting_value = "true";
        self.state_label = "focused=true";
    }

    pub(in crate::visual) fn register_chip_keyboard_dismiss(&mut self, disabled: bool) {
        if disabled {
            self.last_action = "chip_keyboard_blocked";
            self.last_event = "chip_keyboard_ignored";
            self.state_label = "keyboard=false";
            return;
        }
        if !self.button_focused {
            self.last_action = "chip_keyboard_without_focus";
            self.last_event = "chip_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        self.button_pressed = true;
        self.last_action = "chip_dismiss";
        self.last_event = "chip_dismissed";
        self.last_setting = "chip.dismissible";
        self.last_setting_value = "true";
        self.state_label = "dismissed=true";
    }

    pub(in crate::visual) fn register_button_option(
        &mut self,
        control: StorybookButtonOptionControl,
    ) {
        self.settings_revision += 1;
        self.button_options.apply_contract_after(control);
        self.last_action = "button_option_apply";
        self.last_event = "button_option_changed";
        self.last_setting = control.setting_name();
        self.last_setting_value = control.setting_value(self.button_options);
        self.state_label = control.state_label(self.button_options);
    }

    pub(in crate::visual) fn uses_default_button_options(&self) -> bool {
        self.action_count == 0
            && self.settings_revision == 0
            && self.last_action == "none"
            && self.last_event == "none"
            && self.last_setting == "none"
            && self.button_options == StorybookButtonOptions::default()
    }

    pub(in crate::visual) fn set_preview_hovered(&mut self, hovered: bool) -> bool {
        if self.preview_hovered == hovered {
            return false;
        }
        self.preview_hovered = hovered;
        true
    }

    pub(in crate::visual) fn set_hovered_text_input_icon_button_index(
        &mut self,
        index: Option<usize>,
    ) -> bool {
        if self.hovered_text_input_icon_button_index == index {
            return false;
        }
        self.hovered_text_input_icon_button_index = index;
        true
    }

    pub(in crate::visual) fn set_hovered_text_input_clear_action(&mut self, hovered: bool) -> bool {
        if self.hovered_text_input_clear_action == hovered {
            return false;
        }
        self.hovered_text_input_clear_action = hovered;
        true
    }

    pub(in crate::visual) fn set_hovered_text_area_icon_button_index(
        &mut self,
        index: Option<usize>,
    ) -> bool {
        if self.hovered_text_area_icon_button_index == index {
            return false;
        }
        self.hovered_text_area_icon_button_index = index;
        true
    }

    pub(in crate::visual) fn set_hovered_text_area_clear_action(&mut self, hovered: bool) -> bool {
        if self.hovered_text_area_clear_action == hovered {
            return false;
        }
        self.hovered_text_area_clear_action = hovered;
        true
    }

    pub(in crate::visual) fn set_hovered_toolbar_action_index(
        &mut self,
        index: Option<usize>,
    ) -> bool {
        if self.hovered_toolbar_action_index == index {
            return false;
        }
        self.hovered_toolbar_action_index = index;
        true
    }

    pub(in crate::visual) fn set_hovered_summary_index(&mut self, index: Option<usize>) -> bool {
        if self.hovered_summary_index == index {
            return false;
        }
        self.hovered_summary_index = index;
        true
    }

    pub(in crate::visual) fn has_widget_action(&self) -> bool {
        self.action_count > 0
    }

    pub(super) const fn is_button_pressed(&self) -> bool {
        self.button_pressed
    }

    pub(super) const fn is_button_focused(&self) -> bool {
        self.button_focused
    }

    pub(in crate::visual) fn release_button_press(&mut self) -> bool {
        if !self.button_pressed {
            return false;
        }
        self.button_pressed = false;
        self.state_label = "pressed=false";
        true
    }

    pub(in crate::visual) fn has_settings_override(&self) -> bool {
        self.settings_revision % 2 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::{StorybookButtonOptionControl, StorybookScreenState};

    #[test]
    fn button_card_and_chip_rejection_states_do_not_emit_widget_actions() {
        let mut disabled = StorybookScreenState::default();
        disabled.register_button_option(StorybookButtonOptionControl::Disabled);
        disabled.register_button_click("button");
        assert_eq!("button_press_blocked", disabled.last_action);
        disabled.register_button_focus();
        assert_eq!("button_focus_blocked", disabled.last_action);
        disabled.register_button_keyboard_activation("button");
        assert_eq!("button_keyboard_blocked", disabled.last_action);

        let mut unfocusable = StorybookScreenState::default();
        unfocusable.register_button_option(StorybookButtonOptionControl::Focusable);
        unfocusable.register_button_focus();
        assert_eq!("button_focus_blocked", unfocusable.last_action);

        let mut no_keyboard = StorybookScreenState::default();
        no_keyboard.register_button_option(StorybookButtonOptionControl::KeyboardActivation);
        no_keyboard.register_button_keyboard_activation("button");
        assert_eq!("button_keyboard_blocked", no_keyboard.last_action);

        let mut no_focus = StorybookScreenState::default();
        no_focus.register_button_keyboard_activation("button");
        assert_eq!("button_keyboard_without_focus", no_focus.last_action);
        no_focus.register_card_keyboard_activation();
        assert_eq!("card_keyboard_without_focus", no_focus.last_action);
        no_focus.register_chip_focus(true);
        assert_eq!("chip_focus_blocked", no_focus.last_action);
        no_focus.register_chip_keyboard_dismiss(true);
        assert_eq!("chip_keyboard_blocked", no_focus.last_action);
        no_focus.register_chip_keyboard_dismiss(false);
        assert_eq!("chip_keyboard_without_focus", no_focus.last_action);
        assert!(!no_focus.release_button_press());
    }

    #[test]
    fn release_button_press_clears_an_active_press() {
        let mut state = StorybookScreenState::default();
        state.register_button_click("button");
        assert!(state.release_button_press());
        assert!(!state.is_button_pressed());
    }

    #[test]
    fn summary_hover_reports_only_state_changes() {
        let mut state = StorybookScreenState::default();
        assert!(state.set_hovered_summary_index(Some(1)));
        assert!(!state.set_hovered_summary_index(Some(1)));
        assert!(state.set_hovered_summary_index(None));
    }
}
