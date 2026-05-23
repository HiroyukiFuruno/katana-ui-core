use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::selection_control_metrics as sm;
use super::text::{TextRenderer, TextVerticalBox};

const FIELD: u32 = 0x1f242d;
const FILTER_BADGE_X: usize = 62;
const FILTER_BADGE_Y: usize = 7;
const FILTER_BADGE_WIDTH: usize = 28;
const FILTER_BADGE_HEIGHT: usize = 10;
const FILTER_BADGE_TEXT_X_OFFSET: usize = 5;
const COMBO_OPTION_TEXT_Y_OFFSET: usize = 5;
const STATUS_ROW_COUNT: usize = 3;
const CONTROL_BUTTON_X: usize = sm::STATUS_X;
const CONTROL_BUTTON_Y: usize = 116;
const CONTROL_BUTTON_WIDTH: usize = 56;
const CONTROL_BUTTON_HEIGHT: usize = 20;
const CONTROL_BUTTON_GAP: usize = 8;
const CONTROL_TEXT_Y: usize = 6;

pub(super) fn combo_box(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "ComboBox");
    draw_input(canvas, text, palette, scenario, x, y);
    draw_options(canvas, text, palette, scenario, x, y);
    draw_controls(canvas, text, palette, x, y);
    draw_status(canvas, text, palette, scenario, x, y);
}

fn draw_input(
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
        input_value(scenario),
        x + sm::TRIGGER_X + sm::TEXT_X,
        TextVerticalBox::new(y + sm::TRIGGER_Y, sm::TRIGGER_HEIGHT as f32),
        m::FONT_9,
        palette.text,
    );
    if scenario.screen_state.selection.combo_filtered {
        draw_filter_badge(canvas, text, palette, x, y);
    }
}

fn draw_filter_badge(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    canvas.fill_rect(
        x + sm::TRIGGER_X + FILTER_BADGE_X,
        y + sm::TRIGGER_Y + FILTER_BADGE_Y,
        FILTER_BADGE_WIDTH,
        FILTER_BADGE_HEIGHT,
        palette.accent,
    );
    text.draw(
        canvas,
        "tw",
        x + sm::TRIGGER_X + FILTER_BADGE_X + FILTER_BADGE_TEXT_X_OFFSET,
        y + sm::TRIGGER_Y + FILTER_BADGE_Y + 2,
        m::FONT_7,
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
    if !scenario.screen_state.selection.combo_open {
        return;
    }
    canvas.fill_rect(
        x + sm::TRIGGER_X,
        y + sm::COMBO_OPTIONS_Y,
        sm::TRIGGER_WIDTH,
        sm::COMBO_OPTION_HEIGHT * option_count(scenario),
        palette.surface,
    );
    for (index, label) in option_labels(scenario).iter().enumerate() {
        let row_y = y + sm::COMBO_OPTIONS_Y + index * sm::COMBO_OPTION_HEIGHT;
        text.draw(
            canvas,
            label,
            x + sm::TRIGGER_X + sm::TEXT_X,
            row_y + COMBO_OPTION_TEXT_Y_OFFSET,
            m::FONT_8,
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
    for (index, row) in status_rows(scenario).into_iter().enumerate() {
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

fn draw_controls(canvas: &mut Canvas, text: &TextRenderer, palette: &VisualPalette, x: usize, y: usize) {
    for (rect, label) in [
        (combo_state_read_button_rect(x, y), "read"),
        (combo_filter_button_rect(x, y), "filter"),
        (combo_select_button_rect(x, y), "select"),
        (combo_reset_button_rect(x, y), "reset"),
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

fn input_value(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.selection.combo_selected_index == Some(1) {
        return "Two";
    }
    if scenario.screen_state.selection.combo_filtered {
        return "tw";
    }
    "Type command"
}

fn option_labels(scenario: ScenarioContext<'_>) -> &'static [&'static str] {
    if scenario.screen_state.selection.combo_filtered {
        return &["Two"];
    }
    &["One", "Two"]
}

fn option_count(scenario: ScenarioContext<'_>) -> usize {
    option_labels(scenario).len()
}

fn status_rows(scenario: ScenarioContext<'_>) -> [&'static str; STATUS_ROW_COUNT] {
    [
        status_or_default(scenario.screen_state.last_action, "filter ready"),
        status_or_default(scenario.screen_state.last_event, "event ready"),
        status_or_default(scenario.screen_state.state_label, "query=empty"),
    ]
}

fn status_or_default(value: &'static str, default_value: &'static str) -> &'static str {
    if matches!(value, "none" | "idle") {
        return default_value;
    }
    value
}

pub(super) fn combo_state_read_button_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    super::layout_metrics::LayoutRect::new(
        x + CONTROL_BUTTON_X,
        y + CONTROL_BUTTON_Y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn combo_filter_button_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    let read = combo_state_read_button_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        read.right() + CONTROL_BUTTON_GAP,
        read.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn combo_select_button_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    let read = combo_state_read_button_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        read.x,
        read.bottom() + CONTROL_BUTTON_GAP,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}

pub(super) fn combo_reset_button_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    let select = combo_select_button_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        select.right() + CONTROL_BUTTON_GAP,
        select.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_BUTTON_HEIGHT,
    )
}
