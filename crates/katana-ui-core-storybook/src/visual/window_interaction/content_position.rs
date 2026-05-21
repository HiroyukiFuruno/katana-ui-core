use super::StorybookWindowState;
use crate::visual::panel_scroll_state::{PanelScrollRegion, region_at};

pub(in crate::visual) fn click_content_y(
    state: &StorybookWindowState,
    x: usize,
    y: usize,
) -> usize {
    let content_y = y + state.panel_scroll.root_y;
    match region_at(x, content_y) {
        PanelScrollRegion::Root => content_y,
        PanelScrollRegion::Navigation => content_y,
        PanelScrollRegion::Preview => content_y + state.panel_scroll.preview_y,
        PanelScrollRegion::Inspector => content_y + state.panel_scroll.inspector_y,
    }
}
