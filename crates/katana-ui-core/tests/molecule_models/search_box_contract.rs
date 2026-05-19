use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::SearchBox;
use katana_ui_core::render_model::UiTree;

#[test]
fn search_box_owns_input_options_and_clear_affordance() {
    let search = SearchBox::new("Search")
        .placeholder("Find file")
        .value("katana")
        .clear_action("Clear query")
        .submit_on_enter(true)
        .case_sensitive(true);

    assert!(search.submits_on_enter());
    assert!(search.is_case_sensitive());

    let tree = UiTree::new(search);

    assert_eq!("Find file", tree.root().props().placeholder);
    assert_eq!("katana", tree.root().props().interaction.value);
    assert_eq!(
        Some("Clear query"),
        tree.root()
            .props()
            .text_entry
            .clear_action
            .as_ref()
            .map(|it| it.label.as_str())
    );
}

#[test]
fn search_box_input_clear_and_submit_update_owned_state() {
    let mut search = SearchBox::new("Search").value("katana");
    let input = UiAction::input_value(search.state_id().clone(), "query");
    let clear = UiAction::clear_value(search.state_id().clone());
    let submit = UiAction::search_submitted(search.state_id().clone());

    let input_result = search.apply_action(&input);
    let clear_result = search.apply_action(&clear);
    let submit_result = search.apply_action(&submit);

    assert!(input_result.handled);
    assert_eq!("query", input_result.after.value);
    assert!(clear_result.handled);
    assert_eq!("", clear_result.after.value);
    assert!(submit_result.handled);
    assert_eq!("search_submitted", submit_result.callback_log[0].action);
}
