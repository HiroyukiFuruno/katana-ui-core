use super::{
    CLICK_OFFSET, DRAG_AND_DROP_PAGE, StorybookLiveInteractionScenario,
    apply_clickable_keyboard_activation_for_audit, apply_drag_and_drop_drag_for_audit,
    apply_drag_and_drop_resize_for_audit, apply_drag_and_drop_scroll_for_audit, apply_hover_at,
    component_body_pixel_diff, focus_clickable_at_for_audit, page_state, render_state, scenario,
};

pub(super) fn drag_and_drop_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(DRAG_AND_DROP_PAGE);
    let before = render_state(DRAG_AND_DROP_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(DRAG_AND_DROP_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(DRAG_AND_DROP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(DRAG_AND_DROP_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "drag_hover_target"
        && state.screen_state.last_event == "drag_enter"
        && state.screen_state.state_label == "hover=target"
        && state.screen_state.drag_and_drop.hovered()
        && body_pixel_diff > 0;
    scenario(
        DRAG_AND_DROP_PAGE,
        "drag_hover_target",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn drag_and_drop_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(DRAG_AND_DROP_PAGE);
    let before = render_state(DRAG_AND_DROP_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(DRAG_AND_DROP_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(DRAG_AND_DROP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(DRAG_AND_DROP_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "drag_focus_source"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=source"
        && state.screen_state.drag_and_drop.focused()
        && body_pixel_diff > 0;
    scenario(
        DRAG_AND_DROP_PAGE,
        "drag_focus_source",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn drag_and_drop_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(DRAG_AND_DROP_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(DRAG_AND_DROP_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(DRAG_AND_DROP_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(DRAG_AND_DROP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(DRAG_AND_DROP_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "drag_keyboard_drop"
        && state.screen_state.last_event == "drag_end(committed=true)"
        && state.screen_state.state_label == "keyboard=drop"
        && state.screen_state.drag_and_drop.committed()
        && body_pixel_diff > 0;
    scenario(
        DRAG_AND_DROP_PAGE,
        "drag_keyboard_drop",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn drag_and_drop_drag_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(DRAG_AND_DROP_PAGE);
    let before = render_state(DRAG_AND_DROP_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(DRAG_AND_DROP_PAGE);
    let dragged = apply_drag_and_drop_drag_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(DRAG_AND_DROP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(DRAG_AND_DROP_PAGE, &before, &after);
    let passed = dragged
        && state.screen_state.last_action == "drag_start"
        && state.screen_state.last_event == "drag_start"
        && state.screen_state.drag_and_drop.is_dragging()
        && body_pixel_diff > 0;
    scenario(
        DRAG_AND_DROP_PAGE,
        "drag_start",
        "drag",
        dragged,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn drag_and_drop_scroll_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(DRAG_AND_DROP_PAGE);
    let before = render_state(DRAG_AND_DROP_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(DRAG_AND_DROP_PAGE);
    let scrolled = apply_drag_and_drop_scroll_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(DRAG_AND_DROP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(DRAG_AND_DROP_PAGE, &before, &after);
    let passed = scrolled
        && state.screen_state.last_action == "drag_autoscroll"
        && state.screen_state.last_event == "drag_autoscroll_requested"
        && state.screen_state.drag_and_drop.scroll_requested()
        && body_pixel_diff > 0;
    scenario(
        DRAG_AND_DROP_PAGE,
        "drag_autoscroll",
        "scroll",
        scrolled,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn drag_and_drop_resize_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(DRAG_AND_DROP_PAGE);
    let before = render_state(DRAG_AND_DROP_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(DRAG_AND_DROP_PAGE);
    let resized = apply_drag_and_drop_resize_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(DRAG_AND_DROP_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(DRAG_AND_DROP_PAGE, &before, &after);
    let passed = resized
        && state.screen_state.last_action == "drag_resize_target"
        && state.screen_state.last_event == "drag_target_resized"
        && state.screen_state.drag_and_drop.resized()
        && body_pixel_diff > 0;
    scenario(
        DRAG_AND_DROP_PAGE,
        "drag_resize_target",
        "resize",
        resized,
        passed,
        body_pixel_diff,
        &state,
    )
}
