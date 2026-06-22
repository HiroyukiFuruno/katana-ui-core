use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, focus_clickable_at_for_audit,
};
use super::{dedicated_dod_form_binary_choice_live, palette, preview_detail, render};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const PAGE: &str = "checkbox";
const CLICK_OFFSET: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const CHECKBOX_ACCENT: u32 = 0x569cd6;
const CHECKBOX_GLYPH: u32 = 0xf8fafc;

#[test]
fn checkbox_focus_and_hover_render_body_feedback() {
    let mut hover_state = checkbox_state();
    let before_hover = render_checkbox(&hover_state);
    let row = checkbox_row();

    assert!(apply_hover_at(
        &mut hover_state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET
    ));
    let after_hover = render_checkbox(&hover_state);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let mut focus_state = checkbox_state();
    let before_focus = render_checkbox(&focus_state);
    assert!(focus_clickable_at_for_audit(
        &mut focus_state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET
    ));
    let after_focus = render_checkbox(&focus_state);
    assert_eq!("checkbox_focus", focus_state.screen_state.last_action);
    assert_eq!("checkbox_focused", focus_state.screen_state.last_event);
    assert_eq!("focused=true", focus_state.screen_state.state_label);
    assert!(focus_state.screen_state.is_checkbox_focused());
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > 0);
}

#[test]
fn checkbox_hover_does_not_emit_click_event_or_mutate_checked_state() {
    let mut state = checkbox_state();
    let row = checkbox_row();

    assert!(apply_hover_at(
        &mut state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET
    ));
    assert_eq!(0, state.screen_state.action_count);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!("none", state.screen_state.last_event);
    assert_eq!("idle", state.screen_state.state_label);
    assert!(!state.screen_state.is_checkbox_checked());

    assert!(apply_hover_at(
        &mut state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET
    ));
    assert_eq!(0, state.screen_state.action_count);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!("none", state.screen_state.last_event);
    assert!(!state.screen_state.is_checkbox_checked());
}

#[test]
fn checkbox_hover_feedback_tracks_the_actual_row() {
    let mut state = checkbox_state();
    let first_row = checkbox_row_at(0);
    let second_row = checkbox_row_at(1);
    let hover_border = palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).hover_border;

    assert!(apply_hover_at(
        &mut state,
        second_row.x + CLICK_OFFSET,
        second_row.y + CLICK_OFFSET
    ));
    let canvas = render_checkbox(&state);

    assert_eq!(
        0,
        count_color_in_rect(&canvas, first_row, hover_border),
        "hovering the second checkbox row must not paint hover feedback on the first row"
    );
    assert!(
        count_color_in_rect(&canvas, second_row, hover_border) > 0,
        "hovering the second checkbox row must paint hover feedback on that row"
    );
}

