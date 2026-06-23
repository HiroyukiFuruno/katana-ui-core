use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, focus_clickable_at_for_audit,
};
use super::{preview_detail, render};

const DARK_THEME: &str = "dark";
const PAGE: &str = "toggle";
const PRIMARY_INSTANCE: &str = "toggle.primary";
const SECONDARY_INSTANCE: &str = "toggle.secondary";
const DISABLED_PRESET_INDEX: usize = 2;
const NO_BODY_DIFF: usize = 0;

#[test]
fn toggle_window_interaction_click_toggles_on_and_back_off() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    assert!(click_toggle(&mut state));
    assert_eq!("toggle_change", state.screen_state.last_action);
    assert_eq!("toggle_changed", state.screen_state.last_event);
    assert_eq!("checked=true", state.screen_state.state_label);

    assert!(click_toggle(&mut state));
    assert_eq!("toggle_change", state.screen_state.last_action);
    assert_eq!("toggle_changed", state.screen_state.last_event);
    assert_eq!("checked=false", state.screen_state.state_label);
}

#[test]
fn toggle_window_interaction_keyboard_toggles_on_and_back_off() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let rect = preview_detail::component_action_hit_rect(PAGE);

    assert!(focus_clickable_at_for_audit(
        &mut state,
        rect.x + 1,
        rect.y + 1
    ));
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    assert_eq!("checked=true", state.screen_state.state_label);

    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    assert_eq!("toggle_keyboard_toggle", state.screen_state.last_action);
    assert_eq!("toggle_changed", state.screen_state.last_event);
    assert_eq!("checked=false", state.screen_state.state_label);
}

#[test]
fn toggle_window_interaction_keeps_instance_state_isolated() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    state.select_instance(PRIMARY_INSTANCE);
    click_toggle(&mut state);
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("toggle_change", primary.last_action);
    assert_eq!("checked=true", primary.state_label);

    state.select_instance(SECONDARY_INSTANCE);
    let secondary_canvas = render_state(&state);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!("idle", state.screen_state.state_label);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(PAGE, &primary_canvas, &secondary_canvas) > 80,
        "toggle instance-local state must produce distinct rendered bodies"
    );
}

#[test]
fn toggle_window_interaction_disabled_click_does_not_mutate_state() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        preset_index: DISABLED_PRESET_INDEX,
        ..StorybookWindowState::default()
    };
    let before_state = state.screen_state.clone();
    let before_canvas = render_state(&state);

    assert!(click_toggle(&mut state));
    let after_canvas = render_state(&state);

    assert_eq!(before_state, state.screen_state);
    assert_eq!(
        NO_BODY_DIFF,
        component_body_pixel_diff(PAGE, &before_canvas, &after_canvas)
    );
}

#[test]
fn toggle_window_interaction_hover_focus_and_keyboard_update_body_and_state() {
    let mut hover = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let before_hover = render_state(&hover);
    let rect = preview_detail::component_action_hit_rect(PAGE);

    assert!(apply_hover_at(&mut hover, rect.x + 1, rect.y + 1));
    let after_hover = render_state(&hover);
    assert!(hover.screen_state.preview_hovered);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let mut keyboard = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let before_focus = render_state(&keyboard);
    assert!(focus_clickable_at_for_audit(
        &mut keyboard,
        rect.x + 1,
        rect.y + 1
    ));
    let after_focus = render_state(&keyboard);
    assert_eq!("toggle_focus", keyboard.screen_state.last_action);
    assert_eq!("toggle_focused", keyboard.screen_state.last_event);
    assert_eq!("focused=true", keyboard.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > 0);

    let before_keyboard = render_state(&keyboard);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut keyboard));
    let after_keyboard = render_state(&keyboard);
    assert_eq!("toggle_keyboard_toggle", keyboard.screen_state.last_action);
    assert_eq!("toggle_changed", keyboard.screen_state.last_event);
    assert_eq!("checked=true", keyboard.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_keyboard, &after_keyboard) > 0);
}

fn click_toggle(state: &mut StorybookWindowState) -> bool {
    let rect = preview_detail::component_action_hit_rect(PAGE);
    apply_click(state, rect.x + 1, rect.y + 1)
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}
