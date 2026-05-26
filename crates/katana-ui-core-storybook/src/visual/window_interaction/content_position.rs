use super::StorybookWindowState;
use crate::visual::panel_scroll_state::{self, PanelScrollRegion};

pub(in crate::visual) fn click_content_y(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> usize {
    let content_y = y + state.panel_scroll.root_y;
    match panel_scroll_state::PanelScrollRegionModel::region_at(x, content_y) {
        PanelScrollRegion::Root => content_y,
        PanelScrollRegion::Navigation => content_y,
        PanelScrollRegion::Preview => {
            let max_preview_y = panel_scroll_state::PanelScrollOverflowModel::max_scroll_y_for(
                PanelScrollRegion::Preview,
                state.selected_page,
                state.tree_expansion,
            );
            content_y
                + state
                    .panel_scroll
                    .offset_with_max(PanelScrollRegion::Preview, max_preview_y)
        }
        PanelScrollRegion::Inspector => {
            let max_inspector_y = panel_scroll_state::PanelScrollOverflowModel::max_scroll_y_for(
                PanelScrollRegion::Inspector,
                state.selected_page,
                state.tree_expansion,
            );
            content_y
                + state
                    .panel_scroll
                    .offset_with_max(PanelScrollRegion::Inspector, max_inspector_y)
        }
    }
}
