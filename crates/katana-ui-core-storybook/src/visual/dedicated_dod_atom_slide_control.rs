use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const TRACK_X: usize = 24;
const TRACK_Y: usize = 54;
const TRACK_WIDTH: usize = 224;
const TRACK_HEIGHT: usize = 6;
const KNOB_SIZE: usize = 16;
const KNOB_Y_OFFSET: usize = 5;
const STEP_Y: usize = 74;
const STEP_WIDTH: usize = 2;
const STEP_HEIGHT: usize = 8;
const BASE_STEP_MARK_COUNT: usize = 4;
const DENSE_STEP_MARK_COUNT: usize = 6;
const STATUS_X: usize = 24;
const STATUS_Y: usize = 96;
const STATUS_WIDTH: usize = 96;
const STATUS_HEIGHT: usize = 20;
const STATUS_GAP: usize = 8;
const STATUS_ROW_COUNT: usize = 3;
const TEXT_X: usize = 7;
const TEXT_Y: usize = 6;
const DRAG_PRESET_INDEX: usize = 1;
const STEP_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;
const BASE_VALUE: usize = 42;
const DRAG_VALUE: usize = 64;
const STEP_VALUE: usize = 25;
const THEME_VALUE: usize = 72;
const MAX_VALUE: usize = 100;

pub(super) fn slide_control(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "SlideControl");
    draw_track(canvas, palette, scenario, x, y);
    draw_steps(canvas, palette, scenario, x, y);
    draw_status(canvas, text, palette, scenario, x, y);
}

fn draw_track(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let value = value_for(scenario);
    let filled_width = TRACK_WIDTH * value / MAX_VALUE;
    let track_fill = track_color(palette, scenario);
    canvas.fill_rect(
        x + TRACK_X,
        y + TRACK_Y,
        TRACK_WIDTH,
        TRACK_HEIGHT,
        track_fill,
    );
    canvas.fill_rect(
        x + TRACK_X,
        y + TRACK_Y,
        filled_width,
        TRACK_HEIGHT,
        palette.accent,
    );
    let knob_x = x + TRACK_X + filled_width.saturating_sub(KNOB_SIZE / 2);
    canvas.fill_rect(
        knob_x,
        y + TRACK_Y - KNOB_Y_OFFSET,
        KNOB_SIZE,
        KNOB_SIZE,
        knob_color(palette, scenario),
    );
    canvas.stroke_rect(
        knob_x,
        y + TRACK_Y - KNOB_Y_OFFSET,
        KNOB_SIZE,
        KNOB_SIZE,
        palette.border,
    );
}

fn draw_steps(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let marks = if scenario.preset_index == STEP_PRESET_INDEX {
        DENSE_STEP_MARK_COUNT
    } else {
        BASE_STEP_MARK_COUNT
    };
    for index in 0..marks {
        let mark_x = x + TRACK_X + index * (TRACK_WIDTH / (marks - 1));
        canvas.fill_rect(mark_x, y + STEP_Y, STEP_WIDTH, STEP_HEIGHT, palette.border);
    }
}

fn draw_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    for (index, row) in status_rows(scenario).into_iter().enumerate() {
        let row_x = x + STATUS_X + index * (STATUS_WIDTH + STATUS_GAP);
        canvas.fill_rect(
            row_x,
            y + STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            row_x,
            y + STATUS_Y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            row,
            row_x + TEXT_X,
            y + STATUS_Y + TEXT_Y,
            m::FONT_8,
            palette.muted,
        );
    }
}

fn value_for(scenario: ScenarioContext<'_>) -> usize {
    if scenario.screen_state.has_widget_action() || scenario.screen_state.has_settings_override() {
        return DRAG_VALUE;
    }
    match scenario.preset_index {
        DRAG_PRESET_INDEX => DRAG_VALUE,
        STEP_PRESET_INDEX => STEP_VALUE,
        THEME_PRESET_INDEX => THEME_VALUE,
        _ => BASE_VALUE,
    }
}

fn track_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == THEME_PRESET_INDEX {
        return palette.panel;
    }
    palette.surface
}

fn knob_color(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == THEME_PRESET_INDEX {
        return common::TOKEN;
    }
    palette.background
}

fn status_rows(scenario: ScenarioContext<'_>) -> [&'static str; STATUS_ROW_COUNT] {
    [
        action_label(scenario),
        event_label(scenario),
        state_label(scenario),
    ]
}

fn action_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "drag ready";
    }
    scenario.screen_state.last_action
}

fn event_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_event == "none" {
        return "event ready";
    }
    scenario.screen_state.last_event
}

fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label != "idle" {
        return scenario.screen_state.state_label;
    }
    match scenario.preset_index {
        DRAG_PRESET_INDEX => "value=64",
        STEP_PRESET_INDEX => "step=25",
        THEME_PRESET_INDEX => "theme knob",
        _ => "value=42",
    }
}
