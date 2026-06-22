use super::StorybookWindowState;
use crate::visual::dedicated_closeable_tab_strip;
use crate::visual::dedicated_context_menu_popup::{self, ContextMenuPreviewCommand};
use crate::visual::dedicated_menu_button;
use crate::visual::dedicated_tabs;
use crate::visual::layout_metrics::LayoutRect;
use crate::visual::preview_detail;
use crate::visual::screen_state_tabs::TabsContextMenuCommand;
use crate::visual::window_interaction::collapsible_panel_state::CollapsiblePanelStoryAction;

const DISABLED_PRESET_INDEX: usize = 2;

pub(super) fn apply_context_click(state: &mut StorybookWindowState, x: usize, y: usize) -> bool {
    if state.selected_page == "tabs"
        && let Some((group_id, rect)) = tabs_group_context_target(state, x, y)
    {
        let component = preview_detail::component_action_hit_rect("tabs");
        state.screen_state.register_tabs_group_context_menu(
            group_id.as_str(),
            rect.x.saturating_sub(component.x),
            rect.bottom().saturating_sub(component.y),
        );
        return true;
    }
    if state.selected_page == "tabs"
        && let Some((tab_id, rect)) = tabs_context_target(state, x, y)
    {
        state.screen_state.register_tabs_context_menu(
            tab_id.as_str(),
            rect.x
                .saturating_sub(preview_detail::component_action_hit_rect("tabs").x),
            rect.bottom()
                .saturating_sub(preview_detail::component_action_hit_rect("tabs").y),
        );
        return true;
    }
    if state.selected_page == "closeable-tab-strip"
        && let Some((group_id, rect)) = closeable_tab_strip_group_context_target(state, x, y)
    {
        let component = preview_detail::component_action_hit_rect("closeable-tab-strip");
        state.screen_state.register_tabs_group_context_menu(
            group_id.as_str(),
            rect.x.saturating_sub(component.x),
            rect.bottom().saturating_sub(component.y),
        );
        return true;
    }
    if state.selected_page == "closeable-tab-strip"
        && let Some((tab_id, rect)) = closeable_tab_strip_context_target(state, x, y)
    {
        let component = preview_detail::component_action_hit_rect("closeable-tab-strip");
        state.screen_state.register_tabs_context_menu(
            tab_id.as_str(),
            rect.x.saturating_sub(component.x),
            rect.bottom().saturating_sub(component.y),
        );
        return true;
    }
    if matches!(state.selected_page, "tree-view" | "context-menu")
        && preview_detail::component_action_hit_rect(state.selected_page).contains(x, y)
    {
        state
            .screen_state
            .register_context_menu(state.selected_page);
        return true;
    }
    if state.selected_page == "context-menu" && context_menu_is_open(state) {
        let component = preview_detail::component_action_hit_rect("context-menu");
        if !component.contains(x, y) {
            state.screen_state.register_context_menu_outside_dismiss();
            return true;
        }
    }
    if state.selected_page == "menu" && state.screen_state.selection.select_open {
        let component = preview_detail::component_action_hit_rect("menu");
        if !component.contains(x, y) {
            state.screen_state.register_menu_context_dismiss();
            return true;
        }
    }
    if state.selected_page == "menu-button" {
        let component = preview_detail::component_action_hit_rect("menu-button");
        if dedicated_menu_button::trigger_rect(component).contains(x, y) {
            state
                .screen_state
                .register_menu_button_context_open(state.preset_index == DISABLED_PRESET_INDEX);
            return true;
        }
    }
    if state.selected_page == "modal-overlay"
        && preview_detail::component_action_hit_rect("modal-overlay").contains(x, y)
    {
        state.screen_state.register_modal_overlay_context_block();
        return true;
    }
    if state.selected_page == "collapsible-panel"
        && preview_detail::component_action_hit_rect("collapsible-panel").contains(x, y)
    {
        state
            .screen_state
            .register_collapsible_panel_action(CollapsiblePanelStoryAction::ContextPinToggle);
        return true;
    }
    false
}

fn context_menu_is_open(state: &StorybookWindowState) -> bool {
    matches!(
        state.screen_state.state_label,
        "context_menu=open" | "context_menu.submenu=[2]"
    )
}

pub(super) fn tabs_context_command_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<TabsContextMenuCommand> {
    if state.selected_page == "closeable-tab-strip" {
        let component = preview_detail::component_action_hit_rect("closeable-tab-strip");
        return dedicated_closeable_tab_strip::context_menu_command_at(
            component.x,
            component.y,
            x,
            y,
            &state.screen_state.tabs,
        );
    }
    if state.selected_page != "tabs" {
        return None;
    }
    let component = preview_detail::component_action_hit_rect("tabs");
    dedicated_tabs::context_menu_command_at(
        component.x,
        component.y,
        x,
        y,
        &state.screen_state.tabs,
    )
}

pub(super) fn context_menu_command_at(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<ContextMenuPreviewCommand> {
    if state.selected_page != "context-menu" {
        return None;
    }
    let component = preview_detail::component_action_hit_rect("context-menu");
    let submenu_open = state.screen_state.state_label == "context_menu.submenu=[2]";
    dedicated_context_menu_popup::command_at(component.x, component.y, x, y, submenu_open)
}

fn closeable_tab_strip_group_context_target(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<(String, LayoutRect)> {
    let component = preview_detail::component_action_hit_rect("closeable-tab-strip");
    if !component.contains(x, y) {
        return None;
    }
    dedicated_closeable_tab_strip::group_hit_at(
        component.x,
        component.y,
        x,
        y,
        &state.screen_state.tabs,
    )
}

fn closeable_tab_strip_context_target(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<(String, LayoutRect)> {
    let component = preview_detail::component_action_hit_rect("closeable-tab-strip");
    if !component.contains(x, y) {
        return None;
    }
    dedicated_closeable_tab_strip::tab_hit_at(
        component.x,
        component.y,
        x,
        y,
        &state.screen_state.tabs,
    )
}

fn tabs_group_context_target(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<(String, LayoutRect)> {
    let component = preview_detail::component_action_hit_rect("tabs");
    if !component.contains(x, y) {
        return None;
    }
    dedicated_tabs::group_hit_at(component.x, component.y, x, y, &state.screen_state.tabs)
}

fn tabs_context_target(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> Option<(String, LayoutRect)> {
    let component = preview_detail::component_action_hit_rect("tabs");
    if !component.contains(x, y) {
        return None;
    }
    dedicated_tabs::tab_hit_at(component.x, component.y, x, y, &state.screen_state.tabs)
}
