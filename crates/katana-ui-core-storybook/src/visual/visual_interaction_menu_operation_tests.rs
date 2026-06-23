use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_context_click_for_test, apply_hover_at, focus_clickable_at_for_audit,
};
use super::{dedicated_dod_molecule_menu, preview_detail, render};

const DARK_THEME: &str = "dark";
const PAGE: &str = "menu";
const BODY_DIFF_THRESHOLD: usize = 80;
const CLICK_OFFSET: usize = 4;

#[test]
fn menu_focus_and_hover_render_body_feedback() {
    let mut hover_state = menu_state();
    let before_hover = render_menu(&hover_state);
    let row = first_row_rect();

    assert!(apply_hover_at(
        &mut hover_state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET
    ));
    let after_hover = render_menu(&hover_state);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let mut focus_state = menu_state();
    let before_focus = render_menu(&focus_state);
    assert!(focus_clickable_at_for_audit(
        &mut focus_state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET
    ));
    let after_focus = render_menu(&focus_state);
    assert_eq!("menu_focus", focus_state.screen_state.last_action);
    assert_eq!("menu_focused", focus_state.screen_state.last_event);
    assert_eq!("focused=true", focus_state.screen_state.state_label);
    assert!(focus_state.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > 0);
}

#[test]
fn menu_keyboard_open_requires_focus_and_updates_open_state() {
    let mut state = menu_state();

    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    assert_eq!(
        "menu_keyboard_without_focus",
        state.screen_state.last_action
    );
    assert_eq!("menu_keyboard_ignored", state.screen_state.last_event);
    assert_eq!("focused=false", state.screen_state.state_label);
    assert!(!state.screen_state.selection.select_open);

    let row = first_row_rect();
    assert!(focus_clickable_at_for_audit(
        &mut state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET
    ));
    let before = render_menu(&state);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    let after = render_menu(&state);

    assert_eq!("menu_keyboard_open", state.screen_state.last_action);
    assert_eq!("menu_opened", state.screen_state.last_event);
    assert_eq!("open=true", state.screen_state.state_label);
    assert!(state.screen_state.selection.select_open);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn menu_context_click_outside_dismisses_open_menu() {
    let mut state = menu_state();
    let row = first_row_rect();
    assert!(apply_click(
        &mut state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET
    ));
    assert!(state.screen_state.selection.select_open);
    let before = render_menu(&state);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert!(apply_context_click_for_test(
        &mut state,
        component.x + component.width + CLICK_OFFSET,
        component.y + CLICK_OFFSET
    ));
    let after = render_menu(&state);

    assert_eq!("menu_context_dismiss", state.screen_state.last_action);
    assert_eq!("menu_closed", state.screen_state.last_event);
    assert_eq!("open=false", state.screen_state.state_label);
    assert!(!state.screen_state.selection.select_open);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

fn menu_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_menu(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn first_row_rect() -> super::layout_metrics::LayoutRect {
    dedicated_dod_molecule_menu::first_row_rect(preview_detail::component_action_hit_rect(PAGE))
}
