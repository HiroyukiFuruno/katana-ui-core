use super::window_interaction::{StorybookWindowState, apply_tabs_keyboard_shortcut_for_test};
use katana_ui_core::widget::molecules::{CloseableTabKey, CloseableTabKeyboardShortcut};

const PAGE: &str = "closeable-tab-strip";

#[test]
fn closeable_tab_strip_keyboard_shortcuts_route_through_storybook_window_interaction() {
    let mut state = closeable_tab_strip_state();

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

    assert!(apply_tabs_keyboard_shortcut_for_test(
        &mut state,
        shortcut(CloseableTabKey::Tab, true, false),
    ));
    assert_eq!(
        "tab_keyboard_select_relative",
        state.screen_state.last_action
    );
    assert_eq!("closeable_tab_selected", state.screen_state.last_event);
    assert_eq!("preview.rs", state.screen_state.tabs.active_tab_id);

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
}

fn shortcut(
    key: CloseableTabKey,
    command_or_control: bool,
    shift: bool,
) -> CloseableTabKeyboardShortcut {
    CloseableTabKeyboardShortcut::new(key, command_or_control, shift)
}

fn closeable_tab_strip_state() -> StorybookWindowState {
    StorybookWindowState {
        selected_page: PAGE,
        ..StorybookWindowState::default()
    }
}
