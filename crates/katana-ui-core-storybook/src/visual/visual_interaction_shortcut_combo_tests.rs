use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use crate::visual::window_interaction::{
    StorybookWindowState, apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    focus_clickable_at_for_audit,
};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "shortcut-combo";
const MAC_PRESET: usize = 0;
const WINDOWS_PRESET: usize = 1;
const LINUX_PRESET: usize = 2;
const SEPARATOR_PRESET: usize = 3;
const A11Y_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 5;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn shortcut_combo_exposes_leaf_presets_options_and_platform_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert_eq!(options.len(), REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("shortcut_combo.platform_display:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("shortcut_combo.tone:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("shortcut_combo.a11y_label:"))
    );
    assert_eq!("shortcut_platform_preview", spec.action);
    assert_eq!("shortcut_display_changed", spec.event);
    assert_eq!("shortcut.platform", spec.option);
    assert_eq!("MacOS", spec.after);
    assert_eq!("combo=Command+K", spec.state);
}

#[test]
fn shortcut_combo_presets_render_distinct_platform_separator_and_a11y_states() {
    let mac = StorybookVisual.render_preset(DARK_THEME, PAGE, MAC_PRESET, 0);
    let windows = StorybookVisual.render_preset(DARK_THEME, PAGE, WINDOWS_PRESET, 0);
    let linux = StorybookVisual.render_preset(DARK_THEME, PAGE, LINUX_PRESET, 0);
    let separator = StorybookVisual.render_preset(DARK_THEME, PAGE, SEPARATOR_PRESET, 0);
    let a11y = StorybookVisual.render_preset(DARK_THEME, PAGE, A11Y_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &mac, &windows) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &windows, &linux) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &linux, &separator) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &separator, &a11y) > BODY_DIFF_THRESHOLD);
}

#[test]
fn shortcut_combo_setting_option_updates_platform_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn shortcut_combo_preview_action_updates_platform_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn shortcut_combo_live_hover_focus_and_keyboard_use_public_display_api() {
    let target = preview_detail::component_action_hit_rect(PAGE);
    let mut hover = state_for();
    let before_hover = render_state(&hover);
    assert!(apply_hover_at(&mut hover, target.x + 1, target.y + 1));
    let after_hover = render_state(&hover);
    assert_eq!("shortcut_combo_hover", hover.screen_state.last_action);
    assert_eq!("hover_start", hover.screen_state.last_event);
    assert_eq!("hover=true", hover.screen_state.state_label);
    assert!(hover.screen_state.runtime_structured.shortcut_combo.hovered);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let mut keyboard = state_for();
    assert!(focus_clickable_at_for_audit(
        &mut keyboard,
        target.x + 1,
        target.y + 1
    ));
    assert_eq!("shortcut_combo_focus", keyboard.screen_state.last_action);
    assert_eq!("focus", keyboard.screen_state.last_event);
    assert_eq!("focus=true", keyboard.screen_state.state_label);
    let before_key = render_state(&keyboard);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut keyboard));
    let after_key = render_state(&keyboard);
    assert_eq!(
        "shortcut_platform_preview",
        keyboard.screen_state.last_action
    );
    assert_eq!("shortcut_display_changed", keyboard.screen_state.last_event);
    assert_eq!("combo=Command+K", keyboard.screen_state.state_label);
    assert!(
        keyboard
            .screen_state
            .runtime_structured
            .shortcut_combo
            .platform_preview_macos
    );
    assert!(component_body_pixel_diff(PAGE, &before_key, &after_key) > 0);
}

#[test]
fn shortcut_combo_light_and_dark_surface_uses_theme_surface() {
    assert_shortcut_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_shortcut_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn state_for() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    super::render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn assert_shortcut_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, MAC_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + SURFACE_TOKEN_X + SURFACE_SAMPLE_X_OFFSET,
            component.y + SURFACE_TOKEN_Y + SURFACE_SAMPLE_Y_OFFSET
        )
    );
}
