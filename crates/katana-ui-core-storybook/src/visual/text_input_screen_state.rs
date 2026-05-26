use katana_ui_core::atom;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::state::UiComponentState;

const TEXT_INPUT_LABEL: &str = "Storybook TextInput";
const DEFAULT_TEXT_INPUT_VALUE: &str = "日本語 value 🔷";

pub(super) fn default_text_input_state() -> UiComponentState {
    atom::Input::new(TEXT_INPUT_LABEL)
        .focusable(true)
        .value(DEFAULT_TEXT_INPUT_VALUE)
        .state_snapshot()
}

pub(super) fn text_input_value(state: &UiComponentState) -> &str {
    state.interaction.value.as_str()
}

pub(super) fn apply_text_input_focus_state(
    before: &UiComponentState,
    focused: bool,
) -> UiComponentState {
    let mut input = atom::Input::new(TEXT_INPUT_LABEL).set_state(before.clone());
    let action = if focused {
        UiAction::focus(before.state_id.clone())
    } else {
        UiAction::blur(before.state_id.clone())
    };
    let _result = input.apply_action(&action);
    input.state_snapshot()
}

pub(super) fn apply_text_input_value_state(
    before: &UiComponentState,
    value: &str,
) -> UiComponentState {
    let mut input = atom::Input::new(TEXT_INPUT_LABEL).set_state(before.clone());
    let _result = input.apply_action(&UiAction::input_value(before.state_id.clone(), value));
    input.state_snapshot()
}

pub(super) fn apply_text_input_submit_state(before: &UiComponentState) -> UiComponentState {
    let mut input = atom::Input::new(TEXT_INPUT_LABEL).set_state(before.clone());
    let _result = input.apply_action(&UiAction::input_submitted(before.state_id.clone()));
    input.state_snapshot()
}
