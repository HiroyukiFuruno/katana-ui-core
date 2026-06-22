use super::canvas::Canvas;
use super::dedicated_dod_common as common;
pub(super) use super::dedicated_dod_form_input_live_layout::{
    CLEAR_SIZE, CLEAR_X, CLEAR_Y, CONTROL_BUTTON_GAP, FIELD_ICON_X, FIELD_ICON_Y, FIELD_X,
    SEARCH_ICON_STEM_OFFSET, STATUS_TEXT_X, STATUS_TEXT_Y, TEXT_AREA_HEIGHT,
    TEXT_AREA_LINE_FIRST_Y, TEXT_AREA_LINE_STEP, TEXT_AREA_LINE_X, TEXT_AREA_WIDTH, TEXT_AREA_Y,
    search_case_toggle_button_rect, search_clear_button_rect, search_field_rect,
    search_inline_clear_rect, search_regex_toggle_button_rect, search_state_read_button_rect,
    search_submit_button_rect, search_type_query_button_rect, text_input_chip_rects,
    text_input_status_rects, text_input_trailing_icon_button_rects,
};
use super::dedicated_dod_metrics as m;
use super::palette::VisualPalette;
use super::render_context::ScenarioContext;
use super::text::TextRenderer;

#[path = "dedicated_dod_form_input_live_caret.rs"]
mod dedicated_dod_form_input_live_caret;
#[path = "dedicated_dod_form_input_live_chrome.rs"]
mod dedicated_dod_form_input_live_chrome;
#[path = "dedicated_dod_form_input_live_text_area.rs"]
mod dedicated_dod_form_input_live_text_area;
#[path = "dedicated_dod_form_input_live_text_area_chrome.rs"]
mod dedicated_dod_form_input_live_text_area_chrome;
#[path = "dedicated_dod_form_input_live_text_area_content.rs"]
mod dedicated_dod_form_input_live_text_area_content;
#[path = "dedicated_dod_form_input_live_text_area_geometry.rs"]
mod dedicated_dod_form_input_live_text_area_geometry;
#[path = "dedicated_dod_form_input_live_text_area_slots.rs"]
mod dedicated_dod_form_input_live_text_area_slots;
#[path = "dedicated_dod_form_input_live_text_input.rs"]
mod dedicated_dod_form_input_live_text_input;
#[path = "dedicated_dod_form_input_live_text_input_chrome.rs"]
mod dedicated_dod_form_input_live_text_input_chrome;
#[path = "dedicated_dod_form_input_live_text_input_clear.rs"]
mod dedicated_dod_form_input_live_text_input_clear;

const LABEL_SIZE: f32 = 10.0;
const CONTROL_TEXT_Y: usize = 6;
const INPUT_READONLY_PRESET_INDEX: usize = 2;
const INPUT_PLACEHOLDER_PRESET_INDEX: usize = 3;
const INPUT_RESERVED_SLOT_PRESET_INDEX: usize = 4;
const INPUT_LEADING_ICON_PRESET_INDEX: usize = 5;
const INPUT_ICON_BUTTONS_PRESET_INDEX: usize = 6;
const INPUT_INVALID_PRESET_INDEX: usize = 7;
const INPUT_THEME_PRESET_INDEX: usize = 8;

pub(super) use dedicated_dod_form_input_live_caret::text_area_caret_rect;

#[cfg(test)]
pub(super) use super::dedicated_dod_form_input_live_layout::FIELD_TEXT_X;
#[cfg(test)]
pub(super) use super::dedicated_dod_form_input_live_layout::FIELD_TEXT_X_WITH_LEADING_SLOT;
#[cfg(test)]
pub(super) use super::dedicated_dod_form_input_live_layout::{
    text_area_rect, text_area_status_rects,
};
#[cfg(test)]
pub(super) use super::dedicated_dod_form_input_live_layout::{
    text_input_text_clip_width, text_input_text_x,
};
#[cfg(test)]
pub(super) use dedicated_dod_form_input_live_caret::text_input_caret_rect as text_input_caret_rect_for_test;
#[cfg(test)]
pub(super) use dedicated_dod_form_input_live_caret::text_input_caret_rect_with_layout as text_input_caret_rect_with_layout_for_test;
pub(super) use dedicated_dod_form_input_live_text_area::{
    horizontal_scroll_max_offset_for_instance as text_area_horizontal_scroll_max_offset_for_instance,
    vertical_scroll_max_offset_for_instance as text_area_vertical_scroll_max_offset_for_instance,
};
#[cfg(test)]
pub(super) use dedicated_dod_form_input_live_text_area_chrome::text_area_rect_for_state as text_area_rect_for_screen_state;
pub(super) use dedicated_dod_form_input_live_text_area_chrome::{
    horizontal_scroll_enabled_for_instance as text_area_horizontal_scroll_enabled_for_instance,
    resize_enabled_for_instance as text_area_resize_enabled_for_instance,
    text_area_rect_for_instance as text_area_rect_for_screen_state_instance,
    text_area_resize_delta_for_pointer, text_area_resize_grip_rect_for_instance,
    vertical_scroll_enabled_for_instance as text_area_vertical_scroll_enabled_for_instance,
};
pub(super) use dedicated_dod_form_input_live_text_area_slots::{
    TEXT_AREA_CLEAR_ACTION_PRESET_INDEX, TEXT_AREA_TRAILING_BUTTONS_PRESET_INDEX,
    clear_action_rect as text_area_clear_action_rect, text_area_trailing_icon_button_rects,
};
#[cfg(test)]
pub(super) use dedicated_dod_form_input_live_text_input_chrome::search_icon_visual_rect_for_test as text_input_search_icon_visual_rect_for_test;
#[cfg(test)]
pub(super) use dedicated_dod_form_input_live_text_input_chrome::search_svg_fixture_for_test as text_input_search_svg_fixture_for_test;

pub(super) fn input(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    dedicated_dod_form_input_live_text_input::input(canvas, text, palette, scenario, x, y);
}

pub(super) fn text_area(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    dedicated_dod_form_input_live_text_area::text_area(canvas, text, palette, scenario, x, y);
}

pub(super) fn search(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    scenario: ScenarioContext<'_>,
    x: usize,
    y: usize,
) {
    common::frame(canvas, text, palette, x, y, "SearchBox");
    dedicated_dod_form_input_live_text_input::search_field(canvas, text, palette, scenario, x, y);
    dedicated_dod_form_input_live_chrome::draw_search_icon(canvas, palette, x, y);
    dedicated_dod_form_input_live_chrome::draw_clear_button(canvas, x, y);
    draw_search_controls(canvas, text, palette, x, y);
    dedicated_dod_form_input_live_text_input::draw_status(canvas, text, palette, scenario, x, y);
    dedicated_dod_form_input_live_text_input::draw_search_chips(canvas, text, palette, x, y);
}

fn draw_search_controls(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    x: usize,
    y: usize,
) {
    for (rect, label) in [
        (search_state_read_button_rect(x, y), "read"),
        (search_type_query_button_rect(x, y), "type"),
        (search_submit_button_rect(x, y), "submit"),
        (search_clear_button_rect(x, y), "clear"),
        (search_case_toggle_button_rect(x, y), "case"),
        (search_regex_toggle_button_rect(x, y), "regex"),
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
