use super::{
    BRANCH_SEGMENT_INDEX, CLICK_OFFSET, LayoutRect, STATUS_BAR_PAGE,
    StorybookLiveInteractionScenario, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, component_body_pixel_diff, dedicated_status_bar, focus_clickable_at_for_audit,
    page_state, render_state, scenario,
};

pub(super) fn status_bar_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(STATUS_BAR_PAGE);
    let before = render_state(STATUS_BAR_PAGE, &state);
    let target = status_bar_segment_target(BRANCH_SEGMENT_INDEX);
    let hovered = apply_hover_at(
        &mut state,
        target.x + target.width / 2,
        target.y + target.height / 2,
    );
    let after = render_state(STATUS_BAR_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(STATUS_BAR_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "status_bar_segment_hover"
        && state.screen_state.last_event == "status_bar_tooltip_shown"
        && state.screen_state.state_label == "tooltip=branch"
        && state.screen_state.status_bar_hovered_segment_index == Some(BRANCH_SEGMENT_INDEX)
        && body_pixel_diff > 0;
    scenario(
        STATUS_BAR_PAGE,
        "status_bar_segment_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn status_bar_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(STATUS_BAR_PAGE);
    let before = render_state(STATUS_BAR_PAGE, &state);
    let target = status_bar_segment_target(BRANCH_SEGMENT_INDEX);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(STATUS_BAR_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(STATUS_BAR_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "status_bar_segment_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=branch"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        STATUS_BAR_PAGE,
        "status_bar_segment_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn status_bar_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(STATUS_BAR_PAGE);
    let target = status_bar_segment_target(BRANCH_SEGMENT_INDEX);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(STATUS_BAR_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(STATUS_BAR_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(STATUS_BAR_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "status_bar_keyboard_activate"
        && state.screen_state.last_event == "status_bar_popover_opened"
        && state.screen_state.state_label == "open_popover=branch"
        && body_pixel_diff > 0;
    scenario(
        STATUS_BAR_PAGE,
        "status_bar_keyboard_activate",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn status_bar_segment_target(index: usize) -> LayoutRect {
    let component = crate::visual::preview_detail::component_action_hit_rect(STATUS_BAR_PAGE);
    let segment = dedicated_status_bar::segment_rect(index).unwrap_or(LayoutRect::new(0, 0, 0, 0));
    LayoutRect::new(
        component.x + segment.x,
        component.y + segment.y,
        segment.width,
        segment.height,
    )
}
