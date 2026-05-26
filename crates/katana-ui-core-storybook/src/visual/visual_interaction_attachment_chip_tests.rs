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
const PAGE: &str = "attachment-chip";
const FILE_PRESET: usize = 0;
const IMAGE_PRESET: usize = 1;
const URL_PRESET: usize = 2;
const UPLOADING_PRESET: usize = 3;
const ERROR_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn attachment_chip_exposes_leaf_presets_options_and_status_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(rows.iter().any(|row| row.starts_with("attachment.status:")));
    assert!(
        rows.iter()
            .any(|row| row.starts_with("attachment.progress:"))
    );
    assert_eq!("attachment_status", spec.action);
    assert_eq!("attachment_status_changed", spec.event);
    assert_eq!("attachment.status", spec.option);
    assert_eq!("Error", spec.after);
    assert_eq!("status=error", spec.state);
}

#[test]
fn attachment_chip_presets_render_distinct_file_image_url_upload_and_error_states() {
    let file = StorybookVisual.render_preset(DARK_THEME, PAGE, FILE_PRESET, 0);
    let image = StorybookVisual.render_preset(DARK_THEME, PAGE, IMAGE_PRESET, 0);
    let url = StorybookVisual.render_preset(DARK_THEME, PAGE, URL_PRESET, 0);
    let uploading = StorybookVisual.render_preset(DARK_THEME, PAGE, UPLOADING_PRESET, 0);
    let error = StorybookVisual.render_preset(DARK_THEME, PAGE, ERROR_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &file, &image) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &image, &url) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &url, &uploading) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &uploading, &error) > BODY_DIFF_THRESHOLD);
}

#[test]
fn attachment_chip_setting_option_updates_status_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn attachment_chip_preview_action_updates_status_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn attachment_chip_light_and_dark_surface_uses_theme_surface() {
    assert_attachment_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_attachment_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_attachment_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, FILE_PRESET, 0);
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
