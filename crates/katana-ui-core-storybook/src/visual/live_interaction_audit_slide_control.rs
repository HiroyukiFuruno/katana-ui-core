use super::{
    CLICK_OFFSET, StorybookLiveInteractionScenario, component_body_pixel_diff, page_state,
    render_state, scenario,
};
use crate::visual::preview_detail;
use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, apply_slide_drag_for_audit,
    focus_clickable_at_for_audit,
};

const PAGE: &str = "slide-control";

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != PAGE {
        return Vec::new();
    }
    vec![
        slide_focus_scenario(),
        slide_keyboard_scenario(),
        slide_drag_scenario(),
    ]
}

fn slide_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && state.screen_state.is_button_focused()
        && state.screen_state.last_action == "slide_focus"
        && state.screen_state.last_event == "slide_focused"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "slide_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn slide_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(PAGE, &state);
    let typed = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && typed
        && state.screen_state.last_action == "slide_keyboard_increment"
        && state.screen_state.last_event == "slide_changed"
        && state.screen_state.state_label == "value=64"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "slide_keyboard_increment",
        "keyboard",
        typed,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn slide_drag_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let dragged =
        apply_slide_drag_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = dragged
        && state.screen_state.last_action == "slide_drag"
        && state.screen_state.last_event == "slide_changed"
        && state.screen_state.state_label == "value=64"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "slide_drag",
        "drag",
        dragged,
        passed,
        body_pixel_diff,
        &state,
    )
}
