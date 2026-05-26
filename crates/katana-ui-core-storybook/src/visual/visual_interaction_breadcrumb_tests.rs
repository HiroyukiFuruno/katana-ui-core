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
const PAGE: &str = "breadcrumb";
const TRAIL_PRESET: usize = 0;
const CLICK_PRESET: usize = 1;
const OVERFLOW_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const BAR_X: usize = 38;
const BAR_Y: usize = 46;
const BAR_SAMPLE_X_OFFSET: usize = 350;
const BAR_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn breadcrumb_exposes_leaf_presets_options_and_route_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("breadcrumb_click", spec.action);
    assert_eq!("route_changed", spec.event);
    assert_eq!("interaction.selected_index", spec.option);
    assert_eq!("2", spec.after);
    assert_eq!("route=2", spec.state);
}

#[test]
fn breadcrumb_presets_render_distinct_trail_click_overflow_and_theme_states() {
    let trail = StorybookVisual.render_preset(DARK_THEME, PAGE, TRAIL_PRESET, 0);
    let click = StorybookVisual.render_preset(DARK_THEME, PAGE, CLICK_PRESET, 0);
    let overflow = StorybookVisual.render_preset(DARK_THEME, PAGE, OVERFLOW_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &trail, &click) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &click, &overflow) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &trail, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn breadcrumb_setting_option_updates_route_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn breadcrumb_preview_action_updates_route_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn breadcrumb_light_and_dark_bar_uses_theme_surface() {
    assert_bar_token(DARK_THEME, ThemeSnapshot::dark());
    assert_bar_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_bar_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, TRAIL_PRESET, 0);
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
