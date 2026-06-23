use super::super::{
    CLOSEABLE_TAB_DRAG_TAG, WorkspaceTab, WorkspaceTabBar, WorkspaceTabBarAction,
    WorkspaceTabBarEvent, WorkspaceTabDropRules, WorkspaceTabGroup, WorkspaceTabGroupId,
    WorkspaceTabGroupTarget, WorkspaceTabId,
};
use crate::interaction::drag_and_drop::{DropEffect, DropIndicatorOrientation};
use crate::render_model::{UiNode, UiNodeKind};
use std::collections::HashSet;

#[test]
fn pinned_tabs_are_leading_and_unpinned_cannot_drop_into_pinned_area() {
    let bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("main", "Main"))
        .tab(WorkspaceTab::new("pinned", "Pinned").pinned(true))
        .tab(WorkspaceTab::new("settings", "Settings"));
    let visual_ids: Vec<&str> = bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();

    assert_eq!(vec!["pinned", "main", "settings"], visual_ids);
    assert!(!WorkspaceTabDropRules::can_accept(
        &bar.options().tabs,
        &WorkspaceTabId::new("main"),
        0
    ));
    assert!(WorkspaceTabDropRules::can_accept(
        &bar.options().tabs,
        &WorkspaceTabId::new("main"),
        1
    ));
}

#[test]
fn pinned_tabs_are_before_grouped_tabs_and_bulk_close_uses_that_visual_order() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .group(WorkspaceTabGroup::new("docs", "Docs"))
        .tab(WorkspaceTab::new("pinned", "Pinned").pinned(true))
        .tab(WorkspaceTab::new("draft", "Draft").group_id("docs"))
        .tab(WorkspaceTab::new("loose", "Loose"));
    let visual_ids: Vec<&str> = bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();

    assert_eq!(vec!["pinned", "draft", "loose"], visual_ids);

    let events = bar.apply_action(WorkspaceTabBarAction::CloseToRight {
        tab_id: WorkspaceTabId::new("pinned"),
    });
    let remaining_ids: Vec<&str> = bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();

    assert_eq!(
        vec![
            WorkspaceTabBarEvent::TabClosed {
                tab_id: WorkspaceTabId::new("draft")
            },
            WorkspaceTabBarEvent::TabClosed {
                tab_id: WorkspaceTabId::new("loose")
            }
        ],
        events
    );
    assert_eq!(vec!["pinned"], remaining_ids);
}

#[test]
fn visual_tabs_keep_pinned_before_declared_group_order() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .group(WorkspaceTabGroup::new("feature", "Feature"))
        .group(WorkspaceTabGroup::new("docs", "Docs"))
        .tab(WorkspaceTab::new("docs-a", "Docs A").group_id("docs"))
        .tab(WorkspaceTab::new("feature-a", "Feature A").group_id("feature"))
        .tab(WorkspaceTab::new("orphan", "Orphan").group_id("missing"))
        .tab(WorkspaceTab::new("pinned", "Pinned").pinned(true))
        .tab(WorkspaceTab::new("loose", "Loose"));
    let visual_ids: Vec<&str> = bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();
    let node = UiNode::from(bar.clone());
    let child_labels: Vec<&str> = node
        .children()
        .iter()
        .map(|child| child.props().label.as_str())
        .collect();

    assert_eq!(
        vec!["pinned", "feature-a", "docs-a", "orphan", "loose"],
        visual_ids
    );
    assert_eq!(
        vec![
            "Pinned",
            "Feature",
            "Feature A",
            "Docs",
            "Docs A",
            "Orphan",
            "Loose"
        ],
        child_labels
    );

    let closed = bar.apply_action(WorkspaceTabBarAction::CloseToRight {
        tab_id: WorkspaceTabId::new("feature-a"),
    });
    let remaining_ids: Vec<&str> = bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();

    assert_eq!(
        vec![
            WorkspaceTabBarEvent::TabClosed {
                tab_id: WorkspaceTabId::new("docs-a")
            },
            WorkspaceTabBarEvent::TabClosed {
                tab_id: WorkspaceTabId::new("orphan")
            },
            WorkspaceTabBarEvent::TabClosed {
                tab_id: WorkspaceTabId::new("loose")
            }
        ],
        closed
    );
    assert_eq!(vec!["pinned", "feature-a"], remaining_ids);
}

