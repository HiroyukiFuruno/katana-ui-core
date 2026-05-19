use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::{TextRenderer, TextVerticalBox};

const FIELD: u32 = 0x1f242d;
const TRIGGER_X: usize = 18;
const TRIGGER_Y: usize = 32;
const TRIGGER_WIDTH: usize = 166;
const TRIGGER_HEIGHT: usize = 28;
const OPTIONS_Y: usize = 60;
const OPTION_HEIGHT: usize = 14;
const OPTION_COUNT: usize = 4;
const OPTION_ROW_INSET: usize = 4;
const OPTION_ROW_WIDTH_REDUCTION: usize = 8;
const STATUS_X: usize = 204;
const STATUS_Y: usize = 36;
const STATUS_WIDTH: usize = 120;
const STATUS_HEIGHT: usize = 20;
const STATUS_GAP: usize = 8;
const TEXT_X: usize = 10;
const TEXT_Y: usize = 6;

pub(super) fn select_box(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "SelectBox");
    draw_trigger(canvas, text, palette, scenario, x, y);
    draw_options(canvas, text, palette, scenario, x, y);
    draw_status(canvas, text, palette, scenario, x, y);
}

fn draw_trigger(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    canvas.fill_rect(
        x + TRIGGER_X,
        y + TRIGGER_Y,
        TRIGGER_WIDTH,
        TRIGGER_HEIGHT,
        FIELD,
    );
    canvas.stroke_rect(
        x + TRIGGER_X,
        y + TRIGGER_Y,
        TRIGGER_WIDTH,
        TRIGGER_HEIGHT,
        palette.border,
    );
    text.draw_centered(
        canvas,
        select_value(scenario),
        x + TRIGGER_X + TEXT_X,
        TextVerticalBox::new(y + TRIGGER_Y, TRIGGER_HEIGHT as f32),
        m::FONT_9,
        palette.text,
    );
}

fn draw_options(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    canvas.fill_rect(
        x + TRIGGER_X,
        y + OPTIONS_Y,
        TRIGGER_WIDTH,
        OPTION_HEIGHT * OPTION_COUNT,
        palette.surface,
    );
    for (index, label) in ["Placeholder", "Light", "Dark", "System"]
        .into_iter()
        .enumerate()
    {
        let row_y = y + OPTIONS_Y + index * OPTION_HEIGHT;
        if label == select_value(scenario) {
            canvas.fill_rect(
                x + TRIGGER_X + OPTION_ROW_INSET,
                row_y,
                TRIGGER_WIDTH - OPTION_ROW_WIDTH_REDUCTION,
                OPTION_HEIGHT,
                palette.accent,
            );
        }
        text.draw(
            canvas,
            label,
            x + TRIGGER_X + TEXT_X,
            row_y + m::PX_3,
            m::FONT_7,
            palette.text,
        );
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
    let rows = [
        status_action(scenario),
        status_event(scenario),
        status_state(scenario),
    ];
    for (index, row) in rows.into_iter().enumerate() {
        let row_y = y + STATUS_Y + index * (STATUS_HEIGHT + STATUS_GAP);
        canvas.fill_rect(
            x + STATUS_X,
            row_y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            x + STATUS_X,
            row_y,
            STATUS_WIDTH,
            STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            row,
            x + STATUS_X + TEXT_X,
            row_y + TEXT_Y,
            m::FONT_8,
            palette.muted,
        );
    }
}

fn select_value(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.has_widget_action() {
        return "Dark";
    }
    "Placeholder"
}

fn status_action(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "action ready";
    }
    scenario.screen_state.last_action
}

fn status_event(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_event == "none" {
        return "event ready";
    }
    scenario.screen_state.last_event
}

fn status_state(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "selected=none";
    }
    scenario.screen_state.state_label
}
