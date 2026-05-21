use crate::visual::panel_scroll_state::{PanelScrollOffsets, PanelScrollRegion};
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
    visible: bool,
) -> Option<PanelScrollRegion> {
    panel_scrollbars::region_from_thumb(x, y, offsets)
        .filter(|region| panel_scrollbars::vertical_bar_visible(*region, page, visible))
}

pub(super) fn horizontal_region_at(
    x: usize,
    y: usize,
    offsets: PanelScrollOffsets,
    page: &str,
    visible: bool,
) -> Option<PanelScrollRegion> {
    panel_scrollbars::horizontal_region_from_thumb(x, y, offsets)
        .filter(|region| panel_scrollbars::horizontal_bar_visible(*region, page, visible))
}
