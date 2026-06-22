use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::{StorybookVisual, palette, preview_detail, storybook_ui_option_contract};
use super::{render, window_interaction};
use crate::catalog::StoryPresetLabels;
use crate::visual::window_interaction::{
    apply_clickable_keyboard_activation_for_audit, apply_hover_at,
    apply_side_menu_scroll_for_audit, focus_clickable_at_for_audit,
};
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "side-menu";
const NAV_PRESET: usize = 0;
const SELECT_PRESET: usize = 1;
const COLLAPSE_PRESET: usize = 2;
const THEME_PRESET: usize = 3;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const PANEL_X: usize = 32;
const PANEL_Y: usize = 28;
const PANEL_SAMPLE_X_OFFSET: usize = 184;
const PANEL_SAMPLE_Y_OFFSET: usize = 14;
const CLICK_INSET: usize = 2;
const PRIMARY_INSTANCE: &str = "side-menu.primary";
const SECONDARY_INSTANCE: &str = "side-menu.secondary";

#[test]
fn side_menu_exposes_leaf_presets_options_and_route_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("side_menu_select", spec.action);
    assert_eq!("select_box_selected", spec.event);
    assert_eq!("interaction.selected_index", spec.option);
    assert_eq!("1", spec.after);
    assert_eq!("route=1 focus=1", spec.state);
    assert!(
        options
            .iter()
            .any(|option| option.setting == "side_menu.hover_expansion")
    );
}

#[test]
fn side_menu_presets_render_distinct_nav_select_collapse_and_theme_states() {
    let nav = StorybookVisual.render_preset(DARK_THEME, PAGE, NAV_PRESET, 0);
    let select = StorybookVisual.render_preset(DARK_THEME, PAGE, SELECT_PRESET, 0);
    let collapse = StorybookVisual.render_preset(DARK_THEME, PAGE, COLLAPSE_PRESET, 0);
    let themed = StorybookVisual.render_preset(DARK_THEME, PAGE, THEME_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &nav, &select) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &select, &collapse) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &nav, &themed) > BODY_DIFF_THRESHOLD);
}

#[test]
fn side_menu_setting_option_updates_route_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn side_menu_preview_action_updates_route_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn side_menu_window_interaction_keeps_instance_state_isolated() {
    let mut state = state_for();

    state.select_instance(PRIMARY_INSTANCE);
    click_component(&mut state);
    let primary = state.screen_state.clone();
    let primary_canvas = render_state(&state);
    assert_eq!("side_menu_select", primary.last_action);
    assert_eq!("select_box_selected", primary.last_event);
    assert_eq!("route=1 focus=1", primary.state_label);

    state.select_instance(SECONDARY_INSTANCE);
    assert_eq!("none", state.screen_state.last_action);
    assert_eq!("idle", state.screen_state.state_label);
    let secondary_canvas = render_state(&state);

    state.select_instance(PRIMARY_INSTANCE);
    assert_eq!(primary.last_action, state.screen_state.last_action);
    assert_eq!(primary.state_label, state.screen_state.state_label);
    assert!(
        component_body_pixel_diff(PAGE, &primary_canvas, &secondary_canvas) > BODY_DIFF_THRESHOLD
    );
}

#[test]
fn side_menu_live_hover_focus_keyboard_and_scroll_use_core_actions() {
    let target = preview_detail::component_action_hit_rect(PAGE);
    let mut hover = state_for();
    let before_hover = render_state(&hover);
    assert!(apply_hover_at(
        &mut hover,
        target.x + CLICK_INSET,
        target.y + CLICK_INSET
    ));
    let after_hover = render_state(&hover);
    assert_eq!("side_menu_hover", hover.screen_state.last_action);
    assert_eq!("hover_start", hover.screen_state.last_event);
    assert_eq!("hover=true", hover.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_hover, &after_hover) > BODY_DIFF_THRESHOLD);

    let mut keyboard = state_for();
    assert!(focus_clickable_at_for_audit(
        &mut keyboard,
        target.x + CLICK_INSET,
        target.y + CLICK_INSET
    ));
    assert_eq!("side_menu_focus", keyboard.screen_state.last_action);
    assert_eq!("focus", keyboard.screen_state.last_event);
    assert_eq!("route=none focus=0", keyboard.screen_state.state_label);
    let before_key = render_state(&keyboard);
    assert!(apply_clickable_keyboard_activation_for_audit(&mut keyboard));
    let after_key = render_state(&keyboard);
    assert_eq!("side_menu_keyboard_next", keyboard.screen_state.last_action);
    assert_eq!("set_selected_index", keyboard.screen_state.last_event);
    assert_eq!("route=1 focus=1", keyboard.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_key, &after_key) > BODY_DIFF_THRESHOLD);

    let mut scroll = state_for();
    let before_scroll = render_state(&scroll);
    assert!(apply_side_menu_scroll_for_audit(
        &mut scroll,
        target.x + CLICK_INSET,
        target.y + CLICK_INSET
    ));
    let after_scroll = render_state(&scroll);
    assert_eq!("side_menu_scroll", scroll.screen_state.last_action);
    assert_eq!("scroll_by", scroll.screen_state.last_event);
    assert_eq!("scroll=1", scroll.screen_state.state_label);
    assert!(component_body_pixel_diff(PAGE, &before_scroll, &after_scroll) > BODY_DIFF_THRESHOLD);
}

#[test]
fn side_menu_light_and_dark_panel_uses_theme_surface() {
    assert_panel_token(DARK_THEME, ThemeSnapshot::dark());
    assert_panel_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_panel_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, NAV_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + PANEL_X + PANEL_SAMPLE_X_OFFSET,
            component.y + PANEL_Y + PANEL_SAMPLE_Y_OFFSET
        )
    );
}

fn state_for() -> window_interaction::StorybookWindowState {
    window_interaction::StorybookWindowState {
        selected_page: PAGE,
        ..window_interaction::StorybookWindowState::default()
    }
}

fn click_component(state: &mut window_interaction::StorybookWindowState) {
    let component = preview_detail::component_action_hit_rect(PAGE);
    assert!(window_interaction::apply_click(
        state,
        component.x + CLICK_INSET,
        component.y + CLICK_INSET,
    ));
}

fn render_state(state: &window_interaction::StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    )
}
