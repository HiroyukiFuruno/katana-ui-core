use super::screen_state::StorybookScreenState;
use super::screen_state_forms::{
    apply_binary_choice_option, apply_radio_selected_index_state, apply_radio_selected_state,
    radio_state_label,
};
use super::screen_state_setting_semantics::semantic_setting_state;
use super::screen_state_settings::{format_setting_action, format_setting_event};
use super::storybook_ui_option_contract::StorybookUiOptionContract;
use katana_ui_core::atom::Toggle;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
#[cfg(test)]
use katana_ui_core::state::UiComponentState;

#[path = "screen_state_forms_checkbox_bridge.rs"]
mod checkbox_bridge;

impl StorybookScreenState {
    pub(in crate::visual) fn register_radio_state_read(&mut self) {
        self.action_count += 1;
        self.last_action = "radio_state_read";
        self.last_event = "selected_read";
        self.state_label = radio_state_label(self.radio_state.checked, self.radio_state.checked);
    }

    pub(in crate::visual) fn register_radio_select(&mut self) {
        self.register_radio_select_index(0);
    }

    pub(in crate::visual) fn register_radio_select_index(&mut self, selected_index: usize) {
        self.action_count += 1;
        let before = self.radio_state.checked;
        self.radio_state = apply_radio_selected_index_state(&self.radio_state, selected_index);
        self.last_action = "radio_select";
        self.last_event = "radio_selected";
        self.state_label = radio_state_label(before, self.radio_state.checked);
    }

    pub(in crate::visual) fn register_radio_reset(&mut self) {
        self.action_count += 1;
        let before = self.radio_state.checked;
        self.radio_state = apply_radio_selected_state(&self.radio_state, false);
        self.last_action = "radio_reset";
        self.last_event = "radio_selected";
        self.state_label = radio_state_label(before, self.radio_state.checked);
    }

    pub(in crate::visual) fn register_radio_focus(&mut self) {
        if self.radio_state.disabled {
            self.last_action = "radio_focus_blocked";
            self.last_event = "radio_focus_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        let Some(next) = apply_binary_choice_option(&self.radio_state, "focus") else {
            return;
        };
        self.radio_state = next;
        self.last_action = "radio_focus";
        self.last_event = "radio_focused";
        self.state_label = "focused=true";
    }

