use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{
    StorybookWindowState, apply_clickable_keyboard_activation_for_audit,
    apply_context_click_for_test, apply_hover_at, focus_clickable_at_for_audit,
};
use super::{dedicated_context_menu_popup, preview_detail, render};

const DARK_THEME: &str = "dark";
const PAGE: &str = "context-menu";
const BODY_DIFF_THRESHOLD: usize = 80;
const CLICK_OFFSET: usize = 4;

#[test]
fn context_menu_focus_and_hover_render_body_feedback() {
    let mut hover_state = context_menu_state();
    let before_hover = render_context_menu(&hover_state);
    let target = focus_target();

    assert!(apply_hover_at(
        &mut hover_state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    let after_hover = render_context_menu(&hover_state);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let mut focus_state = context_menu_state();
    let before_focus = render_context_menu(&focus_state);
    assert!(focus_clickable_at_for_audit(
        &mut focus_state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    let after_focus = render_context_menu(&focus_state);
    assert_eq!("context_menu_focus", focus_state.screen_state.last_action);
    assert_eq!("context_menu_focused", focus_state.screen_state.last_event);
    assert_eq!("focused=true", focus_state.screen_state.state_label);
    assert!(focus_state.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > 0);
}

#[test]
fn context_menu_keyboard_select_requires_focus_and_uses_core_event() {
    let mut state = context_menu_state();

    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    assert_eq!(
        "context_menu_keyboard_without_focus",
        state.screen_state.last_action
    );
    assert_eq!(
        "context_menu_keyboard_ignored",
        state.screen_state.last_event
    );
    assert_eq!("focused=false", state.screen_state.state_label);

    let target = focus_target();
    assert!(focus_clickable_at_for_audit(
        &mut state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    let before = render_context_menu(&state);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    let after = render_context_menu(&state);

    assert_eq!(
        "context_menu_keyboard_select",
        state.screen_state.last_action
    );
    assert_eq!("context_menu_item_selected", state.screen_state.last_event);
    assert_eq!("context_menu.selected=[1]", state.screen_state.state_label);
    assert_eq!("context_menu.command", state.screen_state.last_setting);
    assert_eq!("copy", state.screen_state.last_setting_value);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn context_menu_outside_context_click_dismisses_open_menu() {
    let mut state = context_menu_state();
    let component = preview_detail::component_action_hit_rect(PAGE);
    assert!(apply_context_click_for_test(
        &mut state,
        component.x + CLICK_OFFSET,
        component.y + CLICK_OFFSET
    ));
    assert_eq!("context_menu=open", state.screen_state.state_label);
    let before = render_context_menu(&state);

    assert!(apply_context_click_for_test(
        &mut state,
        component.x + component.width + CLICK_OFFSET,
        component.y + CLICK_OFFSET
    ));
    let after = render_context_menu(&state);

    assert_eq!(
        "context_menu_outside_dismiss",
        state.screen_state.last_action
    );
    assert_eq!("context_menu_closed", state.screen_state.last_event);
    assert_eq!("context_menu=closed", state.screen_state.state_label);
    assert_eq!("context_menu.dismiss", state.screen_state.last_setting);
    assert_eq!("outside", state.screen_state.last_setting_value);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

fn context_menu_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_context_menu(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn focus_target() -> super::layout_metrics::LayoutRect {
    let component = preview_detail::component_action_hit_rect(PAGE);
    dedicated_context_menu_popup::insert_row_rect(component.x, component.y)
}
