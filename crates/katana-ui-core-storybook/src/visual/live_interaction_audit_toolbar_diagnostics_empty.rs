use super::{
    CLICK_OFFSET, DIAGNOSTICS_LIST_PAGE, EMPTY_STATE_PAGE, StorybookLiveInteractionScenario,
    apply_clickable_keyboard_activation_for_audit, apply_diagnostics_list_scroll_for_audit,
    apply_hover_at, component_body_pixel_diff, focus_clickable_at_for_audit, page_state,
    render_state, scenario,
};

pub(super) fn diagnostics_list_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(DIAGNOSTICS_LIST_PAGE);
    let before = render_state(DIAGNOSTICS_LIST_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(DIAGNOSTICS_LIST_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(DIAGNOSTICS_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(DIAGNOSTICS_LIST_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "diagnostic_hover_item"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=syntax-error"
        && state.screen_state.preview_hovered
        && state.screen_state.diagnostics_list.hovered
        && body_pixel_diff > 0;
    scenario(
        DIAGNOSTICS_LIST_PAGE,
        "diagnostic_hover_item",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn diagnostics_list_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(DIAGNOSTICS_LIST_PAGE);
    let before = render_state(DIAGNOSTICS_LIST_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(DIAGNOSTICS_LIST_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(DIAGNOSTICS_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(DIAGNOSTICS_LIST_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "diagnostic_focus_list"
        && state.screen_state.last_event == "diagnostic_selected"
        && state.screen_state.state_label == "focus=syntax-error"
        && state.screen_state.is_button_focused()
        && state.screen_state.diagnostics_list.focused
        && body_pixel_diff > 0;
    scenario(
        DIAGNOSTICS_LIST_PAGE,
        "diagnostic_focus_list",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn diagnostics_list_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(DIAGNOSTICS_LIST_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(DIAGNOSTICS_LIST_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(DIAGNOSTICS_LIST_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(DIAGNOSTICS_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(DIAGNOSTICS_LIST_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "diagnostic_keyboard_navigate"
        && state.screen_state.last_event == "diagnostic_jump_requested"
        && state.screen_state.state_label == "jump=syntax-error"
        && state.screen_state.diagnostics_list.keyboard_navigated()
        && body_pixel_diff > 0;
    scenario(
        DIAGNOSTICS_LIST_PAGE,
        "diagnostic_keyboard_navigate",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn diagnostics_list_scroll_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(DIAGNOSTICS_LIST_PAGE);
    let before = render_state(DIAGNOSTICS_LIST_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(DIAGNOSTICS_LIST_PAGE);
    let scrolled = apply_diagnostics_list_scroll_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(DIAGNOSTICS_LIST_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(DIAGNOSTICS_LIST_PAGE, &before, &after);
    let passed = scrolled
        && state.screen_state.last_action == "diagnostic_scroll_retained"
        && state.screen_state.last_event == "diagnostic_visible_range_kept"
        && state.screen_state.state_label == "scroll=selection-retained"
        && state.screen_state.diagnostics_list.scroll_retained()
        && body_pixel_diff > 0;
    scenario(
        DIAGNOSTICS_LIST_PAGE,
        "diagnostic_scroll_retained",
        "scroll",
        scrolled,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn empty_state_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(EMPTY_STATE_PAGE);
    let before = render_state(EMPTY_STATE_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(EMPTY_STATE_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(EMPTY_STATE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(EMPTY_STATE_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "empty_state_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=primary"
        && state.screen_state.preview_hovered
        && body_pixel_diff > 0;
    scenario(
        EMPTY_STATE_PAGE,
        "empty_state_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn empty_state_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(EMPTY_STATE_PAGE);
    let before = render_state(EMPTY_STATE_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(EMPTY_STATE_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(EMPTY_STATE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(EMPTY_STATE_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "empty_state_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=primary"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        EMPTY_STATE_PAGE,
        "empty_state_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn empty_state_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(EMPTY_STATE_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(EMPTY_STATE_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(EMPTY_STATE_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(EMPTY_STATE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(EMPTY_STATE_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "empty_state_keyboard_primary"
        && state.screen_state.last_event == "empty_state_actioned"
        && state.screen_state.state_label == "keyboard=reload"
        && body_pixel_diff > 0;
    scenario(
        EMPTY_STATE_PAGE,
        "empty_state_keyboard_primary",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}
