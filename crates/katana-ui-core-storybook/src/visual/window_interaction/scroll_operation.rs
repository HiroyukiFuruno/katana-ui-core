use crate::visual::dedicated_dod_form_input_live as input_live;
use crate::visual::layout_metrics::MAX_SCROLL_Y;
use crate::visual::panel_scroll_state::{self, PanelScrollRegion};
use crate::visual::panel_scrollbars;
use crate::visual::preset_tab_scroll;
use crate::visual::preview_detail;

use super::StorybookWindowState;
use super::panel_scroll_drag::PanelScrollDragTarget;

pub(super) fn apply_scroll_delta(state: &mut StorybookWindowState, delta_y: f32) -> bool {
    apply_scroll_delta_at_root(state, delta_y)
}

pub(super) fn apply_scroll_delta_at(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
    delta_y: f32,
) -> bool {
    if delta_y == 0.0 {
        return false;
    }
    if preset_tab_at(state, x, y) {
        return state.scroll_preset_tabs(delta_y);
    }
    let region =
        panel_scroll_state::PanelScrollRegionModel::region_at(x, y + state.panel_scroll.root_y);
    let mut changed = clamp_vertical_offset(state, region);
    if text_area_at(state, region, x, y) {
        let vertical_enabled = input_live::text_area_vertical_scroll_enabled_for(
            state.preset_index,
            &state.screen_state,
        );
        let max_y = input_live::text_area_vertical_scroll_max_offset_for(
            state.preset_index,
            &state.screen_state,
        );
        let vertical_changed =
            state
                .screen_state
                .scroll_text_area_vertical(delta_y, vertical_enabled, max_y);
        if vertical_changed || changed {
            return true;
        }
        let horizontal_enabled = input_live::text_area_horizontal_scroll_enabled_for(
            state.preset_index,
            &state.screen_state,
        );
        let max_x = input_live::text_area_horizontal_scroll_max_offset_for(
            state.preset_index,
            &state.screen_state,
        );
        return state
            .screen_state
            .scroll_text_area_horizontal(delta_y, horizontal_enabled, max_x);
    }
    if let Some(panel) = panel_child_at(state, region, x, y) {
        return state.screen_state.scroll_panel_vertical(panel, delta_y) || changed;
    }
    if !panel_scrollbars::vertical_region_scrollable_for(
        region,
        state.selected_page,
        state.tree_expansion,
    ) {
        return changed;
    }
    changed |=
        state
            .panel_scroll
            .scroll_delta_with_max(region, max_scroll_y(state, region), delta_y);
    if region == PanelScrollRegion::Root {
        state.scroll_y = state.panel_scroll.root_y;
    }
    changed
}

pub(super) fn apply_scroll_delta_x_at(
    state: &mut StorybookWindowState,
    x: usize,
    y: usize,
    delta_x: f32,
) -> bool {
    if delta_x == 0.0 {
        return false;
    }
    if preset_tab_at(state, x, y) {
        return state.scroll_preset_tabs(delta_x);
    }
    let region =
        panel_scroll_state::PanelScrollRegionModel::region_at(x, y + state.panel_scroll.root_y);
    let mut changed = clamp_horizontal_offset(state, region);
    if text_area_at(state, region, x, y) {
        let enabled = input_live::text_area_horizontal_scroll_enabled_for(
            state.preset_index,
            &state.screen_state,
        );
        let max_x = input_live::text_area_horizontal_scroll_max_offset_for(
            state.preset_index,
            &state.screen_state,
        );
        return state
            .screen_state
            .scroll_text_area_horizontal(delta_x, enabled, max_x)
            || changed;
    }
    if let Some(panel) = panel_child_at(state, region, x, y) {
        return state.screen_state.scroll_panel_horizontal(panel, delta_x) || changed;
    }
    if !panel_scrollbars::horizontal_region_scrollable_for(
        region,
        state.selected_page,
        state.tree_expansion,
    ) {
        return changed;
    }
    changed |=
        state
            .panel_scroll
            .scroll_delta_x_with_max(region, max_scroll_x(state, region), delta_x);
    changed
}