#[test]
fn close_to_right_after_pin_uses_pinned_before_group_visual_order() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .group(WorkspaceTabGroup::new("docs", "Docs"))
        .tab(WorkspaceTab::new("readme", "Readme").pinned(true))
        .tab(WorkspaceTab::new("editor", "Editor").group_id("docs"))
        .tab(WorkspaceTab::new("preview", "Preview").group_id("docs"))
        .tab(WorkspaceTab::new("scratch", "Scratch"))
        .tab(WorkspaceTab::new("terminal", "Terminal"));

    let pinned = bar.apply_action(WorkspaceTabBarAction::PinTab {
        tab_id: WorkspaceTabId::new("scratch"),
    });
    let pinned_order: Vec<String> = bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str().to_string())
        .collect();
    let closed = bar.apply_action(WorkspaceTabBarAction::CloseToRight {
        tab_id: WorkspaceTabId::new("scratch"),
    });
    let remaining_ids: Vec<String> = bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str().to_string())
        .collect();

    assert_eq!(
        vec![WorkspaceTabBarEvent::TabPinChanged {
            tab_id: WorkspaceTabId::new("scratch"),
            pinned: true
        }],
        pinned
    );
    assert_eq!(
        vec!["readme", "scratch", "editor", "preview", "terminal"],
        pinned_order
    );
    assert_eq!(
        vec![
            WorkspaceTabBarEvent::TabClosed {
                tab_id: WorkspaceTabId::new("editor")
            },
            WorkspaceTabBarEvent::TabClosed {
                tab_id: WorkspaceTabId::new("preview")
            },
            WorkspaceTabBarEvent::TabClosed {
                tab_id: WorkspaceTabId::new("terminal")
            }
        ],
        closed
    );
    assert_eq!(vec!["readme", "scratch"], remaining_ids);
}

#[test]
fn pinning_grouped_tab_removes_group_membership_and_moves_to_fixed_region() -> Result<(), String> {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .group(WorkspaceTabGroup::new("docs", "Docs"))
        .tab(WorkspaceTab::new("readme", "Readme").pinned(true))
        .tab(WorkspaceTab::new("editor", "Editor").group_id("docs"))
        .tab(WorkspaceTab::new("preview", "Preview").group_id("docs"))
        .tab(WorkspaceTab::new("scratch", "Scratch"));

    let events = bar.apply_action(WorkspaceTabBarAction::PinTab {
        tab_id: WorkspaceTabId::new("editor"),
    });
    let visual_ids: Vec<&str> = bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();
    let editor = bar
        .options()
        .tabs
        .iter()
        .find(|tab| tab.id == WorkspaceTabId::new("editor"))
        .ok_or_else(|| "editor tab exists".to_string())?;

    assert!(editor.pinned);
    assert_eq!(None, editor.group_id);
    assert_eq!(vec!["readme", "editor", "preview", "scratch"], visual_ids);
    assert_eq!(
        vec![
            WorkspaceTabBarEvent::TabPinChanged {
                tab_id: WorkspaceTabId::new("editor"),
                pinned: true
            },
            WorkspaceTabBarEvent::TabGroupChanged {
                tab_id: WorkspaceTabId::new("editor"),
                group_id: None
            }
        ],
        events
    );
    Ok(())
}

