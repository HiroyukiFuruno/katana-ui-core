use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::selection_control_metrics as sm;
use super::text::TextRenderer;

const ROW_LABELS: [&str; sm::SELECTION_LIST_ROW_COUNT] = ["First", "Second", "Third", "Fourth"];
const CONTROL_BUTTON_X: usize = sm::STATUS_X;
const CONTROL_BUTTON_Y: usize = 116;
const CONTROL_BUTTON_WIDTH: usize = 56;
const CONTROL_BUTTON_HEIGHT: usize = 20;
const CONTROL_BUTTON_GAP: usize = 8;

pub(super) fn selection_list(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "SelectionList");
    draw_rows(canvas, text, palette, scenario, x, y);
    draw_controls(canvas, text, palette, x, y);
    draw_status(canvas, text, palette, scenario, x, y);
}

fn draw_rows(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    canvas.fill_rect(
        x + sm::TRIGGER_X,
        y + sm::SELECTION_LIST_Y,
        sm::TRIGGER_WIDTH,
        sm::SELECTION_LIST_ROW_HEIGHT * ROW_LABELS.len(),
        palette.surface,
    );

    for (index, label) in ROW_LABELS.iter().enumerate() {
        let row_y = y + sm::SELECTION_LIST_Y + index * sm::SELECTION_LIST_ROW_HEIGHT;
        if scenario
            .screen_state
            .selection
            .selection_list_selected_index
            == Some(index)
        {
            canvas.fill_rect(
                x + sm::TRIGGER_X + sm::OPTION_ROW_INSET,
                row_y,
                sm::TRIGGER_WIDTH - sm::OPTION_ROW_WIDTH_REDUCTION,
                sm::SELECTION_LIST_ROW_HEIGHT,
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
        canvas.stroke_rect(
            x + sm::TRIGGER_X,
            row_y + sm::SELECTION_LIST_ROW_HEIGHT - 1,
            sm::TRIGGER_WIDTH,
            1,
            palette.border,
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
        return "single=none multi=none focus=none";
    }
    scenario.screen_state.state_label
}

fn draw_controls(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    for (rect, label) in [
        (selection_list_state_read_button_rect(x, y), "state"),
        (selection_list_select_row_button_rect(x, y), "select"),
        (selection_list_multi_toggle_button_rect(x, y), "multi"),
        (selection_list_keyboard_next_button_rect(x, y), "next"),
        (selection_list_reset_button_rect(x, y), "reset"),
    ] {
        canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.surface);
        canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, palette.border);
        text.draw(
            canvas,
            label,
            rect.x + CONTROL_BUTTON_GAP,
            rect.y + sm::TEXT_Y,
            m::FONT_8,
            palette.text,
        );
    }
}

pub(super) fn selection_list_state_read_button_rect(
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

pub(super) fn selection_list_select_row_button_rect(
    x: usize,
    y: usize,
) -> super::layout_metrics::LayoutRect {
    let state_read = selection_list_state_read_button_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        state_read.right() + CONTROL_BUTTON_GAP,
        state_read.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn selection_list_multi_toggle_button_rect(
    x: usize,
    y: usize,
) -> super::layout_metrics::LayoutRect {
    let state_read = selection_list_state_read_button_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        state_read.x,
        state_read.bottom() + CONTROL_BUTTON_GAP,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn selection_list_keyboard_next_button_rect(
    x: usize,
    y: usize,
) -> super::layout_metrics::LayoutRect {
    let multi = selection_list_multi_toggle_button_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        multi.x,
        multi.bottom() + CONTROL_BUTTON_GAP,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn selection_list_reset_button_rect(
    x: usize,
    y: usize,
) -> super::layout_metrics::LayoutRect {
    let multi = selection_list_multi_toggle_button_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        multi.right() + CONTROL_BUTTON_GAP,
        multi.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}
