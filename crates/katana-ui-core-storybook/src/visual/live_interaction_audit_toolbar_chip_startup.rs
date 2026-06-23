use super::{
    ATTACHMENT_CHIP_PAGE, CHIP_GROUP_PAGE, CLICK_OFFSET, STARTUP_STATE_PAGE,
    StorybookLiveInteractionScenario, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, component_body_pixel_diff, focus_clickable_at_for_audit, page_state,
    render_state, scenario,
};

pub(super) fn startup_state_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(STARTUP_STATE_PAGE);
    let before = render_state(STARTUP_STATE_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(STARTUP_STATE_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(STARTUP_STATE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(STARTUP_STATE_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "startup_state_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=retry"
        && state.screen_state.runtime_structured.startup_state.hovered
        && body_pixel_diff > 0;
    scenario(
        STARTUP_STATE_PAGE,
        "startup_state_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn startup_state_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(STARTUP_STATE_PAGE);
    let before = render_state(STARTUP_STATE_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(STARTUP_STATE_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(STARTUP_STATE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(STARTUP_STATE_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "startup_state_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=retry"
        && state.screen_state.is_button_focused()
        && state.screen_state.runtime_structured.startup_state.focused
        && body_pixel_diff > 0;
    scenario(
        STARTUP_STATE_PAGE,
        "startup_state_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn startup_state_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(STARTUP_STATE_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(STARTUP_STATE_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(STARTUP_STATE_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(STARTUP_STATE_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(STARTUP_STATE_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "startup_state_keyboard_retry"
        && state.screen_state.last_event == "startup_retried"
        && state.screen_state.state_label == "retry=requested"
        && state.screen_state.runtime_structured.startup_state.retried
        && body_pixel_diff > 0;
    scenario(
        STARTUP_STATE_PAGE,
        "startup_state_keyboard_retry",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn attachment_chip_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(ATTACHMENT_CHIP_PAGE);
    let before = render_state(ATTACHMENT_CHIP_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(ATTACHMENT_CHIP_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(ATTACHMENT_CHIP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(ATTACHMENT_CHIP_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "attachment_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=attachment"
        && state
            .screen_state
            .runtime_structured
            .attachment_chip
            .hovered
        && body_pixel_diff > 0;
    scenario(
        ATTACHMENT_CHIP_PAGE,
        "attachment_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn attachment_chip_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(ATTACHMENT_CHIP_PAGE);
    let before = render_state(ATTACHMENT_CHIP_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(ATTACHMENT_CHIP_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(ATTACHMENT_CHIP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(ATTACHMENT_CHIP_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "attachment_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=attachment"
        && state.screen_state.is_button_focused()
        && state
            .screen_state
            .runtime_structured
            .attachment_chip
            .focused
        && body_pixel_diff > 0;
    scenario(
        ATTACHMENT_CHIP_PAGE,
        "attachment_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn attachment_chip_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(ATTACHMENT_CHIP_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(ATTACHMENT_CHIP_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(ATTACHMENT_CHIP_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(ATTACHMENT_CHIP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(ATTACHMENT_CHIP_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "attachment_keyboard_retry"
        && state.screen_state.last_event == "attachment_retry"
        && state.screen_state.state_label == "retry=requested"
        && state
            .screen_state
            .runtime_structured
            .attachment_chip
            .retried
        && body_pixel_diff > 0;
    scenario(
        ATTACHMENT_CHIP_PAGE,
        "attachment_keyboard_retry",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn chip_group_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHIP_GROUP_PAGE);
    let before = render_state(CHIP_GROUP_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(CHIP_GROUP_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(CHIP_GROUP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CHIP_GROUP_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "chip_group_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=chip"
        && state.screen_state.runtime_structured.chip_group.hovered
        && body_pixel_diff > 0;
    scenario(
        CHIP_GROUP_PAGE,
        "chip_group_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn chip_group_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHIP_GROUP_PAGE);
    let before = render_state(CHIP_GROUP_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(CHIP_GROUP_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(CHIP_GROUP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CHIP_GROUP_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "chip_group_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=chip"
        && state.screen_state.is_button_focused()
        && state.screen_state.runtime_structured.chip_group.focused
        && body_pixel_diff > 0;
    scenario(
        CHIP_GROUP_PAGE,
        "chip_group_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn chip_group_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(CHIP_GROUP_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(CHIP_GROUP_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(CHIP_GROUP_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(CHIP_GROUP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(CHIP_GROUP_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "chip_group_keyboard_dismiss"
        && state.screen_state.last_event == "chip_group_chip_dismissed"
        && state.screen_state.state_label == "dismissed=focused"
        && state
            .screen_state
            .runtime_structured
            .chip_group
            .keyboard_dismissed
        && body_pixel_diff > 0;
    scenario(
        CHIP_GROUP_PAGE,
        "chip_group_keyboard_dismiss",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}
