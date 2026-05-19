use super::canvas::Canvas;
use super::layout_metrics::{
    INSPECTOR_HEIGHT, INSPECTOR_WIDTH, INSPECTOR_X, INSPECTOR_Y, LayoutRect,
};
use super::palette::VisualPalette;
use super::panel_scroll_state::{PanelScrollOffsets, PanelScrollRegion};
use super::scrollbar_model::{ScrollbarMarker, ScrollbarMarkerKind, ScrollbarModel};

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
const TRACK_RADIUS: usize = 4;
const THUMB_RADIUS: usize = 4;
const MARKER_X_OFFSET: usize = 1;
const MARKER_WIDTH: usize = 6;
const MARKER_HEIGHT: usize = 3;
const MARKER_WARNING_COLOR: u32 = 0xd7ba7d;
const MARKER_ERROR_COLOR: u32 = 0xf44747;
const MARKER_SEARCH_COLOR: u32 = 0x9cdcfe;
const MARKER_DIFF_COLOR: u32 = 0x6a9955;
const MARKER_HINT_COLOR: u32 = 0xc586c0;

const NAV_MARKERS: &[ScrollbarMarker] = &[
    ScrollbarMarker::new(120, ScrollbarMarkerKind::Hint),
    ScrollbarMarker::new(560, ScrollbarMarkerKind::Search),
];
const PREVIEW_MARKERS: &[ScrollbarMarker] = &[
    ScrollbarMarker::new(180, ScrollbarMarkerKind::Warning),
    ScrollbarMarker::new(420, ScrollbarMarkerKind::Diff),
    ScrollbarMarker::new(760, ScrollbarMarkerKind::Error),
];
const INSPECTOR_MARKERS: &[ScrollbarMarker] =
    &[ScrollbarMarker::new(300, ScrollbarMarkerKind::Hint)];

pub(super) fn draw(canvas: &mut Canvas, palette: &VisualPalette, offsets: PanelScrollOffsets) {
    draw_bar(canvas, palette, PanelScrollRegion::Navigation, offsets);
    draw_bar(canvas, palette, PanelScrollRegion::Preview, offsets);
    draw_bar(canvas, palette, PanelScrollRegion::Inspector, offsets);
}

pub(super) fn thumb_rect_for(region: PanelScrollRegion, offsets: PanelScrollOffsets) -> LayoutRect {
    model_for(region).thumb_rect(offsets.offset(region))
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
    model_for(region).offset_from_thumb_y(y)
}

fn draw_bar(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    region: PanelScrollRegion,
    offsets: PanelScrollOffsets,
) {
    let model = model_for(region);
    let thumb = model.thumb_rect(offsets.offset(region));
    draw_track(canvas, palette, model.track);
    draw_markers(canvas, model, markers_for(region));
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

fn draw_markers(canvas: &mut Canvas, model: ScrollbarModel, markers: &[ScrollbarMarker]) {
    for marker in markers {
        let y = model.marker_y(marker.ratio_per_mille);
        canvas.fill_round_rect(
            model.track.x + MARKER_X_OFFSET,
            y,
            MARKER_WIDTH,
            MARKER_HEIGHT,
            1,
            marker_color(marker.kind),
        );
    }
}

fn model_for(region: PanelScrollRegion) -> ScrollbarModel {
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

fn markers_for(region: PanelScrollRegion) -> &'static [ScrollbarMarker] {
    match region {
        PanelScrollRegion::Navigation => NAV_MARKERS,
        PanelScrollRegion::Preview => PREVIEW_MARKERS,
        PanelScrollRegion::Inspector | PanelScrollRegion::Root => INSPECTOR_MARKERS,
    }
}

fn marker_color(kind: ScrollbarMarkerKind) -> u32 {
    match kind {
        ScrollbarMarkerKind::Warning => MARKER_WARNING_COLOR,
        ScrollbarMarkerKind::Error => MARKER_ERROR_COLOR,
        ScrollbarMarkerKind::Search => MARKER_SEARCH_COLOR,
        ScrollbarMarkerKind::Diff => MARKER_DIFF_COLOR,
        ScrollbarMarkerKind::Hint => MARKER_HINT_COLOR,
    }
}
