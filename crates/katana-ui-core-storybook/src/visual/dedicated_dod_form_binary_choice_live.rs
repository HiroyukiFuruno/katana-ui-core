use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common};
use super::dedicated_dod_form_choice_marks as choice_marks;
use super::dedicated_dod_form_choice_status as choice_status;
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::{TextRenderer, TextVerticalBox};

const CHOICE_ROW_X: usize = 18;
const CHOICE_ROW_WIDTH: usize = 174;
const CHOICE_ROW_HEIGHT: usize = 22;
const CHOICE_ROW_GAP: usize = 10;
const CHOICE_LABEL_X: usize = 32;
#[cfg(test)]
const CHOICE_MARK_X: usize = 10;
#[cfg(test)]
const CHOICE_MARK_SIZE: usize = 12;
const CONTROL_BUTTON_Y: usize = 78;
const CONTROL_STATUS_Y: usize = choice_status::CHOICE_ROW_Y;
const CONTROL_HEIGHT: usize = 20;
const CONTROL_GAP: usize = 8;
const CONTROL_TEXT_Y: usize = 6;
const CONTROL_STATE_X: usize = 214;
const CONTROL_STATE_WIDTH: usize = 120;
const CONTROL_BUTTON_WIDTH: usize = 52;

pub(super) fn checkbox(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "Checkbox");
    draw_checkbox_rows(canvas, text, palette, scenario, x, y);
    draw_checkbox_controls(canvas, text, palette, scenario, x, y);
}

pub(super) fn radio(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "Radio");
    draw_radio_rows(canvas, text, palette, scenario, x, y);
    draw_radio_controls(canvas, text, palette, scenario, x, y);
}

fn draw_checkbox_rows(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let disabled = scenario.preset_index == m::PX_2;
    let checked = if scenario.preset_index == m::PX_0 || scenario.preset_index == m::PX_2 {
        scenario.screen_state.is_checkbox_checked()
    } else {
        true
    };
    for (index, label) in ["Markdown Linter", "Strict mode"].into_iter().enumerate() {
        let row = row_rect(index, x, y);
        draw_choice_row(canvas, text, palette, row, label, disabled);
        choice_marks::draw_checkbox_mark(canvas, palette, x, y, index, checked && index == 0);
    }
}

fn draw_checkbox_controls(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let read = checkbox_state_read_button_rect(x, y);
    let toggle = checkbox_toggle_button_rect(x, y);
    let reset = checkbox_reset_button_rect(x, y);
    for (rect, label) in [(read, "state read"), (toggle, "toggle"), (reset, "reset")] {
        canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.surface);
        canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, palette.border);
        text.draw(
            canvas,
            label,
            rect.x + m::PX_4,
            rect.y + CONTROL_TEXT_Y,
            m::FONT_8,
            palette.text,
        );
    }
    let checked_state = if scenario.screen_state.is_checkbox_checked() {
        "checked=true"
    } else {
        "checked=false"
    };
    draw_status_row(canvas, text, palette, checkbox_state_row_rect(x, y), checked_state);
    draw_status_row(
        canvas,
        text,
        palette,
        checkbox_event_row_rect(x, y),
        event_label(scenario),
    );
    draw_status_row(
        canvas,
        text,
        palette,
        checkbox_log_row_rect(x, y),
        state_log_label(scenario),
    );
}

fn draw_status_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    row: super::layout_metrics::LayoutRect,
    value: &str,
) {
    canvas.fill_rect(row.x, row.y, row.width, row.height, palette.panel);
    canvas.stroke_rect(row.x, row.y, row.width, row.height, palette.border);
    text.draw(canvas, value, row.x + m::PX_4, row.y + CONTROL_TEXT_Y, m::FONT_8, palette.muted);
}

fn event_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_event == "none" {
        return "event ready";
    }
    scenario.screen_state.last_event
}

fn state_log_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "before=false after=false";
    }
    scenario.screen_state.state_label
}

fn radio_state_log_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "before=false after=false";
    }
    scenario.screen_state.state_label
}

fn draw_radio_rows(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let selected = if scenario.preset_index == m::PX_0 {
        usize::from(scenario.screen_state.is_radio_selected())
    } else if scenario.screen_state.has_widget_action() {
        1
    } else {
        scenario.preset_index.min(1)
    };
    for (index, label) in ["Preview", "Code"].into_iter().enumerate() {
        draw_choice_row(canvas, text, palette, row_rect(index, x, y), label, false);
        choice_marks::draw_radio_mark(canvas, palette, x, y, index, index == selected);
    }
}

fn draw_radio_controls(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let read = radio_state_read_button_rect(x, y);
    let select = radio_select_button_rect(x, y);
    let reset = radio_reset_button_rect(x, y);
    for (rect, label) in [(read, "state read"), (select, "select"), (reset, "reset")] {
        canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.surface);
        canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, palette.border);
        text.draw(
            canvas,
            label,
            rect.x + m::PX_4,
            rect.y + CONTROL_TEXT_Y,
            m::FONT_8,
            palette.text,
        );
    }
    let selected_state = if scenario.screen_state.is_radio_selected() {
        "selected=true"
    } else {
        "selected=false"
    };
    draw_status_row(canvas, text, palette, radio_state_row_rect(x, y), selected_state);
    draw_status_row(canvas, text, palette, radio_event_row_rect(x, y), event_label(scenario));
    draw_status_row(
        canvas,
        text,
        palette,
        radio_log_row_rect(x, y),
        radio_state_log_label(scenario),
    );
}

