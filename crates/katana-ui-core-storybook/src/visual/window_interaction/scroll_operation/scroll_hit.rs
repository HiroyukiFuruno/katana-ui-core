use super::super::{StorybookWindowState, component_instance_id_for_page};
use crate::visual::dedicated_dod_form_input_live as input_live;
use crate::visual::panel_scroll_state::PanelScrollRegion;
use crate::visual::preset_tab_scroll;
use crate::visual::preview_detail;
use crate::visual::{dedicated_closeable_tab_strip, dedicated_tabs};

pub(super) fn preset_tab_at(state: &StorybookWindowState, x: usize, y: usize) -> bool {
    preset_tab_scroll::viewport_rect().contains(x, y + state.panel_scroll.root_y)
}

pub(super) fn tab_strip_at(state: &StorybookWindowState, x: usize, y: usize) -> bool {
    if state.selected_page != "tabs" && state.selected_page != "closeable-tab-strip" {
        return false;
    }
    let origin = preview_detail::component_action_hit_rect(state.selected_page);
    if state.selected_page == "tabs" {
        return dedicated_tabs::strip_hit_at(origin.x, origin.y, x, y);
    }
    dedicated_closeable_tab_strip::strip_hit_at(origin.x, origin.y, x, y)
}

pub(super) fn panel_child_at(
    state: &StorybookWindowState,
    region: PanelScrollRegion,
    x: usize,
    y: usize,
) -> Option<crate::visual::panel_screen_state::PanelChildKey> {
    if state.selected_page != "panel" || region != PanelScrollRegion::Preview {
        return None;
    }
    let origin = preview_detail::component_action_hit_rect("panel");
    crate::visual::dedicated_foundation_panel::panel_at(origin.x, origin.y, x, y)
}

pub(super) fn text_area_at(
    state: &StorybookWindowState,
    region: PanelScrollRegion,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "text-area" || region != PanelScrollRegion::Preview {
        return false;
    }
    let origin = preview_detail::component_action_hit_rect("text-area");
    let instance = component_instance_id_for_page(state.selected_page, state.selected_instance_id);
    input_live::text_area_rect_for_screen_state_instance(
        origin.x,
        origin.y,
        &state.screen_state,
        instance,
    )
    .contains(x, y)
}

pub(super) fn list_at(state: &StorybookWindowState, x: usize, y: usize) -> bool {
    if state.selected_page != "list" {
        return false;
    }
    preview_detail::component_action_hit_rect("list").contains(x, y)
}

pub(super) fn select_box_at(state: &StorybookWindowState, x: usize, y: usize) -> bool {
    if state.selected_page != "select-box" {
        return false;
    }
    preview_detail::component_action_hit_rect("select-box").contains(x, y)
}

pub(super) fn scroll_area_at(state: &StorybookWindowState, x: usize, y: usize) -> bool {
    if state.selected_page != "scroll-area" {
        return false;
    }
    preview_detail::component_action_hit_rect("scroll-area").contains(x, y)
}

pub(super) fn tree_view_at(state: &StorybookWindowState, x: usize, y: usize) -> bool {
    if state.selected_page != "tree-view" {
        return false;
    }
    preview_detail::component_action_hit_rect("tree-view").contains(x, y)
}
