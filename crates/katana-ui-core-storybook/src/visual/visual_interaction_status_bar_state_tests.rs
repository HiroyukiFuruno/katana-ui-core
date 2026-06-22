use super::visual_interaction_test_support::{component_body_pixel_diff, require_some};
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, focus_clickable_at_for_audit,
};
use super::{dedicated_status_bar, preview_detail, render};

const DARK_THEME: &str = "dark";
const PAGE: &str = "status-bar";
const PRIMARY_INSTANCE: &str = "status-bar.primary";
const SECONDARY_INSTANCE: &str = "status-bar.secondary";
const BRANCH_SEGMENT_INDEX: usize = 0;
const PROGRESS_SEGMENT_INDEX: usize = 2;

#[test]
fn status_bar_window_interaction_keeps_instance_state_isolated() -> Result<(), String> {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    state.select_instance(PRIMARY_INSTANCE);
    click_segment(&mut state, BRANCH_SEGMENT_INDEX)?;
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("status_bar_segment_popover", primary.last_action);
    assert_eq!("branch", primary.last_setting_value);
    assert_eq!("open_popover=branch", primary.state_label);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    click_segment(&mut state, PROGRESS_SEGMENT_INDEX)?;
    let secondary = state.screen_state.clone();
    let secondary_canvas = render_state(&state);
    assert_eq!("status_bar_segment_popover", secondary.last_action);
    assert_eq!("progress", secondary.last_setting_value);
    assert_eq!("open_popover=progress", secondary.state_label);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(
        primary.last_setting_value,
        state.screen_state.last_setting_value
    );
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(PAGE, &primary_canvas, &secondary_canvas) > 80,
        "status-bar instance-local state must produce distinct rendered bodies"
    );

    Ok(())
}

#[test]
fn status_bar_live_hover_focus_and_keyboard_use_core_actions() -> Result<(), String> {
    let mut hover = state_for();
    let hover_target = segment_target(BRANCH_SEGMENT_INDEX)?;
    let before_hover = render_state(&hover);
    assert!(apply_hover_at(
        &mut hover,
        hover_target.x + 1,
        hover_target.y + 1
    ));
    let after_hover = render_state(&hover);
    assert_eq!("status_bar_segment_hover", hover.screen_state.last_action);
    assert_eq!("status_bar_tooltip_shown", hover.screen_state.last_event);
    assert_eq!("tooltip=branch", hover.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let mut keyboard = state_for();
    let keyboard_target = segment_target(BRANCH_SEGMENT_INDEX)?;
    assert!(focus_clickable_at_for_audit(
        &mut keyboard,
        keyboard_target.x + 1,
        keyboard_target.y + 1
    ));
    assert_eq!(
        "status_bar_segment_focus",
        keyboard.screen_state.last_action
    );
    assert_eq!("focus", keyboard.screen_state.last_event);
    assert_eq!("focus=branch", keyboard.screen_state.state_label);
    let before_key = render_state(&keyboard);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut keyboard));
    let after_key = render_state(&keyboard);
    assert_eq!(
        "status_bar_keyboard_activate",
        keyboard.screen_state.last_action
    );
    assert_eq!(
        "status_bar_popover_opened",
        keyboard.screen_state.last_event
    );
    assert_eq!("open_popover=branch", keyboard.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_key, &after_key) > 0);

    Ok(())
}

fn click_segment(state: &mut StorybookWindowState, index: usize) -> Result<(), String> {
    let rect = segment_target(index)?;

    assert!(apply_click(state, rect.x + 1, rect.y + 1));
    Ok(())
}

fn state_for() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn segment_target(index: usize) -> Result<super::layout_metrics::LayoutRect, String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let rect = require_some(
        dedicated_status_bar::segment_rect_for_test(index),
        "status bar segment rect",
    )?;
    Ok(super::layout_metrics::LayoutRect::new(
        component.x + rect.x,
        component.y + rect.y,
        rect.width,
        rect.height,
    ))
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}
