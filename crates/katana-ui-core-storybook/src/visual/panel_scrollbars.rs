use super::canvas::Canvas;
use super::layout_metrics::LayoutRect;
use super::palette::VisualPalette;
use super::panel_layout;
use super::panel_scroll_state::{PanelScrollOffsets, PanelScrollRegion};
use super::render_context::ScenarioContext;
use super::scrollbar_model::ScrollbarModel;

const TRACK_RADIUS: usize = 4;
const THUMB_RADIUS: usize = 4;

pub(super) fn draw(canvas: &mut Canvas, palette: &VisualPalette, scenario: ScenarioContext<'_>) {
    for region in [
        PanelScrollRegion::Navigation,
        PanelScrollRegion::Preview,
        PanelScrollRegion::Inspector,
    ] {
        if vertical_bar_visible(region, scenario.selected_page, scenario.scrollbar_visible) {
            draw_vertical_bar(canvas, palette, region, scenario.panel_scroll);
        }
    }
    if horizontal_bar_visible(
        PanelScrollRegion::Preview,
        scenario.selected_page,
        scenario.scrollbar_visible,
    ) {
        draw_horizontal_bar(
            canvas,
            palette,
            PanelScrollRegion::Preview,
            scenario.panel_scroll,
        );
    }
}

pub(super) fn vertical_bar_visible(
    region: PanelScrollRegion,
    selected_page: &str,
    scrollbar_visible: bool,
) -> bool {
    if !scrollbar_visible {
        return false;
    }
    vertical_region_scrollable(region, selected_page) && region != PanelScrollRegion::Root
}

pub(super) fn vertical_region_scrollable(region: PanelScrollRegion, selected_page: &str) -> bool {
    super::panel_scroll_state::overflow_for(region, selected_page, Default::default()).overflows_y()
}

pub(super) fn horizontal_bar_visible(
    region: PanelScrollRegion,
    selected_page: &str,
    scrollbar_visible: bool,
) -> bool {
    scrollbar_visible && horizontal_region_scrollable(region, selected_page)
}

pub(super) fn horizontal_region_scrollable(region: PanelScrollRegion, selected_page: &str) -> bool {
    super::panel_scroll_state::overflow_for(region, selected_page, Default::default()).overflows_x()
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

#[cfg(test)]
pub(super) fn horizontal_track_rect_for(region: PanelScrollRegion) -> LayoutRect {
    horizontal_model_for(region).track
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
    let layout = panel_layout::region_layout(region);
    ScrollbarModel::new(
        layout.vertical_track,
        panel_layout::VERTICAL_THUMB_HEIGHT,
        super::panel_scroll_state::max_scroll_y(region),
    )
}

fn horizontal_model_for(region: PanelScrollRegion) -> ScrollbarModel {
    let layout = panel_layout::region_layout(region);
    ScrollbarModel::new(
        layout.horizontal_track,
        panel_layout::HORIZONTAL_THUMB_WIDTH,
        super::panel_scroll_state::max_scroll_x(region),
    )
}
