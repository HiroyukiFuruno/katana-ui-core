use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiActionSource};
use katana_ui_core::molecule::{
    ChoiceItem, ContextMenu, ContextMenuAction, ContextMenuAnchor, ContextMenuCloseReason,
    ContextMenuEvent, ContextMenuItem, ContextMenuItemKind, ContextMenuKeyboardInput,
    ContextMenuKeyboardNavigator, ContextMenuPlacement, ContextMenuPlacementResolver,
    ContextMenuRect, ContextMenuSize, ContextMenuViewport,
};
use katana_ui_core::render_model::{UiNodeKind, UiTree};

#[test]
fn context_menu_renders_typed_anchor_items_and_state() {
    let tree = UiTree::new(
        ContextMenu::new("Editor menu")
            .anchor(ContextMenuAnchor::Pointer { x: 120, y: 64 })
            .placement_used(ContextMenuPlacement::BelowStart)
            .focus_return_target("editor")
            .item(ContextMenuItem::action("copy", "Copy").shortcut("Cmd+C"))
            .item(ContextMenuItem::new(
                "view",
                "View",
                ContextMenuItemKind::Submenu,
            )),
    );
    let root = tree.root();

    assert_eq!(UiNodeKind::ContextMenu, root.kind());
    assert_eq!(2, root.props().context_menu.items.len());
    assert_eq!("editor", root.props().context_menu.focus_return_target);
    assert_eq!(
        ContextMenuAnchor::Pointer { x: 120, y: 64 },
        root.props().context_menu.anchor
    );
}

#[test]
fn context_menu_open_highlight_activate_and_close_emit_events() {
    let mut menu = ContextMenu::new("Explorer menu")
        .item(ContextMenuItem::action("rename", "Rename").disabled(true))
        .item(ContextMenuItem::action("delete", "Delete"));

    let opened = menu.apply_context_action(&ContextMenuAction::Open {
        anchor: ContextMenuAnchor::VirtualRect(ContextMenuRect::new(8, 16, 24, 24)),
    });
    let highlighted = menu.apply_context_action(&ContextMenuAction::Highlight { path: vec![1] });
    let selected = menu.apply_context_action(&ContextMenuAction::Activate { path: vec![1] });
    let closed = menu.apply_context_action(&ContextMenuAction::Close {
        reason: ContextMenuCloseReason::Escape,
    });

    assert_eq!("context_menu_opened", opened.name());
    assert_eq!(
        ContextMenuEvent::ItemHighlighted { path: vec![1] },
        highlighted
    );
    assert_eq!(
        ContextMenuEvent::ItemSelected {
            path: vec![1],
            command: "delete".to_string()
        },
        selected
    );
    assert_eq!("context_menu_closed", closed.name());
    assert_eq!(5, menu.callback_log().len());
    assert!(matches!(
        menu.callback_log().get(3),
        Some(ContextMenuEvent::Closed {
            reason: ContextMenuCloseReason::Selected
        })
    ));
}

#[test]
fn context_menu_disabled_activation_is_blocked_without_closing() {
    let mut menu = ContextMenu::new("Explorer menu")
        .item(ContextMenuItem::action("rename", "Rename").disabled(true))
        .item(ContextMenuItem::action("delete", "Delete"));

    menu.apply_context_action(&ContextMenuAction::Open {
        anchor: ContextMenuAnchor::VirtualRect(ContextMenuRect::new(8, 16, 24, 24)),
    });
    let blocked = menu.apply_context_action(&ContextMenuAction::Activate { path: vec![0] });
    let node = UiTree::new(menu).root().clone();

    assert_eq!(
        ContextMenuEvent::ItemActivationBlocked { path: vec![0] },
        blocked
    );
    assert!(node.props().interaction.open);
    assert_eq!(vec![0], node.props().context_menu.highlighted_path);
}

