use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{StorybookWindowState, apply_hover_at};
use super::{StorybookVisual, palette, preview_detail, render, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "menu-button";
const TRIGGER_PRESET: usize = 0;
const OPEN_PRESET: usize = 1;
const DISABLED_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const TRIGGER_X: usize = 34;
const TRIGGER_Y: usize = 34;
const TRIGGER_SAMPLE_OFFSET: usize = 8;

#[test]
fn menu_button_exposes_leaf_presets_options_and_trigger_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("menu_button_open", spec.action);
    assert_eq!("menu_button_opened", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("open=true", spec.state);
    assert!(
        options
            .iter()
            .any(|option| option.setting == "menu.select_action")
    );
    assert!(
        !options
            .iter()
            .any(|option| option.setting == "theme.marker")
    );
}

#[test]
fn menu_button_presets_render_distinct_trigger_and_menu_states() {
    let trigger = StorybookVisual.render_preset(DARK_THEME, PAGE, TRIGGER_PRESET, 0);
    let open = StorybookVisual.render_preset(DARK_THEME, PAGE, OPEN_PRESET, 0);
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &trigger, &open) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &open, &disabled) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &trigger, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn menu_button_setting_option_updates_trigger_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn menu_button_preview_action_opens_menu_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn menu_button_light_and_dark_trigger_uses_theme_surface() {
    assert_trigger_token(DARK_THEME, ThemeSnapshot::dark());
    assert_trigger_token(LIGHT_THEME, ThemeSnapshot::light());
}

#[test]
fn menu_button_hover_draws_shared_button_family_border_token() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let before = StorybookVisual.render_preset(DARK_THEME, PAGE, TRIGGER_PRESET, 0);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert!(apply_hover_at(
        &mut state,
        component.x + TRIGGER_X + TRIGGER_SAMPLE_OFFSET,
        component.y + TRIGGER_Y + TRIGGER_SAMPLE_OFFSET
    ));
    let after = render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        TRIGGER_PRESET,
        state.screen_state.clone(),
    );
    let hover_border = pixel_at(
        &after,
        component.x + TRIGGER_X + TRIGGER_SAMPLE_OFFSET,
        component.y + TRIGGER_Y,
    );

    assert_ne!(
        pixel_at(
            &before,
            component.x + TRIGGER_X + TRIGGER_SAMPLE_OFFSET,
            component.y + TRIGGER_Y
        ),
        hover_border
    );
    assert_eq!(
        Some(palette::VisualPalette::from_theme(&ThemeSnapshot::dark()).hover_border),
        hover_border
    );
}

fn assert_trigger_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, TRIGGER_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + TRIGGER_X + TRIGGER_SAMPLE_OFFSET,
            component.y + TRIGGER_Y + TRIGGER_SAMPLE_OFFSET
        )
    );
}
