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
const PAGE: &str = "diagnostics-list";
const LINT_PRESET: usize = 0;
const EDITOR_PRESET: usize = 1;
const TOOL_PRESET: usize = 2;
const EMPTY_PRESET: usize = 3;
const LOADING_PRESET: usize = 4;
const BULK_PRESET: usize = 5;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn diagnostics_list_exposes_leaf_presets_options_and_fix_preview_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("diagnostics.group_by:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("diagnostics.fix_preview:"))
    );
    assert_eq!("diagnostic_fix_preview", spec.action);
    assert_eq!("diagnostic_fix_preview_toggled", spec.event);
    assert_eq!("diagnostics.group_by", spec.option);
    assert_eq!("Severity", spec.after);
    assert_eq!("preview=true", spec.state);
}

#[test]
fn diagnostics_list_presets_render_distinct_result_empty_loading_and_bulk_states() {
    let lint = StorybookVisual.render_preset(DARK_THEME, PAGE, LINT_PRESET, 0);
    let editor = StorybookVisual.render_preset(DARK_THEME, PAGE, EDITOR_PRESET, 0);
    let tool = StorybookVisual.render_preset(DARK_THEME, PAGE, TOOL_PRESET, 0);
    let empty = StorybookVisual.render_preset(DARK_THEME, PAGE, EMPTY_PRESET, 0);
    let loading = StorybookVisual.render_preset(DARK_THEME, PAGE, LOADING_PRESET, 0);
    let bulk = StorybookVisual.render_preset(DARK_THEME, PAGE, BULK_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &lint, &editor) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &editor, &tool) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &tool, &empty) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &empty, &loading) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &loading, &bulk) > BODY_DIFF_THRESHOLD);
}

#[test]
fn diagnostics_list_setting_option_updates_grouping_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn diagnostics_list_preview_action_updates_fix_preview_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn diagnostics_list_light_and_dark_surface_uses_theme_surface() {
    assert_diagnostics_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_diagnostics_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_diagnostics_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, LINT_PRESET, 0);
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
