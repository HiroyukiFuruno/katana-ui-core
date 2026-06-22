use katana_ui_core::atom::Input;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;

#[test]
fn callback_action_invokes_named_callback_without_mutating_value() {
    let mut input = Input::new("Search")
        .value("locked")
        .readonly(true)
        .trailing_svg_icon_button("Clear", "<svg />", "search.clear");
    let action = UiAction::invoke_callback(input.state_id().clone(), "search.clear");

    let result = input.apply_action(&action);

    assert!(result.handled);
    assert_eq!("callback_invoked", action.name());
    assert_eq!("search.clear", result.callback_log[0].action);
    assert_eq!("locked", result.before.value);
    assert_eq!("locked", result.after.value);
}

#[test]
fn disabled_input_blocks_callback_action() {
    let mut input = Input::new("Search").disabled(true);
    let action = UiAction::invoke_callback(input.state_id().clone(), "search.clear");

    let result = input.apply_action(&action);

    assert!(!result.handled);
    assert!(result.callback_log.is_empty());
}
