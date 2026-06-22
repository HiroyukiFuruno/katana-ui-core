use super::visual_interaction_test_support::require_some;
use super::window_interaction::{StorybookWindowState, apply_click, apply_context_click_for_test};
use super::{dedicated_tabs, preview_detail};

pub(super) const PAGE: &str = "tabs";
pub(super) const CLOSE_INDEX: usize = 0;
pub(super) const CLOSE_OTHERS_INDEX: usize = 1;
pub(super) const CLOSE_ALL_INDEX: usize = 2;
pub(super) const CLOSE_RIGHT_INDEX: usize = 3;
pub(super) const CLOSE_LEFT_INDEX: usize = 4;
pub(super) const PIN_INDEX: usize = 5;
pub(super) const MOVE_TO_GROUP_INDEX: usize = 6;
pub(super) const NEW_GROUP_INDEX: usize = 7;
pub(super) const DOCS_GROUP_INDEX: usize = 8;
pub(super) const RESTORE_CLOSED_INDEX: usize = 9;

pub(super) fn click_context_command(
    state: &mut StorybookWindowState,
    tab_id: &str,
    command_index: usize,
) -> Result<(), String> {
    open_context_menu(state, tab_id)?;
    click_open_context_command(state, command_index)
}

pub(super) fn click_open_context_command(
    state: &mut StorybookWindowState,
    command_index: usize,
) -> Result<(), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let labels = dedicated_tabs::context_menu_labels_for_test(&state.screen_state.tabs);
    let menu = require_some(
        dedicated_tabs::context_menu_rect_for_test(&state.screen_state.tabs),
        "context menu rect",
    )?;
    let row_height = menu.height / labels.len();
    assert!(apply_click(
        state,
        component.x + menu.x + 1,
        component.y + menu.y + command_index * row_height + 1
    ));
    Ok(())
}

pub(super) fn open_context_menu(
    state: &mut StorybookWindowState,
    tab_id: &str,
) -> Result<(), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let tab = require_some(
        dedicated_tabs::tab_rect_for_test(&state.screen_state.tabs, tab_id),
        "tab rect",
    )?;
    assert!(apply_context_click_for_test(
        state,
        component.x + tab.x + 1,
        component.y + tab.y + 1
    ));
    Ok(())
}

pub(super) fn open_group_context_menu(
    state: &mut StorybookWindowState,
    group_id: &str,
) -> Result<(), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let group = require_some(
        dedicated_tabs::group_rect_for_test(&state.screen_state.tabs, group_id),
        "group rect",
    )?;
    assert!(apply_context_click_for_test(
        state,
        component.x + group.x + 1,
        component.y + group.y + 1
    ));
    Ok(())
}

pub(super) fn tabs_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

pub(super) fn has_tab(state: &StorybookWindowState, tab_id: &str) -> bool {
    state
        .screen_state
        .tabs
        .tabs
        .iter()
        .any(|tab| tab.id == tab_id)
}

pub(super) fn tab_order(state: &StorybookWindowState) -> Vec<&str> {
    dedicated_tabs::tab_ids_for_test(&state.screen_state.tabs)
}

pub(super) fn active_tab(
    state: &StorybookWindowState,
) -> Result<&super::screen_state_tabs::TabsScreenTab, String> {
    require_some(state.screen_state.tabs.active_tab(), "active tab exists")
}

pub(super) fn group_by_id<'a>(
    state: &'a StorybookWindowState,
    group_id: &str,
) -> Result<&'a super::screen_state_tabs::TabsScreenGroup, String> {
    require_some(
        state
            .screen_state
            .tabs
            .groups
            .iter()
            .find(|group| group.id == group_id),
        "group exists",
    )
}
