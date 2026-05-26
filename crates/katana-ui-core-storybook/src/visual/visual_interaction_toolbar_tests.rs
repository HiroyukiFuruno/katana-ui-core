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
const PAGE: &str = "toolbar";
const OVERFLOW_PRESET: usize = 0;
const SPLIT_PRESET: usize = 1;
const DISPLAY_PRESET: usize = 2;
const DENSITY_PRESET: usize = 3;
const ACCELERATOR_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const BAR_X: usize = 44;
const BAR_Y: usize = 42;
const BAR_SAMPLE_X_OFFSET: usize = 382;
const BAR_SAMPLE_Y_OFFSET: usize = 8;

#[test]
fn toolbar_exposes_leaf_presets_options_and_action_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("tool_toggle", spec.action);
    assert_eq!("tool_changed", spec.event);
    assert_eq!("interaction.active", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("active=true", spec.state);
}

#[test]
fn toolbar_presets_render_distinct_overflow_split_display_density_and_accelerator_states() {
    let overflow = StorybookVisual.render_preset(DARK_THEME, PAGE, OVERFLOW_PRESET, 0);
    let split = StorybookVisual.render_preset(DARK_THEME, PAGE, SPLIT_PRESET, 0);
    let display = StorybookVisual.render_preset(DARK_THEME, PAGE, DISPLAY_PRESET, 0);
    let density = StorybookVisual.render_preset(DARK_THEME, PAGE, DENSITY_PRESET, 0);
    let accelerator = StorybookVisual.render_preset(DARK_THEME, PAGE, ACCELERATOR_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &overflow, &split) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &split, &display) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &display, &density) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &density, &accelerator) > BODY_DIFF_THRESHOLD);
}

#[test]
fn toolbar_setting_option_updates_toolbar_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn toolbar_preview_action_updates_toolbar_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn toolbar_light_and_dark_bar_uses_theme_surface() {
    assert_bar_token(DARK_THEME, ThemeSnapshot::dark());
    assert_bar_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_bar_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, SPLIT_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + BAR_X + BAR_SAMPLE_X_OFFSET,
            component.y + BAR_Y + BAR_SAMPLE_Y_OFFSET
        )
    );
}
