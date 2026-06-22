use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_click, apply_clickable_keyboard_activation_for_audit,
    apply_hover_at, focus_clickable_at_for_audit,
};
use super::{StorybookVisual, palette, preview_detail, render, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "chip-group";
const WRAP_PRESET: usize = 0;
const OVERFLOW_PRESET: usize = 1;
const SCROLL_PRESET: usize = 2;
const REORDER_PRESET: usize = 3;
const LABEL_PRESET: usize = 4;
const CHIP_COUNT_PRESET: usize = 5;
const GAP_PRESET: usize = 6;
const AVAILABLE_WIDTH_PRESET: usize = 7;
const TRIGGER_WIDTH_PRESET: usize = 8;
const REQUIRED_PRESET_COUNT: usize = 9;
const REQUIRED_OPTION_COUNT: usize = 9;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;
const CLICK_OFFSET: usize = 4;

#[test]
fn chip_group_exposes_leaf_presets_options_and_overflow_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    for option in [
        "chip_group.label",
        "chip_group.chip_count",
        "chip_group.wrap",
        "chip_group.overflow",
        "chip_group.reorder",
        "chip_group.gap",
        "chip_group.available_width",
        "chip_group.overflow_trigger_width",
        "chip_group.hidden_count",
    ] {
        assert!(
            options.iter().any(|it| it.setting == option),
            "chip-group option is not exposed: {option}"
        );
        assert!(
            rows.iter()
                .any(|row| row.starts_with(&format!("{option}:"))),
            "chip-group settings row is not exposed: {option}"
        );
    }
    assert_eq!("chip_group_overflow", spec.action);
    assert_eq!("chip_group_overflow_opened", spec.event);
    assert_eq!("chip_group.overflow", spec.option);
    assert_eq!("Menu", spec.after);
    assert_eq!("overflow=open", spec.state);
}

#[test]
fn chip_group_presets_render_distinct_wrap_overflow_scroll_and_reorder_states() {
    let wrap = StorybookVisual.render_preset(DARK_THEME, PAGE, WRAP_PRESET, 0);
    let overflow = StorybookVisual.render_preset(DARK_THEME, PAGE, OVERFLOW_PRESET, 0);
    let scroll = StorybookVisual.render_preset(DARK_THEME, PAGE, SCROLL_PRESET, 0);
    let reorder = StorybookVisual.render_preset(DARK_THEME, PAGE, REORDER_PRESET, 0);
    let label = StorybookVisual.render_preset(DARK_THEME, PAGE, LABEL_PRESET, 0);
    let chip_count = StorybookVisual.render_preset(DARK_THEME, PAGE, CHIP_COUNT_PRESET, 0);
    let gap = StorybookVisual.render_preset(DARK_THEME, PAGE, GAP_PRESET, 0);
    let available_width =
        StorybookVisual.render_preset(DARK_THEME, PAGE, AVAILABLE_WIDTH_PRESET, 0);
    let trigger_width = StorybookVisual.render_preset(DARK_THEME, PAGE, TRIGGER_WIDTH_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &wrap, &overflow) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &overflow, &scroll) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &scroll, &reorder) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &reorder, &label) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &label, &chip_count) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &chip_count, &gap) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &gap, &available_width) > BODY_DIFF_THRESHOLD);
    assert!(
        component_body_pixel_diff(PAGE, &available_width, &trigger_width) > BODY_DIFF_THRESHOLD
    );
}

#[test]
fn chip_group_setting_option_updates_overflow_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn chip_group_preview_action_updates_overflow_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn chip_group_live_operations_use_core_group_actions() {
    let target = preview_detail::component_action_hit_rect(PAGE);

    let mut pointer_state = chip_group_state();
    assert!(apply_click(
        &mut pointer_state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    assert_eq!(
        "chip_group_overflow",
        pointer_state.screen_state.last_action
    );
    assert_eq!(
        "chip_group_overflow_opened",
        pointer_state.screen_state.last_event
    );
    assert_eq!("overflow=open", pointer_state.screen_state.state_label);
    assert!(
        pointer_state
            .screen_state
            .runtime_structured
            .chip_group
            .overflow_open
    );

    let mut hover_state = chip_group_state();
    let before_hover = render_state(&hover_state);
    assert!(apply_hover_at(
        &mut hover_state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    let after_hover = render_state(&hover_state);
    assert_eq!("chip_group_hover", hover_state.screen_state.last_action);
    assert_eq!("hover_start", hover_state.screen_state.last_event);
    assert_eq!("hover=chip", hover_state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let mut keyboard_state = chip_group_state();
    assert!(focus_clickable_at_for_audit(
        &mut keyboard_state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    assert_eq!("chip_group_focus", keyboard_state.screen_state.last_action);
    assert!(apply_clickable_keyboard_activation_for_audit(
        &mut keyboard_state
    ));
    assert_eq!(
        "chip_group_keyboard_dismiss",
        keyboard_state.screen_state.last_action
    );
    assert_eq!(
        "chip_group_chip_dismissed",
        keyboard_state.screen_state.last_event
    );
    assert_eq!("dismissed=focused", keyboard_state.screen_state.state_label);
}

#[test]
fn chip_group_light_and_dark_surface_uses_theme_surface() {
    assert_group_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_group_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_group_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, WRAP_PRESET, 0);
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

fn chip_group_state() -> StorybookWindowState {
    let mut state = StorybookWindowState::default();
    state.select_page(PAGE);
    state
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        DARK_THEME,
        PAGE,
        state.preset_index,
        state.screen_state.clone(),
    )
}