    pub(in crate::visual) fn register_radio_keyboard_select(&mut self) {
        if self.radio_state.disabled {
            self.last_action = "radio_keyboard_blocked";
            self.last_event = "radio_keyboard_ignored";
            self.state_label = "keyboard=false";
            return;
        }
        if !self.radio_state.interaction.focused {
            self.last_action = "radio_keyboard_without_focus";
            self.last_event = "radio_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.register_radio_select();
        self.last_action = "radio_keyboard_select";
    }

    pub(in crate::visual) fn register_toggle_focus(&mut self, disabled: bool) {
        if disabled {
            self.last_action = "toggle_focus_blocked";
            self.last_event = "toggle_focus_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "toggle_focus";
        self.last_event = "toggle_focused";
        self.state_label = "focused=true";
    }

    pub(in crate::visual) fn register_toggle_change(&mut self) {
        self.register_toggle_checked_change("toggle_change");
    }

    pub(in crate::visual) fn register_toggle_keyboard_toggle(&mut self, disabled: bool) {
        if disabled {
            self.last_action = "toggle_keyboard_blocked";
            self.last_event = "toggle_keyboard_ignored";
            self.state_label = "keyboard=false";
            return;
        }
        if !self.button_focused {
            self.last_action = "toggle_keyboard_without_focus";
            self.last_event = "toggle_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.register_toggle_checked_change("toggle_keyboard_toggle");
    }

    fn register_toggle_checked_change(&mut self, story_action: &'static str) {
        let next = !self.toggle_checked;
        let mut toggle = Toggle::new("Markdown Linter").checked(self.toggle_checked);
        let result =
            toggle.apply_action(&UiAction::toggle_checked(toggle.state_id().clone(), next));
        if !result.handled {
            return;
        }
        self.toggle_checked = result.after.has_selection;
        self.toggle_checked_overridden = true;
        self.action_count += 1;
        self.last_action = story_action;
        self.last_event = "toggle_changed";
        self.state_label = toggle_checked_label(self.toggle_checked);
    }

    pub(in crate::visual) fn register_slide_focus(&mut self) {
        self.action_count += 1;
        self.button_focused = true;
        self.last_action = "slide_focus";
        self.last_event = "slide_focused";
        self.state_label = "focused=true";
    }

    pub(in crate::visual) fn register_slide_drag(&mut self) {
        self.action_count += 1;
        self.last_action = "slide_drag";
        self.last_event = "slide_changed";
        self.state_label = "value=64";
    }

    pub(in crate::visual) fn register_slide_keyboard_increment(&mut self) {
        if !self.button_focused {
            self.last_action = "slide_keyboard_without_focus";
            self.last_event = "slide_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.action_count += 1;
        self.last_action = "slide_keyboard_increment";
        self.last_event = "slide_changed";
        self.state_label = "value=64";
    }

    pub(super) const fn is_radio_selected(&self) -> bool {
        self.radio_state.checked
    }

    pub(super) const fn is_radio_disabled(&self) -> bool {
        self.radio_state.disabled
    }

    pub(super) const fn is_radio_focused(&self) -> bool {
        self.radio_state.interaction.focused
    }

    pub(super) const fn has_radio_selection(&self) -> bool {
        self.radio_state.interaction.has_selection
    }

    pub(super) const fn radio_selected_index(&self) -> usize {
        self.radio_state.interaction.selected_index
    }

    pub(in crate::visual) fn register_binary_choice_contract_setting(
        &mut self,
        page: &str,
        option: StorybookUiOptionContract,
    ) -> bool {
        if !matches!(page, "checkbox" | "radio") {
            return false;
        }
        if !self.apply_binary_choice_state(page, option.setting) {
            return false;
        }
        self.settings_revision += 1;
        self.last_action = format_setting_action(option.setting);
        self.last_event = format_setting_event(page);
        self.last_setting = option.setting;
        self.last_setting_value = option.after;
        self.state_label = semantic_setting_state(page, option);
        true
    }

    pub(in crate::visual) fn register_toggle_contract_setting(
        &mut self,
        option: StorybookUiOptionContract,
    ) -> bool {
        if option.setting == "checked" {
            self.toggle_checked = option.after == "true";
            self.toggle_checked_overridden = true;
        }
        self.settings_revision += 1;
        self.last_action = format_setting_action(option.setting);
        self.last_event = format_setting_event("toggle");
        self.last_setting = option.setting;
        self.last_setting_value = option.after;
        self.state_label = semantic_setting_state("toggle", option);
        true
    }

    pub(in crate::visual) fn apply_toggle_checked_preset_default(&mut self) {
        self.toggle_checked = true;
        self.toggle_checked_overridden = true;
        self.state_label = "checked=true";
    }

    pub(in crate::visual) fn uses_default_toggle_state(&self) -> bool {
        self.action_count == 0
            && self.last_action == "none"
            && self.last_event == "none"
            && self.state_label == "idle"
            && !self.toggle_checked
            && !self.toggle_checked_overridden
    }

    pub(in crate::visual) fn register_form_field_contract_setting(
        &mut self,
        option: StorybookUiOptionContract,
    ) -> bool {
        match option.setting {
            "form_field.invalid" => self.register_form_field_validate_option(option),
            "form_field.helper_text" => self.register_form_field_helper_option(option),
            "form_field.required" => self.register_form_field_required_option(option),
            _ => return false,
        }
        true
    }

    fn register_form_field_validate_option(&mut self, option: StorybookUiOptionContract) {
        self.settings_revision += 1;
        self.last_action = "field_validate";
        self.last_event = "validation_changed";
        self.last_setting = option.setting;
        self.last_setting_value = option.after;
        self.state_label = "form_field.invalid=true";
    }

    fn register_form_field_helper_option(&mut self, option: StorybookUiOptionContract) {
        self.settings_revision += 1;
        self.last_action = "form_field_helper_text";
        self.last_event = "helper_text_changed";
        self.last_setting = option.setting;
        self.last_setting_value = option.after;
        self.state_label = "form_field.helper_text=long";
    }

    fn register_form_field_required_option(&mut self, option: StorybookUiOptionContract) {
        self.settings_revision += 1;
        self.last_action = "form_field_required";
        self.last_event = "required_changed";
        self.last_setting = option.setting;
        self.last_setting_value = option.after;
        self.state_label = "form_field.required=true";
    }

    fn apply_binary_choice_state(&mut self, page: &str, setting: &str) -> bool {
        let target = match page {
            "checkbox" => &mut self.checkbox_state,
            "radio" => &mut self.radio_state,
            _ => return false,
        };
        let Some(next) = apply_binary_choice_option(target, setting) else {
            return false;
        };
        *target = next;
        true
    }

    #[cfg(test)]
    pub(in crate::visual) fn radio_state_snapshot(&self) -> &UiComponentState {
        &self.radio_state
    }
}

const fn toggle_checked_label(checked: bool) -> &'static str {
    if checked {
        "checked=true"
    } else {
        "checked=false"
    }
}
