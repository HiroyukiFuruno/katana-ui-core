use super::{
    CLICK_OFFSET, StorybookLiveInteractionScenario, component_body_pixel_diff, page_state,
    render_state, scenario,
};
use crate::visual::dedicated_dod_form_input_live;
use crate::visual::dedicated_dod_form_input_live_layout;
use crate::visual::layout_metrics::LayoutRect;
use crate::visual::preview_detail;
use crate::visual::window_interaction::{
    StorybookWindowState, TextAreaKey, TextInputKey, apply_click, apply_hover_at,
    apply_text_area_key, apply_text_input_key, component_instance_id_for_page,
};

const TEXT_INPUT_PAGE: &str = "text-input";
const TEXT_AREA_PAGE: &str = "text-area";

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    match page {
        TEXT_INPUT_PAGE => vec![
            text_input_hover_scenario(),
            text_input_focus_scenario(),
            text_input_keyboard_scenario(),
        ],
        TEXT_AREA_PAGE => vec![
            text_area_hover_scenario(),
            text_area_focus_scenario(),
            text_area_keyboard_scenario(),
        ],
        _ => Vec::new(),
    }
}

fn text_input_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(TEXT_INPUT_PAGE);
    let before = render_state(TEXT_INPUT_PAGE, &state);
    let field = text_input_field();
    let hovered = apply_hover_at(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let after = render_state(TEXT_INPUT_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(TEXT_INPUT_PAGE, &before, &after);
    let passed = hovered && state.screen_state.preview_hovered && body_pixel_diff > 0;
    scenario(
        TEXT_INPUT_PAGE,
        "text_input_field_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn text_input_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(TEXT_INPUT_PAGE);
    let before = render_state(TEXT_INPUT_PAGE, &state);
    let field = text_input_field();
    let focused = apply_click(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let after = render_state(TEXT_INPUT_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(TEXT_INPUT_PAGE, &before, &after);
    let passed = focused
        && state
            .screen_state
            .text_input_focused_for(text_input_instance(&state))
        && state.screen_state.last_action == "text_input_focus"
        && state.screen_state.last_event == "text_input_focused"
        && body_pixel_diff > 0;
    scenario(
        TEXT_INPUT_PAGE,
        "text_input_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn text_input_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(TEXT_INPUT_PAGE);
    let field = text_input_field();
    let focused = apply_click(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let before = render_state(TEXT_INPUT_PAGE, &state);
    let typed = apply_text_input_key(&mut state, TextInputKey::Character('k'));
    let after = render_state(TEXT_INPUT_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(TEXT_INPUT_PAGE, &before, &after);
    let passed = focused
        && typed
        && state
            .screen_state
            .text_input_value_for(text_input_instance(&state))
            .ends_with('k')
        && state.screen_state.last_action == "text_input_type"
        && state.screen_state.last_event == "text_input_changed"
        && body_pixel_diff > 0;
    scenario(
        TEXT_INPUT_PAGE,
        "text_input_keyboard",
        "keyboard",
        typed,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn text_area_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(TEXT_AREA_PAGE);
    let before = render_state(TEXT_AREA_PAGE, &state);
    let field = text_area_field();
    let focused = apply_click(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let after = render_state(TEXT_AREA_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(TEXT_AREA_PAGE, &before, &after);
    let passed = focused
        && state
            .screen_state
            .text_area_focused_for(text_area_instance(&state))
        && state.screen_state.last_action == "text_area_focus"
        && state.screen_state.last_event == "text_area_focused"
        && body_pixel_diff > 0;
    scenario(
        TEXT_AREA_PAGE,
        "text_area_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn text_area_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(TEXT_AREA_PAGE);
    let before = render_state(TEXT_AREA_PAGE, &state);
    let field = text_area_field();
    let hovered = apply_hover_at(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let after = render_state(TEXT_AREA_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(TEXT_AREA_PAGE, &before, &after);
    let passed = hovered && state.screen_state.preview_hovered && body_pixel_diff > 0;
    scenario(
        TEXT_AREA_PAGE,
        "text_area_field_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn text_area_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(TEXT_AREA_PAGE);
    let field = text_area_field();
    let focused = apply_click(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let before = render_state(TEXT_AREA_PAGE, &state);
    let typed = apply_text_area_key(&mut state, TextAreaKey::Character('k'));
    let after = render_state(TEXT_AREA_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(TEXT_AREA_PAGE, &before, &after);
    let passed = focused
        && typed
        && state
            .screen_state
            .text_area_value_for(text_area_instance(&state))
            .ends_with('k')
        && state.screen_state.last_action == "text_area_type"
        && state.screen_state.last_event == "text_area_changed"
        && body_pixel_diff > 0;
    scenario(
        TEXT_AREA_PAGE,
        "text_area_keyboard",
        "keyboard",
        typed,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn text_input_field() -> LayoutRect {
    let origin = preview_detail::component_action_hit_rect(TEXT_INPUT_PAGE);
    dedicated_dod_form_input_live::search_field_rect(origin.x, origin.y)
}

fn text_area_field() -> LayoutRect {
    let origin = preview_detail::component_action_hit_rect(TEXT_AREA_PAGE);
    dedicated_dod_form_input_live_layout::text_area_rect(origin.x, origin.y)
}

fn text_input_instance(state: &StorybookWindowState) -> &'static str {
    component_instance_id_for_page(TEXT_INPUT_PAGE, state.selected_instance_id)
}

fn text_area_instance(state: &StorybookWindowState) -> &'static str {
    component_instance_id_for_page(TEXT_AREA_PAGE, state.selected_instance_id)
}
