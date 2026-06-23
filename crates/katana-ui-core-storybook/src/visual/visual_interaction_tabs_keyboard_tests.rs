use super::visual_interaction_test_support::require_some;
use super::window_interaction::{
    StorybookWindowState, apply_tabs_keyboard_shortcut_for_test, start_tabs_drag_at_for_test,
};
use super::{dedicated_tabs, preview_detail};
use katana_ui_core::widget::molecules::{CloseableTabKey, CloseableTabKeyboardShortcut};

const PAGE: &str = "tabs";

#[test]
fn tabs_keyboard_shortcuts_route_through_storybook_window_interaction() -> Result<(), String> {
    let mut state = tabs_state();

    assert!(apply_tabs_keyboard_shortcut_for_test(
        &mut state,
        shortcut(CloseableTabKey::Digit(2), true, false),
    ));
    assert_eq!(
        "tab_keyboard_select_visible",
        state.screen_state.last_action
    );
    assert_eq!("closeable_tab_selected", state.screen_state.last_event);
    assert_eq!("editor.rs", state.screen_state.tabs.active_tab_id);

    state.screen_state.tabs.active_tab_id = "scratch.md".to_string();
    assert!(apply_tabs_keyboard_shortcut_for_test(
        &mut state,
        shortcut(CloseableTabKey::W, true, false),
    ));
    assert_eq!("tab_keyboard_close", state.screen_state.last_action);
    assert_eq!(
        "closeable_tab_close_requested",
        state.screen_state.last_event
    );
    assert!(
        state
            .screen_state
            .tabs
            .tabs
            .iter()
            .any(|tab| tab.id == "scratch.md")
    );

    start_drag(&mut state, "readme.md")?;
    assert!(apply_tabs_keyboard_shortcut_for_test(
        &mut state,
        shortcut(CloseableTabKey::Escape, false, false),
    ));
    assert_eq!("tab_drag_end", state.screen_state.last_action);
    assert_eq!("closeable_tab_drag_ended", state.screen_state.last_event);
    assert_eq!("cancelled", state.screen_state.last_setting_value);
    assert!(state.tabs_drag_target.is_none());
    Ok(())
}

fn start_drag(state: &mut StorybookWindowState, tab_id: &str) -> Result<(), String> {
    let component = preview_detail::component_action_hit_rect(PAGE);
    let tab = require_some(
        dedicated_tabs::tab_rect_for_test(&state.screen_state.tabs, tab_id),
        "tab rect",
    )?;

    assert!(start_tabs_drag_at_for_test(
        state,
        component.x + tab.x + tab.width / 2,
        component.y + tab.y + tab.height / 2,
    ));
    Ok(())
}

fn shortcut(
    key: CloseableTabKey,
    command_or_control: bool,
    shift: bool,
) -> CloseableTabKeyboardShortcut {
    CloseableTabKeyboardShortcut::new(key, command_or_control, shift)
}

fn tabs_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}
