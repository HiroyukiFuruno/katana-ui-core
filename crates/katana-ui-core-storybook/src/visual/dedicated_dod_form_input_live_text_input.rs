use super::super::dedicated_dod_common::ChipSpec;
use super::super::dedicated_dod_form_input_live_layout::{
    CHIP_LABEL_COUNT, FIELD_HEIGHT, FIELD_WIDTH, FIELD_Y, text_input_text_clip_width,
    text_input_text_x,
};
use super::super::dedicated_dod_form_input_live_values::{
    input_value, search_value, status_action, status_event, status_state,
};
use super::super::text::TextVerticalBox;
use super::dedicated_dod_form_input_live_text_input_chrome::{
    TextInputChrome, chrome_for_preset, draw_leading_slot, draw_trailing_icon_buttons,
};
use super::{
    Canvas, LABEL_SIZE, STATUS_TEXT_X, STATUS_TEXT_Y, ScenarioContext, TextRenderer, VisualPalette,
    common, m, text_input_chip_rects, text_input_status_rects,
};
use super::{INPUT_INVALID_PRESET_INDEX, INPUT_THEME_PRESET_INDEX};
use super::{
    dedicated_dod_form_input_live_caret, dedicated_dod_form_input_live_text_input_clear,
    search_field_rect,
};

const INPUT_CLEAR_ACTION_PRESET_INDEX: usize = 12;

pub(super) fn input(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "Input / TextInput");
    draw_input_field(
        canvas,
        text,
        palette,
        TextInputFieldSpec {
            scenario,
            x,
            y,
            value: input_value(scenario),
            chrome: chrome_for_preset(scenario.preset_index),
        },
    );
    draw_status(canvas, text, palette, scenario, x, y);
    draw_chips(canvas, text, palette, x, y, ["IME", "emoji", "invalid"]);
}

pub(super) fn search_field(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let value = search_value(scenario);
    draw_input_field(
        canvas,
        text,
        palette,
        TextInputFieldSpec {
            scenario,
            x,
            y,
            value,
            chrome: TextInputChrome::search(),
        },
    );
}

pub(super) fn draw_status(
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
    for (rect, row) in text_input_status_rects(x, y).into_iter().zip(rows) {
        canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.panel);
        canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, palette.border);
        text.draw(
            canvas,
            row,
            rect.x + STATUS_TEXT_X,
            rect.y + STATUS_TEXT_Y,
            m::FONT_8,
            palette.muted,
        );
    }
}

pub(super) fn draw_search_chips(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    draw_chips(canvas, text, palette, x, y, ["regex", "word", "case"]);
}

#[derive(Clone, Copy)]
struct TextInputFieldSpec<'a> {
    scenario: ScenarioContext<'a>,
    x: usize,
    y: usize,
    value: &'a str,
    chrome: TextInputChrome,
}

fn draw_input_field(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    field: TextInputFieldSpec<'_>,
) {
    let border = if field.scenario.preset_index == INPUT_INVALID_PRESET_INDEX {
        common::DANGER
    } else if field.scenario.screen_state.preview_hovered {
        palette.hover_border
    } else if field.scenario.screen_state.has_widget_action() {
        palette.accent
    } else {
        palette.border
    };
    let fill = if field.chrome.readonly {
        palette.code_background
    } else if field.scenario.preset_index == INPUT_THEME_PRESET_INDEX {
        palette.background
    } else {
        palette.surface
    };
    let rect = search_field_rect(field.x, field.y);
    canvas.fill_rect(rect.x, rect.y, FIELD_WIDTH, FIELD_HEIGHT, fill);
    canvas.stroke_rect(rect.x, rect.y, FIELD_WIDTH, FIELD_HEIGHT, border);
    draw_leading_slot(canvas, palette, field.x, field.y, field.chrome.leading_slot);
    draw_trailing_icon_buttons(
        canvas,
        text,
        palette,
        field.x,
        field.y,
        field.chrome.trailing_icon_buttons,
        field
            .scenario
            .screen_state
            .hovered_text_input_icon_button_index,
    );
    if clear_action_visible(field.scenario.preset_index) {
        dedicated_dod_form_input_live_text_input_clear::draw_clear_action(
            canvas,
            text,
            palette,
            field.x,
            field.y,
            field.scenario.screen_state.hovered_text_input_clear_action,
        );
    }
    draw_field_value(canvas, text, palette, field);
    draw_text_input_caret(canvas, text, palette, field);
}

fn draw_field_value(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    field: TextInputFieldSpec<'_>,
) {
    let text_x = text_input_text_x(field.x, field.chrome.leading_slot_reserved());
    let clip_width = text_input_text_clip_width(
        field.chrome.leading_slot_reserved(),
        field.chrome.trailing_icon_buttons,
        clear_action_visible(field.scenario.preset_index),
    );
    let (label, color) = if field.value.is_empty() {
        (field.chrome.placeholder.unwrap_or_default(), palette.muted)
    } else {
        (field.value, palette.text)
    };
    canvas.with_clip(
        text_x,
        field.y + FIELD_Y,
        clip_width,
        FIELD_HEIGHT,
        &mut |canvas| {
            text.draw_centered(
                canvas,
                label,
                text_x,
                TextVerticalBox::new(field.y + FIELD_Y, FIELD_HEIGHT as f32),
                LABEL_SIZE,
                color,
            );
        },
    );
}

fn draw_text_input_caret(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    field: TextInputFieldSpec<'_>,
) {
    if field.chrome.readonly {
        return;
    }
    if !field
        .scenario
        .screen_state
        .text_input_focused_for(field.scenario.selected_instance_id)
        || !field
            .scenario
            .screen_state
            .text_input_caret_visible_for(field.scenario.selected_instance_id)
    {
        return;
    }
    let value_width = text.measure_width(field.value, LABEL_SIZE);
    let rect = dedicated_dod_form_input_live_caret::text_input_caret_rect_with_layout(
        text_input_text_x(field.x, field.chrome.leading_slot_reserved()),
        field.y,
        text_input_text_clip_width(
            field.chrome.leading_slot_reserved(),
            field.chrome.trailing_icon_buttons,
            clear_action_visible(field.scenario.preset_index),
        ),
        value_width,
    );
    canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.accent);
}

fn draw_chips(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
    labels: [&'static str; CHIP_LABEL_COUNT],
) {
    let rects = text_input_chip_rects(x, y);
    common::draw_chips(
        canvas,
        text,
        palette,
        x,
        y,
        &[
            ChipSpec::new(
                rects[0].x - x,
                rects[0].y - y,
                rects[0].width,
                rects[0].height,
                labels[0],
                palette.accent,
            ),
            ChipSpec::new(
                rects[1].x - x,
                rects[1].y - y,
                rects[1].width,
                rects[1].height,
                labels[1],
                palette.panel,
            ),
            ChipSpec::new(
                rects[2].x - x,
                rects[2].y - y,
                rects[2].width,
                rects[2].height,
                labels[2],
                palette.code_background,
            ),
        ],
    );
}

fn clear_action_visible(preset_index: usize) -> bool {
    preset_index == INPUT_CLEAR_ACTION_PRESET_INDEX
}
