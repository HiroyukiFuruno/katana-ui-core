use super::canvas::Canvas;
use super::dedicated_dod_common::{self as common};
pub(super) use super::dedicated_dod_form_selection_list_layout::{
    selection_list_keyboard_next_button_rect, selection_list_multi_toggle_button_rect,
    selection_list_reset_button_rect, selection_list_select_row_button_rect,
    selection_list_state_read_button_rect,
};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::selection_control_metrics as sm;
use super::text::TextRenderer;

const ROW_LABELS: [&str; sm::SELECTION_LIST_ROW_COUNT] = ["First", "Second", "Third", "Fourth"];
const CONTROL_BUTTON_GAP: usize = 8;
const ITEMS_PRESET_INDEX: usize = 0;
const SELECT_PRESET_INDEX: usize = 1;
const MULTI_PRESET_INDEX: usize = 2;
const THEME_PRESET_INDEX: usize = 3;
const VIRTUAL_PRESET_INDEX: usize = 4;
const SELECTED_ROW_INDEX: usize = 1;
const MULTI_FOCUS_INDEX: usize = 2;
const MULTI_PRESET_MASK: u8 = 0b0101;
const MULTI_MARK_X_OFFSET: usize = 146;
const MULTI_MARK_Y_OFFSET: usize = 4;
const MULTI_MARK_SIZE: usize = 6;

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
    let fill = list_surface(palette, scenario);
    canvas.fill_rect(
        x + sm::TRIGGER_X,
        y + sm::SELECTION_LIST_Y,
        sm::TRIGGER_WIDTH,
        sm::SELECTION_LIST_ROW_HEIGHT * ROW_LABELS.len(),
        fill,
    );

    for (index, label) in ROW_LABELS.iter().enumerate() {
        let row_y = y + sm::SELECTION_LIST_Y + index * sm::SELECTION_LIST_ROW_HEIGHT;
        draw_row_state(canvas, palette, scenario, x, row_y, index);
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
    if scenario.preset_index == THEME_PRESET_INDEX {
        canvas.stroke_rect(
            x + sm::TRIGGER_X,
            y + sm::SELECTION_LIST_Y,
            sm::TRIGGER_WIDTH,
            sm::SELECTION_LIST_ROW_HEIGHT * ROW_LABELS.len(),
            palette.accent,
        );
    }
}

fn draw_row_state(
    canvas: &mut Canvas,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    row_y: usize,
    index: usize,
) {
    if selection_list_selected_index(scenario) == Some(index) {
        canvas.fill_rect(
            x + sm::TRIGGER_X + sm::OPTION_ROW_INSET,
            row_y,
            sm::TRIGGER_WIDTH - sm::OPTION_ROW_WIDTH_REDUCTION,
            sm::SELECTION_LIST_ROW_HEIGHT,
            palette.accent,
        );
    }
    if selection_list_multi_mask(scenario) & (1u8 << index) != 0 {
        canvas.fill_rect(
            x + sm::TRIGGER_X + MULTI_MARK_X_OFFSET,
            row_y + MULTI_MARK_Y_OFFSET,
            MULTI_MARK_SIZE,
            MULTI_MARK_SIZE,
            palette.accent,
        );
    }
    if selection_list_focus_index(scenario) == Some(index) {
        canvas.stroke_rect(
            x + sm::TRIGGER_X + sm::OPTION_ROW_INSET,
            row_y,
            sm::TRIGGER_WIDTH - sm::OPTION_ROW_WIDTH_REDUCTION,
            sm::SELECTION_LIST_ROW_HEIGHT,
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
    if scenario.preset_index == VIRTUAL_PRESET_INDEX {
        return "virtual rows=1000";
    }
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

fn list_surface(palette: &VisualPalette, scenario: ScenarioContext<'_>) -> u32 {
    if scenario.preset_index == VIRTUAL_PRESET_INDEX {
        return common::TOKEN;
    }
    if scenario.preset_index == THEME_PRESET_INDEX {
        return palette.background;
    }
    palette.surface
}

fn selection_list_selected_index(scenario: ScenarioContext<'_>) -> Option<usize> {
    if scenario
        .screen_state
        .selection
        .selection_list_selected_index
        .is_some()
    {
        return scenario
            .screen_state
            .selection
            .selection_list_selected_index;
    }
    if scenario.preset_index == SELECT_PRESET_INDEX {
        return Some(SELECTED_ROW_INDEX);
    }
    None
}

fn selection_list_multi_mask(scenario: ScenarioContext<'_>) -> u8 {
    if scenario.screen_state.selection.selection_list_multi_mask != 0 {
        return scenario.screen_state.selection.selection_list_multi_mask;
    }
    if scenario.preset_index == MULTI_PRESET_INDEX {
        return MULTI_PRESET_MASK;
    }
    0
}

fn selection_list_focus_index(scenario: ScenarioContext<'_>) -> Option<usize> {
    if scenario
        .screen_state
        .selection
        .selection_list_focus_index
        .is_some()
    {
        return scenario.screen_state.selection.selection_list_focus_index;
    }
    match scenario.preset_index {
        SELECT_PRESET_INDEX => Some(SELECTED_ROW_INDEX),
        MULTI_PRESET_INDEX => Some(MULTI_FOCUS_INDEX),
        ITEMS_PRESET_INDEX | THEME_PRESET_INDEX | VIRTUAL_PRESET_INDEX => None,
        _ => None,
    }
}
