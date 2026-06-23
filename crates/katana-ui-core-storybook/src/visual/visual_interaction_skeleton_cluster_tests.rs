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
const PAGE: &str = "skeleton-cluster";
const LIST_PRESET: usize = 0;
const MESSAGE_PRESET: usize = 1;
const CARD_PRESET: usize = 2;
const PARAGRAPH_PRESET: usize = 3;
const CODE_PRESET: usize = 4;
const IMAGE_PRESET: usize = 5;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const CLICK_OFFSET: usize = 4;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 20;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;

#[test]
fn skeleton_cluster_exposes_leaf_presets_options_and_cluster_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert_eq!(options.len(), REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("skeleton_cluster.preset:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("skeleton_cluster.live_region:"))
    );
    assert_eq!("skeleton_cluster_preset_apply", spec.action);
    assert_eq!("skeleton_cluster_changed", spec.event);
    assert_eq!("skeleton_cluster.preset", spec.option);
    assert_eq!("Card", spec.after);
    assert_eq!("items=2", spec.state);
}

#[test]
fn skeleton_cluster_presets_render_distinct_density_media_and_code_states() {
    let list = StorybookVisual.render_preset(DARK_THEME, PAGE, LIST_PRESET, 0);
    let message = StorybookVisual.render_preset(DARK_THEME, PAGE, MESSAGE_PRESET, 0);
    let card = StorybookVisual.render_preset(DARK_THEME, PAGE, CARD_PRESET, 0);
    let paragraph = StorybookVisual.render_preset(DARK_THEME, PAGE, PARAGRAPH_PRESET, 0);
    let code = StorybookVisual.render_preset(DARK_THEME, PAGE, CODE_PRESET, 0);
    let image = StorybookVisual.render_preset(DARK_THEME, PAGE, IMAGE_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &list, &message) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &message, &card) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &card, &paragraph) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &paragraph, &code) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &code, &image) > BODY_DIFF_THRESHOLD);
}

#[test]
fn skeleton_cluster_setting_option_updates_preset_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn skeleton_cluster_preview_action_updates_cluster_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn skeleton_cluster_light_and_dark_surface_uses_theme_surface() {
    assert_cluster_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_cluster_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

#[test]
fn skeleton_cluster_live_operations_use_core_cluster_and_skeleton_api() {
    let target = preview_detail::component_action_hit_rect(PAGE);

    let mut pointer_state = skeleton_cluster_state();
    assert!(apply_click(
        &mut pointer_state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    assert_eq!(
        "skeleton_cluster_preset_apply",
        pointer_state.screen_state.last_action
    );
    assert_eq!(
        "skeleton_cluster_changed",
        pointer_state.screen_state.last_event
    );
    assert_eq!("items=2", pointer_state.screen_state.state_label);
    assert_eq!(
        2,
        pointer_state
            .screen_state
            .runtime_structured
            .skeleton_cluster
            .preview_child_count
    );

    let mut hover_state = skeleton_cluster_state();
    let before_hover = render_state(&hover_state);
    assert!(apply_hover_at(
        &mut hover_state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    let after_hover = render_state(&hover_state);
    assert_eq!(
        "skeleton_cluster_hover",
        hover_state.screen_state.last_action
    );
    assert_eq!("hover_start", hover_state.screen_state.last_event);
    assert_eq!("hover=cluster", hover_state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let mut keyboard_state = skeleton_cluster_state();
    assert!(focus_clickable_at_for_audit(
        &mut keyboard_state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    assert_eq!(
        "skeleton_cluster_focus",
        keyboard_state.screen_state.last_action
    );
    assert!(apply_clickable_keyboard_activation_for_audit(
        &mut keyboard_state
    ));
    assert_eq!(
        "skeleton_cluster_keyboard_reduce_motion",
        keyboard_state.screen_state.last_action
    );
    assert_eq!(
        "skeleton_reduced_motion_changed",
        keyboard_state.screen_state.last_event
    );
    assert_eq!(
        "reduced_motion=true",
        keyboard_state.screen_state.state_label
    );
}

fn assert_cluster_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, LIST_PRESET, 0);
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

fn skeleton_cluster_state() -> StorybookWindowState {
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
