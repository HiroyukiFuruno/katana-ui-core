use super::settings_list_operation::SettingsListStoryAction;
use super::settings_list_update::SettingsListUpdate;
use katana_ui_core::molecule::{SettingsKeyboardInput, SettingsListAction, SettingsValue};

#[path = "settings_list_state_types.rs"]
mod state_types;

#[cfg(test)]
pub(in crate::visual) use state_types::SettingsListOptionState;
pub(in crate::visual) use state_types::SettingsListScreenState;

const FONT_FIELD_ID: &str = "app.font-size";
const CHAT_SECTION_ID: &str = "chat";
const UPDATED_FONT_SIZE: i64 = 16;
const DEFAULT_FIELD_COUNT: u8 = 1;
const EXPANDED_FIELD_COUNT: u8 = 5;
const DEFAULT_CONTROL_OPTION_COUNT: u8 = 2;
const EXPANDED_CONTROL_OPTION_COUNT: u8 = 4;
const MAX_SCROLL_OFFSET: usize = 3;

impl SettingsListScreenState {
    pub(in crate::visual) fn apply_action(
        &mut self,
        action: SettingsListStoryAction,
    ) -> SettingsListUpdate {
        match action {
            SettingsListStoryAction::UpdateField => self.update_field(
                "settings_filter_update_collapse",
                "settings_field_changed",
                "dirty=font-size",
            ),
            SettingsListStoryAction::SetQuery => self.set_query(),
            SettingsListStoryAction::ToggleSection => self.toggle_section(),
            SettingsListStoryAction::ResetField => self.reset_field(),
            SettingsListStoryAction::FocusField => self.focus_field(),
            SettingsListStoryAction::HoverField => self.hover_field(),
            SettingsListStoryAction::KeyboardNext => self.keyboard_next(),
            SettingsListStoryAction::Scroll => self.scroll(),
        }
    }

    pub(in crate::visual) fn apply_option(&mut self, setting: &str) {
        match setting {
            "settings_list.label" => self.option_state.label_workspace = true,
            "settings_list.density" => self.option_state.density_compact = true,
            "settings_list.dirty_visualization" => self.option_state.dirty_highlight = true,
            "settings_list.query" => {
                let _ = self.apply_action(SettingsListStoryAction::SetQuery);
            }
            "settings_list.sections" => self.option_state.sections_app_lint = true,
            "settings_list.section_label" => self.option_state.section_label_editor = true,
            "settings_list.section_description" => {
                self.option_state.section_description_visible = true;
            }
            "settings_list.section_icon" => self.option_state.section_icon_gear = true,
            "settings_list.field_count" => self.option_state.field_count = EXPANDED_FIELD_COUNT,
            "settings_list.section_footer" => self.option_state.section_footer_policy = true,
            "settings_list.default_collapsed" | "settings_list.section_collapsible" => {
                self.apply_section_option(setting);
                let _ = self.apply_action(SettingsListStoryAction::ToggleSection);
            }
            "settings_list.field_label" => self.option_state.field_label_font_size = true,
            "settings_list.field_description" => {
                self.option_state.field_description_visible = true;
            }
            "settings_list.control_kind" => {
                self.option_state.control_kind_number = true;
                self.record_control_update();
            }
            "settings_list.control_options" => {
                self.option_state.control_option_count = EXPANDED_CONTROL_OPTION_COUNT;
                self.record_control_update();
            }
            "settings_list.custom_control" => {
                self.option_state.custom_control_button = true;
                self.record_control_update();
            }
            "settings_list.set_value" => {
                self.option_state.value_changed = true;
                self.record_control_update();
            }
            "settings_list.reset" => {
                self.option_state.reset_default = true;
                let _ = self.apply_action(SettingsListStoryAction::ResetField);
            }
            _ => {}
        }
    }

