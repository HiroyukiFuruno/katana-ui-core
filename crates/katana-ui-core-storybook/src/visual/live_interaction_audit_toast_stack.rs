use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, focus_clickable_at_for_audit,
};

use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state, scenario,
};

const PAGE: &str = "toast-stack-manager";
const NOTIFICATION_TOAST_PAGE: &str = "notification-toast";
const CLICK_OFFSET: usize = 4;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    match page {
        PAGE => vec![toast_stack_focus_scenario()],
        NOTIFICATION_TOAST_PAGE => vec![
            notification_toast_focus_scenario(),
            notification_toast_keyboard_scenario(),
        ],
        _ => Vec::new(),
    }
}

fn toast_stack_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = super::preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "toast_stack_focus_pause"
        && state.screen_state.last_event == "toast_paused"
        && state.screen_state.state_label == "toast_stack.paused=true"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "toast_stack_focus_pause",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn notification_toast_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(NOTIFICATION_TOAST_PAGE);
    let before = render_state(NOTIFICATION_TOAST_PAGE, &state);
    let target = super::preview_detail::component_action_hit_rect(NOTIFICATION_TOAST_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(NOTIFICATION_TOAST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(NOTIFICATION_TOAST_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "toast_focus"
        && state.screen_state.last_event == "toast_focused"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        NOTIFICATION_TOAST_PAGE,
        "notification_toast_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn notification_toast_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(NOTIFICATION_TOAST_PAGE);
    let target = super::preview_detail::component_action_hit_rect(NOTIFICATION_TOAST_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(NOTIFICATION_TOAST_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(NOTIFICATION_TOAST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(NOTIFICATION_TOAST_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "toast_keyboard_dismiss"
        && state.screen_state.last_event == "toast_dismissed"
        && state.screen_state.state_label == "visible=false"
        && body_pixel_diff > 0;
    scenario(
        NOTIFICATION_TOAST_PAGE,
        "notification_toast_keyboard_dismiss",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}
