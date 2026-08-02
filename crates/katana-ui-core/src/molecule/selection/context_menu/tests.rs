use super::{
    ContextMenu, ContextMenuAction, ContextMenuAnchor, ContextMenuCloseReason, ContextMenuEvent,
    ContextMenuItem, ContextMenuItemKind, ContextMenuKeyboardInput, ContextMenuKeyboardIntent,
    ContextMenuKeyboardNavigator, ContextMenuPlacement, ContextMenuPlacementResolver,
    ContextMenuRect, ContextMenuSize, ContextMenuTypeAheadBuffer, ContextMenuViewport,
};
use crate::component::ComponentAction;
use crate::interaction::UiAction;
use crate::render_model::UiNode;

#[test]
fn open_with_layout_resolves_pointer_anchor_inside_viewport() {
    let mut menu = ContextMenu::new("context").items(sample_items());
    let event = menu.apply_context_action(&ContextMenuAction::OpenWithLayout {
        anchor: ContextMenuAnchor::Pointer { x: 780, y: 560 },
        menu_size: ContextMenuSize::new(180, 160),
        viewport: ContextMenuViewport::new(800, 600),
    });

    assert!(matches!(event, ContextMenuEvent::Opened { .. }));
    let node = UiNode::from(menu);
    assert_eq!(
        ContextMenuPlacement::AboveEnd,
        node.props().context_menu.placement_used
    );
}

#[test]
fn placement_resolver_uses_priority_before_clamping() {
    let result = ContextMenuPlacementResolver::resolve(
        &ContextMenuAnchor::VirtualRect(ContextMenuRect::new(760, 540, 32, 28)),
        ContextMenuSize::new(180, 180),
        ContextMenuViewport::new(800, 600),
        &[
            ContextMenuPlacement::BelowStart,
            ContextMenuPlacement::AboveEnd,
            ContextMenuPlacement::LeftStart,
        ],
    );

    assert_eq!(ContextMenuPlacement::AboveEnd, result.placement);
    assert!(result.x >= 0);
    assert!(result.y >= 0);
}

#[test]
fn keyboard_navigation_skips_non_selectable_items() {
    let items = sample_items();

    assert_eq!(
        Some(1),
        ContextMenuKeyboardNavigator::move_highlight(
            &items,
            None,
            &ContextMenuKeyboardInput::ArrowDown,
        )
    );
    assert_eq!(
        Some(5),
        ContextMenuKeyboardNavigator::move_highlight(
            &items,
            Some(1),
            &ContextMenuKeyboardInput::ArrowUp,
        )
    );
}

#[test]
fn keyboard_intent_maps_activation_and_escape() {
    let items = sample_items();

    assert_eq!(
        ContextMenuKeyboardIntent::Activate,
        ContextMenuKeyboardNavigator::intent(&items, Some(1), &ContextMenuKeyboardInput::Enter)
    );
    assert_eq!(
        ContextMenuKeyboardIntent::Close,
        ContextMenuKeyboardNavigator::intent(&items, Some(1), &ContextMenuKeyboardInput::Escape)
    );
}

#[test]
fn activate_nested_item_emits_selected_then_closed() {
    let mut menu = ContextMenu::new("context").items(sample_items());
    menu.apply_context_action(&ContextMenuAction::Open {
        anchor: ContextMenuAnchor::Pointer { x: 10, y: 10 },
    });

    let selected = menu.apply_context_action(&ContextMenuAction::Activate { path: vec![3, 1] });

    assert_eq!(
        ContextMenuEvent::ItemSelected {
            path: vec![3, 1],
            command: "link".to_string(),
        },
        selected
    );
    assert_eq!(
        Some(&ContextMenuEvent::Closed {
            reason: ContextMenuCloseReason::Selected
        }),
        menu.callback_log().last()
    );
}

#[test]
fn toggle_and_radio_items_mutate_checked_state() {
    let mut menu = ContextMenu::new("context").items(sample_items());
    menu.apply_context_action(&ContextMenuAction::Activate { path: vec![4] });
    menu.apply_context_action(&ContextMenuAction::Activate { path: vec![5] });

    let node = UiNode::from(menu);
    let items = &node.props().context_menu.items;
    assert!(items[4].checked);
    assert!(items[5].checked);
}

#[test]
fn submenu_state_ids_are_distinct_from_parent() {
    let menu = ContextMenu::new("context").items(sample_items());
    let state_id = menu.state_id().clone();
    let submenu_ids = menu.submenu_state_ids();

    assert_eq!(1, submenu_ids.len());
    assert_ne!(&state_id, &submenu_ids[0]);
}

#[test]
fn typeahead_buffer_resets_after_timeout() {
    let mut buffer = ContextMenuTypeAheadBuffer::new(1_000);

    assert_eq!("c", buffer.push("c", 100));
    assert_eq!("co", buffer.push("o", 400));
    assert_eq!("p", buffer.push("p", 1_800));
}

#[test]
fn component_action_routes_close_highlight_dismiss_callback_and_ignored_inputs() {
    let mut menu = ContextMenu::new("context").items(sample_items());
    let target = menu.state_id().clone();
    let foreign = UiAction::press(crate::render_model::UiStateId::new("foreign"));
    assert!(!menu.apply_action(&foreign).handled);

    assert!(
        menu.apply_action(&UiAction::set_open(target.clone(), true))
            .handled
    );
    assert!(
        menu.apply_action(&UiAction::set_selected_index(target.clone(), 1))
            .handled
    );
    assert!(menu.apply_action(&UiAction::press(target.clone())).handled);
    assert!(
        !menu
            .apply_action(&UiAction::invoke_callback(target.clone(), "missing"))
            .handled
    );
    assert!(
        menu.apply_action(&UiAction::invoke_callback(target.clone(), "copy"))
            .handled
    );
    assert!(
        menu.apply_action(&UiAction::dismiss(target.clone()))
            .handled
    );
    assert!(
        menu.apply_action(&UiAction::set_open(target.clone(), false))
            .handled
    );
    assert!(
        menu.apply_action(&UiAction::set_value(target, "ignored"))
            .handled
    );
}

#[test]
fn submenu_typeahead_and_close_paths_preserve_state_events() {
    let mut menu = ContextMenu::new("context").items(sample_items());

    assert_eq!(
        ContextMenuEvent::SubmenuOpened { path: vec![3] },
        menu.apply_context_action(&ContextMenuAction::OpenSubmenu { path: vec![3] })
    );
    assert_eq!(
        ContextMenuEvent::SubmenuClosed { path: vec![3] },
        menu.apply_context_action(&ContextMenuAction::CloseSubmenu { path: vec![3] })
    );
    assert_eq!(
        ContextMenuEvent::TypeAheadMatched {
            prefix: "wr".to_string(),
            path: vec![4]
        },
        menu.apply_context_action(&ContextMenuAction::TypeAhead {
            prefix: "wr".to_string()
        })
    );
    assert_eq!(
        ContextMenuEvent::Closed {
            reason: ContextMenuCloseReason::OutsideClick
        },
        menu.apply_context_action(&ContextMenuAction::Close {
            reason: ContextMenuCloseReason::OutsideClick
        })
    );
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
