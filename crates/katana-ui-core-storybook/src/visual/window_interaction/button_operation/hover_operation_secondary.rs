use super::{
    ColorPickerAction, CommandPaletteStoryAction, DynamicArrayEditorAction,
    SearchControlScreenAction, SegmentedToggleScreenAction, SelectionScreenAction,
    SettingsListStoryAction, SideMenuScreenAction, StorybookWindowState, preview_detail,
    status_bar_operation,
};

pub(super) fn apply(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if state.selected_page == "color-picker-rgba"
        && preview_detail::component_action_hit_rect("color-picker-rgba").contains(x, y)
    {
        state
            .screen_state
            .register_color_picker_action(ColorPickerAction::Hover);
        return true;
    }
    if state.selected_page == "combo-box"
        && preview_detail::component_action_hit_rect("combo-box").contains(x, y)
    {
        state
            .screen_state
            .register_selection_action(SelectionScreenAction::ComboHover);
        return true;
    }
    if state.selected_page == "command-palette"
        && preview_detail::component_action_hit_rect("command-palette").contains(x, y)
    {
        state
            .screen_state
            .register_command_palette_action(CommandPaletteStoryAction::Hover);
        return true;
    }
    if state.selected_page == "dynamic-array-editor"
        && preview_detail::component_action_hit_rect("dynamic-array-editor").contains(x, y)
    {
        state
            .screen_state
            .register_dynamic_array_editor_action(DynamicArrayEditorAction::Hover);
        return true;
    }
    if state.selected_page == "toast-stack-manager"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state.screen_state.register_toast_stack_hover_pause();
        return true;
    }
    if state.selected_page == "notification-toast"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state.screen_state.register_notification_toast_hover();
        return true;
    }
    if state.selected_page == "tooltip" {
        let anchor = crate::visual::dedicated_tooltip::anchor_hit_rect(state.preset_index);
        if anchor.contains(x, y) {
            state.screen_state.register_tooltip_hover_open();
            return true;
        }
        return state.screen_state.register_tooltip_hover_close();
    }
    if state.selected_page == "popover"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state.screen_state.register_popover_hover();
        return true;
    }
    if state.selected_page == "hover-card"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state.screen_state.register_hover_card_hover();
        return true;
    }
    if state.selected_page == "search-control-strip"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state
            .screen_state
            .register_search_control_action(SearchControlScreenAction::Hover);
        return true;
    }
    if state.selected_page == "segmented-toggle"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state
            .screen_state
            .register_segmented_toggle_action(SegmentedToggleScreenAction::Hover);
        return true;
    }
    if state.selected_page == "shortcut-combo"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state.screen_state.register_shortcut_combo_hover();
        return true;
    }
    if state.selected_page == "shortcut-cheatsheet"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state.screen_state.register_shortcut_cheatsheet_hover();
        return true;
    }
    if state.selected_page == "skeleton-cluster"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state.screen_state.register_skeleton_cluster_hover();
        return true;
    }
    if state.selected_page == "motion"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state.screen_state.register_motion_hover();
        return true;
    }
    if state.selected_page == "window-control-button-group"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state.screen_state.register_window_control_hover();
        return true;
    }
    if state.selected_page == "startup-state-panel"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state.screen_state.register_startup_state_hover();
        return true;
    }
    if state.selected_page == "attachment-chip"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state.screen_state.register_attachment_chip_hover();
        return true;
    }
    if state.selected_page == "chip-group"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state.screen_state.register_chip_group_hover();
        return true;
    }
    if state.selected_page == "status-bar"
        && let Some(index) = status_bar_operation::segment_index_at(state.selected_page, x, y)
    {
        state.screen_state.register_status_bar_segment_hover(index);
        return true;
    }
    if state.selected_page == "side-menu"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state
            .screen_state
            .register_side_menu_action(SideMenuScreenAction::Hover);
        return true;
    }
    if state.selected_page == "select-box"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state
            .screen_state
            .register_selection_action(SelectionScreenAction::SelectHover);
        return true;
    }
    if state.selected_page == "selection-list"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state
            .screen_state
            .register_selection_action(SelectionScreenAction::SelectionListHover);
        return true;
    }
    if state.selected_page == "settings-list"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state
            .screen_state
            .register_settings_list_action(SettingsListStoryAction::HoverField);
        return true;
    }
    if state.selected_page == "modal-overlay"
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state.screen_state.register_modal_overlay_hover();
        return true;
    }
    false
}
