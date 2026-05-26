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
const PAGE: &str = "color-swatch";
const PALETTE_PRESET: usize = 0;
const SELECT_PRESET: usize = 1;
const DISABLED_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SWATCH_X: usize = 18;
const SWATCH_Y: usize = 38;
const SWATCH_SAMPLE_OFFSET: usize = 4;

#[test]
fn color_swatch_exposes_leaf_presets_options_and_color_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("color_select", spec.action);
    assert_eq!("color_changed", spec.event);
    assert_eq!("color_swatch.selected_color", spec.option);
    assert_eq!("rgba(64,128,255,1)", spec.after);
    assert_eq!("color=accent", spec.state);
}

#[test]
fn color_swatch_presets_render_distinct_palette_bodies() {
    let palette = StorybookVisual.render_preset(DARK_THEME, PAGE, PALETTE_PRESET, 0);
    let selected = StorybookVisual.render_preset(DARK_THEME, PAGE, SELECT_PRESET, 0);
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &palette, &selected) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &selected, &disabled) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &palette, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn color_swatch_setting_option_updates_palette_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn color_swatch_preview_action_updates_palette_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn color_swatch_light_and_dark_first_swatch_uses_theme_accent() {
    assert_first_swatch_token(DARK_THEME, ThemeSnapshot::dark());
    assert_first_swatch_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_first_swatch_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, PALETTE_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.accent),
        pixel_at(
            &canvas,
            component.x + SWATCH_X + SWATCH_SAMPLE_OFFSET,
            component.y + SWATCH_Y + SWATCH_SAMPLE_OFFSET
        )
    );
}
