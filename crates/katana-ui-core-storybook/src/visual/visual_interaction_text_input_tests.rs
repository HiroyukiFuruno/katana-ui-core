use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{StorybookWindowState, TextInputKey, apply_click};
use super::{
    StorybookVisual, palette, preview_detail, render, storybook_ui_option_contract,
    window_interaction,
};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "text-input";
const VALUE_PRESET: usize = 0;
const IME_PRESET: usize = 1;
const INVALID_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const FIELD_FILL_SAMPLE_X_OFFSET: usize = 8;
const FIELD_FILL_SAMPLE_Y_OFFSET: usize = 8;
const FIELD_CURSOR_SAMPLE_X_OFFSET: usize = 188;
const FIELD_CURSOR_SAMPLE_Y_OFFSET: usize = 10;

#[test]
fn text_input_exposes_leaf_presets_options_and_input_commit_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("input_commit", spec.action);
    assert_eq!("text_committed", spec.event);
    assert_eq!("interaction.value", spec.option);
    assert_eq!("typed 日本語", spec.after);
    assert_eq!("value=typed", spec.state);
}

#[test]
fn text_input_presets_render_distinct_input_bodies() {
    let value = StorybookVisual.render_preset(DARK_THEME, PAGE, VALUE_PRESET, 0);
    let ime = StorybookVisual.render_preset(DARK_THEME, PAGE, IME_PRESET, 0);
    let invalid = StorybookVisual.render_preset(DARK_THEME, PAGE, INVALID_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &value, &ime) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &ime, &invalid) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &invalid, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn text_input_setting_option_updates_input_style() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn text_input_preview_action_updates_committed_value_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn text_input_field_accepts_keyboard_input_after_focus() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let before = render_with_state(&state);
    let field = text_input_field_rect();

    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    assert!(state.screen_state.text_input_focused());
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Character('k')
    ));
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Character('u')
    ));
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Character('c')
    ));

    let after = render_with_state(&state);
    assert!(state.screen_state.text_input_value().ends_with("kuc"));
    assert_eq!("text_input_type", state.screen_state.last_action);
    assert_eq!("text_input_changed", state.screen_state.last_event);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn text_input_keyboard_requires_focus_and_commits_enter() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let initial_value = state.screen_state.text_input_value().to_string();

    assert!(!window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Character('x')
    ));
    assert_eq!(initial_value, state.screen_state.text_input_value());

    let field = text_input_field_rect();
    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Character('x')
    ));
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Backspace
    ));
    assert_eq!(initial_value, state.screen_state.text_input_value());
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Submit
    ));
    assert_eq!("input_commit", state.screen_state.last_action);
    assert_eq!("text_committed", state.screen_state.last_event);
    assert_eq!("value=typed", state.screen_state.state_label);
}

#[test]
fn text_input_light_and_dark_fields_use_theme_tokens() {
    assert_field_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_field_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

fn render_with_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn text_input_field_rect() -> super::layout_metrics::LayoutRect {
    let rect = preview_detail::component_action_hit_rect(PAGE);
    super::dedicated_dod_form_input_live::search_field_rect(rect.x, rect.y)
}

fn assert_field_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, VALUE_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let field = super::dedicated_dod_form_input_live::search_field_rect(rect.x, rect.y);

    assert_eq!(Some(colors.border), pixel_at(&canvas, field.x, field.y));
    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            field.x + FIELD_FILL_SAMPLE_X_OFFSET,
            field.y + FIELD_FILL_SAMPLE_Y_OFFSET
        )
    );
    assert_eq!(
        Some(colors.accent),
        pixel_at(
            &canvas,
            field.x + FIELD_CURSOR_SAMPLE_X_OFFSET,
            field.y + FIELD_CURSOR_SAMPLE_Y_OFFSET
        )
    );
}
