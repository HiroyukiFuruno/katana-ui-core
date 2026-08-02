use super::screen_state::StorybookScreenState;
use super::storybook_ui_option_contract::StorybookUiOptionContract;
use katana_ui_core::widget::atoms::TextAreaAction;

const TEXT_AREA_CARET_BLINK_FRAMES: usize = 30;

pub(super) enum TextAreaInputKey {
    Character(char),
    Backspace,
    Newline,
    Submit,
}

impl StorybookScreenState {
    pub(in crate::visual) fn register_text_area_focus_for(
        &mut self,
        instance: &'static str,
        readonly: bool,
        disabled: bool,
    ) {
        self.action_count += 1;
        {
            let runtime = self.text_area_runtime_mut_for(instance);
            runtime.focused = true;
            runtime.uses_live_value = true;
            runtime.caret_visible = true;
            runtime.readonly = readonly;
            runtime.disabled = disabled;
        }
        self.last_action = "text_area_focus";
        self.last_event = "text_area_focused";
        self.last_setting = "text_area.value";
        self.last_setting_value = "focus";
        self.state_label = if disabled {
            "focused=true disabled=true"
        } else if readonly {
            "focused=true readonly=true"
        } else {
            "focused=true"
        };
    }

    pub(in crate::visual) fn register_text_area_key_for(
        &mut self,
        instance: &'static str,
        key: TextAreaInputKey,
    ) -> bool {
        if !self.text_area_focused_for(instance) {
            return false;
        }
        let handled = match key {
            TextAreaInputKey::Character(value) => {
                self.register_text_area_character_for(instance, value)
            }
            TextAreaInputKey::Backspace => self.register_text_area_backspace_for(instance),
            TextAreaInputKey::Newline => self.register_text_area_newline_for(instance),
            TextAreaInputKey::Submit => self.register_text_area_submit_for(instance),
        };
        if handled {
            return true;
        }
        debug_assert!(
            self.text_area_readonly_for(instance) || self.text_area_disabled_for(instance),
            "focused editable text-area actions must be handled by KUC"
        );
        self.register_text_area_mutation_block_for(instance);
        true
    }

    pub(in crate::visual) fn show_text_area_caret_for(&mut self, instance: &'static str) -> bool {
        self.set_text_area_caret_visibility_for(instance, true)
    }

    #[cfg(test)]
    pub(in crate::visual) fn update_text_area_caret_visibility(
        &mut self,
        elapsed_frames: usize,
    ) -> bool {
        self.update_text_area_caret_visibility_for(
            super::text_area_screen_state::DEFAULT_TEXT_AREA_INSTANCE,
            elapsed_frames,
        )
    }

    pub(in crate::visual) fn update_text_area_caret_visibility_for(
        &mut self,
        instance: &'static str,
        elapsed_frames: usize,
    ) -> bool {
        if !self.text_area_focused_for(instance) {
            return self.set_text_area_caret_visibility_for(instance, false);
        }
        let blink_index = elapsed_frames / TEXT_AREA_CARET_BLINK_FRAMES;
        self.set_text_area_caret_visibility_for(instance, blink_index.is_multiple_of(2))
    }

    pub(in crate::visual) fn register_text_area_resize_drag_for(
        &mut self,
        instance: &'static str,
        width_delta: usize,
        height_delta: usize,
    ) -> bool {
        let runtime = self.text_area_runtime_for(instance);
        if runtime.resize_width_delta() == width_delta
            && runtime.resize_height_delta() == height_delta
        {
            return false;
        }
        self.action_count += 1;
        let outcome = self.apply_core_text_area_resize_action_for(
            instance,
            TextAreaAction::resize(width_delta as u16, height_delta as u16),
        );
        self.sync_text_area_runtime_for(instance, outcome);
        self.last_action = "text_area_resize_drag";
        self.last_event = "text_area_resized";
        self.last_setting = "text_area.resize_enabled";
        self.last_setting_value = "drag";
        self.state_label = "size=changed";
        true
    }

    pub(in crate::visual) fn register_text_area_icon_button(&mut self) {
        self.action_count += 1;
        self.last_action = "text_area_icon_button";
        self.last_event = "text_area_icon_button_clicked";
        self.last_setting = "text_area.trailing_icon_buttons.action";
        self.last_setting_value = "text_area.trailing_icon";
        self.state_label = "icon_button=clicked";
    }

    fn register_text_area_character_for(&mut self, instance: &'static str, value: char) -> bool {
        let outcome =
            self.apply_core_text_area_action_for(instance, TextAreaAction::Type(value.to_string()));
        if !outcome.handled {
            return false;
        }
        self.sync_text_area_runtime_for(instance, outcome);
        self.apply_text_area_value_for(
            instance,
            "text_area_type",
            "text_area_changed",
            "value=typing",
        );
        true
    }

    fn register_text_area_backspace_for(&mut self, instance: &'static str) -> bool {
        let outcome =
            self.apply_core_text_area_action_for(instance, TextAreaAction::DeleteBackward);
        if !outcome.handled {
            return false;
        }
        self.sync_text_area_runtime_for(instance, outcome);
        self.apply_text_area_value_for(
            instance,
            "text_area_delete_backward",
            "text_area_changed",
            "value=typing",
        );
        true
    }

    fn register_text_area_newline_for(&mut self, instance: &'static str) -> bool {
        let outcome = self.apply_core_text_area_action_for(instance, TextAreaAction::InsertNewline);
        if !outcome.handled {
            return false;
        }
        self.sync_text_area_runtime_for(instance, outcome);
        self.apply_text_area_value_for(
            instance,
            "text_area_newline",
            "text_area_changed",
            "newline=inserted",
        );
        true
    }

