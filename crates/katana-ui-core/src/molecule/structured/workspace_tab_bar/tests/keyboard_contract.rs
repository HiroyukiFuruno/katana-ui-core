use super::super::{
    WorkspaceTab, WorkspaceTabBar, WorkspaceTabBarAction, WorkspaceTabBarEvent,
    WorkspaceTabGroupId, WorkspaceTabId, WorkspaceTabKey, WorkspaceTabKeyboardController,
    WorkspaceTabKeyboardInput, WorkspaceTabKeyboardShortcut,
};

#[test]
fn keyboard_digit_selects_nth_visible_tab() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("first", "First"))
        .tab(WorkspaceTab::new("second", "Second"))
        .tab(WorkspaceTab::new("third", "Third"))
        .active_tab_id("first");
    let visible = vec![
        WorkspaceTabId::new("first"),
        WorkspaceTabId::new("second"),
        WorkspaceTabId::new("third"),
    ];
    let input = WorkspaceTabKeyboardInput::from_shortcut(WorkspaceTabKeyboardShortcut::new(
        WorkspaceTabKey::Digit(2),
        true,
        false,
    ));

    let events = input.map_or_else(Vec::new, |it| bar.apply_keyboard_input(it, &visible));

    assert_eq!(
        Some(&WorkspaceTabId::new("second")),
        bar.state().active_tab_id.as_ref()
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabSelected {
            tab_id: WorkspaceTabId::new("second")
        }],
        events
    );
}

#[test]
fn keyboard_ctrl_w_requests_dirty_close_for_active_tab() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("draft", "Draft").dirty(true))
        .active_tab_id("draft");
    let visible = vec![WorkspaceTabId::new("draft")];

    let events = bar.apply_keyboard_input(WorkspaceTabKeyboardInput::CloseActiveTab, &visible);

    assert_eq!(
        Some(&WorkspaceTabId::new("draft")),
        bar.state().active_tab_id.as_ref()
    );
    assert_eq!(
        Some(&WorkspaceTabId::new("draft")),
        bar.state().pending_close_confirm.as_ref()
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabCloseRequested {
            tab_id: WorkspaceTabId::new("draft")
        }],
        events
    );
}

#[test]
fn keyboard_ctrl_tab_cycles_visible_tabs() {
    let mut bar = tab_bar("second");
    let visible = visible_tabs();
    let input = WorkspaceTabKeyboardInput::from_shortcut(WorkspaceTabKeyboardShortcut::new(
        WorkspaceTabKey::Tab,
        true,
        false,
    ));

    let events = input.map_or_else(Vec::new, |it| bar.apply_keyboard_input(it, &visible));

    assert_eq!(
        Some(&WorkspaceTabId::new("third")),
        bar.state().active_tab_id.as_ref()
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabSelected {
            tab_id: WorkspaceTabId::new("third")
        }],
        events
    );
}

#[test]
fn keyboard_shift_ctrl_tab_cycles_backwards() {
    let mut bar = tab_bar("first");
    let visible = visible_tabs();
    let input = WorkspaceTabKeyboardInput::from_shortcut(WorkspaceTabKeyboardShortcut::new(
        WorkspaceTabKey::Tab,
        true,
        true,
    ));

    let events = input.map_or_else(Vec::new, |it| bar.apply_keyboard_input(it, &visible));

    assert_eq!(
        Some(&WorkspaceTabId::new("third")),
        bar.state().active_tab_id.as_ref()
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabSelected {
            tab_id: WorkspaceTabId::new("third")
        }],
        events
    );
}

#[test]
fn keyboard_digit_zero_selects_last_visible_tab() {
    let mut bar = tab_bar("first");
    let visible = visible_tabs();
    let input = WorkspaceTabKeyboardInput::from_shortcut(WorkspaceTabKeyboardShortcut::new(
        WorkspaceTabKey::Digit(0),
        true,
        false,
    ));

    let events = input.map_or_else(Vec::new, |it| bar.apply_keyboard_input(it, &visible));

    assert_eq!(
        Some(&WorkspaceTabId::new("third")),
        bar.state().active_tab_id.as_ref()
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabSelected {
            tab_id: WorkspaceTabId::new("third")
        }],
        events
    );
}

#[test]
fn keyboard_shortcuts_cover_supported_and_rejected_boundaries() {
    let shortcut = |key, command_or_control, shift| {
        WorkspaceTabKeyboardInput::from_shortcut(WorkspaceTabKeyboardShortcut::new(
            key,
            command_or_control,
            shift,
        ))
    };

    assert_eq!(
        Some(WorkspaceTabKeyboardInput::CancelDrag),
        shortcut(WorkspaceTabKey::Escape, false, false)
    );
    assert_eq!(
        Some(WorkspaceTabKeyboardInput::CloseActiveTab),
        shortcut(WorkspaceTabKey::W, true, false)
    );
    assert_eq!(None, shortcut(WorkspaceTabKey::W, false, false));
    assert_eq!(None, shortcut(WorkspaceTabKey::Digit(10), true, false));
}

