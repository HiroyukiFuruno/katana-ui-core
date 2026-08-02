use katana_ui_core::molecule::{
    ArrayEditorItem, DynamicArrayEditor, DynamicArrayEditorAction, DynamicArrayEditorEvent,
};
use katana_ui_core::render_model::UiTree;

#[test]
fn dynamic_array_editor_mutates_items_through_public_actions() {
    let mut editor = DynamicArrayEditor::new("Rows")
        .item(row("row-1", "One"))
        .item(row("row-2", "Two"));

    assert_eq!(
        vec![DynamicArrayEditorEvent::ItemAdded {
            id: "row-3".to_string()
        }],
        editor.apply_array_action(DynamicArrayEditorAction::AddItem(row("row-3", "Three")))
    );
    assert_eq!(3, editor.items().len());
    assert_eq!(
        3,
        UiTree::new(editor.clone())
            .root()
            .props()
            .interaction
            .item_count
    );

    assert_eq!(
        vec![DynamicArrayEditorEvent::ItemReordered {
            id: "row-2".to_string(),
            from: 1,
            to: 0
        }],
        editor.apply_array_action(DynamicArrayEditorAction::ReorderItem { from: 1, to: 0 })
    );
    assert_eq!("row-2", editor.items()[0].id);

    assert_eq!(
        vec![DynamicArrayEditorEvent::ItemEdited {
            id: "row-2".to_string()
        }],
        editor.apply_array_action(DynamicArrayEditorAction::EditItem {
            id: "row-2".to_string(),
            value: "Updated".to_string()
        })
    );
    assert_eq!("Updated", editor.items()[0].value);

    assert_eq!(
        vec![DynamicArrayEditorEvent::ItemRemoved {
            id: "row-1".to_string()
        }],
        editor.apply_array_action(DynamicArrayEditorAction::RemoveItem("row-1".to_string()))
    );
    assert_eq!(2, editor.items().len());
    assert_eq!(2, UiTree::new(editor).root().props().interaction.item_count);
}

#[test]
fn dynamic_array_editor_validation_and_removable_guard_are_core_events() {
    let mut editor = DynamicArrayEditor::new("Rows")
        .item(row("locked", "Locked").removable(false))
        .item(ArrayEditorItem::new("empty", "Empty"));

    assert!(
        editor
            .apply_array_action(DynamicArrayEditorAction::RemoveItem("locked".to_string()))
            .is_empty(),
        "non-removable rows must block remove action"
    );

    assert_eq!(
        vec![DynamicArrayEditorEvent::ValidationChanged {
            valid: false,
            message: "array item value required".to_string()
        }],
        editor.apply_array_action(DynamicArrayEditorAction::Validate)
    );

    editor.apply_array_action(DynamicArrayEditorAction::EditItem {
        id: "empty".to_string(),
        value: "Filled".to_string(),
    });
    assert_eq!(
        vec![DynamicArrayEditorEvent::ValidationChanged {
            valid: true,
            message: "valid".to_string()
        }],
        editor.apply_array_action(DynamicArrayEditorAction::Validate)
    );

    for action in [
        DynamicArrayEditorAction::RemoveItem("missing".to_string()),
        DynamicArrayEditorAction::ReorderItem { from: 0, to: 0 },
        DynamicArrayEditorAction::ReorderItem { from: 99, to: 0 },
        DynamicArrayEditorAction::EditItem {
            id: "missing".to_string(),
            value: "Ignored".to_string(),
        },
    ] {
        assert!(editor.apply_array_action(action).is_empty());
    }
}

fn row(id: &str, label: &str) -> ArrayEditorItem {
    ArrayEditorItem::new(id, label).value(label)
}