#[test]
fn drop_rules_keep_grouped_prefix_and_pinned_region_distinct() {
    let tabs = vec![
        WorkspaceTab::new("draft", "Draft").group_id("docs"),
        WorkspaceTab::new("pinned", "Pinned").pinned(true),
        WorkspaceTab::new("loose", "Loose"),
    ];

    assert!(!WorkspaceTabDropRules::can_accept(
        &tabs,
        &WorkspaceTabId::new("draft"),
        0
    ));
    assert!(WorkspaceTabDropRules::can_accept(
        &tabs,
        &WorkspaceTabId::new("draft"),
        1
    ));
    assert!(!WorkspaceTabDropRules::can_accept(
        &tabs,
        &WorkspaceTabId::new("loose"),
        0
    ));
    assert!(!WorkspaceTabDropRules::can_accept(
        &tabs,
        &WorkspaceTabId::new("loose"),
        1
    ));
    assert!(WorkspaceTabDropRules::can_accept(
        &tabs,
        &WorkspaceTabId::new("loose"),
        2
    ));
    assert!(WorkspaceTabDropRules::can_accept(
        &tabs,
        &WorkspaceTabId::new("pinned"),
        0
    ));
    assert!(!WorkspaceTabDropRules::can_accept(
        &tabs,
        &WorkspaceTabId::new("pinned"),
        1
    ));
}

#[test]
fn dirty_close_requires_confirm_before_tab_closed() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("draft", "Draft").dirty(true))
        .active_tab_id("draft");

    let request = bar.apply_action(WorkspaceTabBarAction::CloseTab {
        tab_id: WorkspaceTabId::new("draft"),
    });
    let confirm = bar.apply_action(WorkspaceTabBarAction::ConfirmClose {
        tab_id: WorkspaceTabId::new("draft"),
    });

    assert_eq!(
        vec![WorkspaceTabBarEvent::TabCloseRequested {
            tab_id: WorkspaceTabId::new("draft")
        }],
        request
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabClosed {
            tab_id: WorkspaceTabId::new("draft")
        }],
        confirm
    );
    let event_names: Vec<&str> = bar
        .event_log()
        .iter()
        .map(WorkspaceTabBarEvent::name)
        .collect();
    assert_eq!(
        vec!["closeable_tab_close_requested", "closeable_tab_closed"],
        event_names
    );
    assert!(bar.options().tabs.is_empty());
}

#[test]
fn add_tab_is_typed_action_and_can_activate_new_tab() {
    let mut bar = WorkspaceTabBar::new("Workspace").tab(WorkspaceTab::new("main", "Main"));

    let events = bar.apply_action(WorkspaceTabBarAction::AddTab {
        tab: WorkspaceTab::new("scratch", "Scratch"),
        activate: true,
    });

    assert_eq!(
        vec![WorkspaceTabBarEvent::TabAdded {
            tab_id: WorkspaceTabId::new("scratch")
        }],
        events
    );
    assert_eq!(2, bar.options().tabs.len());
    assert_eq!(
        Some(&WorkspaceTabId::new("scratch")),
        bar.state().active_tab_id.as_ref()
    );
    assert_eq!("closeable_tab_added", bar.event_log()[0].name());
}

#[test]
fn pinned_tabs_stay_fixed_until_unpinned_before_close() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("pinned", "Pinned").pinned(true))
        .active_tab_id("pinned");

    let blocked = bar.apply_action(WorkspaceTabBarAction::CloseTab {
        tab_id: WorkspaceTabId::new("pinned"),
    });
    let unpinned = bar.apply_action(WorkspaceTabBarAction::UnpinTab {
        tab_id: WorkspaceTabId::new("pinned"),
    });
    let closed = bar.apply_action(WorkspaceTabBarAction::CloseTab {
        tab_id: WorkspaceTabId::new("pinned"),
    });

    assert!(blocked.is_empty());
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabPinChanged {
            tab_id: WorkspaceTabId::new("pinned"),
            pinned: false
        }],
        unpinned
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabClosed {
            tab_id: WorkspaceTabId::new("pinned")
        }],
        closed
    );
    assert!(bar.options().tabs.is_empty());
}

