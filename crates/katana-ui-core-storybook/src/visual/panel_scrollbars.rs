use super::canvas::Canvas;
use super::layout_metrics::{
    INSPECTOR_HEIGHT, INSPECTOR_WIDTH, INSPECTOR_X, INSPECTOR_Y, LayoutRect,
};
use super::palette::VisualPalette;
use super::panel_scroll_state::{PanelScrollOffsets, PanelScrollRegion};
use super::scrollbar_model::ScrollbarModel;

pub(super) const NAV_SCROLL_X: usize = 272;
pub(super) const NAV_SCROLL_Y: usize = 132;
pub(super) const NAV_SCROLL_HEIGHT: usize = 724;
pub(super) const PREVIEW_SCROLL_X: usize = 1028;
pub(super) const PREVIEW_SCROLL_Y: usize = 136;
pub(super) const PREVIEW_SCROLL_HEIGHT: usize = 714;
pub(super) const INSPECTOR_SCROLL_X: usize = INSPECTOR_X + INSPECTOR_WIDTH - 8;
pub(super) const INSPECTOR_SCROLL_Y: usize = INSPECTOR_Y + 78;
pub(super) const INSPECTOR_SCROLL_HEIGHT: usize = INSPECTOR_HEIGHT - 100;
const TRACK_WIDTH: usize = 8;
const THUMB_HEIGHT: usize = 64;
const HORIZONTAL_TRACK_HEIGHT: usize = 8;
const HORIZONTAL_THUMB_WIDTH: usize = 96;
const TRACK_RADIUS: usize = 4;
const THUMB_RADIUS: usize = 4;
const HORIZONTAL_TRACK_Y_GAP: usize = 10;

pub(super) fn draw(canvas: &mut Canvas, palette: &VisualPalette, offsets: PanelScrollOffsets) {
    draw_vertical_bar(canvas, palette, PanelScrollRegion::Navigation, offsets);
    draw_vertical_bar(canvas, palette, PanelScrollRegion::Preview, offsets);
    draw_vertical_bar(canvas, palette, PanelScrollRegion::Inspector, offsets);
    draw_horizontal_bar(canvas, palette, PanelScrollRegion::Preview, offsets);
}

pub(super) fn thumb_rect_for(region: PanelScrollRegion, offsets: PanelScrollOffsets) -> LayoutRect {
    vertical_model_for(region).thumb_rect(offsets.offset(region))
}

pub(super) fn horizontal_thumb_rect_for(
    region: PanelScrollRegion,
    offsets: PanelScrollOffsets,
) -> LayoutRect {
    horizontal_model_for(region).horizontal_thumb_rect(offsets.offset_x(region))
}

#[cfg(test)]
pub(super) fn track_rect_for(region: PanelScrollRegion) -> LayoutRect {
    vertical_model_for(region).track
}

pub(super) fn region_from_thumb(
    x: usize,
    y: usize,
    offsets: PanelScrollOffsets,
) -> Option<PanelScrollRegion> {
    [
        PanelScrollRegion::Navigation,
        PanelScrollRegion::Preview,
        PanelScrollRegion::Inspector,
    ]
    .into_iter()
    .find(|region| thumb_rect_for(*region, offsets).contains(x, y))
}

pub(super) fn offset_from_drag(region: PanelScrollRegion, y: usize) -> usize {
    vertical_model_for(region).offset_from_thumb_y(y)
}

pub(super) fn horizontal_region_from_thumb(
    x: usize,
    y: usize,
    offsets: PanelScrollOffsets,
) -> Option<PanelScrollRegion> {
    [PanelScrollRegion::Preview]
        .into_iter()
        .find(|region| horizontal_thumb_rect_for(*region, offsets).contains(x, y))
}

pub(super) fn horizontal_offset_from_drag(region: PanelScrollRegion, x: usize) -> usize {
    horizontal_model_for(region).offset_from_thumb_x(x)
}

fn draw_vertical_bar(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    region: PanelScrollRegion,
    offsets: PanelScrollOffsets,
) {
    let model = vertical_model_for(region);
    let thumb = model.thumb_rect(offsets.offset(region));
    draw_track(canvas, palette, model.track);
    canvas.fill_round_rect(
        thumb.x,
        thumb.y,
        thumb.width,
        thumb.height,
        THUMB_RADIUS,
        palette.accent,
    );
}

