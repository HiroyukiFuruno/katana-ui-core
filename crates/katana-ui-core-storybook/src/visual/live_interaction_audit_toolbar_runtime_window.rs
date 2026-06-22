use super::{
    CLICK_OFFSET, MOTION_KEYBOARD_PHASE, MOTION_KEYBOARD_STATE_LABEL, MOTION_PAGE,
    SKELETON_CLUSTER_PAGE, StorybookLiveInteractionScenario, WINDOW_CONTROL_PAGE,
    apply_clickable_keyboard_activation_for_audit, apply_hover_at, component_body_pixel_diff,
    focus_clickable_at_for_audit, page_state, render_state, scenario,
};

pub(super) fn skeleton_cluster_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SKELETON_CLUSTER_PAGE);
    let before = render_state(SKELETON_CLUSTER_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(SKELETON_CLUSTER_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SKELETON_CLUSTER_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SKELETON_CLUSTER_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "skeleton_cluster_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=cluster"
        && state
            .screen_state
            .runtime_structured
            .skeleton_cluster
            .hovered
        && body_pixel_diff > 0;
    scenario(
        SKELETON_CLUSTER_PAGE,
        "skeleton_cluster_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn skeleton_cluster_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SKELETON_CLUSTER_PAGE);
    let before = render_state(SKELETON_CLUSTER_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(SKELETON_CLUSTER_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SKELETON_CLUSTER_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SKELETON_CLUSTER_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "skeleton_cluster_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=cluster"
        && state.screen_state.is_button_focused()
        && state
            .screen_state
            .runtime_structured
            .skeleton_cluster
            .focused
        && body_pixel_diff > 0;
    scenario(
        SKELETON_CLUSTER_PAGE,
        "skeleton_cluster_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn skeleton_cluster_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SKELETON_CLUSTER_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(SKELETON_CLUSTER_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(SKELETON_CLUSTER_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(SKELETON_CLUSTER_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SKELETON_CLUSTER_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "skeleton_cluster_keyboard_reduce_motion"
        && state.screen_state.last_event == "skeleton_reduced_motion_changed"
        && state.screen_state.state_label == "reduced_motion=true"
        && state
            .screen_state
            .runtime_structured
            .skeleton_cluster
            .keyboard_reduced_motion
        && body_pixel_diff > 0;
    scenario(
        SKELETON_CLUSTER_PAGE,
        "skeleton_cluster_keyboard_reduce_motion",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn motion_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(MOTION_PAGE);
    let before = render_state(MOTION_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(MOTION_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(MOTION_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(MOTION_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "motion_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=motion"
        && state.screen_state.runtime_structured.motion.hovered
        && body_pixel_diff > 0;
    scenario(
        MOTION_PAGE,
        "motion_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn motion_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(MOTION_PAGE);
    let before = render_state(MOTION_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(MOTION_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(MOTION_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(MOTION_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "motion_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=motion"
        && state.screen_state.is_button_focused()
        && state.screen_state.runtime_structured.motion.focused
        && body_pixel_diff > 0;
    scenario(
        MOTION_PAGE,
        "motion_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn motion_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(MOTION_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(MOTION_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(MOTION_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(MOTION_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(MOTION_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "motion_keyboard_tick"
        && state.screen_state.last_event == "motion_phase_changed"
        && state.screen_state.state_label == MOTION_KEYBOARD_STATE_LABEL
        && state.screen_state.runtime_structured.motion.keyboard_phase == MOTION_KEYBOARD_PHASE
        && body_pixel_diff > 0;
    scenario(
        MOTION_PAGE,
        "motion_keyboard_tick",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn window_control_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(WINDOW_CONTROL_PAGE);
    let before = render_state(WINDOW_CONTROL_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(WINDOW_CONTROL_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(WINDOW_CONTROL_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(WINDOW_CONTROL_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "window_control_hover"
        && state.screen_state.last_event == "window_control_visibility_changed"
        && state.screen_state.state_label == "visible=true"
        && state
            .screen_state
            .runtime_structured
            .window_control
            .hover_visible
        && body_pixel_diff > 0;
    scenario(
        WINDOW_CONTROL_PAGE,
        "window_control_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn window_control_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(WINDOW_CONTROL_PAGE);
    let before = render_state(WINDOW_CONTROL_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(WINDOW_CONTROL_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(WINDOW_CONTROL_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(WINDOW_CONTROL_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "window_control_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=Close"
        && state.screen_state.is_button_focused()
        && state.screen_state.runtime_structured.window_control.focused
        && body_pixel_diff > 0;
    scenario(
        WINDOW_CONTROL_PAGE,
        "window_control_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn window_control_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(WINDOW_CONTROL_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(WINDOW_CONTROL_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(WINDOW_CONTROL_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(WINDOW_CONTROL_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(WINDOW_CONTROL_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "window_control_keyboard_restore"
        && state.screen_state.last_event == "window_control_pressed"
        && state.screen_state.state_label == "pressed=Restore"
        && state
            .screen_state
            .runtime_structured
            .window_control
            .keyboard_restore
        && body_pixel_diff > 0;
    scenario(
        WINDOW_CONTROL_PAGE,
        "window_control_keyboard_restore",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}
