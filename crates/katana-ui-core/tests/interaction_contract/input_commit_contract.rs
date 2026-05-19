use katana_ui_core::atom::Input;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::render_model::UiNode;

#[test]
fn input_action_commits_ime_emoji_and_mixed_text_to_owned_state() {
    let mut input = Input::new("Text input")
        .ime_enabled(true)
        .emoji_enabled(true);
    let values = ["日本語", "UI 🔷", "Text 日本語 🔷"];

    for value in values {
        let result = input.apply_action(&UiAction::input_value(input.state_id().clone(), value));

        assert!(result.handled);
        assert_eq!(value, result.after.value);
        assert_eq!("input_value", result.callback_log[0].action);
    }

    let node = UiNode::from(input);

    assert_eq!("Text 日本語 🔷", node.props().interaction.value);
    assert!(node.props().text_entry.ime_enabled);
    assert!(node.props().text_entry.emoji_enabled);
}
