use super::interaction_spec::StorybookInteractionSpec;
use super::screen_state_tabs::TabsScreenAction;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at, require_some,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{StorybookVisual, dedicated_tabs, palette, preview_detail, render};
use crate::catalog::StoryPresetLabels;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "tabs";
const WORKSPACE_PRESET: usize = 0;
const ADD_CLOSE_PRESET: usize = 1;
const PIN_PRESET: usize = 2;
const MOVE_PRESET: usize = 3;
const GROUP_PRESET: usize = 4;
const OVERFLOW_PRESET: usize = 5;
const REQUIRED_PRESET_COUNT: usize = 6;
const REQUIRED_OPTION_COUNT: usize = 6;
const BODY_DIFF_THRESHOLD: usize = 80;

#[test]
fn tabs_exposes_katana_workspace_tab_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = super::storybook_ui_option_contract::options_for_page(PAGE);
    let rows = super::storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert_eq!(
        &[
            "workspace tabs",
            "add close",
            "pin lock",
            "move reorder",
            "grouped tabs",
            "overflow menu"
        ],
        presets
    );
    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("add_tab", spec.action);
    assert_eq!("closeable_tab_added", spec.event);
    assert_eq!("tabs.add", spec.option);
    assert_eq!("notes.md", spec.after);
    assert_eq!("tabs=6 active=notes.md", spec.state);
}

#[test]
fn tabs_presets_render_distinct_lifecycle_states() {
    let workspace = StorybookVisual.render_preset(DARK_THEME, PAGE, WORKSPACE_PRESET, 0);
    let add_close = StorybookVisual.render_preset(DARK_THEME, PAGE, ADD_CLOSE_PRESET, 0);
    let pinned = StorybookVisual.render_preset(DARK_THEME, PAGE, PIN_PRESET, 0);
    let moved = StorybookVisual.render_preset(DARK_THEME, PAGE, MOVE_PRESET, 0);
    let grouped = StorybookVisual.render_preset(DARK_THEME, PAGE, GROUP_PRESET, 0);
    let overflow = StorybookVisual.render_preset(DARK_THEME, PAGE, OVERFLOW_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &workspace, &add_close) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &add_close, &pinned) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &pinned, &moved) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &moved, &grouped) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &grouped, &overflow) > BODY_DIFF_THRESHOLD);
}

#[test]
fn tabs_setting_option_adds_tab_and_repaints() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn tabs_preview_action_adds_tab() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn tabs_live_controls_cover_add_close_pin_move_group_and_overflow() -> Result<(), String> {
    let mut state = tabs_state();

    click_control(&mut state, TabsScreenAction::AddTab)?;
    assert!(
        state
            .screen_state
            .tabs
            .tabs
            .iter()
            .any(|tab| tab.id == "notes.md")
    );
    assert_eq!("closeable_tab_added", state.screen_state.last_event);

    click_control(&mut state, TabsScreenAction::CloseActive)?;
    assert!(
        !state
            .screen_state
            .tabs
            .tabs
            .iter()
            .any(|tab| tab.id == "notes.md")
    );
    assert_eq!("closeable_tab_closed", state.screen_state.last_event);

    click_control(&mut state, TabsScreenAction::TogglePinActive)?;
    assert!(active_tab(&state)?.pinned);
    assert_eq!("pinned=true left-fixed", state.screen_state.state_label);

    click_control(&mut state, TabsScreenAction::CloseActive)?;
    assert!(active_tab(&state)?.pinned);
    assert_eq!("pinned=true close=blocked", state.screen_state.state_label);

    click_control(&mut state, TabsScreenAction::TogglePinActive)?;
    click_control(&mut state, TabsScreenAction::MoveActiveRight)?;
    let order = tab_order(&state);
    assert!(
        order.iter().position(|id| *id == "scratch.md")
            > order.iter().position(|id| *id == "terminal")
    );
    assert_eq!("closeable_tab_reordered", state.screen_state.last_event);

    click_control(&mut state, TabsScreenAction::GroupActive)?;
    assert_eq!(Some("docs"), active_tab(&state)?.group_id.as_deref());
    assert_eq!("closeable_tab_grouped", state.screen_state.last_event);

    click_control(&mut state, TabsScreenAction::ToggleOverflow)?;
    assert!(state.screen_state.tabs.overflow_open);
    assert_eq!(
        "closeable_tab_overflow_opened",
        state.screen_state.last_event
    );
    Ok(())
}

#[test]
fn tabs_light_and_dark_strip_uses_theme_surface() {
    assert_strip_token(DARK_THEME, ThemeSnapshot::dark());
    assert_strip_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn assert_strip_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, WORKSPACE_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);
    let strip = dedicated_tabs::strip_rect_for_test();

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + strip.x + strip.width - 8,
            component.y + strip.y + 10
        )
    );
}

fn click_control(state: &mut StorybookWindowState, action: TabsScreenAction) -> Result<(), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let rect = require_some(
        dedicated_tabs::control_rect_for_test(action),
        "tabs control rect",
    )?;

    assert!(apply_click(
        state,
        component.x + rect.x + 1,
        component.y + rect.y + 1
    ));
    let canvas = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );
    assert!(
        component_body_pixel_diff(
            PAGE,
            &StorybookVisual.render_preset(DARK_THEME, PAGE, WORKSPACE_PRESET, 0),
            &canvas,
        ) > BODY_DIFF_THRESHOLD
    );
    Ok(())
}

fn tabs_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn active_tab(
    state: &StorybookWindowState,
) -> Result<&super::screen_state_tabs::TabsScreenTab, String> {
    require_some(state.screen_state.tabs.active_tab(), "active tab exists")
}

fn tab_order(state: &StorybookWindowState) -> Vec<&str> {
    state
        .screen_state
        .tabs
        .tabs
        .iter()
        .map(|tab| tab.id.as_str())
        .collect()
}
