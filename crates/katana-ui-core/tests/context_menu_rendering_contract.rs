use katana_ui_core::molecule::{
    ContextMenu, ContextMenuAction, ContextMenuAnchor, ContextMenuDividerTone, ContextMenuItem,
    ContextMenuItemKind, ContextMenuPlacement, ContextMenuSize, ContextMenuViewport,
};
use katana_ui_core::render_model::{
    UiContextMenuAnchor, UiContextMenuItem, UiContextMenuItemKind, UiNode,
};

#[test]
fn context_menu_anchor_variants_render_non_empty_contracts() {
    let anchors = [
        ContextMenuAnchor::Pointer { x: 20, y: 24 },
        ContextMenuAnchor::NodeId("tree-row".to_string()),
        ContextMenuAnchor::VirtualRect(katana_ui_core::molecule::ContextMenuRect::new(
            32, 40, 120, 28,
        )),
    ];

    for anchor in anchors {
        let menu = ContextMenu::new("context")
            .anchor(anchor.clone())
            .item(ContextMenuItem::action("copy", "Copy"));
        let node = UiNode::from(menu);

        assert_eq!(anchor, node.props().context_menu.anchor);
        assert!(!node.props().context_menu.items.is_empty());
    }
}

#[test]
fn submenu_flip_and_internal_scroll_are_numeric_contracts() {
    let mut menu = ContextMenu::new("context")
        .anchor(ContextMenuAnchor::Pointer { x: 760, y: 392 })
        .placement_priority(vec![
            ContextMenuPlacement::BelowStart,
            ContextMenuPlacement::AboveEnd,
        ])
        .items(long_items());
    menu.apply_context_action(&ContextMenuAction::OpenWithLayout {
        anchor: ContextMenuAnchor::Pointer { x: 760, y: 392 },
        menu_size: ContextMenuSize::new(220, 500),
        viewport: ContextMenuViewport::new(800, 400),
    });

    let node = UiNode::from(menu);

    assert_eq!(
        ContextMenuPlacement::AboveEnd,
        node.props().context_menu.placement_used
    );
    assert!(node.props().context_menu.vertical_scroll_enabled);
    assert_eq!(384, node.props().context_menu.render_height);
}

#[test]
fn item_visual_kinds_are_typed_without_image_snapshots() {
    let menu = ContextMenu::new("context")
        .item(ContextMenuItem::new(
            "section",
            "編集",
            ContextMenuItemKind::Section,
        ))
        .item(
            ContextMenuItem::new("divider", "", ContextMenuItemKind::Divider)
                .divider_tone(ContextMenuDividerTone::Emphasis),
        )
        .item(ContextMenuItem::action("delete", "Delete").destructive(true))
        .item(ContextMenuItem::action("disabled", "Disabled").disabled(true))
        .item(ContextMenuItem::new("wrap", "Wrap", ContextMenuItemKind::Toggle).checked(true))
        .item(
            ContextMenuItem::new("mode", "Mode", ContextMenuItemKind::Radio)
                .checked(true)
                .radio_group("view"),
        );
    let node = UiNode::from(menu);
    let items = &node.props().context_menu.items;

    assert!(
        items
            .iter()
            .any(|it| it.kind == ContextMenuItemKind::Section)
    );
    assert!(items.iter().any(|it| {
        it.kind == ContextMenuItemKind::Divider
            && it.divider_tone == ContextMenuDividerTone::Emphasis
    }));
    assert!(items.iter().any(|it| it.destructive));
    assert!(items.iter().any(|it| it.disabled));
    assert!(
        items
            .iter()
            .any(|it| it.kind == ContextMenuItemKind::Toggle && it.checked)
    );
    assert!(
        items
            .iter()
            .any(|it| it.kind == ContextMenuItemKind::Radio && it.radio_group == "view")
    );
}

#[test]
fn render_model_context_item_projects_icon_and_accessibility_builders() {
    let item = UiContextMenuItem::new("open", "Open", UiContextMenuItemKind::Action)
        .leading_icon("folder-open")
        .accessibility_label("Open selected folder");

    assert_eq!("folder-open", item.leading_icon);
    assert_eq!("Open selected folder", item.accessibility_label);
}

#[test]
fn node_anchor_keeps_focus_return_target() {
    let mut menu = ContextMenu::new("context").item(ContextMenuItem::action("open", "Open"));
    menu.apply_context_action(&ContextMenuAction::OpenWithLayout {
        anchor: UiContextMenuAnchor::NodeId("explorer-row".to_string()),
        menu_size: ContextMenuSize::new(160, 120),
        viewport: ContextMenuViewport::new(640, 480),
    });

    let node = UiNode::from(menu);

    assert_eq!(
        "explorer-row",
        node.props().context_menu.focus_return_target
    );
}

fn long_items() -> Vec<ContextMenuItem> {
    (0..36)
        .map(|index| ContextMenuItem::action(format!("item-{index}"), format!("Item {index}")))
        .collect()
}
