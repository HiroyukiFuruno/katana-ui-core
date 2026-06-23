use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    apply_select_scroll_for_audit, focus_clickable_at_for_audit,
};
use super::{
    StorybookVisual, palette, preview_detail, selection_control_metrics as sm,
    storybook_ui_option_contract,
};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "select-box";
const ITEMS_PRESET: usize = 0;
const OPEN_PRESET: usize = 1;
const SELECT_PRESET: usize = 2;
const PLACEHOLDER_PRESET: usize = 3;
const DISABLED_PRESET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 5;
const REQUIRED_OPTION_COUNT: usize = 5;
const BODY_DIFF_THRESHOLD: usize = 80;
const TRIGGER_FILL_SAMPLE_X_OFFSET: usize = 4;
const TRIGGER_FILL_SAMPLE_Y_OFFSET: usize = 4;

#[test]
fn select_box_exposes_leaf_presets_options_and_select_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("select_option", spec.action);
    assert_eq!("select_box_selected", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("false", spec.after);
    assert_eq!("selected=true", spec.state);
    assert!(
        options
            .iter()
            .any(|option| option.setting == "select.items")
    );
    assert!(
        !options
            .iter()
            .any(|option| option.setting == "theme.marker")
    );
}

#[test]
fn select_box_presets_render_distinct_dropdown_bodies() {
    let items = StorybookVisual.render_preset(DARK_THEME, PAGE, ITEMS_PRESET, 0);
    let open = StorybookVisual.render_preset(DARK_THEME, PAGE, OPEN_PRESET, 0);
    let selected = StorybookVisual.render_preset(DARK_THEME, PAGE, SELECT_PRESET, 0);
    let placeholder = StorybookVisual.render_preset(DARK_THEME, PAGE, PLACEHOLDER_PRESET, 0);
    let disabled = StorybookVisual.render_preset(DARK_THEME, PAGE, DISABLED_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &items, &open) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &open, &selected) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &selected, &placeholder) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &placeholder, &disabled) > BODY_DIFF_THRESHOLD);
}

#[test]
fn select_box_setting_option_updates_dropdown_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn select_box_preview_action_updates_dropdown_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn select_box_live_focus_hover_keyboard_and_scroll_use_core_paths() {
    let mut state = page_state();
    let trigger = trigger_rect();
    let before_hover = render_state(&state);
    assert!(apply_hover_at(&mut state, trigger.x + 1, trigger.y + 1));
    let after_hover = render_state(&state);

    assert_eq!("select_hover", state.screen_state.last_action);
    assert_eq!("hover_start", state.screen_state.last_event);
    assert_eq!("hover=true", state.screen_state.state_label);
    assert!(state.screen_state.selection.select_hovered);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > 0);

    let before_focus = render_state(&state);
    assert!(focus_clickable_at_for_audit(
        &mut state,
        trigger.x + 1,
        trigger.y + 1
    ));
    let after_focus = render_state(&state);

    assert_eq!("select_focus", state.screen_state.last_action);
    assert_eq!("focus", state.screen_state.last_event);
    assert_eq!("focus=true", state.screen_state.state_label);
    assert!(state.screen_state.is_button_focused());
    assert!(state.screen_state.selection.select_focused);
    assert!(component_body_pixel_diff(PAGE, &before_focus, &after_focus) > 0);

    let before_keyboard = render_state(&state);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut state));
    let after_keyboard = render_state(&state);

    assert_eq!("select_keyboard_select", state.screen_state.last_action);
    assert_eq!("select_box_selected", state.screen_state.last_event);
    assert_eq!("selected=light", state.screen_state.state_label);
    assert_eq!(Some(1), state.screen_state.selection.select_selected_index);
    assert!(component_body_pixel_diff(PAGE, &before_keyboard, &after_keyboard) > 0);

    let before_scroll = render_state(&state);
    assert!(apply_select_scroll_for_audit(
        &mut state,
        trigger.x + 1,
        trigger.y + 1
    ));
    let after_scroll = render_state(&state);

    assert_eq!("select_option_scroll", state.screen_state.last_action);
    assert_eq!("select_options_scrolled", state.screen_state.last_event);
    assert_eq!("scroll=1", state.screen_state.state_label);
    assert_eq!(1, state.screen_state.selection.select_scroll_offset);
    assert!(component_body_pixel_diff(PAGE, &before_scroll, &after_scroll) > 0);
}

#[test]
fn select_box_light_and_dark_trigger_uses_theme_tokens() {
    assert_trigger_tokens(DARK_THEME, ThemeSnapshot::dark(), ITEMS_PRESET);
    assert_trigger_tokens(LIGHT_THEME, ThemeSnapshot::light(), ITEMS_PRESET);
}

fn assert_trigger_tokens(theme_id: &str, theme: ThemeSnapshot, preset_index: usize) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, preset_index, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);
    let trigger = sm::trigger_rect(component);

    assert_eq!(Some(colors.border), pixel_at(&canvas, trigger.x, trigger.y));
    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            trigger.x + TRIGGER_FILL_SAMPLE_X_OFFSET,
            trigger.y + TRIGGER_FILL_SAMPLE_Y_OFFSET
        )
    );
}

fn trigger_rect() -> super::layout_metrics::LayoutRect {
    sm::trigger_rect(preview_detail::component_action_hit_rect(PAGE))
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
