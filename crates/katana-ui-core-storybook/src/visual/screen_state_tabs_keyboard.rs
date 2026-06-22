use super::screen_state_tabs_core::core_event_name;
use super::screen_state_tabs_types::{TabsScreenState, TabsScreenUpdate, tabs_update};
use katana_ui_core::widget::molecules::CloseableTabKeyboardInput;

impl TabsScreenState {
    pub(in crate::visual) fn apply_keyboard_input(
        &mut self,
        input: CloseableTabKeyboardInput,
    ) -> TabsScreenUpdate {
        let metadata = keyboard_metadata(&input);
        let events = self.apply_core_tab_keyboard_input(input);
        tabs_update(
            metadata.action,
            core_event_name(&events, metadata.fallback_event),
            "tabs.keyboard",
            metadata.value,
            metadata.state,
        )
    }
}

struct KeyboardUpdateMetadata {
    action: &'static str,
    value: &'static str,
    state: &'static str,
    fallback_event: &'static str,
}

fn keyboard_metadata(input: &CloseableTabKeyboardInput) -> KeyboardUpdateMetadata {
    match input {
        CloseableTabKeyboardInput::NextTab | CloseableTabKeyboardInput::PreviousTab => {
            KeyboardUpdateMetadata {
                action: "tab_keyboard_select_relative",
                value: "relative",
                state: "keyboard=select",
                fallback_event: "closeable_tab_select_missing",
            }
        }
        CloseableTabKeyboardInput::SelectVisible(_) => KeyboardUpdateMetadata {
            action: "tab_keyboard_select_visible",
            value: "number",
            state: "keyboard=select",
            fallback_event: "closeable_tab_select_missing",
        },
        CloseableTabKeyboardInput::SelectLastVisible => KeyboardUpdateMetadata {
            action: "tab_keyboard_select_last",
            value: "last",
            state: "keyboard=select",
            fallback_event: "closeable_tab_select_missing",
        },
        CloseableTabKeyboardInput::CloseActiveTab => KeyboardUpdateMetadata {
            action: "tab_keyboard_close",
            value: "close",
            state: "keyboard=close",
            fallback_event: "closeable_tab_close_missing",
        },
        CloseableTabKeyboardInput::CancelDrag => KeyboardUpdateMetadata {
            action: "tab_keyboard_cancel_drag",
            value: "cancel",
            state: "keyboard=cancel",
            fallback_event: "closeable_tab_drag_missing",
        },
    }
}
