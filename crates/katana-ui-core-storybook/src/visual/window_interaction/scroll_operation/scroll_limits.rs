use super::super::StorybookWindowState;
use crate::visual::panel_scroll_state::{PanelScrollOverflowModel, PanelScrollRegion};

pub(super) fn clamp_vertical_offset(
    state: &mut StorybookWindowState,
    region: PanelScrollRegion,
) -> bool {
    let max_offset = max_scroll_y(state, region);
    state.panel_scroll.set_drag_offset_with_max(
        region,
        state.panel_scroll.offset(region),
        max_offset,
    )
}

pub(super) fn clamp_horizontal_offset(
    state: &mut StorybookWindowState,
    region: PanelScrollRegion,
) -> bool {
    let max_offset = max_scroll_x(state, region);
    state.panel_scroll.set_drag_offset_x_with_max(
        region,
        state.panel_scroll.offset_x(region),
        max_offset,
    )
}

pub(super) fn max_scroll_y(state: &StorybookWindowState, region: PanelScrollRegion) -> usize {
    PanelScrollOverflowModel::max_scroll_y_for(region, state.selected_page, state.tree_expansion)
}

pub(super) fn max_scroll_x(state: &StorybookWindowState, region: PanelScrollRegion) -> usize {
    PanelScrollOverflowModel::max_scroll_x_for(region, state.selected_page, state.tree_expansion)
}
