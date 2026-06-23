use katana_ui_core::adapter_contract::AdapterHostActionBridge;
use katana_ui_core::atom::{Button, Input, TextArea};
use katana_ui_core::layout::Column;
use katana_ui_core::molecule::{ContextMenu, ContextMenuItem};
use katana_ui_core::render_model::{UiHostActionKind, UiHostActionPlan, UiNodeId, UiTree};

const COPY_ACTION: &str = "app.copy";
const CLEAR_ACTION: &str = "app.search.clear";
const EXPAND_ACTION: &str = "app.notes.expand";
const MENU_COPY_ACTION: &str = "menu.copy";

#[test]
fn adapter_host_action_bridge_triggers_enabled_button_command() -> Result<(), String> {
    let tree = UiTree::new(Column::new().child(Button::new("Copy").command(COPY_ACTION)));
    let action = trigger(&tree, tree.root().children()[0].id(), COPY_ACTION)?;

    assert_eq!(COPY_ACTION, action.action_id);
    assert_eq!("Copy", action.label);
    assert_eq!(UiHostActionKind::Command, action.kind);
    Ok(())
}

#[test]
fn adapter_host_action_bridge_triggers_text_entry_icon_callback() -> Result<(), String> {
    let tree = UiTree::new(Input::new("Search").trailing_svg_icon_button(
        "Clear",
        "<svg />",
        CLEAR_ACTION,
    ));
    let action = trigger(&tree, tree.root().id(), CLEAR_ACTION)?;

    assert_eq!(CLEAR_ACTION, action.action_id);
    assert_eq!("Clear", action.label);
    assert_eq!(UiHostActionKind::Custom, action.kind);
    Ok(())
}

#[test]
fn adapter_host_action_bridge_triggers_text_area_icon_callback() -> Result<(), String> {
    let tree = UiTree::new(TextArea::new("Notes").trailing_svg_icon_button(
        "Expand",
        "<svg />",
        EXPAND_ACTION,
    ));
    let action = trigger(&tree, tree.root().id(), EXPAND_ACTION)?;

    assert_eq!(EXPAND_ACTION, action.action_id);
    assert_eq!("Expand", action.label);
    assert_eq!(UiHostActionKind::Custom, action.kind);
    Ok(())
}

#[test]
fn adapter_host_action_bridge_rejects_disabled_action() {
    let tree = UiTree::new(Button::new("Copy").command(COPY_ACTION).disabled(true));
    let action = AdapterHostActionBridge::trigger(tree.root(), tree.root().id(), COPY_ACTION);

    assert_eq!(None, action);
}

#[test]
fn adapter_host_action_bridge_triggers_context_menu_item() -> Result<(), String> {
    let tree = UiTree::new(
        ContextMenu::new("Context").item(ContextMenuItem::action(MENU_COPY_ACTION, "Copy path")),
    );
    let action = trigger(&tree, tree.root().id(), MENU_COPY_ACTION)?;

    assert_eq!(MENU_COPY_ACTION, action.action_id);
    assert_eq!("Copy path", action.label);
    assert_eq!(UiHostActionKind::Command, action.kind);
    assert_eq!("path=0 kind=action", action.payload);
    Ok(())
}

fn trigger(tree: &UiTree, target: &UiNodeId, action_id: &str) -> Result<UiHostActionPlan, String> {
    AdapterHostActionBridge::trigger(tree.root(), target, action_id)
        .ok_or_else(|| format!("missing triggered action: {action_id}"))
}