fn draw_choice_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    row: super::layout_metrics::LayoutRect,
    label: &str,
    disabled: bool,
) {
    let text_color = if disabled {
        palette.muted
    } else {
        palette.text
    };
    canvas.fill_rect(row.x, row.y, row.width, row.height, palette.surface);
    canvas.stroke_rect(row.x, row.y, row.width, row.height, palette.border);
    text.draw_centered(
        canvas,
        label,
        row.x + CHOICE_LABEL_X,
        TextVerticalBox::new(row.y, CHOICE_ROW_HEIGHT as f32),
        m::FONT_9,
        text_color,
    );
}

fn row_rect(index: usize, x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    let row_y = y + choice_status::CHOICE_ROW_Y + index * (CHOICE_ROW_HEIGHT + CHOICE_ROW_GAP);
    super::layout_metrics::LayoutRect::new(
        x + CHOICE_ROW_X,
        row_y,
        CHOICE_ROW_WIDTH,
        CHOICE_ROW_HEIGHT,
    )
}

#[cfg(test)]
pub(super) fn checkbox_row_rect(
    index: usize,
    x: usize,
    y: usize,
) -> super::layout_metrics::LayoutRect {
    row_rect(index, x, y)
}

#[cfg(test)]
pub(super) fn checkbox_mark_rect(
    index: usize,
    x: usize,
    y: usize,
) -> super::layout_metrics::LayoutRect {
    let row = checkbox_row_rect(index, x, y);
    super::layout_metrics::LayoutRect::new(
        row.x + CHOICE_MARK_X,
        row.y + 5,
        CHOICE_MARK_SIZE,
        CHOICE_MARK_SIZE,
    )
}

#[cfg(test)]
pub(super) fn checkbox_label_rect(
    index: usize,
    x: usize,
    y: usize,
) -> super::layout_metrics::LayoutRect {
    let row = checkbox_row_rect(index, x, y);
    super::layout_metrics::LayoutRect::new(
        row.x + CHOICE_LABEL_X,
        row.y,
        CHOICE_ROW_WIDTH - CHOICE_LABEL_X - 6,
        CHOICE_ROW_HEIGHT,
    )
}

pub(super) fn checkbox_state_read_button_rect(
    x: usize,
    y: usize,
) -> super::layout_metrics::LayoutRect {
    super::layout_metrics::LayoutRect::new(
        x + CHOICE_ROW_X,
        y + CONTROL_BUTTON_Y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_HEIGHT,
    )
}

pub(super) fn checkbox_toggle_button_rect(
    x: usize,
    y: usize,
) -> super::layout_metrics::LayoutRect {
    let read = checkbox_state_read_button_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        read.right() + CONTROL_GAP,
        read.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_HEIGHT,
    )
}

pub(super) fn checkbox_reset_button_rect(
    x: usize,
    y: usize,
) -> super::layout_metrics::LayoutRect {
    let toggle = checkbox_toggle_button_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        toggle.right() + CONTROL_GAP,
        toggle.y,
        CONTROL_BUTTON_WIDTH,
        CONTROL_HEIGHT,
    )
}

pub(super) fn checkbox_state_row_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    super::layout_metrics::LayoutRect::new(
        x + CONTROL_STATE_X,
        y + CONTROL_STATUS_Y,
        CONTROL_STATE_WIDTH,
        CONTROL_HEIGHT,
    )
}

pub(super) fn checkbox_event_row_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    let state = checkbox_state_row_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        state.x,
        state.bottom() + CONTROL_GAP,
        state.width,
        state.height,
    )
}

pub(super) fn checkbox_log_row_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    let event = checkbox_event_row_rect(x, y);
    super::layout_metrics::LayoutRect::new(
        event.x,
        event.bottom() + CONTROL_GAP,
        event.width,
        event.height,
    )
}

#[cfg(test)]
pub(super) fn radio_row_rect(index: usize, x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    row_rect(index, x, y)
}

#[cfg(test)]
pub(super) fn radio_mark_rect(index: usize, x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    let row = radio_row_rect(index, x, y);
    super::layout_metrics::LayoutRect::new(
        row.x + CHOICE_MARK_X,
        row.y + 5,
        CHOICE_MARK_SIZE,
        CHOICE_MARK_SIZE,
    )
}

#[cfg(test)]
pub(super) fn radio_label_rect(index: usize, x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    let row = radio_row_rect(index, x, y);
    super::layout_metrics::LayoutRect::new(
        row.x + CHOICE_LABEL_X,
        row.y,
        CHOICE_ROW_WIDTH - CHOICE_LABEL_X - 6,
        CHOICE_ROW_HEIGHT,
    )
}

pub(super) fn radio_state_read_button_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    checkbox_state_read_button_rect(x, y)
}

pub(super) fn radio_select_button_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    checkbox_toggle_button_rect(x, y)
}

pub(super) fn radio_reset_button_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    checkbox_reset_button_rect(x, y)
}

pub(super) fn radio_state_row_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    checkbox_state_row_rect(x, y)
}

pub(super) fn radio_event_row_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    checkbox_event_row_rect(x, y)
}

pub(super) fn radio_log_row_rect(x: usize, y: usize) -> super::layout_metrics::LayoutRect {
    checkbox_log_row_rect(x, y)
}
