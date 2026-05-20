use katana_ui_core::molecule::{
    ChoiceItem, ContextMenu, ContextMenuAction, ContextMenuAnchor, ContextMenuCloseReason,
    ContextMenuDividerTone, ContextMenuEvent, ContextMenuItem, ContextMenuItemKind,
    ContextMenuPlacement, ContextMenuRect, ContextMenuSize, ContextMenuViewport,
};
use katana_ui_core::render_model::UiNode;

#[test]
fn open_action_accepts_all_anchor_kinds_and_defaults_focus_for_node_anchor() {
    let anchors = [
        ContextMenuAnchor::Pointer { x: 24, y: 32 },
        ContextMenuAnchor::VirtualRect(ContextMenuRect::new(48, 64, 120, 28)),
        ContextMenuAnchor::NodeId("editor-row".to_string()),
    ];

    for anchor in anchors {
        let mut menu = ContextMenu::new("context").item(ContextMenuItem::action("copy", "Copy"));
        let event = menu.apply_context_action(&ContextMenuAction::OpenWithLayout {
            anchor: anchor.clone(),
            menu_size: ContextMenuSize::new(180, 120),
            viewport: ContextMenuViewport::new(800, 600),
        });
        assert!(matches!(
            event,
            ContextMenuEvent::Opened {
                placement_used: ContextMenuPlacement::BelowStart,
                ..
            }
        ));
        let node = UiNode::from(menu);
        assert_eq!(anchor, node.props().context_menu.anchor);
        if matches!(
            node.props().context_menu.anchor,
            ContextMenuAnchor::NodeId(_)
        ) {
            assert_eq!("editor-row", node.props().context_menu.focus_return_target);
        }
    }

    let node = UiNode::from(
        ContextMenu::new("context")
            .anchor(ContextMenuAnchor::Pointer { x: 8, y: 8 })
            .focus_return_target("caller-button"),
    );
    assert_eq!(
        "caller-button",
        node.props().context_menu.focus_return_target
    );
}

#[test]
fn very_tall_menu_clamps_height_and_enables_internal_scroll() {
    let mut menu = ContextMenu::new("context").items(long_items());
    menu.apply_context_action(&ContextMenuAction::OpenWithLayout {
        anchor: ContextMenuAnchor::Pointer { x: 30, y: 30 },
        menu_size: ContextMenuSize::new(180, 900),
        viewport: ContextMenuViewport::new(800, 600),
    });

    let node = UiNode::from(menu);
    let props = &node.props().context_menu;

    assert_eq!(584, props.render_height);
    assert!(props.vertical_scroll_enabled);
    assert_eq!(vec![0], props.highlighted_path);
}

#[test]
fn submenu_open_highlights_first_enabled_child_and_close_restores_parent() {
    let mut menu = ContextMenu::new("context").item(
        ContextMenuItem::new("insert", "Insert", ContextMenuItemKind::Submenu)
            .child(ContextMenuItem::new(
                "section",
                "Insert",
                ContextMenuItemKind::Section,
            ))
            .child(ContextMenuItem::action("disabled", "Disabled").disabled(true))
            .child(ContextMenuItem::action("table", "Table")),
    );

    let opened = menu.apply_context_action(&ContextMenuAction::OpenSubmenu { path: vec![0] });
    let node_after_open = UiNode::from(menu.clone());
    let closed = menu.apply_context_action(&ContextMenuAction::CloseSubmenu { path: vec![0] });
    let node_after_close = UiNode::from(menu);

    assert_eq!(ContextMenuEvent::SubmenuOpened { path: vec![0] }, opened);
    assert_eq!(
        vec![0, 2],
        node_after_open.props().context_menu.highlighted_path
    );
    assert_eq!(ContextMenuEvent::SubmenuClosed { path: vec![0] }, closed);
    assert_eq!(
        vec![0],
        node_after_close.props().context_menu.highlighted_path
    );
}

#[test]
fn close_reason_keeps_escape_outside_selected_and_focus_return_distinct() {
    for reason in [
        ContextMenuCloseReason::Escape,
        ContextMenuCloseReason::OutsideClick,
        ContextMenuCloseReason::FocusReturn,
    ] {
        let mut menu = ContextMenu::new("context").item(ContextMenuItem::action("copy", "Copy"));
        let event = menu.apply_context_action(&ContextMenuAction::Close { reason });

        assert_eq!(ContextMenuEvent::Closed { reason }, event);
    }

    let mut menu = ContextMenu::new("context").item(ContextMenuItem::action("copy", "Copy"));
    let selected = menu.apply_context_action(&ContextMenuAction::Activate { path: vec![0] });
    assert!(matches!(selected, ContextMenuEvent::ItemSelected { .. }));
    assert!(menu.callback_log().contains(&ContextMenuEvent::Closed {
        reason: ContextMenuCloseReason::Selected
    }));
}

#[test]
fn divider_tone_and_choice_round_trip_are_typed() {
    let divider = ContextMenuItem::new("separator", "", ContextMenuItemKind::Divider)
        .divider_tone(ContextMenuDividerTone::Emphasis);
    let source = ChoiceItem::new("open", "Open").disabled(true);
    let item = ContextMenuItem::from_choice_item(source.clone());
    let round_trip = item.to_choice_item();

    assert_eq!(ContextMenuDividerTone::Emphasis, divider.divider_tone);
    assert_eq!(Some(source), round_trip);
    assert_eq!(None, divider.to_choice_item());
}

fn long_items() -> Vec<ContextMenuItem> {
    (0..40)
        .map(|index| ContextMenuItem::action(format!("item-{index}"), format!("Item {index}")))
        .collect()
}
