use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, focus_clickable_at_for_audit,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "chip";
const LABEL_PRESET: usize = 0;
const LEADING_ICON_PRESET: usize = 1;
const TRAILING_ICON_PRESET: usize = 2;
const VARIANT_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 12;
const REQUIRED_OPTION_COUNT: usize = 12;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;
const CHIP_DISABLED_PRESET: usize = 8;
const CHIP_FOCUS_X_OFFSET: usize = super::dedicated_chip::CHIP_X + 4;
const CHIP_FOCUS_Y_OFFSET: usize = super::dedicated_chip::CHIP_Y + 4;
const CHIP_HOVER_DIFF_THRESHOLD: usize = 24;

#[test]
fn chip_exposes_leaf_presets_options_and_dismiss_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    for setting in [
        "chip.label",
        "chip.leading_icon",
        "chip.trailing_icon",
        "chip.variant",
        "chip.tone",
        "chip.size",
        "chip.interactive",
        "chip.selected",
        "chip.disabled",
        "chip.dismissible",
        "chip.a11y_label",
        "chip.focused",
    ] {
        assert!(
            rows.iter().any(|row| row.starts_with(setting)),
            "chip Inspector row is not exposed: {setting}"
        );
    }
    assert_eq!("chip_dismiss", spec.action);
    assert_eq!("chip_dismissed", spec.event);
    assert_eq!("chip.dismissible", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("dismissed=true", spec.state);
}

#[test]
fn chip_presets_render_distinct_label_icon_and_variant_states() {
    let label = StorybookVisual.render_preset(DARK_THEME, PAGE, LABEL_PRESET, 0);
    let leading = StorybookVisual.render_preset(DARK_THEME, PAGE, LEADING_ICON_PRESET, 0);
    let trailing = StorybookVisual.render_preset(DARK_THEME, PAGE, TRAILING_ICON_PRESET, 0);
    let variant = StorybookVisual.render_preset(DARK_THEME, PAGE, VARIANT_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &label, &leading) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &leading, &trailing) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &trailing, &variant) > BODY_DIFF_THRESHOLD);
}

#[test]
fn chip_setting_option_updates_chip_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn chip_preview_action_updates_dismiss_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn chip_hover_focus_and_keyboard_dismiss_update_live_state_and_body() {
    let mut hover = page_state();
    let hover_before = render_state(&hover);
    let hover_target = preview_detail::component_action_hit_rect(PAGE);

    assert!(apply_hover_at(
        &mut hover,
        hover_target.x + CHIP_FOCUS_X_OFFSET,
        hover_target.y + CHIP_FOCUS_Y_OFFSET
    ));
    assert_eq!(0, hover.screen_state.action_count);
    assert!(hover.screen_state.preview_hovered);
    let hover_after = render_state(&hover);
    assert!(
        component_body_pixel_diff(PAGE, &hover_before, &hover_after) > CHIP_HOVER_DIFF_THRESHOLD
    );

    let mut keyboard = page_state();
    let focused_before = render_state(&keyboard);
    assert!(focus_clickable_at_for_audit(
        &mut keyboard,
        hover_target.x + CHIP_FOCUS_X_OFFSET,
        hover_target.y + CHIP_FOCUS_Y_OFFSET
    ));
    assert_eq!("chip_focus", keyboard.screen_state.last_action);
    assert_eq!("chip_focused", keyboard.screen_state.last_event);
    assert!(keyboard.screen_state.is_button_focused());
    let focused_after = render_state(&keyboard);
    assert!(component_body_pixel_diff(PAGE, &focused_before, &focused_after) > 0);

    let keyboard_before = render_state(&keyboard);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut keyboard));
    assert_eq!("chip_dismiss", keyboard.screen_state.last_action);
    assert_eq!("chip_dismissed", keyboard.screen_state.last_event);
    assert_eq!("dismissed=true", keyboard.screen_state.state_label);
    assert!(keyboard.screen_state.is_button_pressed());
    let keyboard_after = render_state(&keyboard);
    assert!(component_body_pixel_diff(PAGE, &keyboard_before, &keyboard_after) > 0);
}

#[test]
fn disabled_chip_blocks_pointer_focus_and_keyboard_mutation() {
    let mut pointer = page_state();
    pointer.preset_index = CHIP_DISABLED_PRESET;
    let target = preview_detail::component_action_hit_rect(PAGE);
    let before = render_state(&pointer);

    assert!(apply_click(
        &mut pointer,
        target.x + CHIP_FOCUS_X_OFFSET,
        target.y + CHIP_FOCUS_Y_OFFSET
    ));
    let after = render_state(&pointer);
    assert_eq!("none", pointer.screen_state.last_action);
    assert_eq!(0, pointer.screen_state.action_count);
    assert_eq!(0, component_body_pixel_diff(PAGE, &before, &after));

    let mut keyboard = page_state();
    keyboard.preset_index = CHIP_DISABLED_PRESET;
    assert!(focus_clickable_at_for_audit(
        &mut keyboard,
        target.x + CHIP_FOCUS_X_OFFSET,
        target.y + CHIP_FOCUS_Y_OFFSET
    ));
    assert_eq!("chip_focus_blocked", keyboard.screen_state.last_action);
    assert_eq!("chip_focus_ignored", keyboard.screen_state.last_event);
    assert_eq!("focused=false", keyboard.screen_state.state_label);
    assert_eq!(0, keyboard.screen_state.action_count);

    assert!(apply_clickable_keyboard_activation_for_audit(&mut keyboard));
    assert_eq!("chip_keyboard_blocked", keyboard.screen_state.last_action);
    assert_eq!("chip_keyboard_ignored", keyboard.screen_state.last_event);
    assert_eq!("keyboard=false", keyboard.screen_state.state_label);
    assert_eq!(0, keyboard.screen_state.action_count);
}

#[test]
fn chip_light_and_dark_surface_uses_theme_surface() {
    assert_chip_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_chip_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_chip_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, LABEL_PRESET, 0);
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

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    super::render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}

fn page_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}
