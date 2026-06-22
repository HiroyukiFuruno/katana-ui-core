use super::visual_interaction_test_support::{component_body_pixel_diff, pixel_at};
use super::window_interaction::{
    StorybookCursorStyle, StorybookWindowState, apply_click, apply_hover_at,
};
use super::{palette, preview_detail, render, window_interaction};
use katana_ui_core::theme::ThemeSnapshot;

const PAGE: &str = "text-input";
const DARK_THEME: &str = "dark";
const CLEAR_ACTION_PRESET: usize = 12;
const BODY_DIFF_THRESHOLD: usize = 80;

#[test]
fn text_input_clear_action_preset_click_clears_value_through_core_action() {
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
    assert!(apply_click(&mut state, rect.x + 1, rect.y + 1));

    assert_eq!("", state.screen_state.text_input_value());
    assert_eq!("text_input_clear_action", state.screen_state.last_action);
    assert_eq!("text_input_changed", state.screen_state.last_event);
    assert_eq!("text_entry.clear_action", state.screen_state.last_setting);
    assert_eq!("cleared", state.screen_state.last_setting_value);
    assert_eq!("value=cleared", state.screen_state.state_label);

    let after = render_with_state(&state);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn text_input_clear_action_is_blocked_for_readonly_runtime() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    state
        .screen_state
        .register_text_input_clear_action_for("text-input.readonly", "locked", true);

    assert_eq!(
        "locked",
        state
            .screen_state
            .text_input_value_for("text-input.readonly")
    );
    assert_eq!(
        "text_input_readonly_blocked",
        state.screen_state.last_action
    );
    assert_eq!("text_input_readonly_ignored", state.screen_state.last_event);
}

#[test]
fn text_input_clear_action_hover_draws_button_family_border() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        preset_index: CLEAR_ACTION_PRESET,
        ..StorybookWindowState::default()
    };
    let before = render_with_state(&state);
    let rect = clear_action_rect();

    assert!(apply_hover_at(&mut state, rect.x + 1, rect.y + 1));
    assert!(state.screen_state.hovered_text_input_clear_action);

    let after = render_with_state(&state);
    let hover_border = pixel_at(&after, rect.x, rect.y);
    assert_ne!(pixel_at(&before, rect.x, rect.y), hover_border);
    assert_eq!(
        Some(palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).hover_border),
        hover_border
    );
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
    super::dedicated_dod_form_input_live::search_inline_clear_rect(rect.x, rect.y)
}
