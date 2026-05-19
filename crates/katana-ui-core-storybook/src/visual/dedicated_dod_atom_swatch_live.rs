use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

const DISABLED_BORDER: u32 = 0x2d2d30;
const SWATCH_X: usize = 18;
const SWATCH_Y: usize = 38;
const SWATCH_SIZE: usize = 28;
const SWATCH_GAP: usize = 10;
const SWATCH_COUNT: usize = 5;
const SWATCH_RING_OFFSET: usize = 3;
const STATUS_X: usize = 18;
const STATUS_Y: usize = 82;
const STATUS_WIDTH: usize = 96;
const STATUS_HEIGHT: usize = 20;
const STATUS_GAP: usize = 8;
const TEXT_X: usize = 7;
const TEXT_Y: usize = 6;
const SELECTED_INDEX: usize = 2;

pub(super) fn draw(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "ColorSwatch palette");
    draw_grid(canvas, palette, scenario, x, y);
    draw_status(canvas, text, palette, scenario, x, y);
}

fn draw_grid(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let selected_index = if scenario.screen_state.has_widget_action() {
        SELECTED_INDEX
    } else {
        m::PX_0
    };
    for (index, color) in colors(palette).into_iter().enumerate() {
        let swatch_x = x + SWATCH_X + index * (SWATCH_SIZE + SWATCH_GAP);
        if index == selected_index {
            canvas.stroke_rect(
                swatch_x - SWATCH_RING_OFFSET,
                y + SWATCH_Y - SWATCH_RING_OFFSET,
                SWATCH_SIZE + SWATCH_RING_OFFSET + SWATCH_RING_OFFSET,
                SWATCH_SIZE + SWATCH_RING_OFFSET + SWATCH_RING_OFFSET,
                common::WARN,
            );
        }
        canvas.fill_rect(swatch_x, y + SWATCH_Y, SWATCH_SIZE, SWATCH_SIZE, color);
    }
    canvas.stroke_rect(
        x + SWATCH_X + SWATCH_COUNT * (SWATCH_SIZE + SWATCH_GAP),
        y + SWATCH_Y,
        SWATCH_SIZE,
        SWATCH_SIZE,
        DISABLED_BORDER,
    );
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
        action_label(scenario),
        event_label(scenario),
        state_label(scenario),
    ];
    for (index, row) in rows.into_iter().enumerate() {
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

fn colors(palette: &VisualPalette) -> [u32; SWATCH_COUNT] {
    [
        palette.accent,
        common::SUCCESS,
        common::WARN,
        common::DANGER,
        common::PURPLE,
    ]
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

fn state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "color=accent";
    }
    scenario.screen_state.state_label
}
