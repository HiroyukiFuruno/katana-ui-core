use super::interaction_spec::StorybookInteractionSpec;
use super::visual_interaction_test_support::{
    assert_clicked_page_changes_body, assert_settings_page_changes_body, component_body_pixel_diff,
    pixel_at,
};
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{
    StorybookVisual, dedicated_closeable_tab_strip, palette, preview_detail, render,
    storybook_ui_option_contract,
};
use crate::catalog::StoryPresetLabels;
use crate::visual::screen_state_tabs::TabsScreenAction;
use katana_ui_core::theme::ThemeSnapshot;

const DARK_THEME: &str = "dark";
const LIGHT_THEME: &str = "light";
const PAGE: &str = "closeable-tab-strip";
const DEFAULT_PRESET: usize = 0;
const OVERFLOW_PRESET: usize = 1;
const PINNED_PRESET: usize = 2;
const GROUPS_PRESET: usize = 3;
const DIRTY_PRESET: usize = 4;
const DRAGGING_PRESET: usize = 5;
const REQUIRED_PRESET_COUNT: usize = 4;
const REQUIRED_OPTION_COUNT: usize = 4;
const BODY_DIFF_THRESHOLD: usize = 80;
const STRIP_X: usize = 30;
const STRIP_Y: usize = 42;
const STRIP_SAMPLE_X_OFFSET: usize = 430;
const STRIP_SAMPLE_Y_OFFSET: usize = 10;
const HORIZONTAL_SCROLL_DELTA: f32 = 96.0;

#[test]
fn closeable_tab_strip_exposes_leaf_presets_options_and_tab_contract() {
    let presets = StoryPresetLabels::for_page(PAGE);
    let options = storybook_ui_option_contract::options_for_page(PAGE);
    let rows = storybook_ui_option_contract::settings_rows_for(PAGE);
    let spec = StorybookInteractionSpec::for_page(PAGE);

    assert!(presets.len() >= REQUIRED_PRESET_COUNT);
    assert!(options.len() >= REQUIRED_OPTION_COUNT);
    assert_eq!(options.len(), rows.len());
    assert_eq!("select_tab", spec.action);
    assert_eq!("closeable_tab_selected", spec.event);
    assert_eq!("active_tab_id", spec.option);
    assert_eq!("settings", spec.after);
    assert_eq!("tabs.active=settings", spec.state);
}

#[test]
fn closeable_tab_strip_presets_render_distinct_tab_lifecycle_states() {
    let default = StorybookVisual.render_preset(DARK_THEME, PAGE, DEFAULT_PRESET, 0);
    let overflow = StorybookVisual.render_preset(DARK_THEME, PAGE, OVERFLOW_PRESET, 0);
    let pinned = StorybookVisual.render_preset(DARK_THEME, PAGE, PINNED_PRESET, 0);
    let groups = StorybookVisual.render_preset(DARK_THEME, PAGE, GROUPS_PRESET, 0);
    let dirty = StorybookVisual.render_preset(DARK_THEME, PAGE, DIRTY_PRESET, 0);
    let dragging = StorybookVisual.render_preset(DARK_THEME, PAGE, DRAGGING_PRESET, 0);

    assert!(component_body_pixel_diff(PAGE, &default, &overflow) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &overflow, &pinned) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &pinned, &groups) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &groups, &dirty) > BODY_DIFF_THRESHOLD);
    assert!(component_body_pixel_diff(PAGE, &dirty, &dragging) > BODY_DIFF_THRESHOLD);
}

#[test]
fn closeable_tab_strip_setting_option_updates_tab_state() {
    assert_settings_page_changes_body(PAGE);
}

#[test]
fn closeable_tab_strip_active_setting_uses_tab_selection_contract() {
    let mut state = StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    };
    let row = super::layout_metrics::inspector_setting_row_hit_rect(0);

    assert!(apply_click(&mut state, row.x + 1, row.y + 1));
    assert_eq!("select_tab", state.screen_state.last_action);
    assert_eq!("closeable_tab_selected", state.screen_state.last_event);
    assert_eq!("active_tab_id", state.screen_state.last_setting);
    assert_eq!("settings", state.screen_state.last_setting_value);
    assert_eq!("tabs.active=settings", state.screen_state.state_label);
}

#[test]
fn closeable_tab_strip_preview_action_updates_active_tab_state() {
    assert_clicked_page_changes_body(PAGE);
}

#[test]
fn closeable_tab_strip_component_click_selects_real_core_tab() -> Result<(), String> {
    let mut state = closeable_state();
    let component = preview_detail::component_action_hit_rect(PAGE);
    let editor =
        dedicated_closeable_tab_strip::tab_rect_for_test(&state.screen_state.tabs, "editor.rs")
            .ok_or_else(|| "editor tab rect is missing".to_string())?;

    assert!(apply_click(
        &mut state,
        component.x + editor.x + 1,
        component.y + editor.y + 1,
    ));
    assert_eq!("select_tab", state.screen_state.last_action);
    assert_eq!("closeable_tab_selected", state.screen_state.last_event);
    assert_eq!("active_tab_id", state.screen_state.last_setting);
    assert_eq!("editor.rs", state.screen_state.tabs.active_tab_id.as_str());
    Ok(())
}

