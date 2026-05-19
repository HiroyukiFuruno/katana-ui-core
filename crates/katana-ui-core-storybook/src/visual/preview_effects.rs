use super::canvas::Canvas;
use super::layout_metrics::LayoutRect;
use super::render_context::{RenderContext, ScenarioContext};

const PRESET_MARKER_WIDTH: usize = 96;
const PRESET_MARKER_HEIGHT: usize = 16;
const PRESET_MARKER_GAP: usize = 6;
const SETTING_MARKER_WIDTH: usize = 116;
const SETTING_MARKER_HEIGHT: usize = 18;
const TEXT_SIZE: f32 = 9.0;
const TEXT_OFFSET_X: usize = 6;
const TEXT_OFFSET_Y: usize = 5;
const TREE_TRACK_X: usize = 186;
const TREE_TRACK_Y: usize = 32;
const TREE_TRACK_WIDTH: usize = 4;
const TREE_TRACK_HEIGHT: usize = 60;
const TREE_THUMB_HEIGHT: usize = 18;
const TREE_THUMB_STEP: usize = 12;
const TREE_MAX_PRESET_INDEX: usize = 3;
const CONTEXT_MARKER_WIDTH: usize = 92;
const CONTEXT_MARKER_HEIGHT: usize = 14;

pub(super) fn draw(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    scenario: ScenarioContext<'_>,
    rect: LayoutRect,
) {
    if rect.width == 0 {
        return;
    }
    draw_preset_marker(canvas, render, scenario, rect);
    draw_setting_marker(canvas, render, scenario, rect);
    draw_tree_view_scroll(canvas, render, scenario, rect);
}

fn draw_preset_marker(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    scenario: ScenarioContext<'_>,
    rect: LayoutRect,
) {
    if is_button_page(scenario.selected_page) {
        return;
    }
    let fill = if scenario.preset_index == 0 {
        render.palette.panel
    } else {
        render.palette.accent
    };
    let text_color = if scenario.preset_index == 0 {
        render.palette.muted
    } else {
        render.palette.background
    };
    let marker_x = rect.x + rect.width - PRESET_MARKER_WIDTH - PRESET_MARKER_GAP;
    let marker_y = rect.y + PRESET_MARKER_GAP;
    canvas.fill_rect(
        marker_x,
        marker_y,
        PRESET_MARKER_WIDTH,
        PRESET_MARKER_HEIGHT,
        fill,
    );
    render.code_text.draw(
        canvas,
        &format!("preset {}", scenario.preset_index),
        marker_x + TEXT_OFFSET_X,
        marker_y + TEXT_OFFSET_Y,
        TEXT_SIZE,
        text_color,
    );
}

fn draw_setting_marker(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    scenario: ScenarioContext<'_>,
    rect: LayoutRect,
) {
    if is_button_page(scenario.selected_page) {
        return;
    }
    if !scenario.screen_state.has_settings_override() {
        return;
    }
    let marker_x = rect.x + rect.width - SETTING_MARKER_WIDTH - PRESET_MARKER_GAP;
    let marker_y = rect.y + PRESET_MARKER_HEIGHT + PRESET_MARKER_GAP + PRESET_MARKER_GAP;
    canvas.fill_rect(
        marker_x,
        marker_y,
        SETTING_MARKER_WIDTH,
        SETTING_MARKER_HEIGHT,
        render.palette.accent,
    );
    render.code_text.draw(
        canvas,
        scenario.screen_state.last_setting_value,
        marker_x + TEXT_OFFSET_X,
        marker_y + TEXT_OFFSET_Y,
        TEXT_SIZE,
        render.palette.background,
    );
}

fn is_button_page(page: &str) -> bool {
    matches!(
        page,
        "button" | "text-button" | "svg-button" | "icon-text-button"
    )
}

fn draw_tree_view_scroll(
    canvas: &mut Canvas,
    render: RenderContext<'_>,
    scenario: ScenarioContext<'_>,
    rect: LayoutRect,
) {
    if scenario.selected_page != "tree-view" {
        return;
    }
    let track_x = rect.x + TREE_TRACK_X;
    let track_y = rect.y + TREE_TRACK_Y;
    canvas.fill_rect(
        track_x,
        track_y,
        TREE_TRACK_WIDTH,
        TREE_TRACK_HEIGHT,
        render.palette.border,
    );
    let thumb_y = track_y + scenario.preset_index.min(TREE_MAX_PRESET_INDEX) * TREE_THUMB_STEP;
    canvas.fill_rect(
        track_x,
        thumb_y,
        TREE_TRACK_WIDTH,
        TREE_THUMB_HEIGHT,
        render.palette.accent,
    );
    if scenario.screen_state.state_label != "context_menu=open" {
        return;
    }
    let marker_x = rect.x + rect.width - CONTEXT_MARKER_WIDTH - PRESET_MARKER_GAP;
    let marker_y = rect.y + TREE_TRACK_Y + TREE_TRACK_HEIGHT - CONTEXT_MARKER_HEIGHT;
    canvas.fill_rect(
        marker_x,
        marker_y,
        CONTEXT_MARKER_WIDTH,
        CONTEXT_MARKER_HEIGHT,
        render.palette.accent,
    );
    render.code_text.draw(
        canvas,
        "context open",
        marker_x + TEXT_OFFSET_X,
        marker_y + TEXT_OFFSET_Y,
        TEXT_SIZE,
        render.palette.background,
    );
}
