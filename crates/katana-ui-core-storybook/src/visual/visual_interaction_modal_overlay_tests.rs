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
const PAGE: &str = "modal-overlay";
const OVERLAY_PRESET: usize = 0;
const BACKDROP_PRESET: usize = 1;
const ESCAPE_PRESET: usize = 2;
const FOCUS_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const DIALOG_X: usize = 38;
const DIALOG_Y: usize = 42;
const DIALOG_SAMPLE_OFFSET: usize = 8;

#[test]
fn modal_overlay_exposes_leaf_presets_options_and_close_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("overlay_close", spec.action);
    assert_eq!("overlay_closed", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("false", spec.after);
    assert_eq!("open=false", spec.state);
}

#[test]
fn modal_overlay_presets_render_distinct_overlay_backdrop_escape_and_focus_states() {
    let overlay = StorybookVisual.render_preset(DARK_THEME, PAGE, OVERLAY_PRESET, 0);
    let backdrop = StorybookVisual.render_preset(DARK_THEME, PAGE, BACKDROP_PRESET, 0);
    let escape = StorybookVisual.render_preset(DARK_THEME, PAGE, ESCAPE_PRESET, 0);
    let focus = StorybookVisual.render_preset(DARK_THEME, PAGE, FOCUS_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &overlay, &backdrop) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &backdrop, &escape) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &escape, &focus) > BODY_DIFF_THRESHOLD);
}

#[test]
fn modal_overlay_setting_option_updates_dialog_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn modal_overlay_preview_action_updates_dialog_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn modal_overlay_light_and_dark_dialog_uses_theme_surface() {
    assert_dialog_token(DARK_THEME, ThemeSnapshot::dark());
    assert_dialog_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_dialog_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, OVERLAY_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + DIALOG_X + DIALOG_SAMPLE_OFFSET,
            component.y + DIALOG_Y + DIALOG_SAMPLE_OFFSET
        )
    );
}
