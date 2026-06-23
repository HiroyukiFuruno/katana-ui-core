use super::{
    CLICK_OFFSET, StorybookLiveInteractionScenario, component_body_pixel_diff, page_state,
    render_state, scenario,
};
use crate::visual::dedicated_breadcrumb;
use crate::visual::preview_detail;
use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, apply_hover_at, focus_clickable_at_for_audit,
};

const PAGE: &str = "breadcrumb";

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != PAGE {
        return Vec::new();
    }
    vec![
        breadcrumb_hover_scenario(),
        breadcrumb_focus_scenario(),
        breadcrumb_keyboard_scenario(),
    ]
}

fn breadcrumb_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = breadcrumb_target();
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "breadcrumb_hover"
        && state.screen_state.last_event == "breadcrumb_hovered"
        && state.screen_state.state_label == "route=2"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "breadcrumb_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn breadcrumb_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = breadcrumb_target();
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "breadcrumb_focus"
        && state.screen_state.last_event == "breadcrumb_focused"
        && state.screen_state.state_label == "route=2"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "breadcrumb_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn breadcrumb_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let target = breadcrumb_target();
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "breadcrumb_click"
        && state.screen_state.last_event == "route_changed"
        && state.screen_state.state_label == "route=1"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "breadcrumb_keyboard_next",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn breadcrumb_target() -> super::LayoutRect {
    let component = preview_detail::component_action_hit_rect(PAGE);
    dedicated_breadcrumb::file_crumb_rect(component.x, component.y)
}
