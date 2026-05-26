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
const PAGE: &str = "accordion";
const CLOSED_PRESET: usize = 0;
const OPEN_PRESET: usize = 1;
const DISABLED_PRESET: usize = 2;
const CONTROLLED_PRESET: usize = 3;
const MULTIPLE_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_SAMPLE_X: usize = 210;
const SURFACE_SAMPLE_Y: usize = 46;

#[test]
fn accordion_exposes_leaf_presets_options_and_toggle_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("accordion.expanded:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("accordion.trigger_area:"))
    );
    assert_eq!("accordion_toggle", spec.action);
    assert_eq!("accordion_changed", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("false", spec.after);
    assert_eq!("open=false", spec.state);
}

#[test]
fn accordion_presets_render_distinct_open_disabled_controlled_and_multiple_states() {
    let closed = StorybookVisual.render_preset(DARK_THEME, PAGE, CLOSED_PRESET, 0);
    let open = StorybookVisual.render_preset(DARK_THEME, PAGE, OPEN_PRESET, 0);
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);
    let controlled = StorybookVisual.render_preset(DARK_THEME, PAGE, CONTROLLED_PRESET, 0);
    let multiple = StorybookVisual.render_preset(DARK_THEME, PAGE, MULTIPLE_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &closed, &open) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &open, &disabled) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &disabled, &controlled) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &controlled, &multiple) > BODY_DIFF_THRESHOLD);
}

#[test]
fn accordion_setting_option_updates_controlled_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn accordion_preview_action_updates_open_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn accordion_light_and_dark_header_uses_theme_surface() {
    assert_accordion_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_accordion_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_accordion_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, CLOSED_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + SURFACE_SAMPLE_X,
            component.y + SURFACE_SAMPLE_Y
        )
    );
}
