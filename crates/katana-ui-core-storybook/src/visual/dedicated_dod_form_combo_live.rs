use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common};
pub(super) use super::dedicated_dod_form_combo_layout::{
    combo_filter_button_rect, combo_reset_button_rect, combo_select_button_rect,
    combo_state_read_button_rect,
};
use super::dedicated_dod_form_combo_model as combo_model;
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::selection_control_metrics as sm;
use super::text::{TextRenderer, TextVerticalBox};

const FILTER_BADGE_X: usize = 62;
const FILTER_BADGE_Y: usize = 7;
const FILTER_BADGE_WIDTH: usize = 28;
const FILTER_BADGE_HEIGHT: usize = 10;
const FILTER_BADGE_TEXT_X_OFFSET: usize = 5;
const FILTER_BADGE_TEXT_Y_OFFSET: usize = 2;
const COMBO_OPTION_TEXT_Y_OFFSET: usize = 5;
const CONTROL_BUTTON_GAP: usize = 8;
const CONTROL_TEXT_Y: usize = 6;
const FRAME_OUTSET: usize = 4;
const FRAME_GROW: usize = FRAME_OUTSET * 2;

pub(super) fn combo_box(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "ComboBox");
    draw_combo_frame(canvas, palette, scenario, x, y);
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
        combo_model::input_fill(palette, scenario),
    );
    canvas.stroke_rect(
        x + sm::TRIGGER_X,
        y + sm::TRIGGER_Y,
        sm::TRIGGER_WIDTH,
        sm::TRIGGER_HEIGHT,
        combo_model::input_border(palette, scenario),
    );
    text.draw_centered(
        canvas,
        combo_model::input_value(scenario),
        x + sm::TRIGGER_X + sm::TEXT_X,
        TextVerticalBox::new(y + sm::TRIGGER_Y, sm::TRIGGER_HEIGHT as f32),
        m::FONT_9,
        combo_model::input_text_color(palette, scenario),
    );
    if combo_model::filtered(scenario) {
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
        y + sm::TRIGGER_Y + FILTER_BADGE_Y + FILTER_BADGE_TEXT_Y_OFFSET,
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
    if !combo_model::open(scenario) {
        return;
    }
    let panel_y = y + combo_model::options_y(scenario);
    canvas.fill_rect(
        x + sm::TRIGGER_X,
        panel_y,
        sm::TRIGGER_WIDTH,
        sm::COMBO_OPTION_HEIGHT * combo_model::option_count(scenario),
        palette.surface,
    );
    for (index, label) in combo_model::option_labels(scenario).iter().enumerate() {
        let row_y = panel_y + index * sm::COMBO_OPTION_HEIGHT;
        if combo_model::highlighted_index(scenario) == Some(index) {
            canvas.fill_rect(
                x + sm::TRIGGER_X + sm::OPTION_ROW_INSET,
                row_y,
                sm::TRIGGER_WIDTH - sm::OPTION_ROW_WIDTH_REDUCTION,
                sm::COMBO_OPTION_HEIGHT,
                palette.selection,
            );
        }
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
    for (index, row) in combo_model::status_rows(scenario).into_iter().enumerate() {
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

fn draw_combo_frame(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    if !combo_model::framed(scenario) {
        return;
    }
    canvas.stroke_rect(
        x + sm::TRIGGER_X - FRAME_OUTSET,
        y + sm::TRIGGER_Y - FRAME_OUTSET,
        sm::TRIGGER_WIDTH + FRAME_GROW,
        sm::TRIGGER_HEIGHT + FRAME_GROW,
        palette.hover_border,
    );
}
