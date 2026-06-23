use super::StorybookWindowState;

const TEXT_INPUT_PAGE: &str = "text-input";
const TEXT_INPUT_READONLY_PRESET_INDEX: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::visual) enum TextInputKey {
    Character(char),
    Backspace,
    Submit,
}

pub(in crate::visual) fn apply_text_input_key(
    state: &mut StorybookWindowState,
    key: TextInputKey,
) -> bool {
    if state.selected_page != TEXT_INPUT_PAGE {
        return false;
    }
    let readonly = state.preset_index == TEXT_INPUT_READONLY_PRESET_INDEX;
    let instance =
        super::component_instance_id_for_page(state.selected_page, state.selected_instance_id);
    match key {
        TextInputKey::Character(value) => state
            .screen_state
            .register_text_input_character_for(instance, value, readonly),
        TextInputKey::Backspace => state
            .screen_state
            .register_text_input_backspace_for(instance, readonly),
        TextInputKey::Submit => state.screen_state.register_text_input_submit_for(instance),
    }
}
