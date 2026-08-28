use katana_ui_core::interaction::placement::{Rect, Size};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeContractViolation, CommandChromeDropdown,
    CommandChromeDropdownCloseReason, CommandChromeDropdownItem, CommandChromeDropdownKey,
    CommandChromeDropdownLayout, CommandChromeDropdownTrigger, CommandChromeToolbar,
    CommandChromeToolbarAction, CommandChromeToolbarEvent, FloatingCommandToolbar,
    FloatingCommandToolbarAction, FloatingCommandToolbarCloseReason, FloatingCommandToolbarEvent,
    FloatingCommandToolbarLayout,
};
use katana_ui_core::molecule::toolbar::SplitAction;

fn menu_layout() -> CommandChromeDropdownLayout {
    CommandChromeDropdownLayout::new(
        Rect::new(96, 72, 24, 20),
        Rect::new(0, 0, 160, 120),
        Size::new(120, 64),
    )
}

fn menu_only_toolbar() -> CommandChromeToolbar {
    CommandChromeToolbar::new().action(
        CommandChromeAction::new("code-block", "Code block").dropdown(
            CommandChromeDropdown::new(CommandChromeDropdownTrigger::Primary)
                .item(CommandChromeDropdownItem::new("plain", "Plain Text"))
                .item(CommandChromeDropdownItem::new("rust", "Rust").disabled(true))
                .item(CommandChromeDropdownItem::new("markdown", "Markdown")),
        ),
    )
}

#[test]
fn menu_only_primary_opens_generic_dropdown_without_command_activation() {
    let mut toolbar = menu_only_toolbar();
    assert!(
        toolbar
            .apply_action(CommandChromeToolbarAction::update_dropdown_layout(
                "code-block",
                menu_layout(),
            ))
            .is_empty()
    );

    let events = toolbar.apply_action(CommandChromeToolbarAction::activate("code-block"));

    assert!(events.iter().any(|event| {
        matches!(
            event,
            CommandChromeToolbarEvent::DropdownOpened { action_id, placement }
                if action_id.as_str() == "code-block" && placement.clamped
        )
    }));
    assert!(
        events.contains(&CommandChromeToolbarEvent::DropdownFocusChanged {
            action_id: "code-block".into(),
            item_id: "plain".into(),
        })
    );
    assert!(
        !events
            .iter()
            .any(|event| { matches!(event, CommandChromeToolbarEvent::CommandActivated { .. }) })
    );
    assert_eq!(
        Some("code-block"),
        toolbar
            .open_dropdown_model()
            .map(|dropdown| dropdown.action_id().as_str())
    );
}

#[test]
fn roving_focus_skips_disabled_items_then_selects_and_closes() {
    let mut toolbar = menu_only_toolbar();
    let _ = toolbar.apply_action(CommandChromeToolbarAction::update_dropdown_layout(
        "code-block",
        menu_layout(),
    ));
    let _ = toolbar.apply_action(CommandChromeToolbarAction::activate("code-block"));

    let focus_events = toolbar.apply_action(CommandChromeToolbarAction::DropdownKeyboard {
        input: CommandChromeDropdownKey::ArrowDown,
    });
    assert_eq!(
        vec![CommandChromeToolbarEvent::DropdownFocusChanged {
            action_id: "code-block".into(),
            item_id: "markdown".into(),
        }],
        focus_events
    );
    assert!(
        toolbar
            .apply_action(CommandChromeToolbarAction::select_dropdown_item(
                "code-block",
                "rust",
            ))
            .is_empty()
    );

    let activation_events = toolbar.apply_action(CommandChromeToolbarAction::DropdownKeyboard {
        input: CommandChromeDropdownKey::Enter,
    });
    assert_eq!(
        vec![
            CommandChromeToolbarEvent::DropdownItemActivated {
                action_id: "code-block".into(),
                item_id: "markdown".into(),
            },
            CommandChromeToolbarEvent::DropdownClosed {
                action_id: "code-block".into(),
                reason: CommandChromeDropdownCloseReason::ItemActivated,
            },
        ],
        activation_events
    );
    assert!(toolbar.open_dropdown_model().is_none());
}

