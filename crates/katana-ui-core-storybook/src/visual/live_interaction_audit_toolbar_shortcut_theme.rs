use super::{
    CLICK_OFFSET, SHORTCUT_COMBO_PAGE, StorybookLiveInteractionScenario, THEME_TOKENS_PAGE,
    apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    apply_theme_tokens_resize_for_audit, component_body_pixel_diff, focus_clickable_at_for_audit,
    page_state, render_state, scenario,
};

pub(super) fn shortcut_combo_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SHORTCUT_COMBO_PAGE);
    let before = render_state(SHORTCUT_COMBO_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(SHORTCUT_COMBO_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SHORTCUT_COMBO_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SHORTCUT_COMBO_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "shortcut_combo_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=true"
        && state.screen_state.runtime_structured.shortcut_combo.hovered
        && body_pixel_diff > 0;
    scenario(
        SHORTCUT_COMBO_PAGE,
        "shortcut_combo_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn shortcut_combo_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SHORTCUT_COMBO_PAGE);
    let before = render_state(SHORTCUT_COMBO_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(SHORTCUT_COMBO_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SHORTCUT_COMBO_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SHORTCUT_COMBO_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "shortcut_combo_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && body_pixel_diff > 0;
    scenario(
        SHORTCUT_COMBO_PAGE,
        "shortcut_combo_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn shortcut_combo_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SHORTCUT_COMBO_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(SHORTCUT_COMBO_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(SHORTCUT_COMBO_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(SHORTCUT_COMBO_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SHORTCUT_COMBO_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "shortcut_platform_preview"
        && state.screen_state.last_event == "shortcut_display_changed"
        && state.screen_state.state_label == "combo=Command+K"
        && state
            .screen_state
            .runtime_structured
            .shortcut_combo
            .platform_preview_macos
        && body_pixel_diff > 0;
    scenario(
        SHORTCUT_COMBO_PAGE,
        "shortcut_combo_keyboard_preview",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn theme_tokens_hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(THEME_TOKENS_PAGE);
    let before = render_state(THEME_TOKENS_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(THEME_TOKENS_PAGE);
    let hovered = apply_hover_at(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(THEME_TOKENS_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(THEME_TOKENS_PAGE, &before, &after);
    let passed = hovered
        && state.screen_state.last_action == "theme_token_hover"
        && state.screen_state.last_event == "hover_start"
        && state.screen_state.state_label == "hover=accent"
        && state.screen_state.preview_hovered
        && state.screen_state.theme_tokens.hovered()
        && body_pixel_diff > 0;
    scenario(
        THEME_TOKENS_PAGE,
        "theme_token_hover",
        "hover",
        hovered,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn theme_tokens_focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(THEME_TOKENS_PAGE);
    let before = render_state(THEME_TOKENS_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(THEME_TOKENS_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(THEME_TOKENS_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(THEME_TOKENS_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "theme_token_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=swatch"
        && state.screen_state.is_button_focused()
        && state.screen_state.theme_tokens.focused()
        && body_pixel_diff > 0;
    scenario(
        THEME_TOKENS_PAGE,
        "theme_token_focus",
        "focus",
        focused,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn theme_tokens_keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(THEME_TOKENS_PAGE);
    let target = crate::visual::preview_detail::component_action_hit_rect(THEME_TOKENS_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let before = render_state(THEME_TOKENS_PAGE, &state);
    let activated = apply_clickable_keyboard_activation_for_audit(&mut state);
    let after = render_state(THEME_TOKENS_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(THEME_TOKENS_PAGE, &before, &after);
    let passed = focused
        && activated
        && state.screen_state.last_action == "theme_token_keyboard_light"
        && state.screen_state.last_event == "theme_changed"
        && state.screen_state.state_label == "keyboard=light"
        && state.screen_state.theme_tokens.keyboard_selected_light()
        && body_pixel_diff > 0;
    scenario(
        THEME_TOKENS_PAGE,
        "theme_token_keyboard_light",
        "keyboard",
        activated,
        passed,
        body_pixel_diff,
        &state,
    )
}

pub(super) fn theme_tokens_resize_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(THEME_TOKENS_PAGE);
    let before = render_state(THEME_TOKENS_PAGE, &state);
    let target = crate::visual::preview_detail::component_action_hit_rect(THEME_TOKENS_PAGE);
    let resized = apply_theme_tokens_resize_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET,
    );
    let after = render_state(THEME_TOKENS_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(THEME_TOKENS_PAGE, &before, &after);
    let passed = resized
        && state.screen_state.last_action == "theme_token_resize_spacing"
        && state.screen_state.last_event == "theme_spacing_changed"
        && state.screen_state.state_label == "resize=spacing"
        && state.screen_state.theme_tokens.resized()
        && body_pixel_diff > 0;
    scenario(
        THEME_TOKENS_PAGE,
        "theme_token_resize_spacing",
        "resize",
        resized,
        passed,
        body_pixel_diff,
        &state,
    )
}
