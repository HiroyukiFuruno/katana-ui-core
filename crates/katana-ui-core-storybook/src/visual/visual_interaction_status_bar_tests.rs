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
const PAGE: &str = "status-bar";
const EDITOR_PRESET: usize = 0;
const CHAT_PRESET: usize = 1;
const LINTER_PRESET: usize = 2;
const PROGRESS_PRESET: usize = 3;
const POPOVER_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn status_bar_exposes_leaf_presets_options_and_popover_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert_eq!(options.len(), REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(rows.iter().any(|row| row.starts_with("status_bar.mode:")));
    assert!(
        rows.iter()
            .any(|row| row.starts_with("status_bar.progress_popover:"))
    );
    assert_eq!("status_bar_segment_popover", spec.action);
    assert_eq!("status_bar_popover_opened", spec.event);
    assert_eq!("status_bar.open_popover", spec.option);
    assert_eq!("branch", spec.after);
    assert_eq!("open_popover=branch", spec.state);
}

#[test]
fn status_bar_presets_render_distinct_editor_chat_linter_progress_and_popover_states() {
    let editor = StorybookVisual.render_preset(DARK_THEME, PAGE, EDITOR_PRESET, 0);
    let chat = StorybookVisual.render_preset(DARK_THEME, PAGE, CHAT_PRESET, 0);
    let linter = StorybookVisual.render_preset(DARK_THEME, PAGE, LINTER_PRESET, 0);
    let progress = StorybookVisual.render_preset(DARK_THEME, PAGE, PROGRESS_PRESET, 0);
    let popover = StorybookVisual.render_preset(DARK_THEME, PAGE, POPOVER_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &editor, &chat) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &chat, &linter) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &linter, &progress) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &progress, &popover) > BODY_DIFF_THRESHOLD);
}

#[test]
fn status_bar_setting_option_updates_segment_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn status_bar_preview_action_updates_popover_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn status_bar_light_and_dark_surface_uses_theme_surface() {
    assert_status_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_status_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_status_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, EDITOR_PRESET, 0);
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
