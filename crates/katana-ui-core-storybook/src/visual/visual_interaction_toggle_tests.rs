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
const PAGE: &str = "toggle";
const OFF_PRESET: usize = 0;
const ON_PRESET: usize = 1;
const DISABLED_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const ROW_FILL_SAMPLE_X_OFFSET: usize = 4;
const ROW_FILL_SAMPLE_Y_OFFSET: usize = 4;
const SWITCH_TRACK_SAMPLE_X_OFFSET: usize = 6;

#[test]
fn toggle_exposes_leaf_presets_options_and_checked_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("toggle_change", spec.action);
    assert_eq!("toggle_changed", spec.event);
    assert_eq!("checked", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("checked=true", spec.state);
}

#[test]
fn toggle_presets_render_distinct_switch_bodies() {
    let off = StorybookVisual.render_preset(DARK_THEME, PAGE, OFF_PRESET, 0);
    let on = StorybookVisual.render_preset(DARK_THEME, PAGE, ON_PRESET, 0);
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &off, &on) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &off, &disabled) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &disabled, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn toggle_setting_option_updates_switch_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn toggle_preview_action_updates_switch_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn toggle_light_and_dark_rows_use_theme_tokens() {
    assert_row_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_row_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_row_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let off = StorybookVisual.render_preset(theme_id, PAGE, OFF_PRESET, 0);
    let on = StorybookVisual.render_preset(theme_id, PAGE, ON_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let row = preview_detail::component_action_hit_rect(PAGE);
    let switch = super::dedicated_dod_atom_buttons::toggle_switch_rect_for_test();

    assert_eq!(Some(colors.border), pixel_at(&off, row.x, row.y));
    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &off,
            row.x + ROW_FILL_SAMPLE_X_OFFSET,
            row.y + ROW_FILL_SAMPLE_Y_OFFSET
        )
    );
    assert_eq!(
        Some(colors.accent),
        pixel_at(
            &on,
            switch.x + SWITCH_TRACK_SAMPLE_X_OFFSET,
            switch.y + switch.height / 2
        )
    );
}
