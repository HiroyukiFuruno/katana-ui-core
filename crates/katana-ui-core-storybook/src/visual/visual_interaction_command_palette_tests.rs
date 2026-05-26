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
const PAGE: &str = "command-palette";
const PALETTE_PRESET: usize = 0;
const RESULTS_PRESET: usize = 1;
const SLASH_PRESET: usize = 2;
const DISABLED_PRESET: usize = 3;
const VIRTUAL_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn command_palette_exposes_leaf_presets_options_and_query_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("command_palette.query:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("command_palette.highlight:"))
    );
    assert_eq!("command_query_changed", spec.action);
    assert_eq!("command_result_highlighted", spec.event);
    assert_eq!("interaction.value", spec.option);
    assert_eq!("theme", spec.after);
    assert_eq!("highlighted=theme", spec.state);
}

#[test]
fn command_palette_presets_render_distinct_results_slash_disabled_and_virtual_states() {
    let palette = StorybookVisual.render_preset(DARK_THEME, PAGE, PALETTE_PRESET, 0);
    let results = StorybookVisual.render_preset(DARK_THEME, PAGE, RESULTS_PRESET, 0);
    let slash = StorybookVisual.render_preset(DARK_THEME, PAGE, SLASH_PRESET, 0);
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);
    let virtualized = StorybookVisual.render_preset(DARK_THEME, PAGE, VIRTUAL_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &palette, &results) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &results, &slash) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &slash, &disabled) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &disabled, &virtualized) > BODY_DIFF_THRESHOLD);
}

#[test]
fn command_palette_setting_option_updates_query_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn command_palette_preview_action_updates_highlight_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn command_palette_light_and_dark_surface_uses_theme_surface() {
    assert_command_palette_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_command_palette_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_command_palette_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, PALETTE_PRESET, 0);
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