#[test]
fn context_menu_bulk_close_actions_keep_pinned_and_request_dirty_confirmation() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("pinned", "Pinned").pinned(true))
        .tab(WorkspaceTab::new("left", "Left"))
        .tab(WorkspaceTab::new("active", "Active"))
        .tab(WorkspaceTab::new("dirty", "Dirty").dirty(true))
        .tab(WorkspaceTab::new("right", "Right"))
        .active_tab_id("active");

    let events = bar.apply_action(WorkspaceTabBarAction::CloseOthers {
        tab_id: WorkspaceTabId::new("active"),
    });
    let remaining_ids: Vec<&str> = bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();
    let event_names: Vec<&str> = events.iter().map(WorkspaceTabBarEvent::name).collect();

    assert_eq!(vec!["pinned", "active", "dirty"], remaining_ids);
    assert_eq!(
        vec![
            "closeable_tab_close_requested",
            "closeable_tab_closed",
            "closeable_tab_closed"
        ],
        event_names
    );
    assert_eq!(
        Some(&WorkspaceTabId::new("dirty")),
        bar.state().pending_close_confirm.as_ref()
    );
    assert_eq!(
        Some(&WorkspaceTabId::new("active")),
        bar.state().active_tab_id.as_ref()
    );
}

#[test]
fn close_to_left_right_and_all_follow_visual_tab_order() {
    let mut right_bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("one", "One"))
        .tab(WorkspaceTab::new("two", "Two"))
        .tab(WorkspaceTab::new("three", "Three"));
    let right_events = right_bar.apply_action(WorkspaceTabBarAction::CloseToRight {
        tab_id: WorkspaceTabId::new("one"),
    });
    let right_remaining: Vec<&str> = right_bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();

    let mut left_bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("one", "One"))
        .tab(WorkspaceTab::new("two", "Two"))
        .tab(WorkspaceTab::new("three", "Three"));
    let left_events = left_bar.apply_action(WorkspaceTabBarAction::CloseToLeft {
        tab_id: WorkspaceTabId::new("three"),
    });
    let left_remaining: Vec<&str> = left_bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();

    let mut all_bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("pinned", "Pinned").pinned(true))
        .tab(WorkspaceTab::new("scratch", "Scratch"));
    let all_events = all_bar.apply_action(WorkspaceTabBarAction::CloseAll);
    let all_remaining: Vec<&str> = all_bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();

    assert_eq!(vec!["one"], right_remaining);
    assert_eq!(2, right_events.len());
    assert_eq!(
        Some(&WorkspaceTabId::new("one")),
        right_bar.state().active_tab_id.as_ref()
    );
    assert_eq!(vec!["three"], left_remaining);
    assert_eq!(2, left_events.len());
    assert_eq!(
        Some(&WorkspaceTabId::new("three")),
        left_bar.state().active_tab_id.as_ref()
    );
    assert_eq!(vec!["pinned"], all_remaining);
    assert_eq!(1, all_events.len());
}

#[test]
fn closed_tab_history_restores_last_closed_tab_through_typed_action() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("one", "One"))
        .tab(WorkspaceTab::new("two", "Two"))
        .active_tab_id("one");

    let closed = bar.apply_action(WorkspaceTabBarAction::CloseTab {
        tab_id: WorkspaceTabId::new("two"),
    });
    let restored = bar.apply_action(WorkspaceTabBarAction::RestoreClosedTab);
    let visual_ids: Vec<&str> = bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();

    assert_eq!(
        vec![WorkspaceTabBarEvent::TabClosed {
            tab_id: WorkspaceTabId::new("two")
        }],
        closed
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabRestored {
            tab_id: WorkspaceTabId::new("two")
        }],
        restored
    );
    assert_eq!(vec!["one", "two"], visual_ids);
    assert_eq!(
        Some(&WorkspaceTabId::new("two")),
        bar.state().active_tab_id.as_ref()
    );
    assert!(bar.state().recently_closed_tabs.is_empty());
}

