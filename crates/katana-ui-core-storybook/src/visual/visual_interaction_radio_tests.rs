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
const PAGE: &str = "radio";
const UNSELECTED_PRESET: usize = 0;
const SELECTED_PRESET: usize = 1;
const GROUP_PRESET: usize = 2;
const FOCUS_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const ROW_FILL_SAMPLE_X_OFFSET: usize = 4;
const ROW_FILL_SAMPLE_Y_OFFSET: usize = 2;
const MARK_FILL_SAMPLE_X_OFFSET: usize = 6;
const MARK_FILL_SAMPLE_Y_OFFSET: usize = 6;

#[test]
fn radio_exposes_leaf_presets_options_and_selected_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("radio_select", spec.action);
    assert_eq!("radio_selected", spec.event);
    assert_eq!("checked", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("before=false after=true", spec.state);
}

#[test]
fn radio_presets_render_distinct_selection_bodies() {
    let unselected = StorybookVisual.render_preset(DARK_THEME, PAGE, UNSELECTED_PRESET, 0);
    let selected = StorybookVisual.render_preset(DARK_THEME, PAGE, SELECTED_PRESET, 0);
    let group = StorybookVisual.render_preset(DARK_THEME, PAGE, GROUP_PRESET, 0);
    let focused = StorybookVisual.render_preset(DARK_THEME, PAGE, FOCUS_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &unselected, &selected) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &selected, &group) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &selected, &focused) > BODY_DIFF_THRESHOLD);
}

#[test]
fn radio_setting_option_updates_selection_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn radio_preview_action_updates_selection_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn radio_light_and_dark_rows_use_theme_tokens() {
    assert_row_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_row_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_row_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let unselected = StorybookVisual.render_preset(theme_id, PAGE, UNSELECTED_PRESET, 0);
    let selected = StorybookVisual.render_preset(theme_id, PAGE, SELECTED_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let row = super::dedicated_dod_form_binary_choice_live::radio_row_rect(0, rect.x, rect.y);
    let mark = super::dedicated_dod_form_binary_choice_live::radio_mark_rect(0, rect.x, rect.y);

    assert_eq!(Some(colors.border), pixel_at(&unselected, row.x, row.y));
    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &unselected,
            row.x + ROW_FILL_SAMPLE_X_OFFSET,
            row.y + ROW_FILL_SAMPLE_Y_OFFSET
        )
    );
    assert_eq!(
        Some(colors.accent),
        pixel_at(
            &selected,
            mark.x + MARK_FILL_SAMPLE_X_OFFSET,
            mark.y + MARK_FILL_SAMPLE_Y_OFFSET
        )
    );
}