#[test]
fn context_menu_submenus_get_unique_state_ids() {
    let menu = ContextMenu::new("Nested menu")
        .item(
            ContextMenuItem::new("insert", "Insert", ContextMenuItemKind::Submenu)
                .child(ContextMenuItem::action("table", "Table")),
        )
        .item(
            ContextMenuItem::new("view", "View", ContextMenuItemKind::Submenu)
                .child(ContextMenuItem::action("preview", "Preview")),
        );
    let submenu_ids = menu.submenu_state_ids();

    assert_eq!(2, submenu_ids.len());
    assert_ne!(submenu_ids[0], submenu_ids[1]);
    assert!(submenu_ids.iter().all(|it| it != menu.state_id()));
}

#[test]
fn context_menu_supports_generic_ui_actions() {
    let mut menu = ContextMenu::new("Tab menu").item(ContextMenuItem::action("close", "Close"));
    let target = UiTree::new(menu.clone()).root().props().state_id.clone();

    let result = menu.apply_action(&UiAction::Press {
        target: target.clone(),
        source: UiActionSource::Click,
    });
    assert!(result.handled);

    let open = menu.apply_action(&UiAction::SetOpen { target, open: true });
    assert!(open.handled);
    assert!(open.after.open);
}

#[test]
fn context_menu_invokes_callback_as_item_activation() {
    let mut menu = ContextMenu::new("Tab menu")
        .item(ContextMenuItem::action("close", "Close").disabled(true))
        .item(
            ContextMenuItem::new("insert", "Insert", ContextMenuItemKind::Submenu)
                .child(ContextMenuItem::action("table", "Table")),
        );
    let target = UiTree::new(menu.clone()).root().props().state_id.clone();

    let disabled = menu.apply_action(&UiAction::invoke_callback(target.clone(), "close"));
    let activated = menu.apply_action(&UiAction::invoke_callback(target, "table"));

    assert!(!disabled.handled);
    assert!(activated.handled);
    assert_eq!("table", activated.callback_log[0].action);
    assert!(!activated.after.open);
    assert!(matches!(
        menu.callback_log().first(),
        Some(ContextMenuEvent::ItemSelected { path, command })
            if path == &vec![1, 0] && command == "table"
    ));
}

#[test]
fn placement_flips_to_first_fitting_priority() {
    let result = ContextMenuPlacementResolver::resolve(
        &ContextMenuAnchor::VirtualRect(ContextMenuRect::new(740, 520, 40, 32)),
        ContextMenuSize::new(180, 140),
        ContextMenuViewport::new(800, 600),
        &[
            ContextMenuPlacement::BelowStart,
            ContextMenuPlacement::AboveEnd,
            ContextMenuPlacement::LeftStart,
        ],
    );

    assert_eq!(ContextMenuPlacement::AboveEnd, result.placement);
    assert!(result.x + 180 <= 792);
    assert!(result.y >= 8);
}

#[test]
fn keyboard_navigation_skips_non_selectable_items() {
    let items = vec![
        ContextMenuItem::new("section", "Edit", ContextMenuItemKind::Section),
        ContextMenuItem::action("copy", "Copy").disabled(true),
        ContextMenuItem::action("paste", "Paste"),
        ContextMenuItem::new("divider", "", ContextMenuItemKind::Divider),
        ContextMenuItem::action("rename", "Rename"),
    ];

    assert_eq!(
        Some(2),
        ContextMenuKeyboardNavigator::move_highlight(&items, None, &ContextMenuKeyboardInput::Home)
    );
    assert_eq!(
        Some(4),
        ContextMenuKeyboardNavigator::move_highlight(
            &items,
            Some(2),
            &ContextMenuKeyboardInput::ArrowDown
        )
    );
    assert_eq!(
        Some(4),
        ContextMenuKeyboardNavigator::move_highlight(
            &items,
            Some(2),
            &ContextMenuKeyboardInput::TypeAhead("re".to_string())
        )
    );
}

#[test]
fn choice_item_converts_to_context_menu_action_item() {
    let item = ContextMenuItem::from(ChoiceItem::new("open", "Open").disabled(true));

    assert_eq!("open", item.id);
    assert_eq!("Open", item.label);
    assert!(item.disabled);
    assert_eq!(ContextMenuItemKind::Action, item.kind);
}
