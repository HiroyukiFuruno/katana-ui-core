use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{
    StorybookWindowState, apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    focus_clickable_at_for_audit,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "popover";
const ANCHOR_PRESET: usize = 0;
const PLACEMENT_PRESET: usize = 1;
const AUTO_FLIP_PRESET: usize = 2;
const OFFSET_WIDTH_PRESET: usize = 3;
const COMPONENT_HIT_INSET: usize = 4;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const PANEL_X: usize = 116;
const PANEL_Y: usize = 34;
const PANEL_SAMPLE_OFFSET: usize = 8;

#[test]
fn popover_exposes_leaf_presets_options_and_open_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("popover_open", spec.action);
    assert_eq!("popover_opened", spec.event);
    assert_eq!("interaction.open", spec.option);
    assert_eq!("true", spec.after);
    assert_eq!("open=true", spec.state);
}

#[test]
fn popover_presets_render_distinct_anchor_placement_flip_and_width_bodies() {
    let anchor = StorybookVisual.render_preset(DARK_THEME, PAGE, ANCHOR_PRESET, 0);
    let placement = StorybookVisual.render_preset(DARK_THEME, PAGE, PLACEMENT_PRESET, 0);
    let auto_flip = StorybookVisual.render_preset(DARK_THEME, PAGE, AUTO_FLIP_PRESET, 0);
    let offset_width = StorybookVisual.render_preset(DARK_THEME, PAGE, OFFSET_WIDTH_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &anchor, &placement) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &placement, &auto_flip) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &anchor, &offset_width) > BODY_DIFF_THRESHOLD);
}

#[test]
fn popover_setting_option_updates_panel_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn popover_preview_action_opens_panel_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn popover_live_hover_focus_and_keyboard_escape_use_core_actions() {
    let mut hover_state = page_state();
    let hover_before = render_state(&hover_state);
    assert!(apply_hover_at(&mut hover_state, popover_x(), popover_y()));
    let hover_after = render_state(&hover_state);

    assert_eq!("popover_hover", hover_state.screen_state.last_action);
    assert_eq!("popover_hovered", hover_state.screen_state.last_event);
    assert_eq!("hover=true", hover_state.screen_state.state_label);
    assert!(hover_state.screen_state.preview_hovered);
    assert!(component_body_pixel_diff(PAGE, &hover_before, &hover_after) > 0);

    let mut keyboard_state = page_state();
    let focus_before = render_state(&keyboard_state);
    assert!(focus_clickable_at_for_audit(
        &mut keyboard_state,
        popover_x(),
        popover_y()
    ));
    let focus_after = render_state(&keyboard_state);

    assert_eq!("popover_focus", keyboard_state.screen_state.last_action);
    assert_eq!("popover_focused", keyboard_state.screen_state.last_event);
    assert_eq!("focus=true", keyboard_state.screen_state.state_label);
    assert!(keyboard_state.screen_state.is_button_focused());
    assert!(component_body_pixel_diff(PAGE, &focus_before, &focus_after) > 0);

    let keyboard_before = render_state(&keyboard_state);
    assert!(apply_clickable_keyboard_activation_for_audit(
        &mut keyboard_state
    ));
    let keyboard_after = render_state(&keyboard_state);

    assert_eq!(
        "popover_keyboard_escape",
        keyboard_state.screen_state.last_action
    );
    assert_eq!("popover_closed", keyboard_state.screen_state.last_event);
    assert_eq!("open=false", keyboard_state.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &keyboard_before, &keyboard_after) > 0);
}

#[test]
fn popover_light_and_dark_panel_uses_theme_surface() {
    assert_panel_token(DARK_THEME, ThemeSnapshot::dark());
    assert_panel_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_panel_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, ANCHOR_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + PANEL_X + PANEL_SAMPLE_OFFSET,
            component.y + PANEL_Y + PANEL_SAMPLE_OFFSET
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

fn popover_x() -> usize {
    preview_detail::component_action_hit_rect(PAGE).x + COMPONENT_HIT_INSET
}

fn popover_y() -> usize {
    preview_detail::component_action_hit_rect(PAGE).y + COMPONENT_HIT_INSET
}
