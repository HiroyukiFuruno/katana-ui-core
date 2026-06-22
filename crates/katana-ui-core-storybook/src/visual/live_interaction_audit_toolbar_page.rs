use super::{
    CLICK_OFFSET, LayoutRect, PAGE, SAVE_ACTION_INDEX, StorybookLiveInteractionScenario,
    apply_clickable_keyboard_activation_for_audit, apply_hover_at, component_body_pixel_diff,
    dedicated_toolbar, focus_clickable_at_for_audit, page_state, render_state, scenario,
};

pub(super) fn toolbar_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = toolbar_action_target();
    let hovered = apply_hover_at(
        &mut state,
        target.x + target.width / 2,
        target.y + target.height / 2,
    );
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.hovered_toolbar_action_index == Some(SAVE_ACTION_INDEX)
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "toolbar_action_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn toolbar_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PAGE);
    let before = render_state(PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "toolbar_focus"
        && state.screen_state.last_event == "toolbar_focused"
        && state.screen_state.state_label == "focus=save"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "toolbar_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn toolbar_action_target() -> LayoutRect {
    let component = crate::visual::preview_detail::component_action_hit_rect(PAGE);
    let action = dedicated_toolbar::action_rect_for_test(SAVE_ACTION_INDEX)
        .unwrap_or(LayoutRect::new(0, 0, 0, 0));
    LayoutRect::new(
        component.x + action.x,
        component.y + action.y,
        action.width,
        action.height,
    )
}

pub(super) fn toolbar_keyboard_scenario() -> StorybookLiveInteractionScenario {
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
        && state.screen_state.last_action == "tool_toggle"
        && state.screen_state.last_event == "tool_changed"
        && state.screen_state.state_label == "active=true"
        && body_pixel_diff > 0;
    scenario(
        PAGE,
        "toolbar_keyboard_activate",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}
