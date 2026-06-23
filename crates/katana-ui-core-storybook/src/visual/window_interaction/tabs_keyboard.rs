use super::StorybookWindowState;
use katana_ui_core::widget::molecules::{CloseableTabKeyboardInput, CloseableTabKeyboardShortcut};

pub(in crate::visual) fn apply_tabs_keyboard_shortcut(
    state: &mut StorybookWindowState,
    shortcut: CloseableTabKeyboardShortcut,
) -> bool {
    if !is_tab_story_page(state.selected_page) {
        return false;
    }
    let Some(input) = CloseableTabKeyboardInput::from_shortcut(shortcut) else {
        return false;
    };
    apply_tabs_keyboard_input(state, input)
}

fn apply_tabs_keyboard_input(
    state: &mut StorybookWindowState,
    input: CloseableTabKeyboardInput,
) -> bool {
    if input == CloseableTabKeyboardInput::CancelDrag {
        return cancel_drag(state);
    }
    state.screen_state.register_tabs_keyboard_input(input);
    true
}

fn cancel_drag(state: &mut StorybookWindowState) -> bool {
    let Some(target) = state.tabs_drag_target.take() else {
        return false;
    };
    state
        .screen_state
        .register_tabs_drag_end(&target.tab_id, false);
    true
}

fn is_tab_story_page(page: &str) -> bool {
    page == "tabs" || page == "closeable-tab-strip"
}
