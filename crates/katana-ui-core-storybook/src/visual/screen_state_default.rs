use super::button_options::StorybookButtonOptions;
use super::panel_screen_state::PanelScreenState;
use super::screen_state::StorybookScreenState;
use super::screen_state_forms::{default_checkbox_state, default_radio_state};
use super::screen_state_tabs::TabsScreenState;
use super::search_box_screen_state::SearchBoxScreenState;
use super::selection_screen_state::SelectionScreenState;
use super::text_input_screen_state::TextInputStateStore;

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
            hovered_text_input_icon_button_index: None,
            hovered_summary_index: None,
            selection: SelectionScreenState::default(),
            search_box: SearchBoxScreenState::default(),
            tabs: TabsScreenState::default(),
            panel: PanelScreenState::default(),
            checkbox_state: default_checkbox_state(),
            radio_state: default_radio_state(),
            text_inputs: TextInputStateStore::default(),
            text_area_value: "English\n日本語 🔷".to_string(),
            text_area_focused: false,
            text_area_uses_live_value: false,
            text_area_caret_visible: false,
            text_area_wrap_enabled: true,
            text_area_resize_enabled: false,
            text_area_vertical_scroll_enabled: false,
            text_area_horizontal_scroll_enabled: false,
            text_area_vertical_scrollbar_visible: false,
            text_area_horizontal_scrollbar_visible: false,
            text_area_scroll_offset: 0,
            text_area_scroll_x_offset: 0,
            text_area_resize_width_delta: 0,
            text_area_resize_height_delta: 0,
        }
    }
}
