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
const PAGE: &str = "card";
const SLOTS_PRESET: usize = 0;
const CLICK_PRESET: usize = 1;
const NESTED_PRESET: usize = 2;
const THEME_BORDER_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const CARD_X: usize = 22;
const CARD_Y: usize = 30;
const CARD_SAMPLE_X_OFFSET: usize = 260;
const CARD_SAMPLE_Y_OFFSET: usize = 64;

#[test]
fn card_exposes_leaf_presets_options_and_activation_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(rows.iter().any(|row| row.starts_with("card.variant:")));
    assert!(
        rows.iter()
            .any(|row| row.starts_with("card.nested_controls:"))
    );
    assert_eq!("card_click", spec.action);
    assert_eq!("card_activated", spec.event);
    assert_eq!("interaction.active", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("active=true", spec.state);
}

#[test]
fn card_presets_render_distinct_slot_click_nested_and_theme_states() {
    let slots = StorybookVisual.render_preset(DARK_THEME, PAGE, SLOTS_PRESET, 0);
    let click = StorybookVisual.render_preset(DARK_THEME, PAGE, CLICK_PRESET, 0);
    let nested = StorybookVisual.render_preset(DARK_THEME, PAGE, NESTED_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_BORDER_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &slots, &click) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &click, &nested) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &nested, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn card_setting_option_updates_card_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn card_preview_action_updates_activation_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn card_light_and_dark_surface_uses_theme_surface() {
    assert_card_token(DARK_THEME, ThemeSnapshot::dark());
    assert_card_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_card_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, SLOTS_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + CARD_X + CARD_SAMPLE_X_OFFSET,
            component.y + CARD_Y + CARD_SAMPLE_Y_OFFSET
        )
    );
}
