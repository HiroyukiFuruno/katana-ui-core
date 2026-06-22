use super::visual_interaction_test_support::pixel_at;
use super::{palette, preview_detail, render, window_interaction};
use katana_ui_core::theme::ThemeSnapshot;

const PAGE: &str = "text-area";
const DARK_THEME: &str = "dark";
const CHAT_PRESET: usize = 0;
const TRAILING_BUTTONS_PRESET: usize = 11;
const CLEAR_ACTION_PRESET: usize = 12;

#[test]
fn text_area_field_hover_draws_hover_border_without_mutating_value() {
    let mut state = window_interaction::StorybookWindowState {
        selected_page: PAGE,
        preset_index: CHAT_PRESET,
        ..window_interaction::StorybookWindowState::default()
    };
    let before = render_with_state(&state);
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let rect = super::dedicated_dod_form_input_live::text_area_rect(origin.x, origin.y);
    let initial_value = state.screen_state.text_area_value().to_string();

    assert!(window_interaction::apply_hover_at(
        &mut state,
        rect.x + 1,
        rect.y + 1
    ));
    assert!(state.screen_state.preview_hovered);
    assert_eq!(initial_value, state.screen_state.text_area_value());
    assert_eq!(0, state.screen_state.action_count);

    assert_hover_border(&before, &render_with_state(&state), rect.x, rect.y);
}

#[test]
fn text_area_trailing_icon_button_hover_draws_button_family_border() {
    let mut state = window_interaction::StorybookWindowState {
        selected_page: PAGE,
        preset_index: TRAILING_BUTTONS_PRESET,
        ..window_interaction::StorybookWindowState::default()
    };
    let before = render_with_state(&state);
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let rect = super::dedicated_dod_form_input_live::text_area_trailing_icon_button_rects(
        origin.x, origin.y,
    )[0];

    assert!(window_interaction::apply_hover_at(
        &mut state,
        rect.x + 1,
        rect.y + 1
    ));
    assert_eq!(
        Some(0),
        state.screen_state.hovered_text_area_icon_button_index
    );

    assert_hover_border(&before, &render_with_state(&state), rect.x, rect.y);
}

#[test]
fn text_area_clear_action_hover_draws_button_family_border() {
    let mut state = window_interaction::StorybookWindowState {
        selected_page: PAGE,
        preset_index: CLEAR_ACTION_PRESET,
        ..window_interaction::StorybookWindowState::default()
    };
    let before = render_with_state(&state);
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let rect =
        super::dedicated_dod_form_input_live::text_area_clear_action_rect(origin.x, origin.y);

    assert!(window_interaction::apply_hover_at(
        &mut state,
        rect.x + 1,
        rect.y + 1
    ));
    assert!(state.screen_state.hovered_text_area_clear_action);

    assert_hover_border(&before, &render_with_state(&state), rect.x, rect.y);
}

fn render_with_state(state: &window_interaction::StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn assert_hover_border(before: &super::Canvas, after: &super::Canvas, x: usize, y: usize) {
    let hover_border = pixel_at(after, x, y);
    assert_ne!(pixel_at(before, x, y), hover_border);
    assert_eq!(
        Some(palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).hover_border),
        hover_border
    );
}
