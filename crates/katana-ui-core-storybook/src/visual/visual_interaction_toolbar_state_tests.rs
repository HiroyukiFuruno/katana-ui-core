use super::visual_interaction_test_support::{component_body_pixel_diff, require_some};
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_hover_at, cursor_style_at_for_test,
};
use super::{dedicated_toolbar, preview_detail, render};

const DARK_THEME: &str = "dark";
const PAGE: &str = "toolbar";
const PRIMARY_INSTANCE: &str = "toolbar.primary";
const SECONDARY_INSTANCE: &str = "toolbar.secondary";
const SAVE_ACTION_INDEX: usize = 0;
const SPLIT_ACTION_INDEX: usize = 1;
const SEARCH_ACTION_INDEX: usize = 2;
const ACTION_DISABLED_PRESET_INDEX: usize = 12;
const SPLIT_DISABLED_PRESET_INDEX: usize = 15;
const NO_BODY_DIFF: usize = 0;

#[test]
fn toolbar_window_interaction_keeps_instance_state_isolated() -> Result<(), String> {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    state.select_instance(PRIMARY_INSTANCE);
    click_action(&mut state, SAVE_ACTION_INDEX)?;
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("tool_toggle", primary.last_action);
    assert_eq!("tool_changed", primary.last_event);
    assert_eq!("active=true", primary.state_label);
    assert_eq!(None, primary.hovered_toolbar_action_index);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!(0, state.screen_state.action_count);
    hover_action(&mut state, SEARCH_ACTION_INDEX)?;
    let secondary = state.screen_state.clone();
    let secondary_canvas = render_state(&state);
    assert_eq!("none", secondary.last_action);
    assert_eq!(0, secondary.action_count);
    assert_eq!(
        Some(SEARCH_ACTION_INDEX),
        secondary.hovered_toolbar_action_index
    );

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.last_event, state.screen_state.last_event);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert_eq!(
        primary.hovered_toolbar_action_index,
        state.screen_state.hovered_toolbar_action_index
    );
    assert!(
        component_body_pixel_diff(PAGE, &primary_canvas, &secondary_canvas) > 80,
        "toolbar instance-local state must produce distinct rendered bodies"
    );

    Ok(())
}

#[test]
fn toolbar_window_interaction_disabled_action_does_not_mutate_state() -> Result<(), String> {
    let mut state = toolbar_state(ACTION_DISABLED_PRESET_INDEX);
    let before_state = state.screen_state.clone();
    let before_canvas = render_state(&state);

    assert!(click_action(&mut state, SAVE_ACTION_INDEX)?);
    let after_canvas = render_state(&state);

    assert_eq!(before_state, state.screen_state);
    assert_eq!(
        NO_BODY_DIFF,
        component_body_pixel_diff(PAGE, &before_canvas, &after_canvas)
    );
    Ok(())
}

#[test]
fn toolbar_window_interaction_disabled_split_does_not_mutate_state() -> Result<(), String> {
    let mut state = toolbar_state(SPLIT_DISABLED_PRESET_INDEX);
    let before_state = state.screen_state.clone();
    let before_canvas = render_state(&state);

    assert!(click_action(&mut state, SPLIT_ACTION_INDEX)?);
    let after_canvas = render_state(&state);

    assert_eq!(before_state, state.screen_state);
    assert_eq!(
        NO_BODY_DIFF,
        component_body_pixel_diff(PAGE, &before_canvas, &after_canvas)
    );
    Ok(())
}

fn click_action(state: &mut StorybookWindowState, index: usize) -> Result<bool, String> {
    let (x, y) = action_center(index)?;

    Ok(apply_click(state, x, y))
}

fn hover_action(state: &mut StorybookWindowState, index: usize) -> Result<(), String> {
    let (x, y) = action_center(index)?;

    assert_eq!(
        super::window_interaction::StorybookCursorStyle::PointingHand,
        cursor_style_at_for_test(state, x, y)
    );
    assert!(apply_hover_at(state, x, y));
    Ok(())
}

fn action_center(index: usize) -> Result<(usize, usize), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let rect = require_some(
        dedicated_toolbar::action_rect_for_test(index),
        "toolbar action rect",
    )?;

    Ok((
        component.x + rect.x + rect.width / 2,
        component.y + rect.y + rect.height / 2,
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

fn toolbar_state(preset_index: usize) -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        preset_index,
        ..StorybookWindowState::default()
    }
}
