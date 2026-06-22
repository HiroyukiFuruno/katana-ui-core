use super::{
    CollapsiblePanelStoryAction, CommandPaletteStoryAction, DiagnosticsListStoryAction,
    DragAndDropAction, DynamicArrayEditorAction, LayoutStoryAction, ScrollAreaStoryAction,
    SearchControlScreenAction, SegmentedToggleScreenAction, SelectionScreenAction,
    SettingsListStoryAction, SideMenuScreenAction, SplitPaneStoryAction, StorybookWindowState,
    ThemeTokensStoryAction, VirtualizationStoryAction,
};

pub(in crate::visual::window_interaction::clickable_operation) fn keyboard_activate(
    state: &mut StorybookWindowState,
) -> bool {
    if state.selected_page == "command-palette" {
        state
            .screen_state
            .register_command_palette_action(CommandPaletteStoryAction::KeyboardExecute);
        return true;
    }
    if state.selected_page == "collapsible-panel" {
        state
            .screen_state
            .register_collapsible_panel_action(CollapsiblePanelStoryAction::KeyboardToggle);
        return true;
    }
    if state.selected_page == "virtualization" {
        state
            .screen_state
            .register_virtualization_action(VirtualizationStoryAction::KeyboardFocus);
        return true;
    }
    if state.selected_page == "diagnostics-list" {
        state
            .screen_state
            .register_diagnostics_list_action(DiagnosticsListStoryAction::KeyboardNavigate);
        return true;
    }
    if state.selected_page == "empty-state" {
        state.screen_state.register_empty_state_keyboard_action();
        return true;
    }
    if state.selected_page == "tree-view" {
        state.screen_state.register_tree_view_keyboard_select();
        return true;
    }
    if state.selected_page == "drag-and-drop" {
        state
            .screen_state
            .register_drag_and_drop_action(DragAndDropAction::KeyboardDrop);
        return true;
    }
    if state.selected_page == "panel" {
        state.screen_state.register_panel_keyboard_scroll();
        return true;
    }
    if state.selected_page == "row" {
        state
            .screen_state
            .register_layout_action(LayoutStoryAction::RowKeyboard);
        return true;
    }
    if state.selected_page == "column" {
        state
            .screen_state
            .register_layout_action(LayoutStoryAction::ColumnKeyboard);
        return true;
    }
    if state.selected_page == "stack" {
        state
            .screen_state
            .register_layout_action(LayoutStoryAction::StackKeyboard);
        return true;
    }
    if state.selected_page == "grid" {
        state
            .screen_state
            .register_layout_action(LayoutStoryAction::GridKeyboard);
        return true;
    }
    if state.selected_page == "align-center" {
        state
            .screen_state
            .register_layout_action(LayoutStoryAction::AlignCenterKeyboard);
        return true;
    }
    if state.selected_page == "scroll-area" {
        state
            .screen_state
            .register_scroll_area_action(ScrollAreaStoryAction::Keyboard);
        return true;
    }
    if state.selected_page == "split-pane" {
        state
            .screen_state
            .register_split_pane_action(SplitPaneStoryAction::Keyboard);
        return true;
    }
    if state.selected_page == "theme-tokens" {
        state
            .screen_state
            .register_theme_tokens_action(ThemeTokensStoryAction::Keyboard);
        return true;
    }
    if state.selected_page == "dynamic-array-editor" {
        state
            .screen_state
            .register_dynamic_array_editor_action(DynamicArrayEditorAction::KeyboardEdit);
        return true;
    }
    if state.selected_page == "notification-toast" {
        state
            .screen_state
            .register_notification_toast_keyboard_dismiss();
        return true;
    }
    if state.selected_page == "popover" {
        state.screen_state.register_popover_keyboard_escape();
        return true;
    }
    if state.selected_page == "search-box" {
        state.screen_state.register_search_box_action(
            crate::visual::search_box_screen_state::SearchBoxScreenAction::KeyboardSubmit,
        );
        return true;
    }
    if state.selected_page == "search-control-strip" {
        state
            .screen_state
            .register_search_control_action(SearchControlScreenAction::KeyboardNext);
        return true;
    }
    if state.selected_page == "segmented-toggle" {
        state
            .screen_state
            .register_segmented_toggle_action(SegmentedToggleScreenAction::KeyboardSelect);
        return true;
    }
    if state.selected_page == "shortcut-combo" {
        state
            .screen_state
            .register_shortcut_combo_keyboard_preview();
        return true;
    }
    if state.selected_page == "shortcut-cheatsheet" {
        state
            .screen_state
            .register_shortcut_cheatsheet_keyboard_select();
        return true;
    }
    if state.selected_page == "skeleton-cluster" {
        state
            .screen_state
            .register_skeleton_cluster_keyboard_reduce_motion();
        return true;
    }
    if state.selected_page == "motion" {
        state.screen_state.register_motion_keyboard_tick();
        return true;
    }
    if state.selected_page == "window-control-button-group" {
        state
            .screen_state
            .register_window_control_keyboard_restore();
        return true;
    }
    if state.selected_page == "startup-state-panel" {
        state.screen_state.register_startup_state_keyboard_retry();
        return true;
    }
    if state.selected_page == "attachment-chip" {
        state.screen_state.register_attachment_chip_keyboard_retry();
        return true;
    }
    if state.selected_page == "chip-group" {
        state.screen_state.register_chip_group_keyboard_dismiss();
        return true;
    }
    if state.selected_page == "status-bar" {
        state.screen_state.register_status_bar_keyboard_activate();
        return true;
    }
    if state.selected_page == "side-menu" {
        state
            .screen_state
            .register_side_menu_action(SideMenuScreenAction::KeyboardNext);
        return true;
    }
    if state.selected_page == "select-box" {
        state
            .screen_state
            .register_selection_action(SelectionScreenAction::SelectKeyboardSelect);
        return true;
    }
    if state.selected_page == "selection-list" {
        state
            .screen_state
            .register_selection_action(SelectionScreenAction::SelectionListKeyboardNext);
        return true;
    }
    if state.selected_page == "settings-list" {
        state
            .screen_state
            .register_settings_list_action(SettingsListStoryAction::KeyboardNext);
        return true;
    }
    false
}