fn draw_horizontal_bar(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    region: PanelScrollRegion,
    offsets: PanelScrollOffsets,
) {
    let model = horizontal_model_for(region);
    let thumb = model.horizontal_thumb_rect(offsets.offset_x(region));
    draw_track(canvas, palette, model.track);
    canvas.fill_round_rect(
        thumb.x,
        thumb.y,
        thumb.width,
        thumb.height,
        THUMB_RADIUS,
        palette.accent,
    );
}

fn draw_track(canvas: &mut Canvas, palette: &VisualPalette, track: LayoutRect) {
    canvas.fill_round_rect(
        track.x,
        track.y,
        track.width,
        track.height,
        TRACK_RADIUS,
        palette.code_background,
    );
}

fn vertical_model_for(region: PanelScrollRegion) -> ScrollbarModel {
    match region {
        PanelScrollRegion::Navigation => ScrollbarModel::new(
            LayoutRect::new(NAV_SCROLL_X, NAV_SCROLL_Y, TRACK_WIDTH, NAV_SCROLL_HEIGHT),
            THUMB_HEIGHT,
            super::panel_scroll_state::NAV_MAX_SCROLL_Y,
        ),
        PanelScrollRegion::Preview => ScrollbarModel::new(
            LayoutRect::new(
                PREVIEW_SCROLL_X,
                PREVIEW_SCROLL_Y,
                TRACK_WIDTH,
                PREVIEW_SCROLL_HEIGHT,
            ),
            THUMB_HEIGHT,
            super::panel_scroll_state::PREVIEW_MAX_SCROLL_Y,
        ),
        PanelScrollRegion::Inspector | PanelScrollRegion::Root => ScrollbarModel::new(
            LayoutRect::new(
                INSPECTOR_SCROLL_X,
                INSPECTOR_SCROLL_Y,
                TRACK_WIDTH,
                INSPECTOR_SCROLL_HEIGHT,
            ),
            THUMB_HEIGHT,
            super::panel_scroll_state::INSPECTOR_MAX_SCROLL_Y,
        ),
    }
}

fn horizontal_model_for(region: PanelScrollRegion) -> ScrollbarModel {
    match region {
        PanelScrollRegion::Preview => ScrollbarModel::new(
            LayoutRect::new(
                super::layout_metrics::PREVIEW_X,
                PREVIEW_SCROLL_Y + PREVIEW_SCROLL_HEIGHT + HORIZONTAL_TRACK_Y_GAP,
                PREVIEW_SCROLL_X - super::layout_metrics::PREVIEW_X,
                HORIZONTAL_TRACK_HEIGHT,
            ),
            HORIZONTAL_THUMB_WIDTH,
            super::panel_scroll_state::PREVIEW_MAX_SCROLL_X,
        ),
        PanelScrollRegion::Navigation => ScrollbarModel::new(
            LayoutRect::new(
                super::layout_metrics::NAV_ROW_X,
                NAV_SCROLL_Y + NAV_SCROLL_HEIGHT + HORIZONTAL_TRACK_Y_GAP,
                NAV_SCROLL_X - super::layout_metrics::NAV_ROW_X,
                HORIZONTAL_TRACK_HEIGHT,
            ),
            HORIZONTAL_THUMB_WIDTH,
            super::panel_scroll_state::NAV_MAX_SCROLL_X,
        ),
        PanelScrollRegion::Inspector | PanelScrollRegion::Root => ScrollbarModel::new(
            LayoutRect::new(
                INSPECTOR_X,
                INSPECTOR_SCROLL_Y + INSPECTOR_SCROLL_HEIGHT + HORIZONTAL_TRACK_Y_GAP,
                INSPECTOR_WIDTH - TRACK_WIDTH,
                HORIZONTAL_TRACK_HEIGHT,
            ),
            HORIZONTAL_THUMB_WIDTH,
            super::panel_scroll_state::INSPECTOR_MAX_SCROLL_X,
        ),
    }
}