fn apply_scroll_delta_at_root(state: &mut StorybookWindowState, delta_y: f32) -> bool {
    let changed =
        state
            .panel_scroll
            .scroll_delta_with_max(PanelScrollRegion::Root, MAX_SCROLL_Y, delta_y);
    state.scroll_y = state.panel_scroll.root_y.min(MAX_SCROLL_Y);
    changed
}

pub(super) fn apply_scrollbar_drag(
    state: &mut StorybookWindowState,
    region: PanelScrollRegion,
    y: usize,
) -> bool {
    let mut changed = clamp_vertical_offset(state, region);
    let next = panel_scrollbars::offset_from_drag_for(
        region,
        y,
        state.selected_page,
        state.tree_expansion,
    );
    changed |=
        state
            .panel_scroll
            .set_drag_offset_with_max(region, next, max_scroll_y(state, region));
    if region == PanelScrollRegion::Root {
        state.scroll_y = state.panel_scroll.root_y.min(MAX_SCROLL_Y);
    }
    changed
}

fn clamp_vertical_offset(state: &mut StorybookWindowState, region: PanelScrollRegion) -> bool {
    let max_offset = max_scroll_y(state, region);
    state.panel_scroll.set_drag_offset_with_max(
        region,
        state.panel_scroll.offset(region),
        max_offset,
    )
}

fn clamp_horizontal_offset(state: &mut StorybookWindowState, region: PanelScrollRegion) -> bool {
    let max_offset = max_scroll_x(state, region);
    state.panel_scroll.set_drag_offset_x_with_max(
        region,
        state.panel_scroll.offset_x(region),
        max_offset,
    )
}

fn max_scroll_y(state: &StorybookWindowState, region: PanelScrollRegion) -> usize {
    panel_scroll_state::PanelScrollOverflowModel::max_scroll_y_for(
        region,
        state.selected_page,
        state.tree_expansion,
    )
}

fn max_scroll_x(state: &StorybookWindowState, region: PanelScrollRegion) -> usize {
    panel_scroll_state::PanelScrollOverflowModel::max_scroll_x_for(
        region,
        state.selected_page,
        state.tree_expansion,
    )
}

fn preset_tab_at(state: &StorybookWindowState, x: usize, y: usize) -> bool {
    preset_tab_scroll::viewport_rect().contains(x, y + state.panel_scroll.root_y)
}

fn panel_child_at(
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

fn text_area_at(
    state: &StorybookWindowState,
    region: PanelScrollRegion,
    x: usize,
    y: usize,
) -> bool {
    if state.selected_page != "text-area" || region != PanelScrollRegion::Preview {
        return false;
    }
    let origin = preview_detail::component_action_hit_rect("text-area");
    input_live::text_area_rect_for_screen_state(origin.x, origin.y, &state.screen_state)
        .contains(x, y)
}

pub(super) fn apply_scrollbar_drag_target(
    state: &mut StorybookWindowState,
    target: PanelScrollDragTarget,
    x: usize,
    y: usize,
) -> bool {
    match target {
        PanelScrollDragTarget::Vertical(region) => apply_scrollbar_drag(state, region, y),
        PanelScrollDragTarget::Horizontal(region) => {
            apply_horizontal_scrollbar_drag(state, region, x)
        }
    }
}

pub(super) fn apply_horizontal_scrollbar_drag(
    state: &mut StorybookWindowState,
    region: PanelScrollRegion,
    x: usize,
) -> bool {
    let mut changed = clamp_horizontal_offset(state, region);
    let next = panel_scrollbars::horizontal_offset_from_drag_for(
        region,
        x,
        state.selected_page,
        state.tree_expansion,
    );
    changed |=
        state
            .panel_scroll
            .set_drag_offset_x_with_max(region, next, max_scroll_x(state, region));
    changed
}
