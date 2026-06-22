use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_clickable_keyboard_activation_for_audit,
    focus_clickable_at_for_audit,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "search-box";
const ICON_PRESET: usize = 0;
const SUBMIT_PRESET: usize = 1;
const REGEX_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const COMPONENT_HIT_INSET: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const FIELD_FILL_SAMPLE_X_OFFSET: usize = 180;
const FIELD_FILL_SAMPLE_Y_OFFSET: usize = 8;

#[test]
fn search_box_exposes_leaf_presets_options_and_submit_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert_eq!(options.len(), REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert!(
        rows.iter()
            .any(|row| row.starts_with("text_entry.submit_on_enter:"))
    );
    assert!(
        rows.iter()
            .any(|row| row.starts_with("text_entry.regex_case:"))
    );
    assert_eq!("search_submit", spec.action);
    assert_eq!("search_submitted", spec.event);
    assert_eq!("text_entry.submit_on_enter", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("submitted=true", spec.state);
}

#[test]
fn search_box_presets_render_distinct_icon_submit_regex_and_theme_states() {
    let icon = StorybookVisual.render_preset(DARK_THEME, PAGE, ICON_PRESET, 0);
    let submit = StorybookVisual.render_preset(DARK_THEME, PAGE, SUBMIT_PRESET, 0);
    let regex = StorybookVisual.render_preset(DARK_THEME, PAGE, REGEX_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &icon, &submit) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &submit, &regex) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &regex, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn search_box_setting_option_updates_submit_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn search_box_preview_action_updates_submit_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn search_box_live_focus_and_keyboard_submit_use_core_actions() {
    let mut state = page_state();
    let focus_before = render_state(&state);
    assert!(focus_clickable_at_for_audit(
        &mut state,
        search_box_x(),
        search_box_y()
    ));
    let focus_after = render_state(&state);

    assert_eq!("search_focus", state.screen_state.last_action);
    assert_eq!("focus", state.screen_state.last_event);
    assert_eq!("focus=true", state.screen_state.state_label);
    assert!(state.screen_state.is_button_focused());
    assert!(state.screen_state.search_box.focused);
    assert!(component_body_pixel_diff(PAGE, &focus_before, &focus_after) > 0);

    let keyboard_before = render_state(&state);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    let keyboard_after = render_state(&state);

    assert_eq!("search_keyboard_submit", state.screen_state.last_action);
    assert_eq!("search_submitted", state.screen_state.last_event);
    assert_eq!("value=query submitted=true", state.screen_state.state_label);
    assert!(state.screen_state.search_box.submitted);
    assert!(component_body_pixel_diff(PAGE, &keyboard_before, &keyboard_after) > 0);
}

#[test]
fn search_box_light_and_dark_fields_use_theme_tokens() {
    assert_search_field_tokens(DARK_THEME, ThemeSnapshot::dark());
    assert_search_field_tokens(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_search_field_tokens(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, ICON_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let rect = preview_detail::component_action_hit_rect(PAGE);
    let field = super::dedicated_dod_form_input_live::search_field_rect(rect.x, rect.y);

    assert_eq!(Some(colors.border), pixel_at(&canvas, field.x, field.y));
    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            field.x + FIELD_FILL_SAMPLE_X_OFFSET,
            field.y + FIELD_FILL_SAMPLE_Y_OFFSET
        )
    );
    assert_eq!(
        Some(colors.accent),
        pixel_at(
            &canvas,
            rect.x + super::dedicated_dod_form_input_live::FIELD_ICON_X,
            rect.y + super::dedicated_dod_form_input_live::FIELD_ICON_Y
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

fn search_box_x() -> usize {
    preview_detail::component_action_hit_rect(PAGE).x + COMPONENT_HIT_INSET
}

fn search_box_y() -> usize {
    preview_detail::component_action_hit_rect(PAGE).y + COMPONENT_HIT_INSET
}
