use katana_ui_core::atom;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiActionSource};
use katana_ui_core::state::UiComponentState;

pub(in crate::visual) fn default_checkbox_state() -> UiComponentState {
    atom::Checkbox::new("Storybook Checkbox").state_snapshot()
}

pub(in crate::visual) fn default_radio_state() -> UiComponentState {
    atom::Radio::new("Storybook Radio")
        .selected(false)
        .state_snapshot()
}

pub(in crate::visual) fn apply_checkbox_checked_state(
    before: &UiComponentState,
    checked: bool,
) -> UiComponentState {
    let mut checkbox = atom::Checkbox::new("Storybook Checkbox").set_state(before.clone());
    let _result = checkbox.apply_action(&UiAction::checkbox_checked(
        before.state_id.clone(),
        checked,
    ));
    checkbox.state_snapshot()
}

pub(in crate::visual) fn apply_radio_selected_state(
    before: &UiComponentState,
    selected: bool,
) -> UiComponentState {
    let mut radio = atom::Radio::new("Storybook Radio").set_state(before.clone());
    if !selected {
        radio = radio.selected(false);
    }
    if selected {
        let _result = radio.apply_action(&UiAction::radio_selected(before.state_id.clone()));
    }
    radio.state_snapshot()
}

pub(in crate::visual) fn apply_radio_selected_index_state(
    before: &UiComponentState,
    selected_index: usize,
) -> UiComponentState {
    let mut radio = atom::Radio::new("Storybook Radio").set_state(before.clone());
    let _result = radio.apply_action(&UiAction::SetSelectedIndex {
        target: before.state_id.clone(),
        selected_index,
        selected: true,
        source: UiActionSource::Radio,
    });
    radio.state_snapshot()
}

pub(in crate::visual) fn apply_binary_choice_option(
    before: &UiComponentState,
    setting: &str,
) -> Option<UiComponentState> {
    let mut next = before.clone();
    match setting {
        "selected" | "checked" => select_binary_choice(&mut next),
        "disabled" => disable_binary_choice(&mut next),
        "focus" => focus_binary_choice(&mut next),
        _ => return None,
    }
    Some(next)
}

fn select_binary_choice(state: &mut UiComponentState) {
    state.checked = true;
    state.interaction.has_selection = true;
    state.interaction.selected_index = 1;
}

fn disable_binary_choice(state: &mut UiComponentState) {
    state.disabled = true;
    state.common.disabled = true;
}

fn focus_binary_choice(state: &mut UiComponentState) {
    state.focusable = true;
    state.common.focusable = true;
    state.interaction.focused = true;
}

pub(in crate::visual) fn checkbox_state_label(before: bool, after: bool) -> &'static str {
    match (before, after) {
        (false, true) => "before=false after=true",
        (true, false) => "before=true after=false",
        (true, true) => "before=true after=true",
        (false, false) => "before=false after=false",
    }
}

pub(in crate::visual) fn radio_state_label(before: bool, after: bool) -> &'static str {
    match (before, after) {
        (false, true) => "before=false after=true",
        (true, false) => "before=true after=false",
        (true, true) => "before=true after=true",
        (false, false) => "before=false after=false",
    }
}
