use super::button_options::StorybookButtonOptions;
use super::panel_screen_state::PanelScreenState;
use super::screen_state::StorybookScreenState;
use super::screen_state_forms::{default_checkbox_state, default_radio_state};
use super::search_box_screen_state::SearchBoxScreenState;
use super::selection_screen_state::SelectionScreenState;
use super::text_input_screen_state::default_text_input_state;

impl Default for StorybookScreenState {
    fn default() -> Self {
        Self {
            action_count: 0,
            settings_revision: 0,
            last_action: "none",
            last_event: "none",
            last_setting: "none",
            last_setting_value: "none",
            state_label: "idle",
            button_options: StorybookButtonOptions::default(),
            button_pressed: false,
            preview_hovered: false,
            hovered_summary_index: None,
            selection: SelectionScreenState::default(),
            search_box: SearchBoxScreenState::default(),
            panel: PanelScreenState::default(),
            checkbox_state: default_checkbox_state(),
            radio_state: default_radio_state(),
            text_input_state: default_text_input_state(),
            text_input_uses_live_value: false,
        }
    }
}
