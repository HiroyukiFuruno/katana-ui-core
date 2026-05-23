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
    if !panel_scrollbars::vertical_region_scrollable_for(
        region,
        state.selected_page,
        state.tree_expansion,
    ) {
        return false;
    }
    let changed = state.panel_scroll.scroll_delta_with_max(
        region,
        vertical_max_scroll_y(state, region),
        delta_y,
    );
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
    if !panel_scrollbars::horizontal_region_scrollable_for(
        region,
        state.selected_page,
        state.tree_expansion,
    ) {
        return false;
    }
    state.panel_scroll.scroll_delta_x(region, delta_x)
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
    let next = panel_scrollbars::offset_from_drag_for(
        region,
        y,
        state.selected_page,
        state.tree_expansion,
    );
    let changed = state.panel_scroll.set_drag_offset_with_max(
        region,
        next,
        vertical_max_scroll_y(state, region),
    );
    if region == PanelScrollRegion::Root {
        state.scroll_y = state.panel_scroll.root_y.min(MAX_SCROLL_Y);
    }
    changed
}

fn vertical_max_scroll_y(state: &StorybookWindowState, region: PanelScrollRegion) -> usize {
    if region == PanelScrollRegion::Navigation {
        return crate::visual::navigation_tree::max_scroll_y(state.tree_expansion);
    }
    panel_scroll_state::max_scroll_y(region)
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
    let next = panel_scrollbars::horizontal_offset_from_drag_for(
        region,
        x,
        state.selected_page,
        state.tree_expansion,
    );
    state.panel_scroll.set_drag_offset_x(region, next)
}
