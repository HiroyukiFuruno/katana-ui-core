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
    let instance =
        super::component_instance_id_for_page(state.selected_page, state.selected_instance_id);
    state.screen_state.register_text_area_key_for(
        instance,
        match key {
            TextAreaKey::Character(value) => TextAreaInputKey::Character(value),
            TextAreaKey::Backspace => TextAreaInputKey::Backspace,
            TextAreaKey::Newline => TextAreaInputKey::Newline,
            TextAreaKey::Submit => TextAreaInputKey::Submit,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_area_key_rejects_other_pages_and_maps_newline() {
        let mut wrong_page = StorybookWindowState::default();
        assert!(!apply_text_area_key(&mut wrong_page, TextAreaKey::Newline));

        let mut state = StorybookWindowState {
            selected_page: TEXT_AREA_PAGE,
            ..StorybookWindowState::default()
        };
        let instance = super::super::component_instance_id_for_page(
            state.selected_page,
            state.selected_instance_id,
        );
        state
            .screen_state
            .register_text_area_focus_for(instance, false, false);

        assert!(apply_text_area_key(&mut state, TextAreaKey::Newline));
        assert_eq!("text_area_newline", state.screen_state.last_action);
    }
}
