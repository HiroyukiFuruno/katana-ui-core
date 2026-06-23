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
const PAGE: &str = "attachment-chip";
const FILE_PRESET: usize = 0;
const IMAGE_PRESET: usize = 1;
const URL_PRESET: usize = 2;
const UPLOADING_PRESET: usize = 3;
const ERROR_PRESET: usize = 4;
const NAME_PRESET: usize = 5;
const META_PRESET: usize = 6;
const THUMBNAIL_PRESET: usize = 7;
const REQUIRED_PRESET_COUNT: usize = 8;
const REQUIRED_OPTION_COUNT: usize = 7;
const BODY_DIFF_THRESHOLD: usize = 80;
const SURFACE_TOKEN_X: usize = 338;
const SURFACE_TOKEN_Y: usize = 34;
const SURFACE_SAMPLE_X_OFFSET: usize = 120;
const SURFACE_SAMPLE_Y_OFFSET: usize = 12;
const CLICK_OFFSET: usize = 4;

#[test]
fn attachment_chip_exposes_leaf_presets_options_and_status_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    for option in [
        "attachment.kind",
        "attachment.name",
        "attachment.meta",
        "attachment.thumbnail",
        "attachment.status",
        "attachment.progress",
        "attachment.retry",
    ] {
        assert!(
            options.iter().any(|it| it.setting == option),
            "attachment-chip option is not exposed: {option}"
        );
        assert!(
            rows.iter()
                .any(|row| row.starts_with(&format!("{option}:"))),
            "attachment-chip settings row is not exposed: {option}"
        );
    }
    assert_eq!("attachment_status", spec.action);
    assert_eq!("attachment_status_changed", spec.event);
    assert_eq!("attachment.status", spec.option);
    assert_eq!("Error", spec.after);
    assert_eq!("status=error", spec.state);
}

#[test]
fn attachment_chip_presets_render_distinct_file_image_url_upload_and_error_states() {
    let file = StorybookVisual.render_preset(DARK_THEME, PAGE, FILE_PRESET, 0);
    let image = StorybookVisual.render_preset(DARK_THEME, PAGE, IMAGE_PRESET, 0);
    let url = StorybookVisual.render_preset(DARK_THEME, PAGE, URL_PRESET, 0);
    let uploading = StorybookVisual.render_preset(DARK_THEME, PAGE, UPLOADING_PRESET, 0);
    let error = StorybookVisual.render_preset(DARK_THEME, PAGE, ERROR_PRESET, 0);
    let name = StorybookVisual.render_preset(DARK_THEME, PAGE, NAME_PRESET, 0);
    let meta = StorybookVisual.render_preset(DARK_THEME, PAGE, META_PRESET, 0);
    let thumbnail = StorybookVisual.render_preset(DARK_THEME, PAGE, THUMBNAIL_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &file, &image) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &image, &url) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &url, &uploading) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &uploading, &error) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &error, &name) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &name, &meta) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &meta, &thumbnail) > BODY_DIFF_THRESHOLD);
}

#[test]
fn attachment_chip_setting_option_updates_status_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn attachment_chip_preview_action_updates_status_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn attachment_chip_live_operations_use_core_attachment_actions() {
    let target = preview_detail::component_action_hit_rect(PAGE);

    let mut pointer_state = attachment_state();
    assert!(apply_click(
        &mut pointer_state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    assert_eq!("attachment_status", pointer_state.screen_state.last_action);
    assert_eq!(
        "attachment_status_changed",
        pointer_state.screen_state.last_event
    );
    assert_eq!("status=error", pointer_state.screen_state.state_label);
    assert!(
        pointer_state
            .screen_state
            .runtime_structured
            .attachment_chip
            .status_error
    );

    let mut hover_state = attachment_state();
    let before_hover = render_state(&hover_state);
    assert!(apply_hover_at(
        &mut hover_state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    let after_hover = render_state(&hover_state);
    assert_eq!("attachment_hover", hover_state.screen_state.last_action);
    assert_eq!("hover_start", hover_state.screen_state.last_event);
    assert_eq!("hover=attachment", hover_state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let mut keyboard_state = attachment_state();
    assert!(focus_clickable_at_for_audit(
        &mut keyboard_state,
        target.x + CLICK_OFFSET,
        target.y + CLICK_OFFSET
    ));
    assert_eq!("attachment_focus", keyboard_state.screen_state.last_action);
    assert!(apply_clickable_keyboard_activation_for_audit(
        &mut keyboard_state
    ));
    assert_eq!(
        "attachment_keyboard_retry",
        keyboard_state.screen_state.last_action
    );
    assert_eq!("attachment_retry", keyboard_state.screen_state.last_event);
    assert_eq!("retry=requested", keyboard_state.screen_state.state_label);
}

#[test]
fn attachment_chip_light_and_dark_surface_uses_theme_surface() {
    assert_attachment_surface_token(DARK_THEME, ThemeSnapshot::dark());
    assert_attachment_surface_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_attachment_surface_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, FILE_PRESET, 0);
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

fn attachment_state() -> StorybookWindowState {
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
