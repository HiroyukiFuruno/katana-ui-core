use super::layout_metrics::{
    CONTENT_HEIGHT, INSPECTOR_HEIGHT, INSPECTOR_WIDTH, INSPECTOR_X, INSPECTOR_Y, LayoutRect,
    NAV_FIRST_ROW_Y, NAV_ROW_X, PRESET_ACTIVE_HEIGHT, PRESET_ACTIVE_Y, PREVIEW_X,
    navigation_menu_panel_rect,
};
use super::panel_scroll_state::PanelScrollRegion;
use super::render::{VIEWPORT_HEIGHT, WIDTH};

pub(super) const PANEL_EDGE_INSET: usize = 8;
pub(super) const SCROLLBAR_TRACK_WIDTH: usize = 8;
pub(super) const HORIZONTAL_TRACK_HEIGHT: usize = 8;
const PREVIEW_RIGHT_GAP: usize = 24;
const PREVIEW_BOTTOM_GAP: usize = 28;
const INSPECTOR_SCROLL_TOP_OFFSET: usize = 78;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PanelRegionLayout {
    pub(super) frame: LayoutRect,
    pub(super) content_viewport: LayoutRect,
    pub(super) vertical_track: LayoutRect,
    pub(super) horizontal_track: LayoutRect,
}

pub(super) fn region_layout(region: PanelScrollRegion) -> PanelRegionLayout {
    let frame = region_frame(region);
    let scroll_top = scroll_top_for(region, frame);
    let vertical_track = vertical_track(frame, scroll_top);
    let horizontal_track = horizontal_track(frame, vertical_track.x);
    let content_viewport = content_viewport(frame, scroll_top, vertical_track, horizontal_track);

    PanelRegionLayout {
        frame,
        content_viewport,
        vertical_track,
        horizontal_track,
    }
}

pub(super) fn region_frame(region: PanelScrollRegion) -> LayoutRect {
    match region {
        PanelScrollRegion::Root => LayoutRect::new(0, 0, WIDTH, CONTENT_HEIGHT),
        PanelScrollRegion::Navigation => navigation_menu_panel_rect(),
        PanelScrollRegion::Preview => preview_frame(),
        PanelScrollRegion::Inspector => {
            LayoutRect::new(INSPECTOR_X, INSPECTOR_Y, INSPECTOR_WIDTH, INSPECTOR_HEIGHT)
        }
    }
}

fn preview_frame() -> LayoutRect {
    let y = PRESET_ACTIVE_Y + PRESET_ACTIVE_HEIGHT;
    LayoutRect::new(
        PREVIEW_X,
        y,
        INSPECTOR_X.saturating_sub(PREVIEW_X + PREVIEW_RIGHT_GAP),
        VIEWPORT_HEIGHT.saturating_sub(y + PREVIEW_BOTTOM_GAP),
    )
}

fn scroll_top_for(region: PanelScrollRegion, frame: LayoutRect) -> usize {
    match region {
        PanelScrollRegion::Navigation => NAV_FIRST_ROW_Y,
        PanelScrollRegion::Preview => frame.y,
        PanelScrollRegion::Inspector => INSPECTOR_Y + INSPECTOR_SCROLL_TOP_OFFSET,
        PanelScrollRegion::Root => frame.y,
    }
}

fn vertical_track(frame: LayoutRect, scroll_top: usize) -> LayoutRect {
    LayoutRect::new(
        frame
            .right()
            .saturating_sub(PANEL_EDGE_INSET + SCROLLBAR_TRACK_WIDTH),
        scroll_top,
        SCROLLBAR_TRACK_WIDTH,
        frame.bottom().saturating_sub(scroll_top + PANEL_EDGE_INSET),
    )
}

fn horizontal_track(frame: LayoutRect, vertical_track_x: usize) -> LayoutRect {
    LayoutRect::new(
        frame.x + PANEL_EDGE_INSET,
        frame
            .bottom()
            .saturating_sub(PANEL_EDGE_INSET + HORIZONTAL_TRACK_HEIGHT),
        vertical_track_x.saturating_sub(frame.x + PANEL_EDGE_INSET * 2),
        HORIZONTAL_TRACK_HEIGHT,
    )
}

fn content_viewport(
    frame: LayoutRect,
    scroll_top: usize,
    vertical_track: LayoutRect,
    horizontal_track: LayoutRect,
) -> LayoutRect {
    let x = if frame == navigation_menu_panel_rect() {
        NAV_ROW_X
    } else {
        frame.x + PANEL_EDGE_INSET
    };
    LayoutRect::new(
        x,
        scroll_top,
        vertical_track.x.saturating_sub(x + PANEL_EDGE_INSET),
        horizontal_track
            .y
            .saturating_sub(scroll_top + PANEL_EDGE_INSET),
    )
}
