use crate::visual::navigation_tree::TreeExpansionState;
use crate::visual::panel_scroll_state::{PanelScrollOffsets, PanelScrollRegion};
use crate::visual::panel_scrollbar_hit_test::{
    horizontal_region_from_thumb_for, region_from_thumb_for,
};
use crate::visual::panel_scrollbars;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum PanelScrollDragTarget {
    Vertical(PanelScrollRegion),
    Horizontal(PanelScrollRegion),
}

pub(super) fn vertical_region_at(
    x: usize,
    y: usize,
    offsets: PanelScrollOffsets,
    page: &str,
    tree_expansion: TreeExpansionState,
    visible: bool,
) -> Option<PanelScrollRegion> {
    region_from_thumb_for(x, y, offsets, page, tree_expansion).filter(|region| {
        panel_scrollbars::vertical_bar_visible_for(*region, page, tree_expansion, visible)
    })
}

pub(super) fn horizontal_region_at(
    x: usize,
    y: usize,
    offsets: PanelScrollOffsets,
    page: &str,
    tree_expansion: TreeExpansionState,
    visible: bool,
) -> Option<PanelScrollRegion> {
    horizontal_region_from_thumb_for(x, y, offsets, page, tree_expansion).filter(|region| {
        panel_scrollbars::horizontal_bar_visible_for(*region, page, tree_expansion, visible)
    })
}
