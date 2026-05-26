use katana_ui_core::atom;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::state::UiComponentState;

pub(super) fn default_checkbox_state() -> UiComponentState {
    atom::Checkbox::new("Storybook Checkbox").state_snapshot()
}

pub(super) fn default_radio_state() -> UiComponentState {
    atom::Radio::new("Storybook Radio")
        .selected(false)
        .state_snapshot()
}

pub(super) fn apply_checkbox_checked_state(
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

pub(super) fn apply_radio_selected_state(
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

pub(super) fn checkbox_state_label(before: bool, after: bool) -> &'static str {
    match (before, after) {
        (false, true) => "before=false after=true",
        (true, false) => "before=true after=false",
        (true, true) => "before=true after=true",
        (false, false) => "before=false after=false",
    }
}

pub(super) fn radio_state_label(before: bool, after: bool) -> &'static str {
    match (before, after) {
        (false, true) => "before=false after=true",
        (true, false) => "before=true after=false",
        (true, true) => "before=true after=true",
        (false, false) => "before=false after=false",
    }
}
