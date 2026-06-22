use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{
    StorybookWindowState, apply_clickable_keyboard_activation_for_audit,
    focus_clickable_at_for_audit,
};
use super::{dedicated_dod_form_binary_choice_live, preview_detail, render};

const DARK_THEME: &str = "dark";
const PAGE: &str = "radio";
const CLICK_OFFSET: usize = 4;
const NO_BODY_DIFF: usize = 0;

#[test]
fn radio_focus_and_hover_render_body_feedback() {
    let mut hover_state = radio_state();
    let before_hover = render_radio(&hover_state);
    let row = radio_row();
    assert!(super::window_interaction::apply_hover_at(
        &mut hover_state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET,
    ));
    let after_hover = render_radio(&hover_state);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > NO_BODY_DIFF);

    let mut focus_state = radio_state();
    let before_focus = render_radio(&focus_state);
    assert!(focus_clickable_at_for_audit(
        &mut focus_state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET,
    ));
    let after_focus = render_radio(&focus_state);
    assert_eq!("radio_focus", focus_state.screen_state.last_action);
    assert_eq!("radio_focused", focus_state.screen_state.last_event);
    assert_eq!("focused=true", focus_state.screen_state.state_label);
    assert!(focus_state.screen_state.is_radio_focused());
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > NO_BODY_DIFF);
}

#[test]
fn radio_keyboard_select_requires_focus_and_uses_core_selected_state() {
    let mut state = radio_state();

    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    assert_eq!(
        "radio_keyboard_without_focus",
        state.screen_state.last_action
    );
    assert_eq!("radio_keyboard_ignored", state.screen_state.last_event);
    assert_eq!("focused=false", state.screen_state.state_label);
    assert!(!state.screen_state.is_radio_selected());

    let row = radio_row();
    assert!(focus_clickable_at_for_audit(
        &mut state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET,
    ));
    let before = render_radio(&state);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    let after = render_radio(&state);

    assert_eq!("radio_keyboard_select", state.screen_state.last_action);
    assert_eq!("radio_selected", state.screen_state.last_event);
    assert_eq!("before=false after=true", state.screen_state.state_label);
    assert!(state.screen_state.is_radio_selected());
    assert!(component_body_pixel_diff(PAGE, &before, &after) > NO_BODY_DIFF);
}

fn radio_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_radio(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn radio_row() -> super::layout_metrics::LayoutRect {
    let component = preview_detail::component_action_hit_rect(PAGE);
    dedicated_dod_form_binary_choice_live::radio_row_rect(0, component.x, component.y)
}
