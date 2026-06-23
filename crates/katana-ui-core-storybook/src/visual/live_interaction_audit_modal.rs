use crate::visual::window_interaction::{
    apply_click, apply_clickable_keyboard_activation_for_audit, apply_context_click,
    apply_hover_at, focus_clickable_at_for_audit,
};

use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state, scenario,
};

const PAGE: &str = "modal";
const MODAL_OVERLAY_PAGE: &str = "modal-overlay";
const CLICK_OFFSET: usize = 4;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    match page {
        PAGE => vec![
            modal_focus_scenario(),
            modal_keyboard_escape_scenario(),
            modal_escape_removes_surface_scenario(),
            modal_escape_after_close_idempotent_scenario(),
        ],
        MODAL_OVERLAY_PAGE => vec![
            modal_overlay_pointer_scenario(),
            modal_overlay_hover_idempotent_scenario(),
            modal_overlay_focus_scenario(),
            modal_overlay_keyboard_escape_scenario(),
            modal_overlay_context_block_scenario(),
        ],
        _ => Vec::new(),
    }
}

fn modal_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = super::preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "modal_focus_trap"
        && state.screen_state.last_event == "modal_focused"
        && state.screen_state.state_label == "focus=trapped"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "modal_focus_trap",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn modal_keyboard_escape_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let target = super::preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "modal_escape"
        && state.screen_state.last_event == "modal_closed"
        && state.screen_state.state_label == "open=false"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "modal_keyboard_escape",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn modal_escape_removes_surface_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let target = super::preview_detail::component_action_hit_rect(PAGE);
    let before = render_state(PAGE, &state);
    let clicked = apply_click(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = clicked
        && !state.screen_state.modal_open
        && state.screen_state.last_action == "modal_escape"
        && state.screen_state.last_event == "modal_closed"
        && state.screen_state.state_label == "open=false"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "modal_escape_removes_surface",
        "pointer",
        clicked,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn modal_escape_after_close_idempotent_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let target = super::preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let closed_once = apply_clickable_keyboard_activation_for_audit(&mut state);
    let before_second_escape = render_state(PAGE, &state);
    let action_count = state.screen_state.action_count;
    let second_escape = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after_second_escape = render_state(PAGE, &state);
    let body_pixel_diff =
        component_body_pixel_diff(PAGE, &before_second_escape, &after_second_escape);
    let passed = focused
        && closed_once
        && !second_escape
        && state.screen_state.action_count == action_count
        && state.screen_state.last_action == "modal_escape"
        && state.screen_state.last_event == "modal_closed"
        && state.screen_state.state_label == "open=false"
        && body_pixel_diff == 0;
    scenario(
        PAGE,
        "modal_escape_after_close_idempotent",
        "keyboard",
        !second_escape,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn modal_overlay_pointer_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(MODAL_OVERLAY_PAGE);
    let before = render_state(MODAL_OVERLAY_PAGE, &state);
    let target = super::preview_detail::component_action_hit_rect(MODAL_OVERLAY_PAGE);
    let clicked = apply_click(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(MODAL_OVERLAY_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(MODAL_OVERLAY_PAGE, &before, &after);
    let passed = clicked
        && state.screen_state.last_action == "overlay_close"
        && state.screen_state.last_event == "overlay_closed"
        && state.screen_state.state_label == "open=false"
        && body_pixel_diff > 0;
    scenario(
        MODAL_OVERLAY_PAGE,
        "modal_overlay_backdrop_close",
        "pointer",
        clicked,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn modal_overlay_hover_idempotent_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(MODAL_OVERLAY_PAGE);
    let before = render_state(MODAL_OVERLAY_PAGE, &state);
    let target = super::preview_detail::component_action_hit_rect(MODAL_OVERLAY_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(MODAL_OVERLAY_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(MODAL_OVERLAY_PAGE, &before, &after);
    let action_count = state.screen_state.action_count;
    let repeated = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let passed = hovered
        && repeated
        && state.screen_state.action_count == action_count
        && state.screen_state.last_action == "modal_overlay_hover"
        && state.screen_state.last_event == "modal_overlay_hovered"
        && state.screen_state.state_label == "hover=true"
        && body_pixel_diff > 0;
    scenario(
        MODAL_OVERLAY_PAGE,
        "modal_overlay_hover_idempotent",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn modal_overlay_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(MODAL_OVERLAY_PAGE);
    let before = render_state(MODAL_OVERLAY_PAGE, &state);
    let target = super::preview_detail::component_action_hit_rect(MODAL_OVERLAY_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(MODAL_OVERLAY_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(MODAL_OVERLAY_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "modal_overlay_focus"
        && state.screen_state.last_event == "modal_overlay_focused"
        && state.screen_state.state_label == "focus=trapped"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        MODAL_OVERLAY_PAGE,
        "modal_overlay_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn modal_overlay_keyboard_escape_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(MODAL_OVERLAY_PAGE);
    let target = super::preview_detail::component_action_hit_rect(MODAL_OVERLAY_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(MODAL_OVERLAY_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(MODAL_OVERLAY_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(MODAL_OVERLAY_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "modal_overlay_escape"
        && state.screen_state.last_event == "modal_overlay_closed"
        && state.screen_state.state_label == "open=false"
        && body_pixel_diff > 0;
    scenario(
        MODAL_OVERLAY_PAGE,
        "modal_overlay_keyboard_escape",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn modal_overlay_context_block_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(MODAL_OVERLAY_PAGE);
    let before = render_state(MODAL_OVERLAY_PAGE, &state);
    let target = super::preview_detail::component_action_hit_rect(MODAL_OVERLAY_PAGE);
    let blocked = apply_context_click(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(MODAL_OVERLAY_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(MODAL_OVERLAY_PAGE, &before, &after);
    let passed = blocked
        && state.screen_state.last_action == "modal_overlay_context_block"
        && state.screen_state.last_event == "modal_overlay_context_ignored"
        && state.screen_state.state_label == "interaction=blocked"
        && body_pixel_diff > 0;
    scenario(
        MODAL_OVERLAY_PAGE,
        "modal_overlay_context_block",
        "context_menu",
        blocked,
        passed,
        body_pixel_diff,
        &state,
    )
}