#[test]
fn keyboard_controller_covers_empty_unknown_and_wrap_boundaries() {
    let visible = visible_tabs();
    let first = WorkspaceTabId::new("first");
    let last = WorkspaceTabId::new("third");

    assert_eq!(
        Some(WorkspaceTabBarAction::SelectTab {
            tab_id: first.clone()
        }),
        WorkspaceTabKeyboardController::action_for_input(
            &WorkspaceTabKeyboardInput::NextTab,
            Some(&last),
            &visible,
        )
    );
    assert_eq!(
        Some(WorkspaceTabBarAction::SelectTab {
            tab_id: last.clone()
        }),
        WorkspaceTabKeyboardController::action_for_input(
            &WorkspaceTabKeyboardInput::PreviousTab,
            Some(&first),
            &visible,
        )
    );
    assert_eq!(
        Some(WorkspaceTabBarAction::CloseTab {
            tab_id: first.clone()
        }),
        WorkspaceTabKeyboardController::action_for_input(
            &WorkspaceTabKeyboardInput::CloseActiveTab,
            Some(&first),
            &visible,
        )
    );
    assert_eq!(
        None,
        WorkspaceTabKeyboardController::action_for_input(
            &WorkspaceTabKeyboardInput::NextTab,
            None,
            &visible,
        )
    );
    assert_eq!(
        None,
        WorkspaceTabKeyboardController::action_for_input(
            &WorkspaceTabKeyboardInput::PreviousTab,
            Some(&WorkspaceTabId::new("hidden")),
            &visible,
        )
    );
    assert_eq!(
        None,
        WorkspaceTabKeyboardController::action_for_input(
            &WorkspaceTabKeyboardInput::SelectVisible(4),
            Some(&first),
            &visible,
        )
    );
    assert_eq!(
        None,
        WorkspaceTabKeyboardController::action_for_input(
            &WorkspaceTabKeyboardInput::SelectLastVisible,
            None,
            &[],
        )
    );
    assert_eq!(
        None,
        WorkspaceTabKeyboardController::action_for_input(
            &WorkspaceTabKeyboardInput::CancelDrag,
            Some(&first),
            &visible,
        )
    );
}

#[test]
fn event_names_cover_every_workspace_tab_event() {
    let tab_id = WorkspaceTabId::new("tab");
    let group_id = WorkspaceTabGroupId::new("group");
    let events = [
        WorkspaceTabBarEvent::TabAdded {
            tab_id: tab_id.clone(),
        },
        WorkspaceTabBarEvent::TabSelected {
            tab_id: tab_id.clone(),
        },
        WorkspaceTabBarEvent::TabCloseRequested {
            tab_id: tab_id.clone(),
        },
        WorkspaceTabBarEvent::TabClosed {
            tab_id: tab_id.clone(),
        },
        WorkspaceTabBarEvent::TabRestored {
            tab_id: tab_id.clone(),
        },
        WorkspaceTabBarEvent::TabReordered {
            tab_id: tab_id.clone(),
            from: 0,
            to: 1,
        },
        WorkspaceTabBarEvent::TabPinChanged {
            tab_id: tab_id.clone(),
            pinned: true,
        },
        WorkspaceTabBarEvent::TabGroupChanged {
            tab_id: tab_id.clone(),
            group_id: Some(group_id.clone()),
        },
        WorkspaceTabBarEvent::GroupCreated {
            group_id: group_id.clone(),
        },
        WorkspaceTabBarEvent::GroupReordered {
            group_id: group_id.clone(),
            from: 0,
            to: 1,
        },
        WorkspaceTabBarEvent::GroupRenamed {
            group_id: group_id.clone(),
            label: "Group".to_owned(),
        },
        WorkspaceTabBarEvent::GroupColorChanged {
            group_id: group_id.clone(),
            color: "#fff".to_owned(),
        },
        WorkspaceTabBarEvent::GroupRemoved {
            group_id: group_id.clone(),
        },
        WorkspaceTabBarEvent::DragStarted {
            tab_id: tab_id.clone(),
        },
        WorkspaceTabBarEvent::DragEnded {
            tab_id: tab_id.clone(),
            committed: true,
        },
        WorkspaceTabBarEvent::GroupCollapseChanged {
            group_id: group_id.clone(),
            collapsed: true,
        },
        WorkspaceTabBarEvent::OverflowOpened {
            hidden_tab_ids: vec![tab_id],
        },
    ];

    assert_eq!(
        [
            "closeable_tab_added",
            "closeable_tab_selected",
            "closeable_tab_close_requested",
            "closeable_tab_closed",
            "closeable_tab_restored",
            "closeable_tab_reordered",
            "closeable_tab_pin_changed",
            "closeable_tab_group_changed",
            "closeable_tab_group_created",
            "closeable_tab_group_reordered",
            "closeable_tab_group_renamed",
            "closeable_tab_group_color_changed",
            "closeable_tab_group_removed",
            "closeable_tab_drag_started",
            "closeable_tab_drag_ended",
            "closeable_tab_group_collapse_changed",
            "closeable_tab_overflow_opened",
        ],
        events.map(|event| event.name())
    );
}

fn tab_bar(active_tab_id: &str) -> WorkspaceTabBar {
    WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("first", "First"))
        .tab(WorkspaceTab::new("second", "Second"))
        .tab(WorkspaceTab::new("third", "Third"))
        .active_tab_id(active_tab_id)
}

fn visible_tabs() -> Vec<WorkspaceTabId> {
    vec![
        WorkspaceTabId::new("first"),
        WorkspaceTabId::new("second"),
        WorkspaceTabId::new("third"),
    ]
}
