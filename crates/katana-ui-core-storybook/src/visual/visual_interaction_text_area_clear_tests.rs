use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookCursorStyle, StorybookWindowState, TextAreaKey};
use super::{preview_detail, render, window_interaction};

const PAGE: &str = "text-area";
const DARK_THEME: &str = "dark";
const CLEAR_ACTION_PRESET: usize = 12;
const READONLY_PRESET: usize = 17;
const BODY_DIFF_THRESHOLD: usize = 80;

#[test]
fn text_area_clear_action_preset_click_clears_value_through_core_action() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        preset_index: CLEAR_ACTION_PRESET,
        ..StorybookWindowState::default()
    };
    let before = render_with_state(&state);
    let rect = clear_action_rect();

    assert_eq!(
        StorybookCursorStyle::PointingHand,
        window_interaction::cursor_style_at_for_test(&state, rect.x + 1, rect.y + 1)
    );
    assert!(window_interaction::apply_click(
        &mut state,
        rect.x + 1,
        rect.y + 1
    ));

    assert_eq!("", state.screen_state.text_area_value());
    assert_eq!("text_area_clear_action", state.screen_state.last_action);
    assert_eq!("text_area_changed", state.screen_state.last_event);
    assert_eq!("text_area.clear_action", state.screen_state.last_setting);
    assert_eq!("cleared", state.screen_state.last_setting_value);
    assert_eq!("value=cleared", state.screen_state.state_label);

    let after = render_with_state(&state);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn text_area_readonly_preset_blocks_keyboard_mutation() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        preset_index: READONLY_PRESET,
        ..StorybookWindowState::default()
    };
    let field = text_area_field_rect();

    assert!(window_interaction::apply_click(
        &mut state,
        field.x + 1,
        field.y + 1
    ));
    let readonly_value = state.screen_state.text_area_value().to_string();
    assert!(window_interaction::apply_text_area_key(
        &mut state,
        TextAreaKey::Character('x')
    ));

    assert_eq!(readonly_value, state.screen_state.text_area_value());
    assert_eq!("text_area_readonly_blocked", state.screen_state.last_action);
    assert_eq!("text_area_readonly_ignored", state.screen_state.last_event);
    assert_eq!("text_area.readonly", state.screen_state.last_setting);
}

#[test]
fn text_area_clear_action_is_blocked_for_readonly_runtime() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let original_value = state.screen_state.text_area_value().to_string();

    assert!(
        state
            .screen_state
            .register_text_area_clear_action(true, false)
    );

    assert_eq!(original_value, state.screen_state.text_area_value());
    assert_eq!("text_area_readonly_blocked", state.screen_state.last_action);
    assert_eq!("text_area_readonly_ignored", state.screen_state.last_event);
}

fn render_with_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn clear_action_rect() -> super::layout_metrics::LayoutRect {
    let rect = preview_detail::component_action_hit_rect(PAGE);
    super::dedicated_dod_form_input_live::text_area_clear_action_rect(rect.x, rect.y)
}

fn text_area_field_rect() -> super::layout_metrics::LayoutRect {
    let rect = preview_detail::component_action_hit_rect(PAGE);
    super::dedicated_dod_form_input_live::text_area_rect(rect.x, rect.y)
}
