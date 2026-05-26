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
const PAGE: &str = "code-diff";
const LEFT_RIGHT_PRESET: usize = 0;
const TOP_BOTTOM_PRESET: usize = 1;
const INLINE_PRESET: usize = 2;
const COLLAPSED_PRESET: usize = 3;
const JAPANESE_WHITESPACE_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 20;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn code_diff_exposes_leaf_presets_options_and_mode_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(rows.iter().any(|row| row.starts_with("code_diff.mode:")));
    assert!(
        rows.iter()
            .any(|row| row.starts_with("code_diff.whitespace:"))
    );
    assert_eq!("diff_mode_switch", spec.action);
    assert_eq!("diff_mode_changed", spec.event);
    assert_eq!("interaction.value", spec.option);
    assert_eq!("Split", spec.after);
    assert_eq!("mode=split", spec.state);
}

#[test]
fn code_diff_presets_render_distinct_split_inline_collapsed_and_whitespace_states() {
    let left_right = StorybookVisual.render_preset(DARK_THEME, PAGE, LEFT_RIGHT_PRESET, 0);
    let top_bottom = StorybookVisual.render_preset(DARK_THEME, PAGE, TOP_BOTTOM_PRESET, 0);
    let inline = StorybookVisual.render_preset(DARK_THEME, PAGE, INLINE_PRESET, 0);
    let collapsed = StorybookVisual.render_preset(DARK_THEME, PAGE, COLLAPSED_PRESET, 0);
    let whitespace = StorybookVisual.render_preset(DARK_THEME, PAGE, JAPANESE_WHITESPACE_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &left_right, &top_bottom) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &top_bottom, &inline) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &inline, &collapsed) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &collapsed, &whitespace) > BODY_DIFF_THRESHOLD);
}

#[test]
fn code_diff_setting_option_updates_mode_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn code_diff_preview_action_updates_mode_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn code_diff_light_and_dark_surface_token_uses_theme_surface() {
    assert_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, LEFT_RIGHT_PRESET, 0);
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
