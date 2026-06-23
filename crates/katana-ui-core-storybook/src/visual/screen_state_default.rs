use super::button_options::StorybookButtonOptions;
use super::panel_screen_state::PanelScreenState;
use super::screen_state::StorybookScreenState;
use super::screen_state_forms::{default_checkbox_state, default_radio_state};
use super::screen_state_search_control::SearchControlScreenState;
use super::screen_state_segmented_toggle::SegmentedToggleScreenState;
use super::screen_state_tabs::TabsScreenState;
use super::search_box_screen_state::SearchBoxScreenState;
use super::selection_screen_state::SelectionScreenState;
use super::text_area_screen_state::TextAreaStateStore;
use super::text_input_screen_state::TextInputStateStore;

const DEFAULT_PROGRESS_PERCENT: u8 = 65;

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
            breadcrumb_selected_index: 0,
            button_pressed: false,
            button_focused: false,
            preview_hovered: false,
            hovered_text_input_icon_button_index: None,
            hovered_text_input_clear_action: false,
            hovered_text_area_icon_button_index: None,
            hovered_text_area_clear_action: false,
            hovered_toolbar_action_index: None,
            hovered_summary_index: None,
            status_bar_open_segment_index: None,
            status_bar_focused_segment_index: None,
            status_bar_hovered_segment_index: None,
            modal_open: true,
            list: Default::default(),
            selection: SelectionScreenState::default(),
            search_box: SearchBoxScreenState::default(),
            search_control: SearchControlScreenState::default(),
            segmented_toggle: SegmentedToggleScreenState::default(),
            side_menu: Default::default(),
            tabs: TabsScreenState::default(),
            panel: PanelScreenState::default(),
            checkbox_state: default_checkbox_state(),
            checkbox_secondary_state: default_checkbox_state(),
            checkbox_focused_index: 0,
            checkbox_hovered_index: None,
            radio_state: default_radio_state(),
            toggle_checked: false,
            toggle_checked_overridden: false,
            progress_percent: DEFAULT_PROGRESS_PERCENT,
            progress_changed: false,
            progress_elapsed_ms: 0,
            tree_view_scroll_offset: 0,
            tree_view_selected_id: "katana/a.md",
            tree_view_focused_id: "",
            text_inputs: TextInputStateStore::default(),
            text_areas: TextAreaStateStore::default(),
            collapsible_panel: Default::default(),
            command_palette: Default::default(),
            color_picker: Default::default(),
            diagnostics_list: Default::default(),
            settings_list: Default::default(),
            shortcut_cheatsheet: Default::default(),
            drag_and_drop: Default::default(),
            dynamic_array_editor: Default::default(),
            scroll_area: Default::default(),
            split_pane: Default::default(),
            theme_tokens: Default::default(),
            layout: Default::default(),
            runtime_structured: Default::default(),
            virtualization: Default::default(),
        }
    }
}
