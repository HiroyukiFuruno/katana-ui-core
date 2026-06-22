use crate::visual::preview_detail;
use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, apply_hover_at, apply_split_pane_drag_for_audit,
    apply_split_pane_resize_for_audit, focus_clickable_at_for_audit,
};

use super::{
    StorybookLiveInteractionScenario, component_body_pixel_diff, page_state, render_state, scenario,
};

const SPLIT_PANE_PAGE: &str = "split-pane";
const CLICK_OFFSET: usize = 4;
const SPLIT_PANE_KEYBOARD_RATIO_PERCENT: u8 = 58;
const SPLIT_PANE_KEYBOARD_STATE_LABEL: &str = "keyboard=58";

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    match page {
        SPLIT_PANE_PAGE => vec![
            split_pane_drag_scenario(),
            split_pane_hover_scenario(),
            split_pane_focus_scenario(),
            split_pane_keyboard_scenario(),
            split_pane_resize_scenario(),
        ],
        _ => Vec::new(),
    }
}

fn split_pane_drag_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SPLIT_PANE_PAGE);
    let before = render_state(SPLIT_PANE_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SPLIT_PANE_PAGE);
    let dragged = apply_split_pane_drag_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(SPLIT_PANE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SPLIT_PANE_PAGE, &before, &after);
    let passed = dragged
        && state.screen_state.last_action == "split_pane_drag_resize"
        && state.screen_state.last_event == "split_pane_ratio_changed"
        && state.screen_state.state_label == "ratio=64"
        && state.screen_state.split_pane.dragging()
        && body_pixel_diff > 0;
    scenario(
        SPLIT_PANE_PAGE,
        "split_pane_drag_resize",
        "drag",
        dragged,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn split_pane_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SPLIT_PANE_PAGE);
    let before = render_state(SPLIT_PANE_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SPLIT_PANE_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SPLIT_PANE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SPLIT_PANE_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "split_pane_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=handle"
        && state.screen_state.preview_hovered
        && state.screen_state.split_pane.hovered()
        && body_pixel_diff > 0;
    scenario(
        SPLIT_PANE_PAGE,
        "split_pane_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn split_pane_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SPLIT_PANE_PAGE);
    let before = render_state(SPLIT_PANE_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SPLIT_PANE_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SPLIT_PANE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SPLIT_PANE_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "split_pane_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=handle"
        && state.screen_state.is_button_focused()
        && state.screen_state.split_pane.focused()
        && body_pixel_diff > 0;
    scenario(
        SPLIT_PANE_PAGE,
        "split_pane_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn split_pane_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SPLIT_PANE_PAGE);
    let target = preview_detail::component_action_hit_rect(SPLIT_PANE_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(SPLIT_PANE_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(SPLIT_PANE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SPLIT_PANE_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "split_pane_keyboard_resize"
        && state.screen_state.last_event == "split_pane_ratio_changed"
        && state.screen_state.state_label == SPLIT_PANE_KEYBOARD_STATE_LABEL
        && state.screen_state.split_pane.ratio_percent() == SPLIT_PANE_KEYBOARD_RATIO_PERCENT
        && body_pixel_diff > 0;
    scenario(
        SPLIT_PANE_PAGE,
        "split_pane_keyboard_resize",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn split_pane_resize_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SPLIT_PANE_PAGE);
    let before = render_state(SPLIT_PANE_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SPLIT_PANE_PAGE);
    let resized = apply_split_pane_resize_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(SPLIT_PANE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SPLIT_PANE_PAGE, &before, &after);
    let passed = resized
        && state.screen_state.last_action == "split_pane_resize"
        && state.screen_state.last_event == "split_pane_ratio_changed"
        && state.screen_state.state_label == "resize=40"
        && state.screen_state.split_pane.resized()
        && body_pixel_diff > 0;
    scenario(
        SPLIT_PANE_PAGE,
        "split_pane_resize",
        "resize",
        resized,
        passed,
        body_pixel_diff,
        &state,
    )
}