#[test]
fn close_to_left_right_endpoint_targets_are_noop() {
    let mut right_bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("one", "One"))
        .tab(WorkspaceTab::new("two", "Two"));
    let right_events = right_bar.apply_action(WorkspaceTabBarAction::CloseToRight {
        tab_id: WorkspaceTabId::new("two"),
    });

    let mut left_bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("one", "One"))
        .tab(WorkspaceTab::new("two", "Two"));
    let left_events = left_bar.apply_action(WorkspaceTabBarAction::CloseToLeft {
        tab_id: WorkspaceTabId::new("one"),
    });

    assert!(right_events.is_empty());
    assert!(left_events.is_empty());
    assert_eq!(2, right_bar.options().tabs.len());
    assert_eq!(2, left_bar.options().tabs.len());
}

#[test]
fn bulk_close_keeps_pinned_and_non_closeable_tabs() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("pinned", "Pinned").pinned(true))
        .tab(WorkspaceTab::new("fixed", "Fixed").closeable(false))
        .tab(WorkspaceTab::new("active", "Active"))
        .tab(WorkspaceTab::new("right", "Right"))
        .active_tab_id("active");

    let events = bar.apply_action(WorkspaceTabBarAction::CloseAll);
    let remaining_ids: Vec<&str> = bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();

    assert_eq!(vec!["pinned", "fixed"], remaining_ids);
    assert_eq!(2, events.len());
    assert_eq!(
        Some(&WorkspaceTabId::new("pinned")),
        bar.state().active_tab_id.as_ref()
    );
}

#[test]
fn move_tab_emits_reordered_event_and_updates_visual_order() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("one", "One"))
        .tab(WorkspaceTab::new("two", "Two"))
        .tab(WorkspaceTab::new("three", "Three"));

    let events = bar.apply_action(WorkspaceTabBarAction::MoveTab {
        tab_id: WorkspaceTabId::new("one"),
        to_visual_index: 2,
    });
    let visual_ids: Vec<&str> = bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();

    assert_eq!(vec!["two", "three", "one"], visual_ids);
    assert_eq!(
        vec![WorkspaceTabBarEvent::TabReordered {
            tab_id: WorkspaceTabId::new("one"),
            from: 0,
            to: 2
        }],
        events
    );
}

#[test]
fn move_to_group_supports_existing_and_new_group_targets() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .group(WorkspaceTabGroup::new("docs", "Docs"))
        .tab(WorkspaceTab::new("article", "Article"))
        .tab(WorkspaceTab::new("scratch", "Scratch"));

    let existing = bar.apply_action(WorkspaceTabBarAction::MoveToGroup {
        tab_id: WorkspaceTabId::new("article"),
        target: WorkspaceTabGroupTarget::Existing("docs".into()),
    });
    let created = bar.apply_action(WorkspaceTabBarAction::MoveToGroup {
        tab_id: WorkspaceTabId::new("scratch"),
        target: WorkspaceTabGroupTarget::NewGroup(WorkspaceTabGroup::new("notes", "Notes")),
    });

    assert_eq!(
        vec![WorkspaceTabBarEvent::TabGroupChanged {
            tab_id: WorkspaceTabId::new("article"),
            group_id: Some("docs".into())
        }],
        existing
    );
    assert_eq!(
        vec![
            WorkspaceTabBarEvent::GroupCreated {
                group_id: "notes".into()
            },
            WorkspaceTabBarEvent::TabGroupChanged {
                tab_id: WorkspaceTabId::new("scratch"),
                group_id: Some("notes".into())
            }
        ],
        created
    );
    assert_eq!(
        Some(&"docs".into()),
        bar.options().tabs[0].group_id.as_ref()
    );
    assert_eq!(
        Some(&"notes".into()),
        bar.options().tabs[1].group_id.as_ref()
    );
    assert_eq!(2, bar.options().groups.len());
}

