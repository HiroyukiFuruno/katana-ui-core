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
const READONLY_PRESET: usize = 2;
const PLACEHOLDER_PRESET: usize = 3;
const RESERVED_SLOT_PRESET: usize = 4;
const LEADING_ICON_PRESET: usize = 5;
const ICON_BUTTONS_PRESET: usize = 6;
const INVALID_PRESET: usize = 7;
const THEME_PRESET: usize = 8;
const REQUIRED_PRESET_COUNT: usize = 9;
const REQUIRED_OPTION_COUNT: usize = 9;
const BODY_DIFF_THRESHOLD: usize = 80;
const FIELD_FILL_SAMPLE_X_OFFSET: usize = 8;
const FIELD_FILL_SAMPLE_Y_OFFSET: usize = 8;

#[test]
fn text_input_exposes_leaf_presets_options_and_input_commit_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(presets.contains(&"readonly"));
    assert!(presets.contains(&"placeholder"));
    assert!(presets.contains(&"icon slot"));
    assert!(presets.contains(&"search icon"));
    assert!(presets.contains(&"icon buttons"));
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
    let readonly = StorybookVisual.render_preset(DARK_THEME, PAGE, READONLY_PRESET, 0);
    let placeholder = StorybookVisual.render_preset(DARK_THEME, PAGE, PLACEHOLDER_PRESET, 0);
    let reserved = StorybookVisual.render_preset(DARK_THEME, PAGE, RESERVED_SLOT_PRESET, 0);
    let icon = StorybookVisual.render_preset(DARK_THEME, PAGE, LEADING_ICON_PRESET, 0);
    let buttons = StorybookVisual.render_preset(DARK_THEME, PAGE, ICON_BUTTONS_PRESET, 0);
    let invalid = StorybookVisual.render_preset(DARK_THEME, PAGE, INVALID_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &value, &ime) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &ime, &readonly) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &readonly, &placeholder) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &placeholder, &reserved) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &reserved, &icon) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &icon, &buttons) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &buttons, &invalid) > BODY_DIFF_THRESHOLD);
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
fn text_input_readonly_preset_blocks_keyboard_and_keeps_preset_state_isolated() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let field = text_input_field_rect();

    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Character('q')
    ));
    let editable_value = state.screen_state.text_input_value().to_string();

    state.select_preset(READONLY_PRESET);
    assert!(apply_click(&mut state, field.x + 1, field.y + 1));
    let readonly_value = state.screen_state.text_input_value().to_string();

    assert_eq!("readonly value", readonly_value);
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Character('x')
    ));
    assert_eq!(readonly_value, state.screen_state.text_input_value());
    assert_eq!(
        "text_input_readonly_blocked",
        state.screen_state.last_action
    );
    assert_eq!("text_input_readonly_ignored", state.screen_state.last_event);
    assert!(window_interaction::apply_text_input_key(
        &mut state,
        TextInputKey::Backspace
    ));
    assert_eq!(readonly_value, state.screen_state.text_input_value());

    state.select_preset(VALUE_PRESET);
    assert_eq!(editable_value, state.screen_state.text_input_value());
}

#[test]
fn text_input_runtime_state_is_keyed_by_instance() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };

    state
        .screen_state
        .register_text_input_focus_for("text-input.first", "first", false);
    assert!(
        state
            .screen_state
            .register_text_input_character_for("text-input.first", '1', false)
    );
    state
        .screen_state
        .register_text_input_focus_for("text-input.second", "second", false);
    assert!(
        state
            .screen_state
            .register_text_input_character_for("text-input.second", '2', false)
    );

    assert_eq!(
        "first1",
        state.screen_state.text_input_value_for("text-input.first")
    );
    assert_eq!(
        "second2",
        state.screen_state.text_input_value_for("text-input.second")
    );
}

#[test]
fn text_input_icon_button_preset_emits_callback_action() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        preset_index: ICON_BUTTONS_PRESET,
        ..StorybookWindowState::default()
    };
    let before = render_with_state(&state);
    let origin = preview_detail::component_action_hit_rect(PAGE);
    let rect = super::dedicated_dod_form_input_live::text_input_trailing_icon_button_rects(
        origin.x, origin.y,
    )[0];

    assert!(apply_click(&mut state, rect.x + 1, rect.y + 1));
    assert_eq!("text_input_icon_button", state.screen_state.last_action);
    assert_eq!(
        "text_input_icon_button_clicked",
        state.screen_state.last_event
    );
    assert_eq!("input.trailing_icon", state.screen_state.last_setting_value);

    let after = render_with_state(&state);
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn text_input_light_and_dark_fields_use_theme_tokens() {
    for (theme_id, theme) in [
        (DARK_THEME, ThemeSnapshot::dark()),
        (LIGHT_THEME, ThemeSnapshot::light()),
    ] {
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
    }
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
