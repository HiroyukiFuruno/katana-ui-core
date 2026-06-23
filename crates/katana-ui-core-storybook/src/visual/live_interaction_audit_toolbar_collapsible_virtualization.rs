use super::{
    CLICK_OFFSET, COLLAPSIBLE_PANEL_PAGE, StorybookLiveInteractionScenario, VIRTUALIZATION_PAGE,
    apply_clickable_keyboard_activation_for_audit, apply_context_click, apply_hover_at,
    apply_virtualization_scroll_for_audit, component_body_pixel_diff, focus_clickable_at_for_audit,
    page_state, render_state, scenario,
};

pub(super) fn collapsible_panel_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COLLAPSIBLE_PANEL_PAGE);
    let before = render_state(COLLAPSIBLE_PANEL_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(COLLAPSIBLE_PANEL_PAGE);
    let hovered = apply_hover_at(
        &mut state,
        target.x + target.width / 2,
        target.y + target.height / 2,
    );
    let after = render_state(COLLAPSIBLE_PANEL_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COLLAPSIBLE_PANEL_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "collapsible_panel_hover"
        && state.screen_state.last_event == "collapsible_panel_hover_expanded"
        && state.screen_state.state_label == "hover=expanded"
        && state.screen_state.preview_hovered
        && body_pixel_diff > 0;
    scenario(
        COLLAPSIBLE_PANEL_PAGE,
        "collapsible_panel_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn collapsible_panel_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COLLAPSIBLE_PANEL_PAGE);
    let before = render_state(COLLAPSIBLE_PANEL_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(COLLAPSIBLE_PANEL_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(COLLAPSIBLE_PANEL_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COLLAPSIBLE_PANEL_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "collapsible_panel_focus"
        && state.screen_state.last_event == "collapsible_panel_mode_changed"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && state.screen_state.collapsible_panel.focused
        && body_pixel_diff > 0;
    scenario(
        COLLAPSIBLE_PANEL_PAGE,
        "collapsible_panel_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn collapsible_panel_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COLLAPSIBLE_PANEL_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(COLLAPSIBLE_PANEL_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(COLLAPSIBLE_PANEL_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(COLLAPSIBLE_PANEL_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COLLAPSIBLE_PANEL_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "collapsible_panel_keyboard_toggle"
        && state.screen_state.last_event == "collapsible_panel_mode_changed"
        && state.screen_state.state_label == "mode=toggled"
        && body_pixel_diff > 0;
    scenario(
        COLLAPSIBLE_PANEL_PAGE,
        "collapsible_panel_keyboard_toggle",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn collapsible_panel_context_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(COLLAPSIBLE_PANEL_PAGE);
    let before = render_state(COLLAPSIBLE_PANEL_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(COLLAPSIBLE_PANEL_PAGE);
    let opened = apply_context_click(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(COLLAPSIBLE_PANEL_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(COLLAPSIBLE_PANEL_PAGE, &before, &after);
    let passed = opened
        && state.screen_state.last_action == "collapsible_panel_context_pin"
        && state.screen_state.last_event == "collapsible_panel_pin_changed"
        && state.screen_state.state_label == "pinned=false"
        && state.screen_state.collapsible_panel.context_open
        && body_pixel_diff > 0;
    scenario(
        COLLAPSIBLE_PANEL_PAGE,
        "collapsible_panel_context_pin",
        "context_menu",
        opened,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn virtualization_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(VIRTUALIZATION_PAGE);
    let before = render_state(VIRTUALIZATION_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(VIRTUALIZATION_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(VIRTUALIZATION_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(VIRTUALIZATION_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "virtualized_focus"
        && state.screen_state.last_event == "virtualized_focus_kept"
        && state.screen_state.state_label == "focus=42"
        && state.screen_state.is_button_focused()
        && state.screen_state.virtualization.focused
        && body_pixel_diff > 0;
    scenario(
        VIRTUALIZATION_PAGE,
        "virtualized_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn virtualization_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(VIRTUALIZATION_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(VIRTUALIZATION_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(VIRTUALIZATION_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(VIRTUALIZATION_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(VIRTUALIZATION_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "virtualized_keyboard_focus"
        && state.screen_state.last_event == "virtualized_focus_kept"
        && state.screen_state.state_label == "focus=43"
        && body_pixel_diff > 0;
    scenario(
        VIRTUALIZATION_PAGE,
        "virtualized_keyboard_focus",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn virtualization_scroll_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(VIRTUALIZATION_PAGE);
    let before = render_state(VIRTUALIZATION_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(VIRTUALIZATION_PAGE);
    let scrolled = apply_virtualization_scroll_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(VIRTUALIZATION_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(VIRTUALIZATION_PAGE, &before, &after);
    let passed = scrolled
        && state.screen_state.last_action == "virtualized_scroll"
        && state.screen_state.last_event == "virtual_range_changed"
        && state.screen_state.state_label == "rows=visible"
        && state.screen_state.virtualization.range.start > 0
        && body_pixel_diff > 0;
    scenario(
        VIRTUALIZATION_PAGE,
        "virtualized_scroll",
        "scroll",
        scrolled,
        passed,
        body_pixel_diff,
        &state,
    )
}
