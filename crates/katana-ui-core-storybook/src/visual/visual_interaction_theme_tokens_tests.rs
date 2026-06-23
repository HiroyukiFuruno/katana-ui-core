use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, apply_theme_tokens_resize_for_audit, focus_clickable_at_for_audit,
};

const PAGE: &str = "theme-tokens";

#[test]
fn theme_tokens_spec_uses_theme_id_option_contract_name() {
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert_eq!("theme_switch", spec.action);
    assert_eq!("theme_changed", spec.event);
    assert_eq!("theme.id", spec.option);
    assert_eq!("light", spec.after);
    assert_eq!("theme=light", spec.state);
}

#[test]
fn theme_tokens_theme_id_setting_changes_storybook_theme() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let row = super::layout_metrics::inspector_setting_row_hit_rect(0);

    assert_eq!("dark", state.theme_id);
    assert!(apply_click(&mut state, row.x + 1, row.y + 1));
    assert_eq!("light", state.theme_id);
    assert_eq!("theme.id", state.screen_state.last_setting);
    assert_eq!("settings_theme_id", state.screen_state.last_action);
    assert_eq!("theme_settings_changed", state.screen_state.last_event);
}

#[test]
fn theme_tokens_live_operations_update_token_state_and_body() {
    assert_theme_tokens_live_operation(
        "theme_token_hover",
        apply_hover_at,
        "theme_token_hover",
        "hover_start",
        "hover=accent",
    );
    assert_theme_tokens_live_operation(
        "theme_token_focus",
        focus_clickable_at_for_audit,
        "theme_token_focus",
        "focus",
        "focus=swatch",
    );

    let mut keyboard_state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let keyboard_target = super::preview_detail::component_action_hit_rect(PAGE);
    assert!(focus_clickable_at_for_audit(
        &mut keyboard_state,
        keyboard_target.x + 1,
        keyboard_target.y + 1,
    ));
    let before = render_theme_tokens_state(&keyboard_state);
    assert!(apply_clickable_keyboard_activation_for_audit(
        &mut keyboard_state
    ));
    assert_eq!(
        "theme_token_keyboard_light",
        keyboard_state.screen_state.last_action
    );
    assert_eq!("theme_changed", keyboard_state.screen_state.last_event);
    assert_eq!("keyboard=light", keyboard_state.screen_state.state_label);
    let after = render_theme_tokens_state(&keyboard_state);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > 0);

    assert_theme_tokens_live_operation(
        "theme_token_resize",
        apply_theme_tokens_resize_for_audit,
        "theme_token_resize_spacing",
        "theme_spacing_changed",
        "resize=spacing",
    );
}

fn assert_theme_tokens_live_operation(
    label: &str,
    operation: impl FnOnce(&mut StorybookWindowState, usize, usize) -> bool,
    expected_action: &str,
    expected_event: &str,
    expected_state: &str,
) {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let before = render_theme_tokens_state(&state);
    let target = super::preview_detail::component_action_hit_rect(PAGE);

    assert!(
        operation(&mut state, target.x + 1, target.y + 1),
        "{label} should be handled"
    );
    assert_eq!(
        expected_action, state.screen_state.last_action,
        "{label} action"
    );
    assert_eq!(
        expected_event, state.screen_state.last_event,
        "{label} event"
    );
    assert_eq!(
        expected_state, state.screen_state.state_label,
        "{label} state"
    );
    let after = render_theme_tokens_state(&state);

    assert!(
        component_body_pixel_diff(PAGE, &before, &after) > 0,
        "{label} should update the theme-tokens component body"
    );
}

fn render_theme_tokens_state(state: &StorybookWindowState) -> super::Canvas {
    super::render::render_storybook_canvas_with_options(super::render::StorybookRenderOptions {
        theme_id: state.theme_id,
        selected_page: state.selected_page,
        selected_instance_id: state.selected_instance_id,
        preset_index: state.preset_index,
        preset_tab_scroll_x: state.preset_tab_scroll_x,
        scroll_y: state.scroll_y,
        scrollbar_visible: state.scrollbar_visible,
        panel_scroll: state.panel_scroll,
        tree_expansion: state.tree_expansion,
        show_navigation_lines: state.show_navigation_lines,
        show_navigation_text_connectors: state.show_navigation_text_connectors,
        screen_state: state.screen_state.clone(),
    })
}

#[test]
fn theme_tokens_preview_actions_expose_hover_focus_keyboard_and_resize_ports() {
    assert_theme_tokens_preview_action(
        "theme-tokens-hover",
        "theme_token_hover",
        "hover_start",
        "hover=accent",
    );
    assert_theme_tokens_preview_action(
        "theme-tokens-focus",
        "theme_token_focus",
        "focus",
        "focus=swatch",
    );
    assert_theme_tokens_preview_action(
        "theme-tokens-keyboard",
        "theme_token_keyboard_light",
        "theme_changed",
        "keyboard=light",
    );
    assert_theme_tokens_preview_action(
        "theme-tokens-resize",
        "theme_token_resize_spacing",
        "theme_spacing_changed",
        "resize=spacing",
    );
}

fn assert_theme_tokens_preview_action(
    page: &str,
    expected_action: &str,
    expected_event: &str,
    expected_state: &str,
) {
    let mut state = super::screen_state::StorybookScreenState::default();

    state.register_preview_action(page);

    assert_eq!(expected_action, state.last_action);
    assert_eq!(expected_event, state.last_event);
    assert_eq!(expected_state, state.state_label);
}