#[test]
fn split_secondary_preserves_primary_command_and_escape_closes_only_dropdown() {
    let mut toolbar = CommandChromeToolbar::new().action(
        CommandChromeAction::new("insert", "Insert")
            .split(SplitAction::new(Default::default(), Default::default()))
            .dropdown(
                CommandChromeDropdown::new(CommandChromeDropdownTrigger::SplitSecondary)
                    .item(CommandChromeDropdownItem::new("table", "Table")),
            ),
    );
    let _ = toolbar.apply_action(CommandChromeToolbarAction::update_dropdown_layout(
        "insert",
        menu_layout(),
    ));

    assert_eq!(
        vec![CommandChromeToolbarEvent::CommandActivated {
            action_id: "insert".into(),
        }],
        toolbar.apply_action(CommandChromeToolbarAction::activate("insert"))
    );
    let open_events =
        toolbar.apply_action(CommandChromeToolbarAction::open_split_dropdown("insert"));
    assert!(matches!(
        open_events.first(),
        Some(CommandChromeToolbarEvent::DropdownOpened { action_id, .. }) if action_id.as_str() == "insert"
    ));
    assert_eq!(
        vec![CommandChromeToolbarEvent::DropdownClosed {
            action_id: "insert".into(),
            reason: CommandChromeDropdownCloseReason::Escape,
        }],
        toolbar.apply_action(CommandChromeToolbarAction::DropdownKeyboard {
            input: CommandChromeDropdownKey::Escape,
        })
    );
}

#[test]
fn empty_dropdown_is_a_typed_contract_violation_and_is_not_presented() {
    let mut toolbar = CommandChromeToolbar::new().action(
        CommandChromeAction::new("invalid", "Invalid").dropdown(CommandChromeDropdown::new(
            CommandChromeDropdownTrigger::Primary,
        )),
    );

    assert_eq!(
        vec![CommandChromeContractViolation::EmptyDropdownItems {
            action_id: "invalid".into(),
        }],
        toolbar.validate()
    );
    let _ = toolbar.apply_action(CommandChromeToolbarAction::update_dropdown_layout(
        "invalid",
        menu_layout(),
    ));
    assert!(
        toolbar
            .apply_action(CommandChromeToolbarAction::activate("invalid"))
            .is_empty()
    );
    assert!(toolbar.open_dropdown_model().is_none());
}

#[test]
fn floating_escape_closes_dropdown_before_closing_the_toolbar() {
    let mut floating = FloatingCommandToolbar::new(
        menu_only_toolbar(),
        FloatingCommandToolbarLayout::new(
            Rect::new(48, 48, 16, 16),
            Size::new(120, 32),
            Rect::new(0, 0, 240, 160),
        ),
    );
    let _ = floating.apply_action(FloatingCommandToolbarAction::Open);
    let _ = floating.apply_action(FloatingCommandToolbarAction::Toolbar {
        action: CommandChromeToolbarAction::update_dropdown_layout("code-block", menu_layout()),
    });
    let _ = floating.apply_action(FloatingCommandToolbarAction::Toolbar {
        action: CommandChromeToolbarAction::activate("code-block"),
    });

    let first_escape = floating.apply_action(FloatingCommandToolbarAction::Dismiss {
        reason: FloatingCommandToolbarCloseReason::Escape,
    });

    assert!(floating.is_open());
    assert!(first_escape.iter().any(|event| {
        matches!(
            event,
            FloatingCommandToolbarEvent::Toolbar {
                event: CommandChromeToolbarEvent::DropdownClosed {
                    reason: CommandChromeDropdownCloseReason::Escape,
                    ..
                }
            }
        )
    }));
    assert!(
        !first_escape
            .iter()
            .any(|event| matches!(event, FloatingCommandToolbarEvent::Closed { .. }))
    );

    let second_escape = floating.apply_action(FloatingCommandToolbarAction::Dismiss {
        reason: FloatingCommandToolbarCloseReason::Escape,
    });
    assert!(!floating.is_open());
    assert!(
        second_escape.contains(&FloatingCommandToolbarEvent::Closed {
            reason: FloatingCommandToolbarCloseReason::Escape,
        })
    );
}
