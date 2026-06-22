use super::{
    ALIGN_CENTER_PAGE, CLICK_OFFSET, GRID_PAGE, StorybookLiveInteractionScenario,
    apply_align_center_resize_for_audit, apply_clickable_keyboard_activation_for_audit,
    apply_grid_resize_for_audit, apply_hover_at, component_body_pixel_diff,
    focus_clickable_at_for_audit, page_state, render_state, scenario,
};

pub(super) fn grid_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(GRID_PAGE);
    let before = render_state(GRID_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(GRID_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(GRID_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(GRID_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "grid_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=grid"
        && state.screen_state.preview_hovered
        && state.screen_state.layout.hovered()
        && body_pixel_diff > 0;
    scenario(
        GRID_PAGE,
        "grid_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn grid_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(GRID_PAGE);
    let before = render_state(GRID_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(GRID_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(GRID_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(GRID_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "grid_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=grid"
        && state.screen_state.is_button_focused()
        && state.screen_state.layout.focused()
        && body_pixel_diff > 0;
    scenario(
        GRID_PAGE,
        "grid_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn grid_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(GRID_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(GRID_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(GRID_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(GRID_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(GRID_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "grid_keyboard_select"
        && state.screen_state.last_event == "grid_cell_selected"
        && state.screen_state.state_label == "keyboard=cell"
        && body_pixel_diff > 0;
    scenario(
        GRID_PAGE,
        "grid_keyboard_select",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn grid_resize_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(GRID_PAGE);
    let before = render_state(GRID_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(GRID_PAGE);
    let resized =
        apply_grid_resize_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(GRID_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(GRID_PAGE, &before, &after);
    let passed = resized
        && state.screen_state.last_action == "grid_resize"
        && state.screen_state.last_event == "layout_resized"
        && state.screen_state.state_label == "resize=grid"
        && state.screen_state.layout.resized()
        && body_pixel_diff > 0;
    scenario(
        GRID_PAGE,
        "grid_resize",
        "resize",
        resized,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn align_center_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(ALIGN_CENTER_PAGE);
    let before = render_state(ALIGN_CENTER_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(ALIGN_CENTER_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(ALIGN_CENTER_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(ALIGN_CENTER_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "align_center_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=center"
        && state.screen_state.preview_hovered
        && state.screen_state.layout.hovered()
        && body_pixel_diff > 0;
    scenario(
        ALIGN_CENTER_PAGE,
        "align_center_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn align_center_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(ALIGN_CENTER_PAGE);
    let before = render_state(ALIGN_CENTER_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(ALIGN_CENTER_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(ALIGN_CENTER_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(ALIGN_CENTER_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "align_center_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=center"
        && state.screen_state.is_button_focused()
        && state.screen_state.layout.focused()
        && body_pixel_diff > 0;
    scenario(
        ALIGN_CENTER_PAGE,
        "align_center_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn align_center_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(ALIGN_CENTER_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(ALIGN_CENTER_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(ALIGN_CENTER_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(ALIGN_CENTER_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(ALIGN_CENTER_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "align_center_keyboard_measure"
        && state.screen_state.last_event == "alignment_changed"
        && state.screen_state.state_label == "keyboard=center"
        && body_pixel_diff > 0;
    scenario(
        ALIGN_CENTER_PAGE,
        "align_center_keyboard_measure",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn align_center_resize_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(ALIGN_CENTER_PAGE);
    let before = render_state(ALIGN_CENTER_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(ALIGN_CENTER_PAGE);
    let resized = apply_align_center_resize_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(ALIGN_CENTER_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(ALIGN_CENTER_PAGE, &before, &after);
    let passed = resized
        && state.screen_state.last_action == "align_center_resize"
        && state.screen_state.last_event == "layout_resized"
        && state.screen_state.state_label == "resize=center"
        && state.screen_state.layout.resized()
        && body_pixel_diff > 0;
    scenario(
        ALIGN_CENTER_PAGE,
        "align_center_resize",
        "resize",
        resized,
        passed,
        body_pixel_diff,
        &state,
    )
}
