use super::super::screen_state::StorybookScreenState;
use super::super::screen_state_forms::{
    apply_binary_choice_option, apply_checkbox_checked_state, checkbox_state_label,
};
use katana_ui_core::state::UiComponentState;

impl StorybookScreenState {
    pub(in crate::visual) fn register_checkbox_state_read(&mut self) {
        self.action_count += 1;
        self.last_action = "checkbox_state_read";
        self.last_event = "checked_read";
        self.state_label = self.checkbox_read_state_label();
    }

    pub(in crate::visual) fn register_checkbox_toggle(&mut self) {
        self.register_checkbox_toggle_at(self.checkbox_focused_index);
    }

    pub(in crate::visual) fn register_checkbox_toggle_at(&mut self, index: usize) {
        self.action_count += 1;
        let before = self.checkbox_checked_at(index);
        let next = apply_checkbox_checked_state(self.checkbox_state_at(index), !before);
        self.set_checkbox_state_at(index, next);
        self.checkbox_state.interaction.focused = false;
        self.checkbox_secondary_state.interaction.focused = false;
        if let Some(focused) = apply_binary_choice_option(self.checkbox_state_at(index), "focus") {
            self.set_checkbox_state_at(index, focused);
        }
        self.checkbox_focused_index = checkbox_index(index);
        self.last_action = "checkbox_toggle";
        self.last_event = "checked_changed";
        self.state_label = checkbox_state_label(before, self.checkbox_checked_at(index));
    }

    pub(in crate::visual) fn register_checkbox_reset(&mut self) {
        self.action_count += 1;
        let before = self.checkbox_state.checked || self.checkbox_secondary_state.checked;
        self.checkbox_state = apply_checkbox_checked_state(&self.checkbox_state, false);
        self.checkbox_secondary_state =
            apply_checkbox_checked_state(&self.checkbox_secondary_state, false);
        self.last_action = "checkbox_reset";
        self.last_event = "checked_changed";
        self.state_label = checkbox_state_label(before, false);
    }

    pub(in crate::visual) fn register_checkbox_focus_at(&mut self, index: usize, disabled: bool) {
        if disabled || self.checkbox_state_at(index).disabled {
            self.last_action = "checkbox_focus_blocked";
            self.last_event = "checkbox_focus_ignored";
            self.state_label = "disabled=true";
            return;
        }
        self.action_count += 1;
        self.checkbox_state.interaction.focused = false;
        self.checkbox_secondary_state.interaction.focused = false;
        let Some(next) = apply_binary_choice_option(self.checkbox_state_at(index), "focus") else {
            return;
        };
        self.set_checkbox_state_at(index, next);
        self.checkbox_focused_index = checkbox_index(index);
        self.last_action = "checkbox_focus";
        self.last_event = "checkbox_focused";
        self.state_label = "focused=true";
    }

    pub(in crate::visual) fn register_checkbox_keyboard_toggle(&mut self, disabled: bool) {
        if disabled || self.checkbox_state_at(self.checkbox_focused_index).disabled {
            self.last_action = "checkbox_keyboard_blocked";
            self.last_event = "checkbox_keyboard_ignored";
            self.state_label = "disabled=true";
            return;
        }
        if !self
            .checkbox_state_at(self.checkbox_focused_index)
            .interaction
            .focused
        {
            self.last_action = "checkbox_keyboard_without_focus";
            self.last_event = "checkbox_keyboard_ignored";
            self.state_label = "focused=false";
            return;
        }
        self.register_checkbox_toggle();
        self.last_action = "checkbox_keyboard_toggle";
    }

    pub(in crate::visual) fn apply_checkbox_checked_preset_default(&mut self) {
        self.checkbox_state = apply_checkbox_checked_state(&self.checkbox_state, true);
        self.state_label = "checked=true";
    }

    pub(in crate::visual) fn apply_checkbox_disabled_preset_default(&mut self) {
        if let Some(primary) = apply_binary_choice_option(&self.checkbox_state, "disabled") {
            self.checkbox_state = primary;
        }
        if let Some(secondary) =
            apply_binary_choice_option(&self.checkbox_secondary_state, "disabled")
        {
            self.checkbox_secondary_state = secondary;
        }
        self.state_label = "disabled=true";
    }

    pub(in crate::visual) fn apply_checkbox_focus_preset_default(&mut self) {
        self.checkbox_state.interaction.focused = false;
        self.checkbox_secondary_state.interaction.focused = false;
        if let Some(primary) = apply_binary_choice_option(&self.checkbox_state, "focus") {
            self.checkbox_state = primary;
        }
        self.checkbox_focused_index = 0;
        self.state_label = "focused=true";
    }

    pub(in crate::visual) fn set_checkbox_hovered_index(&mut self, index: Option<usize>) -> bool {
        let next = index.map(checkbox_index);
        if self.checkbox_hovered_index == next {
            return false;
        }
        self.checkbox_hovered_index = next;
        true
    }

    pub(in crate::visual) fn uses_default_checkbox_state(&self) -> bool {
        self.action_count == 0
            && self.last_action == "none"
            && self.last_event == "none"
            && self.state_label == "idle"
            && !self.checkbox_state.checked
            && !self.checkbox_secondary_state.checked
    }

    pub(in crate::visual) const fn is_checkbox_checked(&self) -> bool {
        self.checkbox_state.checked
    }

    pub(in crate::visual) fn is_checkbox_checked_at(&self, index: usize) -> bool {
        self.checkbox_checked_at(index)
    }

    pub(in crate::visual) const fn checkbox_focused_index(&self) -> usize {
        self.checkbox_focused_index
    }

    pub(in crate::visual) const fn checkbox_hovered_index(&self) -> Option<usize> {
        self.checkbox_hovered_index
    }

    pub(in crate::visual) const fn is_checkbox_disabled(&self) -> bool {
        self.checkbox_state.disabled
    }

    pub(in crate::visual) const fn is_checkbox_focused(&self) -> bool {
        self.checkbox_state.interaction.focused
    }

    pub(in crate::visual) fn is_checkbox_focused_at(&self, index: usize) -> bool {
        self.checkbox_state_at(index).interaction.focused
    }

    #[cfg(test)]
    pub(in crate::visual) fn checkbox_state_snapshot(&self) -> &UiComponentState {
        &self.checkbox_state
    }

    #[cfg(test)]
    pub(in crate::visual) fn checkbox_state_snapshot_at(&self, index: usize) -> &UiComponentState {
        self.checkbox_state_at(index)
    }

    fn checkbox_state_at(&self, index: usize) -> &UiComponentState {
        if checkbox_index(index) == 0 {
            &self.checkbox_state
        } else {
            &self.checkbox_secondary_state
        }
    }

    fn set_checkbox_state_at(&mut self, index: usize, state: UiComponentState) {
        if checkbox_index(index) == 0 {
            self.checkbox_state = state;
        } else {
            self.checkbox_secondary_state = state;
        }
    }

    fn checkbox_checked_at(&self, index: usize) -> bool {
        self.checkbox_state_at(index).checked
    }

    fn checkbox_read_state_label(&self) -> &'static str {
        let state = self.checkbox_state_at(self.checkbox_focused_index);
        if state.disabled {
            return "disabled=true";
        }
        if state.interaction.focused {
            return "focused=true";
        }
        if state.checked {
            return "checked=true";
        }
        "checked=false"
    }
}

const fn checkbox_index(index: usize) -> usize {
    if index == 0 { 0 } else { 1 }
}
