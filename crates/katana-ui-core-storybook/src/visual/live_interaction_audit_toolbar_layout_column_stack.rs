use super::{
    CLICK_OFFSET, COLUMN_PAGE, STACK_PAGE, StorybookLiveInteractionScenario,
    apply_clickable_keyboard_activation_for_audit, apply_column_resize_for_audit, apply_hover_at,
    apply_stack_resize_for_audit, component_body_pixel_diff, focus_clickable_at_for_audit,
    page_state, render_state, scenario,
};

pub(super) fn column_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COLUMN_PAGE);
    let before = render_state(COLUMN_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(COLUMN_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(COLUMN_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COLUMN_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "column_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=column"
        && state.screen_state.preview_hovered
        && state.screen_state.layout.hovered()
        && body_pixel_diff > 0;
    scenario(
        COLUMN_PAGE,
        "column_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn column_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COLUMN_PAGE);
    let before = render_state(COLUMN_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(COLUMN_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(COLUMN_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COLUMN_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "column_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=column"
        && state.screen_state.is_button_focused()
        && state.screen_state.layout.focused()
        && body_pixel_diff > 0;
    scenario(
        COLUMN_PAGE,
        "column_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn column_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COLUMN_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(COLUMN_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(COLUMN_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(COLUMN_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COLUMN_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "column_keyboard_align"
        && state.screen_state.last_event == "layout_changed"
        && state.screen_state.state_label == "keyboard=align-center"
        && body_pixel_diff > 0;
    scenario(
        COLUMN_PAGE,
        "column_keyboard_align",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn column_resize_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COLUMN_PAGE);
    let before = render_state(COLUMN_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(COLUMN_PAGE);
    let resized =
        apply_column_resize_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(COLUMN_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COLUMN_PAGE, &before, &after);
    let passed = resized
        && state.screen_state.last_action == "column_resize"
        && state.screen_state.last_event == "layout_resized"
        && state.screen_state.state_label == "resize=column"
        && state.screen_state.layout.resized()
        && body_pixel_diff > 0;
    scenario(
        COLUMN_PAGE,
        "column_resize",
        "resize",
        resized,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn stack_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(STACK_PAGE);
    let before = render_state(STACK_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(STACK_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(STACK_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(STACK_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "stack_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=stack"
        && state.screen_state.preview_hovered
        && state.screen_state.layout.hovered()
        && body_pixel_diff > 0;
    scenario(
        STACK_PAGE,
        "stack_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn stack_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(STACK_PAGE);
    let before = render_state(STACK_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(STACK_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(STACK_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(STACK_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "stack_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=stack"
        && state.screen_state.is_button_focused()
        && state.screen_state.layout.focused()
        && body_pixel_diff > 0;
    scenario(
        STACK_PAGE,
        "stack_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn stack_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(STACK_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(STACK_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(STACK_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(STACK_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(STACK_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "stack_keyboard_reorder"
        && state.screen_state.last_event == "z_order_changed"
        && state.screen_state.state_label == "keyboard=z-order"
        && body_pixel_diff > 0;
    scenario(
        STACK_PAGE,
        "stack_keyboard_reorder",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn stack_resize_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(STACK_PAGE);
    let before = render_state(STACK_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(STACK_PAGE);
    let resized =
        apply_stack_resize_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(STACK_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(STACK_PAGE, &before, &after);
    let passed = resized
        && state.screen_state.last_action == "stack_resize"
        && state.screen_state.last_event == "layout_resized"
        && state.screen_state.state_label == "resize=stack"
        && state.screen_state.layout.resized()
        && body_pixel_diff > 0;
    scenario(
        STACK_PAGE,
        "stack_resize",
        "resize",
        resized,
        passed,
        body_pixel_diff,
        &state,
    )
}