    pub(in crate::visual) const fn callback_action(&self) -> &'static str {
        self.callback_action
    }

    #[cfg(test)]
    pub(in crate::visual) const fn option_state(&self) -> SettingsListOptionState {
        self.option_state
    }

    #[cfg(test)]
    pub(in crate::visual) const fn has_query_filter(&self) -> bool {
        self.query_filter
    }

    #[cfg(test)]
    pub(in crate::visual) fn has_collapsed_chat_section(&self) -> bool {
        self.settings
            .collapsed_section_ids()
            .contains(CHAT_SECTION_ID)
    }

    #[cfg(test)]
    pub(in crate::visual) fn has_dirty_font_size(&self) -> bool {
        self.settings.dirty_field_ids().contains(FONT_FIELD_ID)
    }

    fn set_query(&mut self) -> SettingsListUpdate {
        self.settings
            .apply_settings_action(SettingsListAction::SetQuery(Some("format".to_string())));
        self.query_filter = true;
        self.callback_action = "settings_query_filter";
        SettingsListUpdate::new(
            "settings_query_filter",
            "settings_query_changed",
            "settings_list.query=format",
            "settings_list.query",
        )
    }

    fn update_field(
        &mut self,
        action: &'static str,
        event: &'static str,
        state: &'static str,
    ) -> SettingsListUpdate {
        self.settings
            .apply_settings_action(SettingsListAction::UpdateField {
                field_id: FONT_FIELD_ID.to_string(),
                value: SettingsValue::Number(UPDATED_FONT_SIZE),
            });
        self.callback_action = "settings_update_field";
        SettingsListUpdate::new(action, event, state, "settings_list.set_value")
    }

    fn toggle_section(&mut self) -> SettingsListUpdate {
        self.settings
            .apply_settings_action(SettingsListAction::ToggleSection {
                section_id: CHAT_SECTION_ID.to_string(),
            });
        self.callback_action = "settings_toggle_section";
        SettingsListUpdate::new(
            "settings_toggle_section",
            "settings_list_section_collapsed",
            "settings_list.section.collapsed=true",
            "settings_list.default_collapsed",
        )
    }

    fn reset_field(&mut self) -> SettingsListUpdate {
        self.settings
            .apply_settings_action(SettingsListAction::ResetField {
                field_id: FONT_FIELD_ID.to_string(),
            });
        self.callback_action = "settings_reset_field";
        SettingsListUpdate::new(
            "settings_reset_field",
            "settings_field_reset",
            "settings_list.reset=default",
            "settings_list.reset",
        )
    }

    fn focus_field(&mut self) -> SettingsListUpdate {
        self.settings
            .apply_settings_action(SettingsListAction::FocusField {
                field_id: Some(FONT_FIELD_ID.to_string()),
            });
        self.focused = true;
        self.callback_action = "settings_focus_field";
        SettingsListUpdate::new(
            "settings_focus_field",
            "settings_field_focused",
            "focus=app.font-size",
            "settings_list.focus",
        )
    }

    fn hover_field(&mut self) -> SettingsListUpdate {
        self.hovered = true;
        self.callback_action = "settings_hover_field";
        SettingsListUpdate::new(
            "settings_hover_field",
            "hover_start",
            "hover=app.font-size",
            "settings_list.hover",
        )
    }

    fn keyboard_next(&mut self) -> SettingsListUpdate {
        self.settings
            .apply_settings_action(SettingsListAction::KeyboardField {
                field_id: FONT_FIELD_ID.to_string(),
                input: SettingsKeyboardInput::Tab,
            });
        self.focused = true;
        self.callback_action = "settings_keyboard_next";
        SettingsListUpdate::new(
            "settings_keyboard_next",
            "settings_field_focused",
            "focus=next",
            "settings_list.keyboard",
        )
    }

    fn scroll(&mut self) -> SettingsListUpdate {
        self.scroll_offset = (self.scroll_offset + 1).min(MAX_SCROLL_OFFSET);
        self.callback_action = "settings_scroll";
        SettingsListUpdate::new(
            "settings_scroll",
            "scroll_by",
            self.scroll_label(),
            "settings_list.scroll",
        )
    }

    fn apply_section_option(&mut self, setting: &str) {
        if setting == "settings_list.section_collapsible" {
            self.option_state.section_collapsible = true;
            return;
        }
        self.option_state.default_collapsed = true;
    }

    fn record_control_update(&mut self) {
        let _ = self.update_field(
            "settings_update_field",
            "settings_field_changed",
            "settings_list.value=changed",
        );
    }

    fn scroll_label(&self) -> &'static str {
        match self.scroll_offset {
            1 => "scroll=1",
            2 => "scroll=2",
            _ => "scroll=3",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_option_is_a_noop_and_scroll_label_clamps_at_three() {
        let mut state = SettingsListScreenState::default();
        let initial = state.clone();

        state.apply_option("settings_list.unknown");
        assert_eq!(initial, state);

        assert_eq!(
            "scroll=1",
            state.apply_action(SettingsListStoryAction::Scroll).state
        );
        assert_eq!(
            "scroll=2",
            state.apply_action(SettingsListStoryAction::Scroll).state
        );
        assert_eq!(
            "scroll=3",
            state.apply_action(SettingsListStoryAction::Scroll).state
        );
        assert_eq!(
            "scroll=3",
            state.apply_action(SettingsListStoryAction::Scroll).state
        );
    }
}
