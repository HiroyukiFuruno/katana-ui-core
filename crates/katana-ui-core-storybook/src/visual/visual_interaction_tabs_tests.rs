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
const PAGE: &str = "tabs";
const BROWSER_PRESET: usize = 0;
const SWITCH_PRESET: usize = 1;
const OVERFLOW_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const PANEL_X: usize = 42;
const PANEL_Y: usize = 70;
const PANEL_SAMPLE_X_OFFSET: usize = 300;
const PANEL_SAMPLE_Y_OFFSET: usize = 20;

#[test]
fn tabs_exposes_leaf_presets_options_and_selection_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("tab_select", spec.action);
    assert_eq!("tab_changed", spec.event);
    assert_eq!("interaction.selected_index", spec.option);
    assert_eq!("1", spec.after);
    assert_eq!("tab=1", spec.state);
}

#[test]
fn tabs_presets_render_distinct_browser_switch_overflow_and_theme_states() {
    let browser = StorybookVisual.render_preset(DARK_THEME, PAGE, BROWSER_PRESET, 0);
    let switch = StorybookVisual.render_preset(DARK_THEME, PAGE, SWITCH_PRESET, 0);
    let overflow = StorybookVisual.render_preset(DARK_THEME, PAGE, OVERFLOW_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &browser, &switch) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &switch, &overflow) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &browser, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn tabs_setting_option_updates_tab_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn tabs_preview_action_updates_selected_tab_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn tabs_light_and_dark_panel_uses_theme_surface() {
    assert_panel_token(DARK_THEME, ThemeSnapshot::dark());
    assert_panel_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_panel_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, BROWSER_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + PANEL_X + PANEL_SAMPLE_X_OFFSET,
            component.y + PANEL_Y + PANEL_SAMPLE_Y_OFFSET
        )
    );
}