#[test]
fn move_group_reorders_declared_groups_and_visual_tabs() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .group(WorkspaceTabGroup::new("docs", "Docs"))
        .group(WorkspaceTabGroup::new("notes", "Notes"))
        .tab(WorkspaceTab::new("docs-a", "Docs A").group_id("docs"))
        .tab(WorkspaceTab::new("notes-a", "Notes A").group_id("notes"))
        .tab(WorkspaceTab::new("pinned", "Pinned").pinned(true))
        .tab(WorkspaceTab::new("loose", "Loose"));

    let events = bar.apply_action(WorkspaceTabBarAction::MoveGroup {
        group_id: "docs".into(),
        to_index: 1,
    });
    let group_ids: Vec<&str> = bar
        .options()
        .groups
        .iter()
        .map(|group| group.id.as_str())
        .collect();
    let visual_ids: Vec<&str> = bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();

    assert_eq!(vec!["notes", "docs"], group_ids);
    assert_eq!(vec!["pinned", "notes-a", "docs-a", "loose"], visual_ids);
    assert_eq!(
        vec![WorkspaceTabBarEvent::GroupReordered {
            group_id: "docs".into(),
            from: 0,
            to: 1
        }],
        events
    );
    assert_eq!("closeable_tab_group_reordered", events[0].name());
}

#[test]
fn rename_group_updates_label_and_emits_typed_event() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .group(WorkspaceTabGroup::new("docs", "Docs"))
        .tab(WorkspaceTab::new("docs-a", "Docs A").group_id("docs"));

    let events = bar.apply_action(WorkspaceTabBarAction::RenameGroup {
        group_id: "docs".into(),
        label: "Reference".to_string(),
    });
    let node = UiNode::from(bar.clone());

    assert_eq!("Reference", bar.options().groups[0].label);
    assert_eq!(
        vec![WorkspaceTabBarEvent::GroupRenamed {
            group_id: "docs".into(),
            label: "Reference".to_string()
        }],
        events
    );
    assert_eq!("closeable_tab_group_renamed", events[0].name());
    assert!(
        node.children()
            .iter()
            .any(|child| child.props().label == "Reference")
    );
}

#[test]
fn move_to_group_rejects_pinned_and_ungroupable_tabs() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .group(WorkspaceTabGroup::new("docs", "Docs"))
        .tab(WorkspaceTab::new("pinned", "Pinned").pinned(true))
        .tab(WorkspaceTab::new("virtual", "Virtual").groupable(false));

    let pinned_events = bar.apply_action(WorkspaceTabBarAction::MoveToGroup {
        tab_id: WorkspaceTabId::new("pinned"),
        target: WorkspaceTabGroupTarget::Existing("docs".into()),
    });
    let ungroupable_events = bar.apply_action(WorkspaceTabBarAction::MoveToGroup {
        tab_id: WorkspaceTabId::new("virtual"),
        target: WorkspaceTabGroupTarget::NewGroup(WorkspaceTabGroup::new("created", "Created")),
    });

    assert!(pinned_events.is_empty());
    assert!(ungroupable_events.is_empty());
    assert_eq!(None, bar.options().tabs[0].group_id);
    assert_eq!(None, bar.options().tabs[1].group_id);
    assert!(
        bar.options()
            .groups
            .iter()
            .all(|group| group.id != WorkspaceTabGroupId::new("created"))
    );
}

