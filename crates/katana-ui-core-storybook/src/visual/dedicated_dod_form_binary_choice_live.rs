use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common};
pub(super) use super::dedicated_dod_form_binary_choice_layout::{
    CHOICE_LABEL_X, CHOICE_ROW_HEIGHT, CONTROL_TEXT_Y, checkbox_event_row_rect,
    checkbox_log_row_rect, checkbox_reset_button_rect, checkbox_state_read_button_rect,
    checkbox_state_row_rect, checkbox_toggle_button_rect, radio_event_row_rect, radio_log_row_rect,
    radio_reset_button_rect, radio_select_button_rect, radio_state_read_button_rect,
    radio_state_row_rect, row_rect,
};
#[cfg(test)]
pub(super) use super::dedicated_dod_form_binary_choice_layout::{
    checkbox_label_rect, checkbox_mark_rect, checkbox_row_rect, radio_label_rect, radio_mark_rect,
    radio_row_rect,
};
use super::dedicated_dod_form_choice_marks as choice_marks;
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::{TextRenderer, TextVerticalBox};

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
    let focused = scenario.preset_index == m::PX_3;
    let checked = if scenario.preset_index == m::PX_0 || scenario.preset_index == m::PX_2 {
        scenario.screen_state.is_checkbox_checked()
    } else {
        true
    };
    for (index, label) in ["Markdown Linter", "Strict mode"].into_iter().enumerate() {
        let row = row_rect(index, x, y);
        draw_choice_row(canvas, text, palette, row, label, disabled);
        if focused && index == m::PX_0 {
            canvas.stroke_rect(row.x, row.y, row.width, row.height, palette.accent);
        }
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
    draw_status_row(
        canvas,
        text,
        palette,
        checkbox_state_row_rect(x, y),
        checked_state,
    );
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
    text.draw(
        canvas,
        value,
        row.x + m::PX_4,
        row.y + CONTROL_TEXT_Y,
        m::FONT_8,
        palette.muted,
    );
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
    let selected = radio_selected_index(scenario);
    let focused = scenario.preset_index == m::PX_3;
    for (index, label) in ["Preview", "Code"].into_iter().enumerate() {
        let row = row_rect(index, x, y);
        draw_choice_row(canvas, text, palette, row, label, false);
        if selected == Some(index) || (focused && index == m::PX_1) {
            canvas.stroke_rect(row.x, row.y, row.width, row.height, palette.accent);
        }
        choice_marks::draw_radio_mark(canvas, palette, x, y, index, selected == Some(index));
    }
}

fn radio_selected_index(scenario: ScenarioContext<'_>) -> Option<usize> {
    if scenario.preset_index == m::PX_0 {
        return scenario.screen_state.is_radio_selected().then_some(m::PX_0);
    }
    if scenario.preset_index == m::PX_2 {
        return Some(m::PX_1);
    }
    Some(m::PX_0)
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
    draw_status_row(
        canvas,
        text,
        palette,
        radio_state_row_rect(x, y),
        selected_state,
    );
    draw_status_row(
        canvas,
        text,
        palette,
        radio_event_row_rect(x, y),
        event_label(scenario),
    );
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
