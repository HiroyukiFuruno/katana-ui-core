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
    choice_status::draw(canvas, text, palette, scenario, x, y, "checked=false");
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
    choice_status::draw(canvas, text, palette, scenario, x, y, "selected=none");
}

fn draw_checkbox_rows(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let checked = scenario.screen_state.has_widget_action() || scenario.preset_index == m::PX_1;
    for (index, label) in ["Markdown Linter", "Strict mode"].into_iter().enumerate() {
        draw_choice_row(canvas, text, palette, x, y, index, label);
        choice_marks::draw_checkbox_mark(canvas, palette, x, y, index, checked && index == 0);
    }
}

fn draw_radio_rows(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let selected = if scenario.screen_state.has_widget_action() {
        1
    } else {
        scenario.preset_index.min(1)
    };
    for (index, label) in ["Preview", "Code"].into_iter().enumerate() {
        draw_choice_row(canvas, text, palette, x, y, index, label);
        choice_marks::draw_radio_mark(canvas, palette, x, y, index, index == selected);
    }
}

fn draw_choice_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    index: usize,
    label: &str,
) {
    let row_y = y + choice_status::CHOICE_ROW_Y + index * (CHOICE_ROW_HEIGHT + CHOICE_ROW_GAP);
    canvas.fill_rect(
        x + CHOICE_ROW_X,
        row_y,
        CHOICE_ROW_WIDTH,
        CHOICE_ROW_HEIGHT,
        palette.surface,
    );
    canvas.stroke_rect(
        x + CHOICE_ROW_X,
        row_y,
        CHOICE_ROW_WIDTH,
        CHOICE_ROW_HEIGHT,
        palette.border,
    );
    text.draw_centered(
        canvas,
        label,
        x + CHOICE_ROW_X + CHOICE_LABEL_X,
        TextVerticalBox::new(row_y, CHOICE_ROW_HEIGHT as f32),
        m::FONT_9,
        palette.text,
    );
}
