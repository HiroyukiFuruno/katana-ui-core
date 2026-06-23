use super::{
    CLICK_OFFSET, StorybookLiveInteractionScenario, component_body_pixel_diff, page_state,
    render_state, scenario,
};
use crate::visual::preview_detail;
use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, apply_code_diff_scroll_sync_for_audit,
    apply_hover_at, focus_clickable_at_for_audit,
};

const PAGE: &str = "code-diff";

pub(super) fn scenarios(page: &'static str) -> Vec<StorybookLiveInteractionScenario> {
    if page != PAGE {
        return Vec::new();
    }
    vec![
        code_diff_hover_scenario(),
        code_diff_focus_scenario(),
        code_diff_keyboard_scenario(),
        code_diff_scroll_sync_scenario(),
    ]
}

fn code_diff_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "code_diff_hover"
        && state.screen_state.last_event == "code_diff_hovered"
        && state.screen_state.state_label == "hover=true"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "code_diff_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn code_diff_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "code_diff_focus"
        && state.screen_state.last_event == "code_diff_focused"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "code_diff_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn code_diff_keyboard_scenario() -> StorybookLiveInteractionScenario {
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
        && state.screen_state.last_action == "code_diff_expand"
        && state.screen_state.last_event == "code_diff_block_expanded"
        && state.screen_state.state_label == "collapsed=false"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "code_diff_keyboard_expand",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

fn code_diff_scroll_sync_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = preview_detail::component_action_hit_rect(PAGE);
    let scrolled = apply_code_diff_scroll_sync_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = scrolled
        && state.screen_state.last_action == "code_diff_scroll_sync"
        && state.screen_state.last_event == "code_diff_scroll_sync_changed"
        && state.screen_state.state_label == "scroll_sync=true"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "code_diff_scroll_sync",
        "scroll",
        scrolled,
        passed,
        body_pixel_diff,
        &state,
    )
}
