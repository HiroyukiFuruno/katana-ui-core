use super::{
    StorybookWindowState, focus_align_center, focus_attachment_chip, focus_chip_group,
    focus_collapsible_panel, focus_column, focus_command_palette, focus_diagnostics_list,
    focus_drag_and_drop, focus_dynamic_array_editor, focus_empty_state, focus_grid,
    focus_hover_card, focus_motion, focus_notification_toast, focus_panel, focus_popover,
    focus_row, focus_scroll_area, focus_search_box, focus_search_control, focus_segmented_toggle,
    focus_select_box, focus_selection_list, focus_settings_list, focus_shortcut_cheatsheet,
    focus_shortcut_combo, focus_side_menu, focus_skeleton_cluster, focus_split_pane, focus_stack,
    focus_startup_state, focus_status_bar, focus_theme_tokens, focus_tree_view,
    focus_virtualization, focus_window_control,
};

pub(in crate::visual::window_interaction::clickable_operation) fn focus_at(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page == "command-palette" {
        return focus_command_palette(state, x, y);
    }
    if state.selected_page == "collapsible-panel" {
        return focus_collapsible_panel(state, x, y);
    }
    if state.selected_page == "virtualization" {
        return focus_virtualization(state, x, y);
    }
    if state.selected_page == "diagnostics-list" {
        return focus_diagnostics_list(state, x, y);
    }
    if state.selected_page == "empty-state" {
        return focus_empty_state(state, x, y);
    }
    if state.selected_page == "tree-view" {
        return focus_tree_view(state, x, y);
    }
    if state.selected_page == "drag-and-drop" {
        return focus_drag_and_drop(state, x, y);
    }
    if state.selected_page == "panel" {
        return focus_panel(state, x, y);
    }
    if state.selected_page == "row" {
        return focus_row(state, x, y);
    }
    if state.selected_page == "column" {
        return focus_column(state, x, y);
    }
    if state.selected_page == "stack" {
        return focus_stack(state, x, y);
    }
    if state.selected_page == "grid" {
        return focus_grid(state, x, y);
    }
    if state.selected_page == "align-center" {
        return focus_align_center(state, x, y);
    }
    if state.selected_page == "scroll-area" {
        return focus_scroll_area(state, x, y);
    }
    if state.selected_page == "split-pane" {
        return focus_split_pane(state, x, y);
    }
    if state.selected_page == "theme-tokens" {
        return focus_theme_tokens(state, x, y);
    }
    if state.selected_page == "dynamic-array-editor" {
        return focus_dynamic_array_editor(state, x, y);
    }
    if state.selected_page == "notification-toast" {
        return focus_notification_toast(state, x, y);
    }
    if state.selected_page == "popover" {
        return focus_popover(state, x, y);
    }
    if state.selected_page == "hover-card" {
        return focus_hover_card(state, x, y);
    }
    if state.selected_page == "search-box" {
        return focus_search_box(state, x, y);
    }
    if state.selected_page == "search-control-strip" {
        return focus_search_control(state, x, y);
    }
    if state.selected_page == "segmented-toggle" {
        return focus_segmented_toggle(state, x, y);
    }
    if state.selected_page == "shortcut-combo" {
        return focus_shortcut_combo(state, x, y);
    }
    if state.selected_page == "shortcut-cheatsheet" {
        return focus_shortcut_cheatsheet(state, x, y);
    }
    if state.selected_page == "skeleton-cluster" {
        return focus_skeleton_cluster(state, x, y);
    }
    if state.selected_page == "motion" {
        return focus_motion(state, x, y);
    }
    if state.selected_page == "window-control-button-group" {
        return focus_window_control(state, x, y);
    }
    if state.selected_page == "startup-state-panel" {
        return focus_startup_state(state, x, y);
    }
    if state.selected_page == "attachment-chip" {
        return focus_attachment_chip(state, x, y);
    }
    if state.selected_page == "chip-group" {
        return focus_chip_group(state, x, y);
    }
    if state.selected_page == "status-bar" {
        return focus_status_bar(state, x, y);
    }
    if state.selected_page == "side-menu" {
        return focus_side_menu(state, x, y);
    }
    if state.selected_page == "select-box" {
        return focus_select_box(state, x, y);
    }
    if state.selected_page == "selection-list" {
        return focus_selection_list(state, x, y);
    }
    if state.selected_page == "settings-list" {
        return focus_settings_list(state, x, y);
    }
    false
}
