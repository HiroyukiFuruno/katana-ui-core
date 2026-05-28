use super::StorybookWindowState;
use crate::visual::screen_state_text_area::TextAreaInputKey;

const TEXT_AREA_PAGE: &str = "text-area";

pub(in crate::visual) enum TextAreaKey {
    Character(char),
    Backspace,
    Newline,
    Submit,
}

pub(in crate::visual) fn apply_text_area_key(
    state: &mut StorybookWindowState,
    key: TextAreaKey,
) -> bool {
    if state.selected_page != TEXT_AREA_PAGE {
        return false;
    }
    state.screen_state.register_text_area_key(match key {
        TextAreaKey::Character(value) => TextAreaInputKey::Character(value),
        TextAreaKey::Backspace => TextAreaInputKey::Backspace,
        TextAreaKey::Newline => TextAreaInputKey::Newline,
        TextAreaKey::Submit => TextAreaInputKey::Submit,
    })
}
