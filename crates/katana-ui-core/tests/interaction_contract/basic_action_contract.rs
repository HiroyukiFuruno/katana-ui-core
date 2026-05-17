use katana_ui_core::atom::{Button, Checkbox, Input, ProgressBar, Radio, Toggle};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::render_model::UiNode;

const PROGRESS_PERCENT: u8 = 64;

#[test]
fn action_targets_only_the_matching_component_state() {
    let mut first = Button::new("Save");
    let mut second = Button::new("Save");
    let action = UiAction::button_press(first.state_id().clone());

    let first_result = first.apply_action(&action);
    let second_result = second.apply_action(&action);

    assert!(first_result.handled);
    assert!(!second_result.handled);
    assert_eq!(1, first_result.callback_log.len());
    assert_eq!("button_press", first_result.callback_log[0].action);
}

#[test]
fn state_id_is_unique_and_value_updates_only_matching_target() {
    let mut first = Input::new("Name");
    let mut second = Input::new("Name");
    let action = UiAction::input_value(first.state_id().clone(), "KUC");

    assert_ne!(first.state_id(), second.state_id());
    let first_result = first.apply_action(&action);
    let second_result = second.apply_action(&action);

    assert!(first_result.handled);
    assert!(!second_result.handled);
    assert_eq!("KUC", UiNode::from(first).props().interaction.value);
    assert_eq!("", UiNode::from(second).props().interaction.value);
}

#[test]
fn component_action_updates_owned_state_before_rendering() {
    let mut input = Input::new("Text input");
    let action = UiAction::input_value(input.state_id().clone(), "typed");

    let result = input.apply_action(&action);
    let node = UiNode::from(input);

    assert!(result.handled);
    assert_eq!("typed", result.after.value);
    assert_eq!("typed", node.props().interaction.value);
    assert_eq!("input_value", result.callback_log[0].action);
}

#[test]
fn readonly_input_rejects_value_mutation_actions() {
    let mut input = Input::new("Text input").value("locked").readonly(true);
    let input_value_result =
        input.apply_action(&UiAction::input_value(input.state_id().clone(), "changed"));
    let clear_value_result = input.apply_action(&UiAction::clear_value(input.state_id().clone()));
    let node = UiNode::from(input);

    assert!(!input_value_result.handled);
    assert!(!clear_value_result.handled);
    assert!(input_value_result.callback_log.is_empty());
    assert!(clear_value_result.callback_log.is_empty());
    assert_eq!("locked", node.props().interaction.value);
}

#[test]
fn clear_action_clears_input_value_only_when_editable() {
    let mut editable = Input::new("Text input")
        .value("typed")
        .clear_action("Clear");
    let mut disabled = Input::new("Text input").value("locked").disabled(true);
    let clear_editable = UiAction::clear_value(editable.state_id().clone());
    let clear_disabled = UiAction::clear_value(disabled.state_id().clone());

    let editable_result = editable.apply_action(&clear_editable);
    let disabled_result = disabled.apply_action(&clear_disabled);

    assert!(editable_result.handled);
    assert!(!disabled_result.handled);
    assert_eq!("", UiNode::from(editable).props().interaction.value);
    assert_eq!("locked", UiNode::from(disabled).props().interaction.value);
}

#[test]
fn selection_actions_update_only_their_owned_state() {
    let mut checkbox = Checkbox::new("Enabled");
    let mut radio = Radio::new("Manual");
    let mut toggle = Toggle::new("Live");

    let checkbox_result = checkbox.apply_action(&UiAction::checkbox_checked(
        checkbox.state_id().clone(),
        true,
    ));
    let radio_result = radio.apply_action(&UiAction::radio_selected(radio.state_id().clone()));
    let toggle_result =
        toggle.apply_action(&UiAction::toggle_checked(toggle.state_id().clone(), true));

    assert!(checkbox_result.handled);
    assert!(radio_result.handled);
    assert!(toggle_result.handled);
    assert!(UiNode::from(checkbox).props().checked);
    assert!(UiNode::from(radio).props().checked);
    assert!(UiNode::from(toggle).props().checked);
    assert_eq!("checkbox_checked", checkbox_result.callback_log[0].action);
    assert_eq!("radio_selected", radio_result.callback_log[0].action);
    assert_eq!("toggle_checked", toggle_result.callback_log[0].action);
}

#[test]
fn progress_action_updates_typed_progress_props() {
    let mut progress = ProgressBar::new("Sync");
    let result = progress.apply_action(&UiAction::progress_changed(
        progress.state_id().clone(),
        true,
        PROGRESS_PERCENT,
    ));
    let node = UiNode::from(progress);

    assert!(result.handled);
    assert!(node.props().determinate);
    assert_eq!(PROGRESS_PERCENT, node.props().progress_percent);
    assert_eq!("progress_changed", result.callback_log[0].action);
}
