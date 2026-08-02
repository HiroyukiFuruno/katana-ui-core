use katana_ui_core::atom::Text;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{ChoiceItem, SegmentedToggle};
use katana_ui_core::render_model::UiTree;

#[test]
fn segmented_toggle_owns_items_selection_and_keyboard_model() {
    let toggle = SegmentedToggle::new("Mode")
        .item(ChoiceItem::new("preview", "Preview"))
        .item(ChoiceItem::new("code", "Code").disabled(true))
        .selected_index(0)
        .keyboard_navigation("left-right");

    assert_eq!(2, toggle.items().len());
    assert_eq!("left-right", toggle.keyboard_navigation_model());

    let tree = UiTree::new(toggle);

    assert_eq!(2, tree.root().props().interaction.item_count);
    assert_eq!(0, tree.root().props().interaction.selected_index);
    assert!(tree.root().props().interaction.has_selection);
}

#[test]
fn segmented_toggle_action_ignores_disabled_segment() {
    let mut toggle = SegmentedToggle::new("Mode")
        .item(ChoiceItem::new("preview", "Preview"))
        .item(ChoiceItem::new("code", "Code").disabled(true));
    let disabled_action = UiAction::segmented_toggle_selected(toggle.state_id().clone(), 1);
    let enabled_action = UiAction::segmented_toggle_selected(toggle.state_id().clone(), 0);

    let disabled_result = toggle.apply_action(&disabled_action);
    let enabled_result = toggle.apply_action(&enabled_action);

    assert!(!disabled_result.handled);
    assert!(enabled_result.handled);
    assert_eq!(
        "segmented_toggle_selected",
        enabled_result.callback_log[0].action
    );
    assert_eq!(0, enabled_result.after.selected_index);
}

#[test]
fn segmented_toggle_child_and_generic_component_actions_are_projected() {
    let mut toggle = SegmentedToggle::new("Mode")
        .item_count(2)
        .child(Text::new("Consumer hint"));
    let target = toggle.state_id().clone();

    assert!(
        toggle
            .apply_action(&UiAction::focus(target.clone()))
            .handled
    );
    assert!(
        !toggle
            .apply_action(&UiAction::focus(
                katana_ui_core::render_model::UiStateId::new("other-toggle"),
            ))
            .handled
    );
    let tree = UiTree::new(toggle);
    assert_eq!(2, tree.root().props().interaction.item_count);
    assert_eq!("Consumer hint", tree.root().children()[0].props().label);
}
