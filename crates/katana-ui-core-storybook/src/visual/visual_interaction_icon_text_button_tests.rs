use super::button_options::{StorybookButtonOptionControl, control_rect};
use super::visual_interaction_test_support::{
    component_body_pixel_diff, pixel_at, rect_non_background_pixels, rect_pixel_diff,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{StorybookVisual, palette, preview_detail, render, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "icon-text-button";
const TEXT_BUTTON_PAGE: &str = "text-button";
const SVG_BUTTON_PAGE: &str = "svg-button";
const DEFAULT_PRESET: usize = 0;
const INTERACTIVE_PRESET: usize = 1;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const MATERIAL_DIFF_THRESHOLD: usize = 160;
const SURFACE_SAMPLE_INSET: usize = 8;

#[test]
fn icon_text_button_exposes_leaf_presets_and_options() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!(StorybookButtonOptionControl::all().len(), options.len());
}

#[test]
fn icon_text_button_preset_changes_visible_body() {
    let before = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let after = StorybookVisual.render_preset(DARK_THEME, PAGE, INTERACTIVE_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn icon_text_button_click_updates_own_action_event_and_body() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    let target = preview_detail::button_action_hit_rect(PAGE);

    assert!(apply_click(
        &mut state,
        target.x + target.width / 2,
        target.y + target.height / 2,
    ));
    assert_eq!("icon_text_button_press", state.screen_state.last_action);
    assert_eq!("icon_text_button_clicked", state.screen_state.last_event);

    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn icon_text_button_option_click_updates_preview_body() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    let target = control_rect(StorybookButtonOptionControl::Label);

    assert!(apply_click(&mut state, target.x + 1, target.y + 1));
    assert_eq!("button_option_apply", state.screen_state.last_action);
    assert_eq!("button_option_changed", state.screen_state.last_event);
    assert_eq!("label=ja", state.screen_state.state_label);

    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn icon_text_button_material_shape_keeps_icon_and_label() {
    let icon_text = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let text_button =
        StorybookVisual.render_preset(DARK_THEME, TEXT_BUTTON_PAGE, DEFAULT_PRESET, 0);
    let svg_button = StorybookVisual.render_preset(DARK_THEME, SVG_BUTTON_PAGE, DEFAULT_PRESET, 0);
    let icon_rect = preview_detail::button_action_hit_rect(PAGE);
    let text_rect = preview_detail::button_action_hit_rect(TEXT_BUTTON_PAGE);
    let svg_rect = preview_detail::button_action_hit_rect(SVG_BUTTON_PAGE);

    assert!(icon_rect.width > text_rect.width);
    assert!(
        rect_non_background_pixels(icon_rect, &icon_text, palette::DEFAULT_BACKGROUND)
            > rect_non_background_pixels(svg_rect, &svg_button, palette::DEFAULT_BACKGROUND)
    );
    assert!(rect_pixel_diff(icon_rect, &icon_text, &text_button) > MATERIAL_DIFF_THRESHOLD);
}

#[test]
fn icon_text_button_light_and_dark_surfaces_use_theme_tokens() {
    let dark = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let light = StorybookVisual.render_preset(LIGHT_THEME, PAGE, DEFAULT_PRESET, 0);
    let target = preview_detail::button_action_hit_rect(PAGE);
    let sample_x = target.x + SURFACE_SAMPLE_INSET;
    let sample_y = target.y + SURFACE_SAMPLE_INSET;

    assert_ne!(
        pixel_at(&dark, sample_x, sample_y),
        pixel_at(&light, sample_x, sample_y)
    );
}
