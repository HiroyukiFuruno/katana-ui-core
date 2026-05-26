use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{
    StorybookVisual, palette, preview_detail, selection_control_metrics as sm,
    storybook_ui_option_contract,
};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "select-box";
const TRIGGER_PRESET: usize = 0;
const SELECT_PRESET: usize = 1;
const LONG_LIST_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const TRIGGER_FILL_SAMPLE_X_OFFSET: usize = 4;
const TRIGGER_FILL_SAMPLE_Y_OFFSET: usize = 4;

#[test]
fn select_box_exposes_leaf_presets_options_and_select_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("select_option", spec.action);
    assert_eq!("select_changed", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("false", spec.after);
    assert_eq!("selected=true", spec.state);
}

#[test]
fn select_box_presets_render_distinct_dropdown_bodies() {
    let trigger = StorybookVisual.render_preset(DARK_THEME, PAGE, TRIGGER_PRESET, 0);
    let selected = StorybookVisual.render_preset(DARK_THEME, PAGE, SELECT_PRESET, 0);
    let long_list = StorybookVisual.render_preset(DARK_THEME, PAGE, LONG_LIST_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &trigger, &selected) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &selected, &long_list) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &trigger, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn select_box_setting_option_updates_dropdown_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn select_box_preview_action_updates_dropdown_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn select_box_light_and_dark_trigger_uses_theme_tokens() {
    assert_trigger_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_trigger_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_trigger_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, TRIGGER_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);
    let trigger = sm::trigger_rect(component);

    assert_eq!(Some(colors.border), pixel_at(&canvas, trigger.x, trigger.y));
    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            trigger.x + TRIGGER_FILL_SAMPLE_X_OFFSET,
            trigger.y + TRIGGER_FILL_SAMPLE_Y_OFFSET
        )
    );
}
