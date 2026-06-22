use super::{
    CLICK_OFFSET, StorybookLiveInteractionScenario, component_body_pixel_diff, page_state,
    render_state, scenario,
};
use crate::visual::preview_detail;
use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, focus_clickable_at_for_audit,
};

const PAGE: &str = "card";

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != PAGE {
        return Vec::new();
    }
    vec![card_focus_scenario(), card_keyboard_scenario()]
}

fn card_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "card_focus"
        && state.screen_state.last_event == "card_focused"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "card_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn card_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "card_click"
        && state.screen_state.last_event == "card_activated"
        && state.screen_state.state_label == "active=true"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "card_keyboard_activate",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}
