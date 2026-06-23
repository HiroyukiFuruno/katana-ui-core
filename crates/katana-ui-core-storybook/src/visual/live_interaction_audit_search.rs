use super::{
    CLICK_OFFSET, StorybookLiveInteractionScenario, component_body_pixel_diff, page_state,
    render_state, scenario,
};
use crate::visual::dedicated_dod_form_input_live;
use crate::visual::layout_metrics::LayoutRect;
use crate::visual::preview_detail;
use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, apply_hover_at, focus_clickable_at_for_audit,
};

const SEARCH_BOX_PAGE: &str = "search-box";
const SEARCH_CONTROL_STRIP_PAGE: &str = "search-control-strip";

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    match page {
        SEARCH_BOX_PAGE => vec![search_box_focus_scenario(), search_box_keyboard_scenario()],
        SEARCH_CONTROL_STRIP_PAGE => vec![
            search_control_hover_scenario(),
            search_control_focus_scenario(),
            search_control_regex_scenario(),
            search_control_keyboard_scenario(),
        ],
        _ => Vec::new(),
    }
}

fn search_control_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SEARCH_CONTROL_STRIP_PAGE);
    let before = render_state(SEARCH_CONTROL_STRIP_PAGE, &state);
    let field = search_control_field();
    let hovered = apply_hover_at(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let after = render_state(SEARCH_CONTROL_STRIP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SEARCH_CONTROL_STRIP_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "search_control_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=true"
        && state.screen_state.search_control.hovered
        && body_pixel_diff > 0;
    scenario(
        SEARCH_CONTROL_STRIP_PAGE,
        "search_control_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn search_control_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SEARCH_CONTROL_STRIP_PAGE);
    let before = render_state(SEARCH_CONTROL_STRIP_PAGE, &state);
    let field = search_control_field();
    let focused =
        focus_clickable_at_for_audit(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let after = render_state(SEARCH_CONTROL_STRIP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SEARCH_CONTROL_STRIP_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "search_control_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && state.screen_state.search_control.focused
        && body_pixel_diff > 0;
    scenario(
        SEARCH_CONTROL_STRIP_PAGE,
        "search_control_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn search_control_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SEARCH_CONTROL_STRIP_PAGE);
    let field = search_control_field();
    let focused =
        focus_clickable_at_for_audit(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let before = render_state(SEARCH_CONTROL_STRIP_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(SEARCH_CONTROL_STRIP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SEARCH_CONTROL_STRIP_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "search_control_keyboard_next"
        && state.screen_state.last_event == "search_navigation_requested"
        && state.screen_state.state_label == "navigation=next"
        && state.screen_state.search_control.navigated_next
        && body_pixel_diff > 0;
    scenario(
        SEARCH_CONTROL_STRIP_PAGE,
        "search_control_keyboard_next",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn search_control_regex_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SEARCH_CONTROL_STRIP_PAGE);
    let before = render_state(SEARCH_CONTROL_STRIP_PAGE, &state);
    state.screen_state.register_search_control_action(
        crate::visual::screen_state_search_control::SearchControlScreenAction::ToggleRegex,
    );
    let after = render_state(SEARCH_CONTROL_STRIP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SEARCH_CONTROL_STRIP_PAGE, &before, &after);
    let passed = state.screen_state.last_action == "search_regex_toggle"
        && state.screen_state.last_event == "search_option_changed"
        && state.screen_state.state_label == "regex=true"
        && state.screen_state.search_control.regex_enabled
        && body_pixel_diff > 0;
    scenario(
        SEARCH_CONTROL_STRIP_PAGE,
        "search_control_regex_toggle",
        "click",
        true,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn search_box_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SEARCH_BOX_PAGE);
    let before = render_state(SEARCH_BOX_PAGE, &state);
    let field = search_box_field();
    let focused =
        focus_clickable_at_for_audit(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let after = render_state(SEARCH_BOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SEARCH_BOX_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "search_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        SEARCH_BOX_PAGE,
        "search_box_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn search_box_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SEARCH_BOX_PAGE);
    let field = search_box_field();
    let focused =
        focus_clickable_at_for_audit(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let before = render_state(SEARCH_BOX_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(SEARCH_BOX_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SEARCH_BOX_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "search_keyboard_submit"
        && state.screen_state.last_event == "search_submitted"
        && state.screen_state.state_label == "value=query submitted=true"
        && body_pixel_diff > 0;
    scenario(
        SEARCH_BOX_PAGE,
        "search_box_keyboard_submit",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn search_box_field() -> LayoutRect {
    let origin = preview_detail::component_action_hit_rect(SEARCH_BOX_PAGE);
    dedicated_dod_form_input_live::search_field_rect(origin.x, origin.y)
}

fn search_control_field() -> LayoutRect {
    preview_detail::component_action_hit_rect(SEARCH_CONTROL_STRIP_PAGE)
}
