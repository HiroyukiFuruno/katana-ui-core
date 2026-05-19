use super::canvas::Canvas;
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) const CHOICE_ROW_Y: usize = 34;

const STATUS_WIDTH: usize = 120;
const STATUS_HEIGHT: usize = 20;
const STATUS_GAP: usize = 8;
const TEXT_X: usize = 10;
const TEXT_Y: usize = 6;
const CHOICE_STATUS_X: usize = 214;

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
    default_state: &'static str,
) {
    let rows = [
        action_label(scenario),
        event_label(scenario),
        state_or_default(scenario, default_state),
    ];
    for (index, row) in rows.into_iter().enumerate() {
        draw_row(canvas, text, palette, x, y, index, row);
    }
}

fn draw_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    index: usize,
    row: &str,
) {
    let row_y = y + CHOICE_ROW_Y + index * (STATUS_HEIGHT + STATUS_GAP);
    canvas.fill_rect(
        x + CHOICE_STATUS_X,
        row_y,
        STATUS_WIDTH,
        STATUS_HEIGHT,
        palette.panel,
    );
    canvas.stroke_rect(
        x + CHOICE_STATUS_X,
        row_y,
        STATUS_WIDTH,
        STATUS_HEIGHT,
        palette.border,
    );
    text.draw(
        canvas,
        row,
        x + CHOICE_STATUS_X + TEXT_X,
        row_y + TEXT_Y,
        m::FONT_8,
        palette.muted,
    );
}

fn action_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "action ready";
    }
    scenario.screen_state.last_action
}

fn event_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_event == "none" {
        return "event ready";
    }
    scenario.screen_state.last_event
}

fn state_or_default(scenario: ScenarioContext<'_>, default_state: &'static str) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return default_state;
    }
    scenario.screen_state.state_label
}
