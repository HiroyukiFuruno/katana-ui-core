use super::window_interaction::{StorybookWindowState, apply_click};
use super::{dedicated_dod_form_binary_choice_live, preview_detail};

const PAGE: &str = "checkbox";
const CLICK_OFFSET: usize = 4;

#[test]
fn checkbox_checked_state_read_preserves_checked_state_metadata() {
    let mut state = checkbox_state();
    state.select_preset(1);
    click_state_read(&mut state);

    assert_eq!("checkbox_state_read", state.screen_state.last_action);
    assert_eq!("checked_read", state.screen_state.last_event);
    assert_eq!("checked=true", state.screen_state.state_label);
    assert!(state.screen_state.is_checkbox_checked());
}

#[test]
fn checkbox_disabled_state_read_control_is_blocked_like_other_muted_controls() {
    let mut state = checkbox_state();
    state.select_preset(2);
    click_state_read(&mut state);

    assert_eq!(0, state.screen_state.action_count);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!("none", state.screen_state.last_event);
    assert_eq!("disabled=true", state.screen_state.state_label);
    assert!(state.screen_state.is_checkbox_disabled());
    assert!(!state.screen_state.is_checkbox_checked());
}

#[test]
fn checkbox_focus_state_read_preserves_focus_state_metadata() {
    let mut state = checkbox_state();
    state.select_preset(3);
    click_state_read(&mut state);

    assert_eq!("checkbox_state_read", state.screen_state.last_action);
    assert_eq!("checked_read", state.screen_state.last_event);
    assert_eq!("focused=true", state.screen_state.state_label);
    assert!(state.screen_state.is_checkbox_focused());
    assert!(!state.screen_state.is_checkbox_checked());
}

fn click_state_read(state: &mut StorybookWindowState) {
    let read = checkbox_state_read_control();
    assert!(apply_click(
        state,
        read.x + CLICK_OFFSET,
        read.y + CLICK_OFFSET
    ));
}

fn checkbox_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn checkbox_state_read_control() -> super::layout_metrics::LayoutRect {
    let component = preview_detail::component_action_hit_rect(PAGE);
    dedicated_dod_form_binary_choice_live::checkbox_state_read_button_rect(component.x, component.y)
}