#[test]
fn checkbox_keyboard_toggle_requires_focus_and_uses_core_checked_state() {
    let mut state = checkbox_state();

    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    assert_eq!(
        "checkbox_keyboard_without_focus",
        state.screen_state.last_action
    );
    assert_eq!("checkbox_keyboard_ignored", state.screen_state.last_event);
    assert_eq!("focused=false", state.screen_state.state_label);
    assert!(!state.screen_state.is_checkbox_checked());

    let row = checkbox_row();
    assert!(focus_clickable_at_for_audit(
        &mut state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET
    ));
    let before = render_checkbox(&state);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    let after = render_checkbox(&state);

    assert_eq!("checkbox_keyboard_toggle", state.screen_state.last_action);
    assert_eq!("checked_changed", state.screen_state.last_event);
    assert_eq!("before=false after=true", state.screen_state.state_label);
    assert!(state.screen_state.is_checkbox_checked());
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn checkbox_keyboard_second_toggle_removes_checked_mark_and_state() {
    let mut state = checkbox_state();
    let row = checkbox_row();
    let mark = checkbox_mark();
    assert!(focus_clickable_at_for_audit(
        &mut state,
        row.x + CLICK_OFFSET,
        row.y + CLICK_OFFSET
    ));
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    let checked = render_checkbox(&state);
    assert!(state.screen_state.is_checkbox_checked());
    assert!(count_color_in_rect(&checked, mark, CHECKBOX_ACCENT) > 0);
    assert!(count_color_in_rect(&checked, mark, CHECKBOX_GLYPH) > 0);

    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    let unchecked = render_checkbox(&state);

    assert_eq!("checkbox_keyboard_toggle", state.screen_state.last_action);
    assert_eq!("checked_changed", state.screen_state.last_event);
    assert_eq!("before=true after=false", state.screen_state.state_label);
    assert!(!state.screen_state.is_checkbox_checked());
    assert_eq!(0, count_color_in_rect(&unchecked, mark, CHECKBOX_ACCENT));
    assert_eq!(0, count_color_in_rect(&unchecked, mark, CHECKBOX_GLYPH));
}

#[test]
fn checkbox_keyboard_toggle_applies_to_focused_secondary_row_only() {
    let mut state = checkbox_state();
    let first_mark = checkbox_mark_at(0);
    let second_row = checkbox_row_at(1);
    let second_mark = checkbox_mark_at(1);

    assert!(focus_clickable_at_for_audit(
        &mut state,
        second_row.x + CLICK_OFFSET,
        second_row.y + CLICK_OFFSET
    ));
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    let canvas = render_checkbox(&state);

    assert_eq!("checkbox_keyboard_toggle", state.screen_state.last_action);
    assert_eq!("checked_changed", state.screen_state.last_event);
    assert_eq!("before=false after=true", state.screen_state.state_label);
    assert!(!state.screen_state.is_checkbox_checked_at(0));
    assert!(state.screen_state.is_checkbox_checked_at(1));
    assert_eq!(0, count_color_in_rect(&canvas, first_mark, CHECKBOX_ACCENT));
    assert_eq!(0, count_color_in_rect(&canvas, first_mark, CHECKBOX_GLYPH));
    assert!(count_color_in_rect(&canvas, second_mark, CHECKBOX_ACCENT) > 0);
    assert!(count_color_in_rect(&canvas, second_mark, CHECKBOX_GLYPH) > 0);
}

#[test]
fn checkbox_pointer_click_focuses_clicked_row_for_followup_keyboard_toggle() {
    let mut state = checkbox_state();
    let second_row = checkbox_row_at(1);

    assert!(apply_click(
        &mut state,
        second_row.x + CLICK_OFFSET,
        second_row.y + CLICK_OFFSET
    ));
    assert_eq!("checkbox_toggle", state.screen_state.last_action);
    assert_eq!(1, state.screen_state.checkbox_focused_index());
    assert!(state.screen_state.is_checkbox_focused_at(1));
    assert!(state.screen_state.is_checkbox_checked_at(1));

    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    assert_eq!("checkbox_keyboard_toggle", state.screen_state.last_action);
    assert_eq!("checked_changed", state.screen_state.last_event);
    assert_eq!("before=true after=false", state.screen_state.state_label);
    assert_eq!(1, state.screen_state.checkbox_focused_index());
    assert!(!state.screen_state.is_checkbox_checked_at(0));
    assert!(!state.screen_state.is_checkbox_checked_at(1));
}

#[test]
fn checkbox_control_toggle_and_reset_update_mark_and_state_together() {
    let mut state = checkbox_state();
    let toggle = checkbox_toggle_control();
    let reset = checkbox_reset_control();
    let mark = checkbox_mark();

    assert!(apply_click(
        &mut state,
        toggle.x + CLICK_OFFSET,
        toggle.y + CLICK_OFFSET
    ));
    let checked = render_checkbox(&state);
    assert_eq!("checkbox_toggle", state.screen_state.last_action);
    assert_eq!("checked_changed", state.screen_state.last_event);
    assert_eq!("before=false after=true", state.screen_state.state_label);
    assert!(state.screen_state.is_checkbox_checked());
    assert!(count_color_in_rect(&checked, mark, CHECKBOX_ACCENT) > 0);
    assert!(count_color_in_rect(&checked, mark, CHECKBOX_GLYPH) > 0);

    assert!(apply_click(
        &mut state,
        reset.x + CLICK_OFFSET,
        reset.y + CLICK_OFFSET
    ));
    let unchecked = render_checkbox(&state);
    assert_eq!("checkbox_reset", state.screen_state.last_action);
    assert_eq!("checked_changed", state.screen_state.last_event);
    assert_eq!("before=true after=false", state.screen_state.state_label);
    assert!(!state.screen_state.is_checkbox_checked());
    assert_eq!(0, count_color_in_rect(&unchecked, mark, CHECKBOX_ACCENT));
    assert_eq!(0, count_color_in_rect(&unchecked, mark, CHECKBOX_GLYPH));
}

fn checkbox_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_checkbox(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn checkbox_row() -> super::layout_metrics::LayoutRect {
    checkbox_row_at(0)
}

fn checkbox_row_at(index: usize) -> super::layout_metrics::LayoutRect {
    let component = preview_detail::component_action_hit_rect(PAGE);
    dedicated_dod_form_binary_choice_live::checkbox_row_rect(index, component.x, component.y)
}

fn checkbox_mark() -> super::layout_metrics::LayoutRect {
    checkbox_mark_at(0)
}

fn checkbox_mark_at(index: usize) -> super::layout_metrics::LayoutRect {
    let component = preview_detail::component_action_hit_rect(PAGE);
    dedicated_dod_form_binary_choice_live::checkbox_mark_rect(index, component.x, component.y)
}

fn checkbox_toggle_control() -> super::layout_metrics::LayoutRect {
    let component = preview_detail::component_action_hit_rect(PAGE);
    dedicated_dod_form_binary_choice_live::checkbox_toggle_button_rect(component.x, component.y)
}

fn checkbox_reset_control() -> super::layout_metrics::LayoutRect {
    let component = preview_detail::component_action_hit_rect(PAGE);
    dedicated_dod_form_binary_choice_live::checkbox_reset_button_rect(component.x, component.y)
}

fn count_color_in_rect(
    canvas: &super::Canvas,
    rect: super::layout_metrics::LayoutRect,
    color: u32,
) -> usize {
    (rect.y..rect.bottom())
        .flat_map(|y| (rect.x..rect.right()).map(move |x| (x, y)))
        .filter(|(x, y)| pixel_at(canvas, *x, *y) == Some(color))
        .count()
}

fn pixel_at(canvas: &super::Canvas, x: usize, y: usize) -> Option<u32> {
    if x >= canvas.width() || y >= canvas.height() {
        return None;
    }
    Some(canvas.pixels()[y * canvas.width() + x])
}
