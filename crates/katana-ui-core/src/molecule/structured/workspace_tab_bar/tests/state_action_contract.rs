use super::super::{
    WorkspaceTab, WorkspaceTabBar, WorkspaceTabBarAction, WorkspaceTabBarEvent,
    WorkspaceTabDropRules, WorkspaceTabGroup, WorkspaceTabGroupTarget, WorkspaceTabId,
};
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
        vec!["workspace_tab_close_requested", "workspace_tab_closed"],
        event_names
    );
    assert!(bar.options().tabs.is_empty());
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

    bar.apply_action(WorkspaceTabBarAction::MoveToGroup {
        tab_id: WorkspaceTabId::new("article"),
        target: WorkspaceTabGroupTarget::Existing("docs".into()),
    });
    bar.apply_action(WorkspaceTabBarAction::MoveToGroup {
        tab_id: WorkspaceTabId::new("scratch"),
        target: WorkspaceTabGroupTarget::NewGroup(WorkspaceTabGroup::new("notes", "Notes")),
    });

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
fn group_collapse_and_overflow_emit_typed_events() {
    let mut bar = WorkspaceTabBar::new("Workspace").group(WorkspaceTabGroup::new("docs", "Docs"));

    let collapse = bar.apply_action(WorkspaceTabBarAction::ToggleGroupCollapse {
        group_id: "docs".into(),
    });
    let overflow = bar.apply_action(WorkspaceTabBarAction::OpenOverflow {
        hidden_tab_ids: vec![WorkspaceTabId::new("hidden")],
    });

    assert_eq!("workspace_tab_group_collapse_changed", collapse[0].name());
    assert!(bar.options().groups[0].collapsed);
    assert_eq!("workspace_tab_overflow_opened", overflow[0].name());
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
