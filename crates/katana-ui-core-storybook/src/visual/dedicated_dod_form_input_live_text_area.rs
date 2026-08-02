use super::{
    Canvas, LABEL_SIZE, STATUS_TEXT_X, STATUS_TEXT_Y, ScenarioContext, TEXT_AREA_LINE_FIRST_Y,
    TEXT_AREA_LINE_STEP, TEXT_AREA_LINE_X, TextRenderer, VisualPalette, common, m,
    text_area_caret_rect,
};

use super::dedicated_dod_form_input_live_text_area_chrome as chrome;
use super::dedicated_dod_form_input_live_text_area_content as content;
use super::dedicated_dod_form_input_live_text_area_slots as slots;

const TEXT_AREA_LINE_COUNT: usize = 4;
const HORIZONTAL_SCROLL_MAX_OFFSET: usize = 96;

pub(super) fn text_area(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "TextArea");
    let border = if scenario.screen_state.has_settings_override() {
        palette.accent
    } else if scenario.screen_state.preview_hovered {
        palette.hover_border
    } else {
        palette.border
    };
    let rect = chrome::text_area_rect_for_instance(
        x,
        y,
        scenario.screen_state,
        scenario.selected_instance_id,
    );
    canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.surface);
    canvas.stroke_rect(rect.x, rect.y, rect.width, rect.height, border);
    slots::draw_text_area_entry_slots(canvas, text, palette, scenario, x, y);
    draw_text_area_lines(canvas, text, palette, scenario, x, y);
    chrome::draw_vertical_scrollbar(canvas, palette, scenario, x, y);
    chrome::draw_horizontal_scrollbar(canvas, palette, scenario, x, y);
    chrome::draw_resize_grip(canvas, palette, scenario, x, y);
    draw_text_area_caret(canvas, text, palette, scenario, x, y);
    draw_text_area_status(canvas, text, palette, scenario, x, y);
}

fn draw_text_area_lines(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    let rect = chrome::text_area_rect_for_instance(
        x,
        y,
        scenario.screen_state,
        scenario.selected_instance_id,
    );
    canvas.with_clip(
        rect.x + 1,
        rect.y + 1,
        rect.width - 2,
        rect.height - 2,
        &mut |canvas| {
            for (index, line) in text_area_lines(scenario).iter().enumerate() {
                text.draw(
                    canvas,
                    line.as_str(),
                    (x + TEXT_AREA_LINE_X)
                        .saturating_add(slots::line_text_x_offset(scenario.preset_index))
                        .saturating_sub(chrome::horizontal_scroll_offset(scenario)),
                    y + TEXT_AREA_LINE_FIRST_Y + index * TEXT_AREA_LINE_STEP,
                    LABEL_SIZE,
                    palette.text,
                );
            }
        },
    );
}

fn draw_text_area_status(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    for (rect, row) in chrome::text_area_status_rects_for_instance(
        x,
        y,
        scenario.screen_state,
        scenario.selected_instance_id,
    )
    .into_iter()
    .zip(content::status_rows(scenario))
    {
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

fn draw_text_area_caret(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    if !scenario
        .screen_state
        .text_area_focused_for(scenario.selected_instance_id)
        || !scenario
            .screen_state
            .text_area_caret_visible_for(scenario.selected_instance_id)
    {
        return;
    }
    let value = scenario
        .screen_state
        .text_area_value_for(scenario.selected_instance_id);
    let line = value.lines().last().unwrap_or_default();
    let line_index = value.lines().count().saturating_sub(1);
    let value_width = text.measure_width(line, LABEL_SIZE);
    let rect = text_area_caret_rect(x, y, value_width, line_index);
    canvas.fill_rect(rect.x, rect.y, rect.width, rect.height, palette.accent);
}

fn text_area_lines(scenario: ScenarioContext<'_>) -> [String; TEXT_AREA_LINE_COUNT] {
    content::visible_lines(
        content::content_lines(scenario),
        chrome::vertical_scroll_offset(scenario),
    )
}

pub(in crate::visual) fn vertical_scroll_max_offset_for_instance(
    preset_index: usize,
    screen_state: &crate::visual::screen_state::StorybookScreenState,
    instance: &'static str,
) -> usize {
    content::line_count(preset_index, screen_state, instance).saturating_sub(TEXT_AREA_LINE_COUNT)
}

pub(in crate::visual) fn horizontal_scroll_max_offset_for_instance(
    preset_index: usize,
    screen_state: &crate::visual::screen_state::StorybookScreenState,
    instance: &'static str,
) -> usize {
    if chrome::horizontal_scroll_enabled_for_instance(preset_index, screen_state, instance) {
        return HORIZONTAL_SCROLL_MAX_OFFSET;
    }
    0
}
