use super::visual_interaction_test_support::component_body_pixel_diff;
use super::window_interaction::{StorybookWindowState, apply_click};
use super::{layout_metrics, render, storybook_ui_option_contract};

const DARK_THEME: &str = "dark";
const PAGE: &str = "closeable-tab-strip";

#[test]
fn closeable_tab_strip_inspector_options_mutate_active_overflow_pin_and_group_semantic_state()
-> Result<(), String> {
    for &(setting, expected_value, expected_state) in expected_states() {
        let mut state = page_state();
        let before = render_state(&state);
        click_option(&mut state, setting)?;
        let after = render_state(&state);

        assert_eq!(setting, state.screen_state.last_setting);
        assert_eq!(expected_value, state.screen_state.last_setting_value);
        assert_eq!(expected_state, state.screen_state.state_label);
        assert_closeable_tab_event(&state);
        assert_model_state(setting, &state)?;
        assert!(
            component_body_pixel_diff(PAGE, &before, &after) > 0,
            "closeable-tab-strip option `{setting}` must repaint the live component"
        );
    }
    Ok(())
}

fn assert_closeable_tab_event(state: &StorybookWindowState) {
    assert_ne!("none", state.screen_state.last_action);
    assert!(
        state.screen_state.last_event.starts_with("closeable_tab"),
        "closeable-tab-strip option must emit a closeable tab event, got `{}`",
        state.screen_state.last_event
    );
}

fn expected_states() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("active_tab_id", "settings", "tabs.active=settings"),
        ("tabs.overflow", "menu", "tabs.overflow=menu"),
        ("tabs.pin", "true", "tabs.pinned=true left-fixed"),
        ("tabs.group", "created", "tabs.group=Docs"),
    ]
}

fn assert_model_state(setting: &str, state: &StorybookWindowState) -> Result<(), String> {
    match setting {
        "active_tab_id" => assert_eq!("settings", state.screen_state.tabs.active_tab_id),
        "tabs.overflow" => assert!(state.screen_state.tabs.overflow_open),
        "tabs.pin" => assert!(active_tab(state)?.pinned),
        "tabs.group" => assert_eq!(Some("docs"), active_tab(state)?.group_id.as_deref()),
        _ => return Err(format!("unhandled closeable-tab-strip option `{setting}`")),
    }
    Ok(())
}

fn active_tab(
    state: &StorybookWindowState,
) -> Result<&super::screen_state_tabs::TabsScreenTab, String> {
    state
        .screen_state
        .tabs
        .active_tab()
        .ok_or_else(|| "active closeable tab is missing".to_string())
}

fn click_option(state: &mut StorybookWindowState, setting: &str) -> Result<(), String> {
    let index = option_index(setting)?;
    let row = layout_metrics::inspector_setting_row_hit_rect(index);

    assert!(apply_click(state, row.x + 1, row.y + 1));
    Ok(())
}

fn option_index(setting: &str) -> Result<usize, String> {
    storybook_ui_option_contract::options_for_page(PAGE)
        .iter()
        .position(|option| option.setting == setting)
        .ok_or_else(|| format!("missing closeable-tab-strip option `{setting}`"))
}

fn render_state(state: &StorybookWindowState) -> super::Canvas {
    render::render_storybook_canvas_with_screen_state(
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
