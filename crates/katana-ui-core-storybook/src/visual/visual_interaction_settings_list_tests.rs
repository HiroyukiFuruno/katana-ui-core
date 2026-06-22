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
const PAGE: &str = "settings-list";
const APP_PRESET: usize = 0;
const CHAT_PRESET: usize = 1;
const LINT_PRESET: usize = 2;
const DIRTY_PRESET: usize = 3;
const QUERY_PRESET: usize = 4;
const RESET_PRESET: usize = 5;
const LABEL_PRESET: usize = 6;
const SECTION_LABEL_PRESET: usize = 7;
const SECTION_DESCRIPTION_PRESET: usize = 8;
const SECTION_ICON_PRESET: usize = 9;
const FIELD_COUNT_PRESET: usize = 10;
const SECTION_FOOTER_PRESET: usize = 11;
const SECTION_COLLAPSE_PRESET: usize = 12;
const DEFAULT_COLLAPSED_PRESET: usize = 13;
const FIELD_LABEL_PRESET: usize = 14;
const FIELD_DESCRIPTION_PRESET: usize = 15;
const CONTROL_OPTIONS_PRESET: usize = 16;
const CUSTOM_CONTROL_PRESET: usize = 17;
const SET_VALUE_PRESET: usize = 18;
const REQUIRED_PRESET_COUNT: usize = 19;
const REQUIRED_OPTION_COUNT: usize = 19;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn settings_list_exposes_leaf_presets_options_and_field_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    for option in [
        "settings_list.label",
        "settings_list.density",
        "settings_list.dirty_visualization",
        "settings_list.query",
        "settings_list.sections",
        "settings_list.section_label",
        "settings_list.section_description",
        "settings_list.section_icon",
        "settings_list.field_count",
        "settings_list.section_footer",
        "settings_list.section_collapsible",
        "settings_list.default_collapsed",
        "settings_list.field_label",
        "settings_list.field_description",
        "settings_list.control_kind",
        "settings_list.control_options",
        "settings_list.custom_control",
        "settings_list.set_value",
        "settings_list.reset",
    ] {
        assert!(
            options.iter().any(|it| it.setting == option),
            "settings-list option is not exposed: {option}"
        );
    }
    assert!(
        rows.iter()
            .any(|row| row.starts_with("settings_list.density:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("settings_list.query:"))
    );
    assert_eq!("settings_filter_update_collapse", spec.action);
    assert_eq!("settings_field_changed", spec.event);
    assert_eq!("settings.query", spec.option);
    assert_eq!("font", spec.after);
    assert_eq!("dirty=font-size", spec.state);
}

#[test]
fn settings_list_presets_render_distinct_sections_dirty_query_and_reset_states() {
    let app = StorybookVisual.render_preset(DARK_THEME, PAGE, APP_PRESET, 0);
    let chat = StorybookVisual.render_preset(DARK_THEME, PAGE, CHAT_PRESET, 0);
    let lint = StorybookVisual.render_preset(DARK_THEME, PAGE, LINT_PRESET, 0);
    let dirty = StorybookVisual.render_preset(DARK_THEME, PAGE, DIRTY_PRESET, 0);
    let query = StorybookVisual.render_preset(DARK_THEME, PAGE, QUERY_PRESET, 0);
    let reset = StorybookVisual.render_preset(DARK_THEME, PAGE, RESET_PRESET, 0);
    let label = StorybookVisual.render_preset(DARK_THEME, PAGE, LABEL_PRESET, 0);
    let section_label = StorybookVisual.render_preset(DARK_THEME, PAGE, SECTION_LABEL_PRESET, 0);
    let section_description =
        StorybookVisual.render_preset(DARK_THEME, PAGE, SECTION_DESCRIPTION_PRESET, 0);
    let section_icon = StorybookVisual.render_preset(DARK_THEME, PAGE, SECTION_ICON_PRESET, 0);
    let field_count = StorybookVisual.render_preset(DARK_THEME, PAGE, FIELD_COUNT_PRESET, 0);
    let section_footer = StorybookVisual.render_preset(DARK_THEME, PAGE, SECTION_FOOTER_PRESET, 0);
    let section_collapse =
        StorybookVisual.render_preset(DARK_THEME, PAGE, SECTION_COLLAPSE_PRESET, 0);
    let default_collapsed =
        StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_COLLAPSED_PRESET, 0);
    let field_label = StorybookVisual.render_preset(DARK_THEME, PAGE, FIELD_LABEL_PRESET, 0);
    let field_description =
        StorybookVisual.render_preset(DARK_THEME, PAGE, FIELD_DESCRIPTION_PRESET, 0);
    let control_options =
        StorybookVisual.render_preset(DARK_THEME, PAGE, CONTROL_OPTIONS_PRESET, 0);
    let custom_control = StorybookVisual.render_preset(DARK_THEME, PAGE, CUSTOM_CONTROL_PRESET, 0);
    let set_value = StorybookVisual.render_preset(DARK_THEME, PAGE, SET_VALUE_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &app, &chat) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &chat, &lint) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &lint, &dirty) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &dirty, &query) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &query, &reset) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &reset, &label) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &label, &section_label) > BODY_DIFF_THRESHOLD);
    assert!(
        component_body_pixel_diff(PAGE, &section_label, &section_description) > BODY_DIFF_THRESHOLD
    );
    assert!(
        component_body_pixel_diff(PAGE, &section_description, &section_icon) > BODY_DIFF_THRESHOLD
    );
    assert!(component_body_pixel_diff(PAGE, &section_icon, &field_count) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &field_count, &section_footer) > BODY_DIFF_THRESHOLD);
    assert!(
        component_body_pixel_diff(PAGE, &section_footer, &section_collapse) > BODY_DIFF_THRESHOLD
    );
    assert!(
        component_body_pixel_diff(PAGE, &section_collapse, &default_collapsed)
            > BODY_DIFF_THRESHOLD
    );
    assert!(
        component_body_pixel_diff(PAGE, &default_collapsed, &field_label) > BODY_DIFF_THRESHOLD
    );
    assert!(
        component_body_pixel_diff(PAGE, &field_label, &field_description) > BODY_DIFF_THRESHOLD
    );
    assert!(
        component_body_pixel_diff(PAGE, &field_description, &control_options) > BODY_DIFF_THRESHOLD
    );
    assert!(
        component_body_pixel_diff(PAGE, &control_options, &custom_control) > BODY_DIFF_THRESHOLD
    );
    assert!(component_body_pixel_diff(PAGE, &custom_control, &set_value) > BODY_DIFF_THRESHOLD);
}

#[test]
fn settings_list_setting_option_updates_query_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn settings_list_preview_action_updates_field_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn settings_list_light_and_dark_surface_uses_theme_surface() {
    assert_settings_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_settings_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_settings_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, APP_PRESET, 0);
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
