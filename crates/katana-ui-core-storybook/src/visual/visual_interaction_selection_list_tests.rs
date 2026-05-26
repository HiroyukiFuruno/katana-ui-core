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
const PAGE: &str = "selection-list";
const ITEMS_PRESET: usize = 0;
const SELECT_PRESET: usize = 1;
const MULTI_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const LIST_FILL_SAMPLE_X_OFFSET: usize = 4;
const LIST_FILL_SAMPLE_Y_OFFSET: usize = 4;

#[test]
fn selection_list_exposes_leaf_presets_options_and_selection_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("selection_toggle", spec.action);
    assert_eq!("selection_changed", spec.event);
    assert_eq!("interaction.selected_index", spec.option);
    assert_eq!("2", spec.after);
    assert_eq!("selected=2", spec.state);
}

#[test]
fn selection_list_presets_render_distinct_list_bodies() {
    let items = StorybookVisual.render_preset(DARK_THEME, PAGE, ITEMS_PRESET, 0);
    let selected = StorybookVisual.render_preset(DARK_THEME, PAGE, SELECT_PRESET, 0);
    let multi = StorybookVisual.render_preset(DARK_THEME, PAGE, MULTI_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &items, &selected) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &selected, &multi) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &items, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn selection_list_setting_option_updates_list_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn selection_list_preview_action_updates_list_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn selection_list_light_and_dark_rows_use_theme_tokens() {
    assert_list_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_list_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_list_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, ITEMS_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);
    let list_x = component.x + sm::TRIGGER_X;
    let list_y = component.y + sm::SELECTION_LIST_Y;
    let border_y = list_y + sm::SELECTION_LIST_ROW_HEIGHT - 1;

    assert_eq!(Some(colors.border), pixel_at(&canvas, list_x, border_y));
    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            list_x + LIST_FILL_SAMPLE_X_OFFSET,
            list_y + LIST_FILL_SAMPLE_Y_OFFSET
        )
    );
}