#[test]
fn group_color_ungroup_and_close_group_emit_typed_events() {
    let mut color_bar = WorkspaceTabBar::new("Workspace")
        .group(WorkspaceTabGroup::new("docs", "Docs").color("#4a90d9"));
    let color_events = color_bar.apply_action(WorkspaceTabBarAction::SetGroupColor {
        group_id: "docs".into(),
        color: "#5aa65a".to_string(),
    });

    assert_eq!("#5aa65a", color_bar.options().groups[0].color);
    assert_eq!(
        vec![WorkspaceTabBarEvent::GroupColorChanged {
            group_id: "docs".into(),
            color: "#5aa65a".to_string()
        }],
        color_events
    );

    let mut ungroup_bar = WorkspaceTabBar::new("Workspace")
        .group(WorkspaceTabGroup::new("docs", "Docs"))
        .tab(WorkspaceTab::new("docs-a", "Docs A").group_id("docs"))
        .tab(WorkspaceTab::new("docs-b", "Docs B").group_id("docs"));
    let ungroup_events = ungroup_bar.apply_action(WorkspaceTabBarAction::Ungroup {
        group_id: "docs".into(),
    });

    assert!(ungroup_bar.options().groups.is_empty());
    assert!(
        ungroup_bar
            .options()
            .tabs
            .iter()
            .all(|tab| tab.group_id.is_none())
    );
    assert_eq!(
        vec![
            WorkspaceTabBarEvent::TabGroupChanged {
                tab_id: WorkspaceTabId::new("docs-a"),
                group_id: None
            },
            WorkspaceTabBarEvent::TabGroupChanged {
                tab_id: WorkspaceTabId::new("docs-b"),
                group_id: None
            },
            WorkspaceTabBarEvent::GroupRemoved {
                group_id: "docs".into()
            }
        ],
        ungroup_events
    );

    let mut close_bar = WorkspaceTabBar::new("Workspace")
        .group(WorkspaceTabGroup::new("docs", "Docs"))
        .tab(WorkspaceTab::new("docs-a", "Docs A").group_id("docs"))
        .tab(WorkspaceTab::new("docs-b", "Docs B").group_id("docs"))
        .tab(WorkspaceTab::new("loose", "Loose"));
    let close_events = close_bar.apply_action(WorkspaceTabBarAction::CloseGroup {
        group_id: "docs".into(),
    });
    let remaining_ids: Vec<&str> = close_bar
        .options()
        .tabs
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();

    assert_eq!(vec!["loose"], remaining_ids);
    assert!(close_bar.options().groups.is_empty());
    assert_eq!(
        vec![
            WorkspaceTabBarEvent::TabClosed {
                tab_id: WorkspaceTabId::new("docs-a")
            },
            WorkspaceTabBarEvent::TabClosed {
                tab_id: WorkspaceTabId::new("docs-b")
            },
            WorkspaceTabBarEvent::GroupRemoved {
                group_id: "docs".into()
            }
        ],
        close_events
    );
}

#[test]
fn move_group_clamps_out_of_range_target_index_to_last_declared_group() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .group(WorkspaceTabGroup::new("docs", "Docs"))
        .group(WorkspaceTabGroup::new("notes", "Notes"))
        .tab(WorkspaceTab::new("docs-a", "Docs A").group_id("docs"))
        .tab(WorkspaceTab::new("notes-a", "Notes A").group_id("notes"));

    let events = bar.apply_action(WorkspaceTabBarAction::MoveGroup {
        group_id: "docs".into(),
        to_index: 99,
    });
    let group_ids: Vec<&str> = bar
        .options()
        .groups
        .iter()
        .map(|group| group.id.as_str())
        .collect();
    let visual_ids: Vec<&str> = bar
        .visual_tabs()
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();

    assert_eq!(vec!["notes", "docs"], group_ids);
    assert_eq!(vec!["notes-a", "docs-a"], visual_ids);
    assert_eq!(
        vec![WorkspaceTabBarEvent::GroupReordered {
            group_id: "docs".into(),
            from: 0,
            to: 1
        }],
        events
    );
}