#[test]
fn closeable_tab_strip_render_uses_runtime_tab_model() {
    let mut state = closeable_state();
    state.screen_state.action_count = 1;
    let before = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );

    state.screen_state.tabs.add_many_for_overflow();
    state.screen_state.tabs.active_tab_id = "terminal".to_string();
    let after = render::render_storybook_canvas_with_screen_state(
        state.theme_id,
        state.selected_page,
        state.preset_index,
        state.screen_state.clone(),
    );

    assert!(component_body_pixel_diff(PAGE, &before, &after) > BODY_DIFF_THRESHOLD);
}

#[test]
fn closeable_tab_strip_context_menu_uses_real_core_commands() -> Result<(), String> {
    let mut state = closeable_state();
    let component = preview_detail::component_action_hit_rect(PAGE);
    let terminal =
        dedicated_closeable_tab_strip::tab_rect_for_test(&state.screen_state.tabs, "terminal")
            .ok_or_else(|| "terminal tab rect is missing".to_string())?;

    assert!(super::window_interaction::apply_context_click_for_test(
        &mut state,
        component.x + terminal.x + 1,
        component.y + terminal.y + 1,
    ));
    assert_eq!("tab_context_menu", state.screen_state.last_action);
    assert_eq!(
        "closeable_tab_context_menu_opened",
        state.screen_state.last_event
    );
    assert!(
        dedicated_closeable_tab_strip::context_menu_labels_for_test(&state.screen_state.tabs)
            .contains(&"閉じる")
    );

    let menu = dedicated_closeable_tab_strip::context_menu_rect_for_test(&state.screen_state.tabs)
        .ok_or_else(|| "context menu rect is missing".to_string())?;
    assert!(apply_click(
        &mut state,
        component.x + menu.x + 1,
        component.y + menu.y + 1,
    ));
    assert_eq!("tab_context_close", state.screen_state.last_action);
    assert_eq!("closeable_tab_closed", state.screen_state.last_event);
    assert!(
        !state
            .screen_state
            .tabs
            .tabs
            .iter()
            .any(|tab| tab.id == "terminal")
    );
    Ok(())
}

#[test]
fn closeable_tab_strip_live_controls_use_core_tab_actions() -> Result<(), String> {
    assert_closeable_control(
        TabsScreenAction::AddTab,
        "add_tab",
        "closeable_tab_added",
        "notes.md",
    )?;
    assert_closeable_control(
        TabsScreenAction::CloseActive,
        "close_tab",
        "closeable_tab_closed",
        "removed",
    )?;
    assert_closeable_control(
        TabsScreenAction::TogglePinActive,
        "toggle_pin_tab",
        "closeable_tab_pin_changed",
        "toggle",
    )?;
    assert_closeable_control(
        TabsScreenAction::MoveActiveRight,
        "move_tab",
        "closeable_tab_reordered",
        "right",
    )?;
    assert_closeable_control(
        TabsScreenAction::GroupActive,
        "move_to_group",
        "closeable_tab_group_changed",
        "Docs",
    )?;
    assert_closeable_control(
        TabsScreenAction::ToggleOverflow,
        "open_overflow",
        "closeable_tab_overflow_opened",
        "menu",
    )
}

#[test]
fn closeable_tab_strip_horizontal_wheel_scrolls_overflowing_tab_row() {
    let mut state = closeable_state();
    state.screen_state.tabs.add_many_for_overflow();
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert!(super::window_interaction::apply_scroll_delta_x_at_for_test(
        &mut state,
        component.x + STRIP_X + 1,
        component.y + STRIP_Y + 1,
        HORIZONTAL_SCROLL_DELTA,
    ));
    assert_eq!("tab_strip_scroll", state.screen_state.last_action);
    assert_eq!(
        "closeable_tab_overflow_scrolled",
        state.screen_state.last_event
    );
    assert!(state.screen_state.tabs.scroll_x > 0);
}

#[test]
fn closeable_tab_strip_light_and_dark_strip_uses_theme_surface() {
    assert_strip_token(DARK_THEME, ThemeSnapshot::dark());
    assert_strip_token(LIGHT_THEME, ThemeSnapshot::light());
}

fn closeable_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}

fn assert_closeable_control(
    action: TabsScreenAction,
    expected_action: &str,
    expected_event: &str,
    expected_value: &str,
) -> Result<(), String> {
    let mut state = closeable_state();
    let component = preview_detail::component_action_hit_rect(PAGE);
    let control = dedicated_closeable_tab_strip::control_rect_for_test(action)
        .ok_or_else(|| format!("control rect is missing for {action:?}"))?;

    assert!(apply_click(
        &mut state,
        component.x + control.x + 1,
        component.y + control.y + 1,
    ));
    assert_eq!(expected_action, state.screen_state.last_action);
    assert_eq!(expected_event, state.screen_state.last_event);
    assert_eq!(expected_value, state.screen_state.last_setting_value);
    Ok(())
}

fn assert_strip_token(theme_id: &str, theme: ThemeSnapshot) {
    let canvas = StorybookVisual.render_preset(theme_id, PAGE, DEFAULT_PRESET, 0);
    let colors = palette::VisualPalette::from_theme(&theme);
    let component = preview_detail::component_action_hit_rect(PAGE);

    assert_eq!(
        Some(colors.surface),
        pixel_at(
            &canvas,
            component.x + STRIP_X + STRIP_SAMPLE_X_OFFSET,
            component.y + STRIP_Y + STRIP_SAMPLE_Y_OFFSET
        )
    );
}
