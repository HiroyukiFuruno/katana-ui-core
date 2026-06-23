use super::layout_metrics::LayoutRect;
use super::navigation_tree::TreeExpansionState;
use super::panel_layout;
use super::panel_scroll_state::{PanelScrollOffsets, PanelScrollRegion};
use super::scrollbar_model::ScrollbarModel;

pub(super) const PANEL_SCROLLBAR_THUMB_MIN_LENGTH: usize = 32;

pub(super) fn vertical_bar_visible_for(
    region: PanelScrollRegion,
    selected_page: &str,
    tree_expansion: TreeExpansionState,
    scrollbar_visible: bool,
) -> bool {
    if !scrollbar_visible {
        return false;
    }
    vertical_region_scrollable_for(region, selected_page, tree_expansion)
        && region != PanelScrollRegion::Root
}

pub(super) fn vertical_region_scrollable_for(
    region: PanelScrollRegion,
    selected_page: &str,
    tree_expansion: TreeExpansionState,
) -> bool {
    super::panel_scroll_state::PanelScrollOverflowModel::overflow_for(
        region,
        selected_page,
        tree_expansion,
    )
    .overflows_y()
}

pub(super) fn horizontal_bar_visible_for(
    region: PanelScrollRegion,
    selected_page: &str,
    tree_expansion: TreeExpansionState,
    scrollbar_visible: bool,
) -> bool {
    scrollbar_visible && horizontal_region_scrollable_for(region, selected_page, tree_expansion)
}

pub(super) fn horizontal_region_scrollable_for(
    region: PanelScrollRegion,
    selected_page: &str,
    tree_expansion: TreeExpansionState,
) -> bool {
    super::panel_scroll_state::PanelScrollOverflowModel::overflow_for(
        region,
        selected_page,
        tree_expansion,
    )
    .overflows_x()
}

#[cfg(test)]
pub(super) fn thumb_rect_for(region: PanelScrollRegion, offsets: PanelScrollOffsets) -> LayoutRect {
    thumb_rect_for_state(region, offsets, "", Default::default())
}

pub(super) fn thumb_rect_for_state(
    region: PanelScrollRegion,
    offsets: PanelScrollOffsets,
    selected_page: &str,
    tree_expansion: TreeExpansionState,
) -> LayoutRect {
    vertical_model_for(region, selected_page, tree_expansion).thumb_rect(offsets.offset(region))
}

#[cfg(test)]
pub(super) fn horizontal_thumb_rect_for(
    region: PanelScrollRegion,
    offsets: PanelScrollOffsets,
) -> LayoutRect {
    horizontal_thumb_rect_for_state(region, offsets, "", Default::default())
}

pub(super) fn horizontal_thumb_rect_for_state(
    region: PanelScrollRegion,
    offsets: PanelScrollOffsets,
    selected_page: &str,
    tree_expansion: TreeExpansionState,
) -> LayoutRect {
    horizontal_model_for(region, selected_page, tree_expansion)
        .horizontal_thumb_rect(offsets.offset_x(region))
}

pub(super) fn offset_from_drag_for(
    region: PanelScrollRegion,
    y: usize,
    selected_page: &str,
    tree_expansion: TreeExpansionState,
) -> usize {
    vertical_model_for(region, selected_page, tree_expansion).offset_from_thumb_y(y)
}

pub(super) fn horizontal_offset_from_drag_for(
    region: PanelScrollRegion,
    x: usize,
    selected_page: &str,
    tree_expansion: TreeExpansionState,
) -> usize {
    horizontal_model_for(region, selected_page, tree_expansion).offset_from_thumb_x(x)
}

#[cfg(test)]
pub(super) fn track_rect_for(region: PanelScrollRegion) -> LayoutRect {
    panel_layout::region_layout(region).vertical_track
}

#[cfg(test)]
pub(super) fn horizontal_track_rect_for(region: PanelScrollRegion) -> LayoutRect {
    panel_layout::region_layout(region).horizontal_track
}

pub(super) fn vertical_track_rect(region: PanelScrollRegion) -> LayoutRect {
    panel_layout::region_layout(region).vertical_track
}

pub(super) fn horizontal_track_rect(region: PanelScrollRegion) -> LayoutRect {
    panel_layout::region_layout(region).horizontal_track
}

fn vertical_model_for(
    region: PanelScrollRegion,
    selected_page: &str,
    tree_expansion: TreeExpansionState,
) -> ScrollbarModel {
    let overflow = super::panel_scroll_state::PanelScrollOverflowModel::overflow_for(
        region,
        selected_page,
        tree_expansion,
    );
    ScrollbarModel::vertical(
        vertical_track_rect(region),
        overflow.viewport_height,
        overflow.content_height,
        PANEL_SCROLLBAR_THUMB_MIN_LENGTH,
    )
}

fn horizontal_model_for(
    region: PanelScrollRegion,
    selected_page: &str,
    tree_expansion: TreeExpansionState,
) -> ScrollbarModel {
    let overflow = super::panel_scroll_state::PanelScrollOverflowModel::overflow_for(
        region,
        selected_page,
        tree_expansion,
    );
    ScrollbarModel::horizontal(
        horizontal_track_rect(region),
        overflow.viewport_width,
        overflow.content_width,
        PANEL_SCROLLBAR_THUMB_MIN_LENGTH,
    )
}
