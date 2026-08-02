use super::navigation_tree::TreeExpansionState;
use super::panel_scroll_state::{PanelScrollOffsets, PanelScrollRegion};
use super::panel_scrollbar_metrics;

pub(super) fn region_from_thumb_for(
    x: usize,
    y: usize,
    offsets: PanelScrollOffsets,
    selected_page: &str,
    tree_expansion: TreeExpansionState,
) -> Option<PanelScrollRegion> {
    [
        PanelScrollRegion::Navigation,
        PanelScrollRegion::Preview,
        PanelScrollRegion::Inspector,
    ]
    .into_iter()
    .find(|region| {
        panel_scrollbar_metrics::thumb_rect_for_state(
            *region,
            offsets,
            selected_page,
            tree_expansion,
        )
        .contains(x, y)
    })
}

#[cfg(test)]
pub(super) fn horizontal_region_from_thumb_for(
    x: usize,
    y: usize,
    offsets: PanelScrollOffsets,
    selected_page: &str,
    tree_expansion: TreeExpansionState,
) -> Option<PanelScrollRegion> {
    [
        PanelScrollRegion::Navigation,
        PanelScrollRegion::Preview,
        PanelScrollRegion::Inspector,
    ]
    .into_iter()
    .find(|region| {
        panel_scrollbar_metrics::horizontal_thumb_rect_for_state(
            *region,
            offsets,
            selected_page,
            tree_expansion,
        )
        .contains(x, y)
    })
}