    fn register_text_area_submit_for(&mut self, instance: &'static str) -> bool {
        let outcome = self.apply_core_text_area_action_for(instance, TextAreaAction::Submit);
        debug_assert!(
            outcome.handled,
            "focused text-area submit must be handled by KUC"
        );
        self.sync_text_area_runtime_for(instance, outcome);
        self.apply_text_area_value_for(
            instance,
            "text_area_submit",
            "text_area_submitted",
            "value=typed",
        );
        true
    }

    fn apply_text_area_value_for(
        &mut self,
        instance: &'static str,
        action: &'static str,
        event: &'static str,
        state: &'static str,
    ) {
        self.action_count += 1;
        let scroll_offset = if self
            .text_area_runtime_for(instance)
            .vertical_scroll_enabled()
        {
            self.text_area_max_scroll_offset_for(instance)
        } else {
            0
        };
        {
            let runtime = self.text_area_runtime_mut_for(instance);
            runtime.uses_live_value = true;
            runtime.caret_visible = true;
            runtime.scroll_offset = scroll_offset;
        }
        self.last_action = action;
        self.last_event = event;
        self.last_setting = "text_area.value";
        self.last_setting_value = "keyboard";
        self.state_label = state;
    }

    fn set_text_area_caret_visibility_for(
        &mut self,
        instance: &'static str,
        visible: bool,
    ) -> bool {
        let runtime = self.text_area_runtime_mut_for(instance);
        if runtime.caret_visible == visible {
            return false;
        }
        runtime.caret_visible = visible;
        true
    }

    pub(in crate::visual) fn apply_text_area_contract_option_for(
        &mut self,
        instance: &'static str,
        option: StorybookUiOptionContract,
    ) {
        let runtime = self.text_area_runtime_mut_for(instance);
        match option.setting {
            "text_area.wrap_policy" => runtime.wrap_enabled = option.after != "None",
            "text_area.resize_enabled" => runtime.resize_enabled = option.after == "true",
            "text_area.disabled" => runtime.disabled = option.after == "true",
            "text_area.readonly" => runtime.readonly = option.after == "true",
            "text_area.value" => {
                runtime.value = option.after.to_string();
                runtime.uses_live_value = true;
            }
            "text_area.vertical_scroll_enabled" => {
                runtime.vertical_scroll_enabled = option.after == "true";
            }
            "text_area.horizontal_scroll_enabled" => {
                runtime.horizontal_scroll_enabled = option.after == "true";
                runtime.wrap_enabled = false;
            }
            "text_area.vertical_scrollbar_visible" => {
                runtime.vertical_scrollbar_visible =
                    option.after == "true" && runtime.vertical_scroll_enabled;
            }
            "text_area.horizontal_scrollbar_visible" => {
                runtime.horizontal_scrollbar_visible =
                    option.after == "true" && runtime.horizontal_scroll_enabled;
                runtime.wrap_enabled = false;
            }
            _ => {}
        }
    }

    pub(in crate::visual) fn register_text_area_mutation_block_for(
        &mut self,
        instance: &'static str,
    ) {
        self.action_count += 1;
        if self.text_area_disabled_for(instance) {
            self.last_action = "text_area_disabled_blocked";
            self.last_event = "text_area_disabled_ignored";
            self.last_setting = "text_area.disabled";
            self.last_setting_value = "true";
            self.state_label = "disabled=true";
            return;
        }
        self.last_action = "text_area_readonly_blocked";
        self.last_event = "text_area_readonly_ignored";
        self.last_setting = "text_area.readonly";
        self.last_setting_value = "true";
        self.state_label = "readonly=true";
    }
}

#[cfg(test)]
mod tests {
    use super::{StorybookScreenState, TextAreaInputKey};
    use crate::visual::storybook_ui_option_contract::StorybookUiOptionContract;
    use crate::visual::text_area_screen_state::DEFAULT_TEXT_AREA_INSTANCE;

    #[test]
    fn text_area_keys_cover_newline_submit_blocking_caret_and_resize_paths() {
        let instance = DEFAULT_TEXT_AREA_INSTANCE;
        let mut state = StorybookScreenState::default();
        assert!(!state.update_text_area_caret_visibility_for(instance, 30));
        assert!(!state.register_text_area_resize_drag_for(instance, 0, 0));

        state.register_text_area_focus_for(instance, false, false);
        assert!(state.register_text_area_key_for(instance, TextAreaInputKey::Newline));
        assert!(state.register_text_area_key_for(instance, TextAreaInputKey::Submit));
        assert!(state.register_text_area_resize_drag_for(instance, 12, 8));
        assert!(!state.register_text_area_resize_drag_for(instance, 12, 8));

        let mut readonly = StorybookScreenState::default();
        readonly.register_text_area_focus_for(instance, true, false);
        assert!(readonly.register_text_area_key_for(instance, TextAreaInputKey::Newline));
        assert_eq!("text_area_readonly_blocked", readonly.last_action);
        assert!(readonly.register_text_area_key_for(instance, TextAreaInputKey::Submit));
        assert_eq!("text_area_submit", readonly.last_action);
    }

    #[test]
    fn text_area_value_update_covers_vertical_scroll_enabled_runtime() {
        let instance = DEFAULT_TEXT_AREA_INSTANCE;
        let mut state = StorybookScreenState::default();
        state.apply_text_area_contract_option_for(
            instance,
            StorybookUiOptionContract::new("text_area.vertical_scroll_enabled", "false", "true"),
        );
        state.register_text_area_focus_for(instance, false, false);
        assert!(state.register_text_area_key_for(instance, TextAreaInputKey::Character('x')));
    }
}
