use super::{
    ContextMenu, ContextMenuAnchor, ContextMenuCloseReason, ContextMenuEvent, ContextMenuItem,
    ContextMenuItemKind, ContextMenuKeyboardInput, ContextMenuKeyboardIntent,
    ContextMenuKeyboardNavigator, ContextMenuPlacement, ContextMenuPlacementResolver,
    ContextMenuRect, ContextMenuSize, ContextMenuViewport,
};
use crate::render_model::{UiContextMenuProps, UiNode};

#[test]
fn keyboard_navigator_covers_empty_end_submenu_and_non_move_intents() {
    assert_eq!(
        ContextMenuKeyboardIntent::None,
        ContextMenuKeyboardNavigator::intent(&[], None, &ContextMenuKeyboardInput::ArrowDown)
    );
    let items = sample_items();
    assert_eq!(
        Some(5),
        ContextMenuKeyboardNavigator::move_highlight(&items, None, &ContextMenuKeyboardInput::End)
    );
    assert_eq!(
        None,
        ContextMenuKeyboardNavigator::move_highlight(
            &items,
            Some(3),
            &ContextMenuKeyboardInput::ArrowRight
        )
    );
    assert_eq!(
        ContextMenuKeyboardIntent::OpenSubmenu,
        ContextMenuKeyboardNavigator::intent(
            &items,
            Some(3),
            &ContextMenuKeyboardInput::ArrowRight
        )
    );
    assert_eq!(
        ContextMenuKeyboardIntent::CloseSubmenu,
        ContextMenuKeyboardNavigator::intent(&items, Some(3), &ContextMenuKeyboardInput::ArrowLeft)
    );
    assert_eq!(
        Some(5),
        ContextMenuKeyboardNavigator::move_highlight(
            &items,
            None,
            &ContextMenuKeyboardInput::ArrowUp
        )
    );
}

#[test]
fn item_state_handles_invalid_paths_empty_submenus_and_nested_checked_items() {
    let mut props = UiContextMenuProps {
        items: vec![
            ContextMenuItem::new("empty", "Empty", ContextMenuItemKind::Submenu),
            ContextMenuItem::new("nested", "Nested", ContextMenuItemKind::Submenu).child(
                ContextMenuItem::new("toggle", "Toggle", ContextMenuItemKind::Toggle),
            ),
        ],
        ..UiContextMenuProps::default()
    };

    assert_eq!(
        vec![0],
        super::item_state::first_enabled_child_path(&props, &[0])
    );
    assert_eq!(
        vec![99],
        super::item_state::first_enabled_child_path(&props, &[99])
    );
    assert!(super::item_state::command_for_path(&props, &[]).is_empty());
    super::item_state::apply_checked_state(&mut props, &[]);
    super::item_state::apply_checked_state(&mut props, &[1, 0]);
    assert!(props.items[1].children[0].checked);
}

#[test]
fn option_builder_event_names_and_remaining_placements_are_observable() {
    let node = UiNode::from(
        ContextMenu::new("context")
            .highlighted_path(vec![2])
            .items(sample_items()),
    );
    assert_eq!(vec![2], node.props().context_menu.highlighted_path);

    for (event, expected) in [
        (
            ContextMenuEvent::Opened {
                anchor: ContextMenuAnchor::Pointer { x: 0, y: 0 },
                placement_used: ContextMenuPlacement::BelowStart,
            },
            "context_menu_opened",
        ),
        (
            ContextMenuEvent::Closed {
                reason: ContextMenuCloseReason::Escape,
            },
            "context_menu_closed",
        ),
        (
            ContextMenuEvent::ItemHighlighted { path: vec![0] },
            "context_menu_item_highlighted",
        ),
        (
            ContextMenuEvent::ItemSelected {
                path: vec![0],
                command: "open".to_string(),
            },
            "context_menu_item_selected",
        ),
        (
            ContextMenuEvent::ItemActivationBlocked { path: vec![0] },
            "context_menu_item_activation_blocked",
        ),
        (
            ContextMenuEvent::SubmenuOpened { path: vec![0] },
            "context_menu_submenu_opened",
        ),
        (
            ContextMenuEvent::SubmenuClosed { path: vec![0] },
            "context_menu_submenu_closed",
        ),
        (
            ContextMenuEvent::TypeAheadMatched {
                prefix: "x".to_string(),
                path: Vec::new(),
            },
            "context_menu_typeahead_matched",
        ),
    ] {
        assert_eq!(expected, event.name());
    }

    for placement in [
        ContextMenuPlacement::BelowEnd,
        ContextMenuPlacement::RightStart,
        ContextMenuPlacement::LeftStart,
    ] {
        let result = ContextMenuPlacementResolver::resolve(
            &ContextMenuAnchor::VirtualRect(ContextMenuRect::new(200, 200, 20, 20)),
            ContextMenuSize::new(40, 40),
            ContextMenuViewport::new(500, 500),
            &[placement],
        );
        assert_eq!(placement, result.placement);
    }
}

fn sample_items() -> Vec<ContextMenuItem> {
    vec![
        ContextMenuItem::new("editing", "編集", ContextMenuItemKind::Section),
        ContextMenuItem::action("copy", "Copy").shortcut("Cmd+C"),
        ContextMenuItem::new("divider", "", ContextMenuItemKind::Divider),
        ContextMenuItem::new("insert", "Insert", ContextMenuItemKind::Submenu)
            .child(ContextMenuItem::action("table", "Table"))
            .child(ContextMenuItem::action("link", "Link")),
        ContextMenuItem::new("wrap", "Wrap line", ContextMenuItemKind::Toggle),
        ContextMenuItem::new("compact", "Compact", ContextMenuItemKind::Radio)
            .radio_group("density"),
        ContextMenuItem::action("delete", "Delete")
            .destructive(true)
            .disabled(true),
    ]
}
