use super::{
    DynamicArrayEditorAction, SearchControlScreenAction, SegmentedToggleScreenAction,
    SelectionScreenAction, SettingsListStoryAction, SideMenuScreenAction, SplitPaneStoryAction,
    StorybookWindowState, ThemeTokensStoryAction, dedicated_status_bar, preview_detail,
};

pub(super) fn focus_split_pane(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("split-pane").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_split_pane_action(SplitPaneStoryAction::Focus);
    true
}

pub(super) fn focus_theme_tokens(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("theme-tokens").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_theme_tokens_action(ThemeTokensStoryAction::Focus);
    true
}

pub(super) fn focus_dynamic_array_editor(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if !preview_detail::component_action_hit_rect("dynamic-array-editor").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_dynamic_array_editor_action(DynamicArrayEditorAction::Focus);
    true
}

pub(super) fn focus_notification_toast(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if !preview_detail::component_action_hit_rect("notification-toast").contains(x, y) {
        return false;
    }
    state.screen_state.register_notification_toast_focus();
    true
}

pub(super) fn focus_popover(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("popover").contains(x, y) {
        return false;
    }
    state.screen_state.register_popover_focus();
    true
}

pub(super) fn focus_hover_card(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("hover-card").contains(x, y) {
        return false;
    }
    state.screen_state.register_hover_card_focus();
    true
}

pub(super) fn focus_search_box(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("search-box").contains(x, y) {
        return false;
    }
    state.screen_state.register_search_box_action(
        crate::visual::search_box_screen_state::SearchBoxScreenAction::Focus,
    );
    true
}

pub(super) fn focus_search_control(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("search-control-strip").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_search_control_action(SearchControlScreenAction::Focus);
    true
}

pub(super) fn focus_segmented_toggle(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("segmented-toggle").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_segmented_toggle_action(SegmentedToggleScreenAction::Focus);
    true
}

pub(super) fn focus_shortcut_combo(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("shortcut-combo").contains(x, y) {
        return false;
    }
    state.screen_state.register_shortcut_combo_focus();
    true
}

pub(super) fn focus_shortcut_cheatsheet(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
) -> bool {
    if !preview_detail::component_action_hit_rect("shortcut-cheatsheet").contains(x, y) {
        return false;
    }
    state.screen_state.register_shortcut_cheatsheet_focus();
    true
}

pub(super) fn focus_skeleton_cluster(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("skeleton-cluster").contains(x, y) {
        return false;
    }
    state.screen_state.register_skeleton_cluster_focus();
    true
}

pub(super) fn focus_motion(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("motion").contains(x, y) {
        return false;
    }
    state.screen_state.register_motion_focus();
    true
}

pub(super) fn focus_window_control(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("window-control-button-group").contains(x, y) {
        return false;
    }
    state.screen_state.register_window_control_focus();
    true
}

pub(super) fn focus_startup_state(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("startup-state-panel").contains(x, y) {
        return false;
    }
    state.screen_state.register_startup_state_focus();
    true
}

pub(super) fn focus_attachment_chip(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("attachment-chip").contains(x, y) {
        return false;
    }
    state.screen_state.register_attachment_chip_focus();
    true
}

pub(super) fn focus_chip_group(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("chip-group").contains(x, y) {
        return false;
    }
    state.screen_state.register_chip_group_focus();
    true
}

pub(super) fn focus_status_bar(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    let origin = preview_detail::component_action_hit_rect("status-bar");
    let Some(index) = dedicated_status_bar::segment_index_at(origin.x, origin.y, x, y) else {
        return false;
    };
    state.screen_state.register_status_bar_segment_focus(index);
    true
}

pub(super) fn focus_side_menu(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("side-menu").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_side_menu_action(SideMenuScreenAction::Focus);
    true
}

pub(super) fn focus_select_box(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("select-box").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_selection_action(SelectionScreenAction::SelectFocus);
    true
}

pub(super) fn focus_selection_list(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("selection-list").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_selection_action(SelectionScreenAction::SelectionListFocus);
    true
}

pub(super) fn focus_settings_list(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if !preview_detail::component_action_hit_rect("settings-list").contains(x, y) {
        return false;
    }
    state
        .screen_state
        .register_settings_list_action(SettingsListStoryAction::FocusField);
    true
}
