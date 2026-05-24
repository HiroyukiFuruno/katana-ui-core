use crate::visual::layout_metrics::MAX_SCROLL_Y;
use crate::visual::panel_scroll_state::{self, PanelScrollRegion, region_at};
use crate::visual::panel_scrollbars;

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
    let region = region_at(x, y + state.panel_scroll.root_y);
    let mut changed = clamp_vertical_offset(state, region);
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
    let region = region_at(x, y + state.panel_scroll.root_y);
    let mut changed = clamp_horizontal_offset(state, region);
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
    panel_scroll_state::max_scroll_y_for(region, state.selected_page, state.tree_expansion)
}

fn max_scroll_x(state: &StorybookWindowState, region: PanelScrollRegion) -> usize {
    panel_scroll_state::max_scroll_x_for(region, state.selected_page, state.tree_expansion)
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
