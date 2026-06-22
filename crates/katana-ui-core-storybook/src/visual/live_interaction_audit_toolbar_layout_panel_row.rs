use super::{
    CLICK_OFFSET, PANEL_PAGE, ROW_PAGE, StorybookLiveInteractionScenario,
    apply_clickable_keyboard_activation_for_audit, apply_hover_at, apply_panel_resize_for_audit,
    apply_row_resize_for_audit, component_body_pixel_diff, focus_clickable_at_for_audit,
    page_state, render_state, scenario,
};

pub(super) fn panel_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PANEL_PAGE);
    let before = render_state(PANEL_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(PANEL_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PANEL_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PANEL_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "panel_hover"
        && state.screen_state.last_event == "panel_hovered"
        && state.screen_state.state_label == "hover=navigation"
        && state.screen_state.preview_hovered
        && state.screen_state.panel.hovered
        && body_pixel_diff > 0;
    scenario(
        PANEL_PAGE,
        "panel_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn panel_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PANEL_PAGE);
    let before = render_state(PANEL_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(PANEL_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PANEL_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PANEL_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "panel_focus"
        && state.screen_state.last_event == "panel_focused"
        && state.screen_state.state_label == "focus=details"
        && state.screen_state.is_button_focused()
        && state.screen_state.panel.focused
        && body_pixel_diff > 0;
    scenario(
        PANEL_PAGE,
        "panel_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn panel_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PANEL_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(PANEL_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(PANEL_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(PANEL_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PANEL_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "panel_keyboard_scroll"
        && state.screen_state.last_event == "panel_scroll_changed"
        && state.screen_state.state_label == "keyboard_scroll=preview"
        && state.screen_state.panel.focused
        && body_pixel_diff > 0;
    scenario(
        PANEL_PAGE,
        "panel_keyboard_scroll",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn panel_resize_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(PANEL_PAGE);
    let before = render_state(PANEL_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(PANEL_PAGE);
    let resized =
        apply_panel_resize_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(PANEL_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(PANEL_PAGE, &before, &after);
    let passed = resized
        && state.screen_state.last_action == "panel_resize"
        && state.screen_state.last_event == "panel_resized"
        && state.screen_state.state_label == "resize=preview"
        && state.screen_state.panel.resized
        && body_pixel_diff > 0;
    scenario(
        PANEL_PAGE,
        "panel_resize",
        "resize",
        resized,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn row_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(ROW_PAGE);
    let before = render_state(ROW_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(ROW_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(ROW_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(ROW_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "row_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=row"
        && state.screen_state.preview_hovered
        && state.screen_state.layout.hovered()
        && body_pixel_diff > 0;
    scenario(
        ROW_PAGE,
        "row_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn row_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(ROW_PAGE);
    let before = render_state(ROW_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(ROW_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(ROW_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(ROW_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "row_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=row"
        && state.screen_state.is_button_focused()
        && state.screen_state.layout.focused()
        && body_pixel_diff > 0;
    scenario(
        ROW_PAGE,
        "row_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn row_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(ROW_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(ROW_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(ROW_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(ROW_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(ROW_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "row_keyboard_align"
        && state.screen_state.last_event == "layout_changed"
        && state.screen_state.state_label == "keyboard=align-center"
        && body_pixel_diff > 0;
    scenario(
        ROW_PAGE,
        "row_keyboard_align",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn row_resize_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(ROW_PAGE);
    let before = render_state(ROW_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(ROW_PAGE);
    let resized =
        apply_row_resize_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(ROW_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(ROW_PAGE, &before, &after);
    let passed = resized
        && state.screen_state.last_action == "row_resize"
        && state.screen_state.last_event == "layout_resized"
        && state.screen_state.state_label == "resize=row"
        && state.screen_state.layout.resized()
        && body_pixel_diff > 0;
    scenario(
        ROW_PAGE,
        "row_resize",
        "resize",
        resized,
        passed,
        body_pixel_diff,
        &state,
    )
}
