use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{component_body_pixel_diff, pixel_at};
use super::{StorybookVisual, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "badge";
const DEFAULT_PRESET: usize = 0;
const PASSIVE_PRESET: usize = 1;
const SMALL_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SAMPLE_X_OFFSET: usize = 22;
const SAMPLE_Y_OFFSET: usize = 44;

#[test]
fn badge_exposes_leaf_presets_options_and_passive_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("badge_passive", spec.action);
    assert_eq!("none", spec.event);
    assert_eq!("use Chip for dismiss", spec.state);
}

#[test]
fn badge_presets_render_distinct_passive_bodies() {
    let tone_grid = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let passive = StorybookVisual.render_preset(DARK_THEME, PAGE, PASSIVE_PRESET, 0);
    let small = StorybookVisual.render_preset(DARK_THEME, PAGE, SMALL_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &tone_grid, &passive) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &passive, &small) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &small, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn badge_preview_action_changes_passive_style_evidence() {
    let before = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let after = StorybookVisual.render_clicked_preset_with_scrollbar(
        DARK_THEME,
        PAGE,
        DEFAULT_PRESET,
        0,
        true,
    );

    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn badge_light_and_dark_surfaces_use_theme_tokens() {
    let dark = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let light = StorybookVisual.render_preset(LIGHT_THEME, PAGE, DEFAULT_PRESET, 0);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let sample_x = rect.x + SAMPLE_X_OFFSET;
    let sample_y = rect.y + SAMPLE_Y_OFFSET;

    assert_ne!(
        pixel_at(&dark, sample_x, sample_y),
        pixel_at(&light, sample_x, sample_y)
    );
}
