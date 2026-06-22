use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{
    StorybookVisual, layout_metrics, preview_detail, render, storybook_ui_option_contract,
};
use crate::catalog::{StoryCatalog, StoryPresetLabels};
use crate::visual::window_interaction::{
    StorybookWindowState, apply_click, focus_clickable_at_for_audit,
};

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "form-field";
const LABEL_PRESET: usize = 0;
const INVALID_PRESET: usize = 1;
const HELPER_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const SAMPLE_X_OFFSET: usize = 24;
const SAMPLE_Y_OFFSET: usize = 42;
const INVALID_OPTION_INDEX: usize = 2;
const HELPER_OPTION_INDEX: usize = 3;
const REQUIRED_OPTION_INDEX: usize = 4;

#[test]
fn form_field_exposes_leaf_presets_options_and_validation_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("field_validate", spec.action);
    assert_eq!("validation_changed", spec.event);
    assert_eq!("form_field.invalid=true", spec.state);
    assert!(
        options
            .iter()
            .any(|option| option.setting == "form_field.invalid")
    );
    assert!(
        options
            .iter()
            .any(|option| option.setting == "form_field.helper_text")
    );
    assert!(!options.iter().any(|option| option.setting == "overflow"));
}

#[test]
fn form_field_story_materializes_validation_callback_and_field_props() -> Result<(), &'static str> {
    let examples = StoryCatalog.examples();
    let story = examples
        .iter()
        .find(|it| it.page == PAGE)
        .ok_or("form-field story missing")?;
    let root = story.tree.root();

    assert_eq!("Repository name", root.props().label);
    assert!(!root.props().invalid);
    assert!(root.props().form_field.required);
    assert_eq!("Visible helper text", root.props().placeholder);
    assert_eq!("Visible helper text", root.props().form_field.helper_text);
    assert_eq!(
        Some("field:repository-name"),
        root.props()
            .form_field
            .control_state_id
            .as_ref()
            .map(|id| id.as_str())
    );
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action == "field_validate"
                && it.before == "invalid=false helper=Visible helper text"
                && it.after == "invalid=true helper=Repository name is required")
    );
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action == "form_field_helper_text"
                && it.after == "invalid=false helper=Used for release notes and package metadata")
    );
    Ok(())
}

#[test]
fn form_field_presets_render_distinct_wrapper_bodies() {
    let label = StorybookVisual.render_preset(DARK_THEME, PAGE, LABEL_PRESET, 0);
    let invalid = StorybookVisual.render_preset(DARK_THEME, PAGE, INVALID_PRESET, 0);
    let helper = StorybookVisual.render_preset(DARK_THEME, PAGE, HELPER_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &label, &invalid) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &invalid, &helper) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &helper, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn form_field_setting_option_updates_wrapper_style() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn form_field_preview_action_updates_validation_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn form_field_focus_link_records_control_focus_state() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );

    assert!(focus_clickable_at_for_audit(
        &mut state,
        rect.x + SAMPLE_X_OFFSET,
        rect.y + SAMPLE_Y_OFFSET
    ));

    assert_eq!("form_field_focus_link", state.screen_state.last_action);
    assert_eq!("form_field_control_focused", state.screen_state.last_event);
    assert_eq!(
        "form_field.control_state_id",
        state.screen_state.last_setting
    );
    assert_eq!(
        "field:repository-name",
        state.screen_state.last_setting_value
    );
    assert_eq!("focus=control", state.screen_state.state_label);
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn form_field_preview_click_records_validation_state_action_and_event() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );

    assert!(apply_click(
        &mut state,
        rect.x + SAMPLE_X_OFFSET,
        rect.y + SAMPLE_Y_OFFSET
    ));

    assert_eq!("field_validate", state.screen_state.last_action);
    assert_eq!("validation_changed", state.screen_state.last_event);
    assert_eq!("form_field.invalid=true", state.screen_state.state_label);
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn form_field_inspector_invalid_option_records_validation_state_action_and_event() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let rect = layout_metrics::inspector_setting_row_hit_rect(INVALID_OPTION_INDEX);
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );

    assert!(apply_click(&mut state, rect.x + 1, rect.y + 1));

    assert_eq!("field_validate", state.screen_state.last_action);
    assert_eq!("validation_changed", state.screen_state.last_event);
    assert_eq!("form_field.invalid", state.screen_state.last_setting);
    assert_eq!("true", state.screen_state.last_setting_value);
    assert_eq!("form_field.invalid=true", state.screen_state.state_label);
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn form_field_inspector_helper_option_records_helper_state_action_and_event() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let rect = layout_metrics::inspector_setting_row_hit_rect(HELPER_OPTION_INDEX);

    assert!(apply_click(&mut state, rect.x + 1, rect.y + 1));

    assert_eq!("form_field_helper_text", state.screen_state.last_action);
    assert_eq!("helper_text_changed", state.screen_state.last_event);
    assert_eq!("form_field.helper_text", state.screen_state.last_setting);
    assert_eq!("long", state.screen_state.last_setting_value);
    assert_eq!(
        "form_field.helper_text=long",
        state.screen_state.state_label
    );
}

#[test]
fn form_field_inspector_required_option_records_required_state_action_and_event() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let rect = layout_metrics::inspector_setting_row_hit_rect(REQUIRED_OPTION_INDEX);
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );

    assert!(apply_click(&mut state, rect.x + 1, rect.y + 1));

    assert_eq!("form_field_required", state.screen_state.last_action);
    assert_eq!("required_changed", state.screen_state.last_event);
    assert_eq!("form_field.required", state.screen_state.last_setting);
    assert_eq!("true", state.screen_state.last_setting_value);
    assert_eq!("form_field.required=true", state.screen_state.state_label);
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn form_field_light_and_dark_surfaces_use_theme_tokens() {
    let dark = StorybookVisual.render_preset(DARK_THEME, PAGE, LABEL_PRESET, 0);
    let light = StorybookVisual.render_preset(LIGHT_THEME, PAGE, LABEL_PRESET, 0);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let sample_x = rect.x + SAMPLE_X_OFFSET;
    let sample_y = rect.y + SAMPLE_Y_OFFSET;

    assert_ne!(
        pixel_at(&dark, sample_x, sample_y),
        pixel_at(&light, sample_x, sample_y)
    );
}
