use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common};
use super::dedicated_dod_form_binary_choice_chrome as choice_chrome;
pub(super) use super::dedicated_dod_form_binary_choice_layout::{
    BINARY_CHOICE_AREA_HEIGHT, CONTROL_TEXT_Y, checkbox_event_row_rect, checkbox_label_rect,
    checkbox_log_row_rect, checkbox_mark_rect, checkbox_reset_button_rect,
    checkbox_state_read_button_rect, checkbox_state_row_rect, checkbox_toggle_button_rect,
    radio_event_row_rect, radio_log_row_rect, radio_reset_button_rect, radio_select_button_rect,
    radio_state_read_button_rect, radio_state_row_rect, row_rect,
};
#[cfg(test)]
pub(super) use super::dedicated_dod_form_binary_choice_layout::{
    checkbox_row_rect, radio_label_rect, radio_mark_rect, radio_row_rect,
};
use super::dedicated_dod_form_choice_marks as choice_marks;
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

pub(super) fn checkbox(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame_with_height(
        canvas,
        text,
        palette,
        x,
        y,
        BINARY_CHOICE_AREA_HEIGHT,
        "Checkbox",
    );
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
    common::frame_with_height(
        canvas,
        text,
        palette,
        x,
        y,
        BINARY_CHOICE_AREA_HEIGHT,
        "Radio",
    );
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
    let disabled = scenario.screen_state.is_checkbox_disabled() || scenario.preset_index == m::PX_2;
    for (index, label) in ["Markdown Linter", "Strict mode"].into_iter().enumerate() {
        let focused = scenario.screen_state.is_checkbox_focused_at(index)
            || (scenario.preset_index == m::PX_3
                && index == scenario.screen_state.checkbox_focused_index());
        let checked = scenario.screen_state.is_checkbox_checked_at(index);
        let row = row_rect(index, x, y);
        let hovered_index = scenario.screen_state.checkbox_hovered_index();
        let hovered = hovered_index == Some(index)
            || (hovered_index.is_none()
                && scenario.screen_state.preview_hovered
                && index == m::PX_0);
        let border = choice_chrome::choice_row_border(palette, disabled, hovered, focused);
        choice_chrome::draw_choice_row_with_border(
            canvas, text, palette, row, label, disabled, border,
        );
        choice_marks::draw_checkbox_mark(canvas, palette, x, y, index, checked);
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
    let disabled = scenario.screen_state.is_checkbox_disabled() || scenario.preset_index == m::PX_2;
    let read = checkbox_state_read_button_rect(x, y);
    let toggle = checkbox_toggle_button_rect(x, y);
    let reset = checkbox_reset_button_rect(x, y);
    for (rect, label) in [(read, "state read"), (toggle, "toggle"), (reset, "reset")] {
        choice_chrome::draw_control_background(canvas, palette, rect);
        let text_color = if disabled {
            palette.muted
        } else {
            palette.text
        };
        text.draw(
            canvas,
            label,
            rect.x + m::PX_4,
            rect.y + CONTROL_TEXT_Y,
            m::FONT_13,
            text_color,
        );
    }
    let checked_state = if scenario
        .screen_state
        .is_checkbox_checked_at(scenario.screen_state.checkbox_focused_index())
    {
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
    choice_chrome::draw_status_background(canvas, palette, row);
    text.draw(
        canvas,
        value,
        row.x + m::PX_4,
        row.y + CONTROL_TEXT_Y,
        m::FONT_13,
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
    let focused = scenario.screen_state.is_radio_focused() || scenario.preset_index == m::PX_3;
    let focused_index = if scenario.screen_state.is_radio_focused() {
        m::PX_0
    } else {
        m::PX_1
    };
    let disabled = scenario.screen_state.is_radio_disabled();
    for (index, label) in ["Preview", "Code"].into_iter().enumerate() {
        let row = row_rect(index, x, y);
        let hovered = scenario.screen_state.preview_hovered && index == m::PX_0;
        let active = selected == Some(index) || (focused && index == focused_index);
        let border = choice_chrome::choice_row_border(palette, disabled, hovered, active);
        choice_chrome::draw_choice_row_with_border(
            canvas, text, palette, row, label, disabled, border,
        );
        choice_marks::draw_radio_mark(canvas, palette, x, y, index, selected == Some(index));
    }
}

fn radio_selected_index(scenario: ScenarioContext<'_>) -> Option<usize> {
    if scenario.screen_state.has_radio_selection() {
        return Some(scenario.screen_state.radio_selected_index());
    }
    if scenario.screen_state.is_radio_selected() {
        return Some(m::PX_0);
    }
    if scenario.preset_index == m::PX_0 {
        return None;
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
        choice_chrome::draw_control_background(canvas, palette, rect);
        text.draw(
            canvas,
            label,
            rect.x + m::PX_4,
            rect.y + CONTROL_TEXT_Y,
            m::FONT_13,
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
