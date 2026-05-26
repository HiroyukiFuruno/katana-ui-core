use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "chip";
const FILTER_PRESET: usize = 0;
const DISMISS_PRESET: usize = 1;
const SELECTED_PRESET: usize = 2;
const TONE_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn chip_exposes_leaf_presets_options_and_dismiss_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(rows.iter().any(|row| row.starts_with("chip.variant:")));
    assert!(rows.iter().any(|row| row.starts_with("chip.dismissible:")));
    assert_eq!("chip_dismiss", spec.action);
    assert_eq!("chip_dismissed", spec.event);
    assert_eq!("chip.dismissible", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("dismissed=true", spec.state);
}

#[test]
fn chip_presets_render_distinct_filter_dismiss_selected_and_tone_states() {
    let filter = StorybookVisual.render_preset(DARK_THEME, PAGE, FILTER_PRESET, 0);
    let dismiss = StorybookVisual.render_preset(DARK_THEME, PAGE, DISMISS_PRESET, 0);
    let selected = StorybookVisual.render_preset(DARK_THEME, PAGE, SELECTED_PRESET, 0);
    let tone = StorybookVisual.render_preset(DARK_THEME, PAGE, TONE_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &filter, &dismiss) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &dismiss, &selected) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &selected, &tone) > BODY_DIFF_THRESHOLD);
}

#[test]
fn chip_setting_option_updates_chip_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn chip_preview_action_updates_dismiss_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn chip_light_and_dark_surface_uses_theme_surface() {
    assert_chip_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_chip_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_chip_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, FILTER_PRESET, 0);
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
