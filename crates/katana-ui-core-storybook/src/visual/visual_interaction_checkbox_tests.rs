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
const PAGE: &str = "checkbox";
const UNCHECKED_PRESET: usize = 0;
const CHECKED_PRESET: usize = 1;
const DISABLED_PRESET: usize = 2;
const FOCUS_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const ROW_FILL_SAMPLE_X_OFFSET: usize = 4;
const ROW_FILL_SAMPLE_Y_OFFSET: usize = 2;
const MARK_FILL_SAMPLE_X_OFFSET: usize = 6;
const MARK_FILL_SAMPLE_Y_OFFSET: usize = 6;

#[test]
fn checkbox_exposes_leaf_presets_options_and_checked_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("checkbox_toggle", spec.action);
    assert_eq!("checked_changed", spec.event);
    assert_eq!("checked", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("before=false after=true", spec.state);
}

#[test]
fn checkbox_presets_render_distinct_selection_bodies() {
    let unchecked = StorybookVisual.render_preset(DARK_THEME, PAGE, UNCHECKED_PRESET, 0);
    let checked = StorybookVisual.render_preset(DARK_THEME, PAGE, CHECKED_PRESET, 0);
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);
    let focused = StorybookVisual.render_preset(DARK_THEME, PAGE, FOCUS_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &unchecked, &checked) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &checked, &disabled) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &checked, &focused) > BODY_DIFF_THRESHOLD);
}

#[test]
fn checkbox_setting_option_updates_selection_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn checkbox_preview_action_updates_selection_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn checkbox_light_and_dark_rows_use_theme_tokens() {
    assert_row_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_row_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_row_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let unchecked = StorybookVisual.render_preset(theme_id, PAGE, UNCHECKED_PRESET, 0);
    let checked = StorybookVisual.render_preset(theme_id, PAGE, CHECKED_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let row = super::dedicated_dod_form_binary_choice_live::checkbox_row_rect(0, rect.x, rect.y);
    let mark = super::dedicated_dod_form_binary_choice_live::checkbox_mark_rect(0, rect.x, rect.y);

    assert_eq!(Some(colors.border), pixel_at(&unchecked, row.x, row.y));
    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &unchecked,
            row.x + ROW_FILL_SAMPLE_X_OFFSET,
            row.y + ROW_FILL_SAMPLE_Y_OFFSET
        )
    );
    assert_eq!(
        Some(colors.accent),
        pixel_at(
            &checked,
            mark.x + MARK_FILL_SAMPLE_X_OFFSET,
            mark.y + MARK_FILL_SAMPLE_Y_OFFSET
        )
    );
}
