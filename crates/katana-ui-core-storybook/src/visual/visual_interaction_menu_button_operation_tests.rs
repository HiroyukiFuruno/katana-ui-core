use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_context_click_for_test, focus_clickable_at_for_audit,
};
use super::{dedicated_menu_button, preview_detail, render};

const DARK_THEME: &str = "dark";
const PAGE: &str = "menu-button";
const DISABLED_PRESET: usize = 2;
const BODY_DIFF_THRESHOLD: usize = 80;
const CLICK_OFFSET: usize = 4;

#[test]
fn menu_button_trigger_click_opens_via_core_menu_button_state() {
    let mut state = menu_button_state();
    let before = render_menu_button(&state);
    let trigger = trigger_rect();

    assert!(apply_click(
        &mut state,
        trigger.x + CLICK_OFFSET,
        trigger.y + CLICK_OFFSET
    ));
    let after = render_menu_button(&state);

    assert_eq!("menu_button_open", state.screen_state.last_action);
    assert_eq!("menu_button_opened", state.screen_state.last_event);
    assert_eq!("open=true", state.screen_state.state_label);
    assert!(state.screen_state.selection.select_open);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn menu_button_item_click_selects_and_closes_via_core_menu_button_state() {
    let mut state = menu_button_state();
    let trigger = trigger_rect();
    assert!(apply_click(
        &mut state,
        trigger.x + CLICK_OFFSET,
        trigger.y + CLICK_OFFSET
    ));
    let before = render_menu_button(&state);
    let item = first_item_rect();

    assert!(apply_click(
        &mut state,
        item.x + CLICK_OFFSET,
        item.y + CLICK_OFFSET
    ));
    let after = render_menu_button(&state);

    assert_eq!("menu_button_select", state.screen_state.last_action);
    assert_eq!("menu_button_item_selected", state.screen_state.last_event);
    assert_eq!("selected=new-file", state.screen_state.state_label);
    assert!(!state.screen_state.selection.select_open);
    assert_eq!(Some(0), state.screen_state.selection.select_selected_index);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn menu_button_close_item_click_closes_open_menu() {
    let mut state = menu_button_state();
    let trigger = trigger_rect();
    assert!(apply_click(
        &mut state,
        trigger.x + CLICK_OFFSET,
        trigger.y + CLICK_OFFSET
    ));
    let before = render_menu_button(&state);
    let item = second_item_rect();

    assert!(apply_click(
        &mut state,
        item.x + CLICK_OFFSET,
        item.y + CLICK_OFFSET
    ));
    let after = render_menu_button(&state);

    assert_eq!("menu_button_close", state.screen_state.last_action);
    assert_eq!("menu_button_closed", state.screen_state.last_event);
    assert_eq!("open=false", state.screen_state.state_label);
    assert!(!state.screen_state.selection.select_open);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn menu_button_disabled_trigger_blocks_open_without_component_mutation() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        preset_index: DISABLED_PRESET,
        ..StorybookWindowState::default()
    };
    let before = render_menu_button(&state);
    let trigger = trigger_rect();

    assert!(apply_click(
        &mut state,
        trigger.x + CLICK_OFFSET,
        trigger.y + CLICK_OFFSET
    ));
    let after = render_menu_button(&state);

    assert_eq!(
        "menu_button_disabled_trigger",
        state.screen_state.last_action
    );
    assert_eq!(
        "menu_button_disabled_ignored",
        state.screen_state.last_event
    );
    assert_eq!("disabled=true", state.screen_state.state_label);
    assert!(!state.screen_state.selection.select_open);
    assert_eq!(0, component_body_pixel_diff(PAGE, &before, &after));
}

#[test]
fn menu_button_focus_keyboard_and_context_menu_are_live_operations() {
    let mut state = menu_button_state();
    let trigger = trigger_rect();
    let before_focus = render_menu_button(&state);

    assert!(focus_clickable_at_for_audit(
        &mut state,
        trigger.x + CLICK_OFFSET,
        trigger.y + CLICK_OFFSET
    ));
    let after_focus = render_menu_button(&state);
    assert_eq!("menu_button_focus", state.screen_state.last_action);
    assert_eq!("menu_button_focused", state.screen_state.last_event);
    assert_eq!("focused=true", state.screen_state.state_label);
    assert!(state.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > 0);

    let before_keyboard = render_menu_button(&state);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    let after_keyboard = render_menu_button(&state);
    assert_eq!("menu_button_keyboard_open", state.screen_state.last_action);
    assert_eq!("menu_button_opened", state.screen_state.last_event);
    assert_eq!("open=true", state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_keyboard, &after_keyboard) > 0);

    let mut context_state = menu_button_state();
    let before_context = render_menu_button(&context_state);
    assert!(apply_context_click_for_test(
        &mut context_state,
        trigger.x + CLICK_OFFSET,
        trigger.y + CLICK_OFFSET
    ));
    let after_context = render_menu_button(&context_state);
    assert_eq!(
        "menu_button_context_open",
        context_state.screen_state.last_action
    );
    assert_eq!("menu_button_opened", context_state.screen_state.last_event);
    assert_eq!("open=true", context_state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_context, &after_context) > 0);
}

fn menu_button_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_menu_button(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn trigger_rect() -> super::layout_metrics::LayoutRect {
    dedicated_menu_button::trigger_rect(preview_detail::component_action_hit_rect(PAGE))
}

fn first_item_rect() -> super::layout_metrics::LayoutRect {
    dedicated_menu_button::first_item_rect(preview_detail::component_action_hit_rect(PAGE))
}

fn second_item_rect() -> super::layout_metrics::LayoutRect {
    dedicated_menu_button::second_item_rect(preview_detail::component_action_hit_rect(PAGE))
}
