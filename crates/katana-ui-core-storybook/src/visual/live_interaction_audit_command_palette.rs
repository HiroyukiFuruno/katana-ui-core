use super::{
    CLICK_OFFSET, StorybookLiveInteractionScenario, component_body_pixel_diff, page_state,
    render_state, scenario,
};
use crate::visual::layout_metrics::LayoutRect;
use crate::visual::preview_detail;
use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, apply_command_palette_escape_for_audit,
    apply_hover_at, focus_clickable_at_for_audit,
};

const COMMAND_PALETTE_PAGE: &str = "command-palette";

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    match page {
        COMMAND_PALETTE_PAGE => vec![
            command_palette_hover_scenario(),
            command_palette_focus_scenario(),
            command_palette_keyboard_execute_scenario(),
            command_palette_keyboard_close_scenario(),
        ],
        _ => Vec::new(),
    }
}

fn command_palette_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COMMAND_PALETTE_PAGE);
    let before = render_state(COMMAND_PALETTE_PAGE, &state);
    let field = command_palette_field();
    let hovered = apply_hover_at(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let after = render_state(COMMAND_PALETTE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COMMAND_PALETTE_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "command_palette_hover"
        && state.screen_state.last_event == "command_palette_hovered"
        && state.screen_state.state_label == "hover=true"
        && state.screen_state.preview_hovered
        && body_pixel_diff > 0;
    scenario(
        COMMAND_PALETTE_PAGE,
        "command_palette_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn command_palette_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COMMAND_PALETTE_PAGE);
    let before = render_state(COMMAND_PALETTE_PAGE, &state);
    let field = command_palette_field();
    let focused =
        focus_clickable_at_for_audit(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let after = render_state(COMMAND_PALETTE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COMMAND_PALETTE_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "command_palette_focus"
        && state.screen_state.last_event == "command_palette_focused"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        COMMAND_PALETTE_PAGE,
        "command_palette_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn command_palette_keyboard_execute_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COMMAND_PALETTE_PAGE);
    let field = command_palette_field();
    let focused =
        focus_clickable_at_for_audit(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let before = render_state(COMMAND_PALETTE_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(COMMAND_PALETTE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COMMAND_PALETTE_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "command_palette_keyboard_execute"
        && state.screen_state.last_event == "command_palette_result_executed"
        && state.screen_state.state_label == "executed=format"
        && body_pixel_diff > 0;
    scenario(
        COMMAND_PALETTE_PAGE,
        "command_palette_keyboard_execute",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn command_palette_keyboard_close_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COMMAND_PALETTE_PAGE);
    let field = command_palette_field();
    let focused =
        focus_clickable_at_for_audit(&mut state, field.x + CLICK_OFFSET, field.y + CLICK_OFFSET);
    let before = render_state(COMMAND_PALETTE_PAGE, &state);
    let closed = apply_command_palette_escape_for_audit(&mut state);
    let after = render_state(COMMAND_PALETTE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COMMAND_PALETTE_PAGE, &before, &after);
    let passed = focused
        && closed
        && state.screen_state.last_action == "command_palette_keyboard_close"
        && state.screen_state.last_event == "command_palette_closed"
        && state.screen_state.state_label == "closed=true"
        && body_pixel_diff > 0;
    scenario(
        COMMAND_PALETTE_PAGE,
        "command_palette_keyboard_close",
        "keyboard",
        closed,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn command_palette_field() -> LayoutRect {
    preview_detail::component_action_hit_rect(COMMAND_PALETTE_PAGE)
}
