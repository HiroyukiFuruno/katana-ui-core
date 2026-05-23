use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::selection_control_metrics as sm;
use super::text::{TextRenderer, TextVerticalBox};

const FIELD: u32 = 0x1f242d;
const CONTROL_BUTTON_X: usize = sm::STATUS_X;
const CONTROL_BUTTON_Y: usize = 116;
const CONTROL_BUTTON_WIDTH: usize = 56;
const CONTROL_BUTTON_HEIGHT: usize = 20;
const CONTROL_BUTTON_GAP: usize = 8;
const CONTROL_TEXT_Y: usize = 6;
const LIGHT_OPTION_INDEX: usize = 1;
const DARK_OPTION_INDEX: usize = 2;
const SYSTEM_OPTION_INDEX: usize = 3;

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
    draw_controls(canvas, text, palette, x, y);
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
        x + sm::TRIGGER_X,
        y + sm::TRIGGER_Y,
        sm::TRIGGER_WIDTH,
        sm::TRIGGER_HEIGHT,
        FIELD,
    );
    canvas.stroke_rect(
        x + sm::TRIGGER_X,
        y + sm::TRIGGER_Y,
        sm::TRIGGER_WIDTH,
        sm::TRIGGER_HEIGHT,
        palette.border,
    );
    text.draw_centered(
        canvas,
        select_value(scenario),
        x + sm::TRIGGER_X + sm::TEXT_X,
        TextVerticalBox::new(y + sm::TRIGGER_Y, sm::TRIGGER_HEIGHT as f32),
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
    if !scenario.screen_state.selection.select_open {
        return;
    }
    canvas.fill_rect(
        x + sm::TRIGGER_X,
        y + sm::SELECT_OPTIONS_Y,
        sm::TRIGGER_WIDTH,
        sm::SELECT_OPTION_HEIGHT * sm::SELECT_OPTION_COUNT,
        palette.surface,
    );
    for (index, label) in ["Placeholder", "Light", "Dark", "System"]
        .into_iter()
        .enumerate()
    {
        let row_y = y + sm::SELECT_OPTIONS_Y + index * sm::SELECT_OPTION_HEIGHT;
        if scenario.screen_state.selection.select_selected_index == Some(index) {
            canvas.fill_rect(
                x + sm::TRIGGER_X + sm::OPTION_ROW_INSET,
                row_y,
                sm::TRIGGER_WIDTH - sm::OPTION_ROW_WIDTH_REDUCTION,
                sm::SELECT_OPTION_HEIGHT,
                palette.accent,
            );
        }
        text.draw(
            canvas,
            label,
            x + sm::TRIGGER_X + sm::TEXT_X,
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
        let row_y = y + sm::STATUS_Y + index * (sm::STATUS_HEIGHT + sm::STATUS_GAP);
        canvas.fill_rect(
            x + sm::STATUS_X,
            row_y,
            sm::STATUS_WIDTH,
            sm::STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            x + sm::STATUS_X,
            row_y,
            sm::STATUS_WIDTH,
            sm::STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            row,
            x + sm::STATUS_X + sm::TEXT_X,
            row_y + sm::TEXT_Y,
            m::FONT_8,
            palette.muted,
        );
    }
}

fn draw_controls(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    for (rect, label) in [
        (select_state_read_button_rect(x, y), "read"),
        (select_open_button_rect(x, y), "open"),
        (select_close_button_rect(x, y), "close"),
        (select_reset_button_rect(x, y), "reset"),
    ] {
        canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.surface);
        canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, palette.border);
        text.draw(
            canvas,
            label,
            rect.x + CONTROL_BUTTON_GAP,
            rect.y + CONTROL_TEXT_Y,
            m::FONT_8,
            palette.text,
        );
    }
}

fn select_value(scenario: ScenarioContext<'_>) -> &'static str {
    match scenario.screen_state.selection.select_selected_index {
        Some(LIGHT_OPTION_INDEX) => "Light",
        Some(DARK_OPTION_INDEX) => "Dark",
        Some(SYSTEM_OPTION_INDEX) => "System",
        _ => "Placeholder",
    }
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

pub(super) fn select_state_read_button_rect(
    x: usize,
    y: usize,
) -> super::layout_metrics::LayoutRect {
    super::layout_metrics::LayoutRect::new(
        x + CONTROL_BUTTON_X,
        y + CONTROL_BUTTON_Y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn select_open_button_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    let read = select_state_read_button_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        read.right() + CONTROL_BUTTON_GAP,
        read.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn select_close_button_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    let read = select_state_read_button_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        read.x,
        read.bottom() + CONTROL_BUTTON_GAP,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn select_reset_button_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    let close = select_close_button_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        close.right() + CONTROL_BUTTON_GAP,
        close.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}
