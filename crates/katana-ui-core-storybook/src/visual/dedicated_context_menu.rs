use super::canvas::Canvas;
use super::dedicated_context_menu_anchor;
use super::dedicated_context_menu_labels as labels;
use super::dedicated_context_menu_metrics as cm;
use super::dedicated_context_menu_popup;
use super::dedicated_dod_common::{self as common, Rect};
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) fn context_menu(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(
        canvas,
        text,
        palette,
        x,
        y,
        labels::preset_title(scenario.preset_index),
    );
    dedicated_context_menu_anchor::draw_anchor_surface(
        canvas,
        text,
        palette,
        scenario.preset_index,
        x,
        y,
    );
    dedicated_context_menu_popup::draw_menu(canvas, text, palette, scenario.preset_index, x, y);
    draw_markers(canvas, text, palette, scenario, x, y);
}

fn draw_markers(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let marker_labels = [
        labels::action_label(scenario),
        labels::event_label(scenario),
        labels::state_label(scenario),
        "visual:anchor",
    ];
    for (index, label) in marker_labels.into_iter().enumerate() {
        draw_marker(canvas, text, palette, x, y, index, label);
    }
}

fn draw_marker(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    index: usize,
    label: &str,
) {
    let marker_x = x + cm::MARKER_X_OFFSET + index * (cm::MARKER_WIDTH + cm::MARKER_GAP);
    common::chip(
        canvas,
        text,
        palette,
        Rect::new(
            marker_x,
            y + cm::MARKER_Y,
            cm::MARKER_WIDTH,
            cm::MARKER_HEIGHT,
        ),
        label,
        labels::marker_color(index, palette),
    );
}
