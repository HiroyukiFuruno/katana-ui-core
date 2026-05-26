use super::canvas::Canvas;
use super::dedicated_dod_atom_button_live;
use super::dedicated_dod_atom_swatch_live;
use super::dedicated_dod_common::{self as common, Rect};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::switch_control;
use super::text::{TextRenderer, TextVerticalBox};

const TOGGLE_ROW_X: usize = 18;
const TOGGLE_ROW_Y: usize = 36;
pub(super) const TOGGLE_ROW_WIDTH: usize = 294;
const TOGGLE_ROW_HEIGHT: usize = 34;
const TOGGLE_LABEL_X: usize = 14;
const TOGGLE_SWITCH_WIDTH: usize = 48;
const TOGGLE_SWITCH_HEIGHT: usize = 22;
const TOGGLE_SWITCH_RIGHT_INSET: usize = 14;
const TOGGLE_SWITCH_Y_INSET: usize = 6;
const TOGGLE_STATUS_X: usize = 18;
const TOGGLE_STATUS_Y: usize = 78;
const TOGGLE_STATUS_WIDTH: usize = 94;
const TOGGLE_STATUS_HEIGHT: usize = 20;
const TOGGLE_STATUS_GAP: usize = 8;
const TOGGLE_TEXT_X: usize = 8;
const TOGGLE_TEXT_Y: usize = 6;
const TOGGLE_STATUS_COUNT: usize = 3;

pub(super) fn button_matrix(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
    title: &str,
) {
    dedicated_dod_atom_button_live::draw(canvas, text, palette, scenario, x, y, title);
}
pub(super) fn toggle(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "Toggle switch");
    draw_toggle_row(canvas, text, palette, scenario, x, y);
    draw_toggle_status(canvas, text, palette, scenario, x, y);
}
pub(super) fn swatch(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    dedicated_dod_atom_swatch_live::draw(canvas, text, palette, scenario, x, y);
}

fn draw_toggle_row(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let on = scenario.screen_state.has_widget_action() || scenario.preset_index == m::PX_1;
    let disabled = scenario.preset_index == m::PX_2;
    let themed = scenario.preset_index == m::PX_3;
    let row = Rect::new(
        x + TOGGLE_ROW_X,
        y + TOGGLE_ROW_Y,
        TOGGLE_ROW_WIDTH,
        TOGGLE_ROW_HEIGHT,
    );
    let fill = if themed {
        palette.background
    } else {
        palette.surface
    };
    let border = if themed {
        palette.accent
    } else if disabled {
        palette.muted
    } else {
        palette.border
    };
    let text_color = if disabled {
        palette.muted
    } else {
        palette.text
    };
    canvas.fill_rect(row.x, row.y, row.width, row.height, fill);
    canvas.stroke_rect(row.x, row.y, row.width, row.height, border);
    text.draw_centered(
        canvas,
        "Markdown Linter",
        row.x + TOGGLE_LABEL_X,
        TextVerticalBox::new(row.y, row.height as f32),
        m::FONT_10,
        text_color,
    );
    switch_control::draw_switch(
        canvas,
        palette,
        row.x + row.width - TOGGLE_SWITCH_RIGHT_INSET - TOGGLE_SWITCH_WIDTH,
        row.y + TOGGLE_SWITCH_Y_INSET,
        TOGGLE_SWITCH_WIDTH,
        TOGGLE_SWITCH_HEIGHT,
        on,
    );
}

fn draw_toggle_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    for (index, row) in toggle_rows(scenario).into_iter().enumerate() {
        let row_x = x + TOGGLE_STATUS_X + index * (TOGGLE_STATUS_WIDTH + TOGGLE_STATUS_GAP);
        let row_y = y + TOGGLE_STATUS_Y;
        canvas.fill_rect(
            row_x,
            row_y,
            TOGGLE_STATUS_WIDTH,
            TOGGLE_STATUS_HEIGHT,
            palette.panel,
        );
        canvas.stroke_rect(
            row_x,
            row_y,
            TOGGLE_STATUS_WIDTH,
            TOGGLE_STATUS_HEIGHT,
            palette.border,
        );
        text.draw(
            canvas,
            row,
            row_x + TOGGLE_TEXT_X,
            row_y + TOGGLE_TEXT_Y,
            m::FONT_8,
            palette.muted,
        );
    }
}

fn toggle_rows(scenario: ScenarioContext<'_>) -> [&'static str; TOGGLE_STATUS_COUNT] {
    [
        toggle_action_label(scenario),
        toggle_event_label(scenario),
        toggle_state_label(scenario),
    ]
}

#[cfg(test)]
pub(super) const fn toggle_switch_rect_for_test() -> super::layout_metrics::LayoutRect {
    let row_x = super::preview_detail::HERO_PREVIEW_X_FOR_TEST + TOGGLE_ROW_X;
    let row_y = super::preview_detail::HERO_PREVIEW_Y_FOR_TEST + TOGGLE_ROW_Y;
    let switch_x = row_x + TOGGLE_ROW_WIDTH - TOGGLE_SWITCH_RIGHT_INSET - TOGGLE_SWITCH_WIDTH;
    super::layout_metrics::LayoutRect::new(
        switch_x,
        row_y + TOGGLE_SWITCH_Y_INSET,
        TOGGLE_SWITCH_WIDTH,
        TOGGLE_SWITCH_HEIGHT,
    )
}

fn toggle_action_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_action == "none" {
        return "action ready";
    }
    scenario.screen_state.last_action
}

fn toggle_event_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.last_event == "none" {
        return "event ready";
    }
    scenario.screen_state.last_event
}

fn toggle_state_label(scenario: ScenarioContext<'_>) -> &'static str {
    if scenario.screen_state.state_label == "idle" {
        return "checked=false";
    }
    scenario.screen_state.state_label
}