#[test]
fn group_collapse_and_overflow_emit_typed_events() {
    let mut bar = WorkspaceTabBar::new("Workspace").group(WorkspaceTabGroup::new("docs", "Docs"));

    let collapse = bar.apply_action(WorkspaceTabBarAction::ToggleGroupCollapse {
        group_id: "docs".into(),
    });
    let overflow = bar.apply_action(WorkspaceTabBarAction::OpenOverflow {
        hidden_tab_ids: vec![WorkspaceTabId::new("hidden")],
    });

    assert_eq!("closeable_tab_group_collapse_changed", collapse[0].name());
    assert!(bar.options().groups[0].collapsed);
    assert_eq!("closeable_tab_overflow_opened", overflow[0].name());
    assert!(bar.state().overflow_visible);
}

#[test]
fn child_state_ids_are_unique_and_separate_from_parent_state() {
    let bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("one", "One"))
        .tab(WorkspaceTab::new("two", "Two"));
    let child_ids: HashSet<&str> = bar
        .state()
        .child_states
        .iter()
        .map(|child| child.state_id.as_str())
        .collect();

    assert_eq!(2, child_ids.len());
    assert!(
        bar.state()
            .child_states
            .iter()
            .all(|child| child.state_id != bar.state().state_id)
    );
}

#[test]
fn collapsed_group_auto_expands_after_drop_hover_delay() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .group(WorkspaceTabGroup::new("docs", "Docs").collapsed(true))
        .tab(WorkspaceTab::new("draft", "Draft").group_id("docs"));
    let delay = bar.options().collapsed_group_auto_expand_ms;

    let early = bar.apply_action(WorkspaceTabBarAction::HoverCollapsedGroupForDrop {
        group_id: "docs".into(),
        elapsed_ms: delay.saturating_sub(1),
    });
    let expanded = bar.apply_action(WorkspaceTabBarAction::HoverCollapsedGroupForDrop {
        group_id: "docs".into(),
        elapsed_ms: delay,
    });

    assert!(early.is_empty());
    assert_eq!(
        vec![WorkspaceTabBarEvent::GroupCollapseChanged {
            group_id: "docs".into(),
            collapsed: false
        }],
        expanded
    );
    assert!(!bar.options().groups[0].collapsed);
}

#[test]
fn tab_drag_lifecycle_sets_state_and_uses_drag_primitives() -> Result<(), String> {
    let mut bar = WorkspaceTabBar::new("Workspace").tab(
        WorkspaceTab::new("draft", "Draft")
            .icon("<svg/>")
            .dirty(true),
    );
    let tab_id = WorkspaceTabId::new("draft");

    let source = bar
        .drag_source(&tab_id)
        .ok_or_else(|| "drag source is missing".to_string())?;
    let target = bar
        .drop_target_for_tab(&tab_id)
        .ok_or_else(|| "drop target is missing".to_string())?;
    let preview = bar
        .drag_preview_for_tab(&tab_id)
        .ok_or_else(|| "drag preview is missing".to_string())?;
    let started = bar.apply_action(WorkspaceTabBarAction::StartDrag {
        tab_id: tab_id.clone(),
    });
    let cancelled = bar.apply_action(WorkspaceTabBarAction::CancelDrag);

    assert_eq!(CLOSEABLE_TAB_DRAG_TAG, source.payload.tag);
    assert_eq!(DropEffect::Move, target.effect);
    assert!(
        target
            .accepted_tags
            .contains(&CLOSEABLE_TAB_DRAG_TAG.to_string())
    );
    assert_eq!(
        DropIndicatorOrientation::Vertical,
        target.indicator_orientation
    );
    assert_eq!(
        UiNodeKind::DragPreview,
        crate::render_model::UiNode::from(preview).kind()
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::DragStarted {
            tab_id: tab_id.clone()
        }],
        started
    );
    assert_eq!(
        vec![WorkspaceTabBarEvent::DragEnded {
            tab_id,
            committed: false
        }],
        cancelled
    );
    assert!(!bar.state().drag_in_progress);
    Ok(())
}
