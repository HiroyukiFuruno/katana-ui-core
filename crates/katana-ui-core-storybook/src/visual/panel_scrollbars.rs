use super::canvas::Canvas;
use super::layout_metrics::LayoutRect;
use super::navigation_tree::TreeExpansionState;
use super::palette::VisualPalette;
use super::panel_scroll_state::{PanelScrollOffsets, PanelScrollRegion};
#[cfg(test)]
pub(super) use super::panel_scrollbar_metrics::{
    PANEL_SCROLLBAR_THUMB_MIN_LENGTH, horizontal_thumb_rect_for,
};
pub(super) use super::panel_scrollbar_metrics::{
    horizontal_bar_visible_for, horizontal_offset_from_drag_for, horizontal_region_scrollable_for,
    horizontal_thumb_rect_for_state, offset_from_drag_for, thumb_rect_for, thumb_rect_for_state,
    vertical_bar_visible_for, vertical_region_scrollable_for,
};
#[cfg(test)]
pub(super) use super::panel_scrollbar_metrics::{horizontal_track_rect_for, track_rect_for};
use super::render_context::ScenarioContext;

const TRACK_RADIUS: usize = 4;
const THUMB_RADIUS: usize = 4;

pub(super) fn draw(canvas: &mut Canvas, palette: &VisualPalette, scenario: ScenarioContext<'_>) {
    for region in [
        PanelScrollRegion::Navigation,
        PanelScrollRegion::Preview,
        PanelScrollRegion::Inspector,
    ] {
        if vertical_bar_visible_for(
            region,
            scenario.selected_page,
            scenario.tree_expansion,
            scenario.scrollbar_visible,
        ) {
            draw_vertical_bar(
                canvas,
                palette,
                region,
                scenario.panel_scroll,
                scenario.selected_page,
                scenario.tree_expansion,
            );
        }
    }
    for region in [
        PanelScrollRegion::Navigation,
        PanelScrollRegion::Preview,
        PanelScrollRegion::Inspector,
    ] {
        if horizontal_bar_visible_for(
            region,
            scenario.selected_page,
            scenario.tree_expansion,
            scenario.scrollbar_visible,
        ) {
            draw_horizontal_bar(
                canvas,
                palette,
                region,
                scenario.panel_scroll,
                scenario.selected_page,
                scenario.tree_expansion,
            );
        }
    }
}

fn draw_vertical_bar(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    region: PanelScrollRegion,
    offsets: PanelScrollOffsets,
    selected_page: &str,
    tree_expansion: TreeExpansionState,
) {
    let track = super::panel_scrollbar_metrics::vertical_track_rect(region);
    let thumb = thumb_rect_for_state(region, offsets, selected_page, tree_expansion);
    draw_track(canvas, palette, track);
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
    selected_page: &str,
    tree_expansion: TreeExpansionState,
) {
    let track = super::panel_scrollbar_metrics::horizontal_track_rect(region);
    let thumb = horizontal_thumb_rect_for_state(region, offsets, selected_page, tree_expansion);
    draw_track(canvas, palette, track);
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
