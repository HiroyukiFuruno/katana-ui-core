use crate::visual::preview_detail;
use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, apply_hover_at, focus_clickable_at_for_audit,
};

use super::{
    CLICK_OFFSET, SHORTCUT_COMBO_PAGE, StorybookLiveInteractionScenario, component_body_pixel_diff,
    page_state, render_state, scenario,
};

pub(super) fn scenarios() -> Vec<StorybookLiveInteractionScenario> {
    vec![hover_scenario(), focus_scenario(), keyboard_scenario()]
}

fn hover_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SHORTCUT_COMBO_PAGE);
    let before = render_state(SHORTCUT_COMBO_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SHORTCUT_COMBO_PAGE);
    let hovered = apply_hover_at(
        &mut state,
        target.x + target.width / 2,
        target.y + target.height / 2,
    );
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

fn focus_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SHORTCUT_COMBO_PAGE);
    let before = render_state(SHORTCUT_COMBO_PAGE, &state);
    let target = preview_detail::component_action_hit_rect(SHORTCUT_COMBO_PAGE);
    let focused =
        focus_clickable_at_for_audit(&mut state, target.x + CLICK_OFFSET, target.y + CLICK_OFFSET);
    let after = render_state(SHORTCUT_COMBO_PAGE, &state);
    let body_pixel_diff = component_body_pixel_diff(SHORTCUT_COMBO_PAGE, &before, &after);
    let passed = focused
        && state.screen_state.last_action == "shortcut_combo_focus"
        && state.screen_state.last_event == "focus"
        && state.screen_state.state_label == "focus=true"
        && state.screen_state.is_button_focused()
        && state.screen_state.runtime_structured.shortcut_combo.focused
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

fn keyboard_scenario() -> StorybookLiveInteractionScenario {
    let mut state = page_state(SHORTCUT_COMBO_PAGE);
    let target = preview_detail::component_action_hit_rect(SHORTCUT_COMBO_PAGE);
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
