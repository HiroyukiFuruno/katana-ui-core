use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, focus_clickable_at_for_audit,
};

use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state, scenario,
};

const PAGE: &str = "popover";
const HOVER_CARD_PAGE: &str = "hover-card";
const CLICK_OFFSET: usize = 4;

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    match page {
        PAGE => vec![popover_focus_scenario(), popover_keyboard_escape_scenario()],
        HOVER_CARD_PAGE => vec![hover_card_focus_scenario()],
        _ => Vec::new(),
    }
}

fn popover_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "popover_focus"
        && state.screen_state.last_event == "popover_focused"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "popover_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn popover_keyboard_escape_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "popover_keyboard_escape"
        && state.screen_state.last_event == "popover_closed"
        && state.screen_state.state_label == "open=false"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "popover_keyboard_escape",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn hover_card_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(HOVER_CARD_PAGE);
    let before = render_state(HOVER_CARD_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(HOVER_CARD_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(HOVER_CARD_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(HOVER_CARD_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "hover_card_focus"
        && state.screen_state.last_event == "hover_card_opened"
        && state.screen_state.state_label == "focus=true open=true"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        HOVER_CARD_PAGE,
        "hover_card_focus_open",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}
