use super::super::{
    WORKSPACE_TAB_DRAG_TAG, WorkspaceGroupContextCommand, WorkspaceTab, WorkspaceTabBar,
    WorkspaceTabBarAction, WorkspaceTabBarOptions, WorkspaceTabBarState, WorkspaceTabChildState,
    WorkspaceTabContextCommand, WorkspaceTabContextMenu, WorkspaceTabDropPosition,
    WorkspaceTabGroup, WorkspaceTabGroupId, WorkspaceTabGroupTarget, WorkspaceTabId,
    WorkspaceTabOverflowPlan, WorkspaceTabTone,
};
use crate::render_model::UiStateId;

#[test]
fn typed_options_cover_workspace_tab_and_group_contract() {
    let tab = WorkspaceTab::new("draft", "Draft")
        .icon("<svg/>")
        .dirty(true)
        .pinned(true)
        .closeable(false)
        .tone(WorkspaceTabTone::Warning)
        .tooltip("Unsaved")
        .group_id("docs")
        .accessibility_label("Draft modified");
    let group = WorkspaceTabGroup::new("docs", "Docs")
        .color("accent")
        .collapsed(true);

    assert_eq!("katana-ui-core/workspace-tab", WORKSPACE_TAB_DRAG_TAG);
    assert_eq!(Some("<svg/>"), tab.icon.as_deref());
    assert_eq!(WorkspaceTabTone::Warning, tab.tone);
    assert_eq!(
        Some("docs"),
        tab.group_id.as_ref().map(WorkspaceTabGroupId::as_str)
    );
    assert!(!tab.closeable);
    assert!(tab.dirty);
    assert!(tab.pinned);
    assert_eq!("accent", group.color);
    assert!(group.collapsed);
}

#[test]
fn context_command_sets_match_tab_and_group_state() {
    let pinned = WorkspaceTab::new("pinned", "Pinned").pinned(true);
    let group = WorkspaceTabGroup::new("docs", "Docs").collapsed(true);

    let tab_commands = WorkspaceTabContextMenu::tab_commands(&pinned, std::slice::from_ref(&group));
    let group_commands = WorkspaceTabContextMenu::group_commands(&group);

    assert!(tab_commands.contains(&WorkspaceTabContextCommand::Unpin));
    assert!(tab_commands.contains(&WorkspaceTabContextCommand::MoveToGroup));
    assert!(group_commands.contains(&WorkspaceGroupContextCommand::Rename));
    assert!(group_commands.contains(&WorkspaceGroupContextCommand::Expand));
}

#[test]
fn state_and_overflow_models_are_typed_and_addressable() {
    let tab = WorkspaceTab::new("draft", "Draft");
    let state = WorkspaceTabBarState::new(std::slice::from_ref(&tab));
    let plan = WorkspaceTabOverflowPlan {
        visible_tab_ids: vec![WorkspaceTabId::new("draft")],
        hidden_tab_ids: Vec::new(),
        overflow_visible: false,
    };
    let child = WorkspaceTabChildState {
        tab_id: WorkspaceTabId::new("draft"),
        state_id: UiStateId::new("child"),
    };

    assert_eq!(Some(&tab.id), plan.visible_tab_ids.first());
    assert!(state.child_state_id(&tab.id).is_some());
    assert_eq!("draft", child.tab_id.as_str());
}

#[test]
fn less_common_actions_and_options_remain_part_of_the_contract() {
    let mut bar = WorkspaceTabBar::new("Workspace")
        .tab(WorkspaceTab::new("one", "One"))
        .tab(WorkspaceTab::new("two", "Two"));
    let default_options = WorkspaceTabBarOptions::default();
    let all_drop_positions = [
        WorkspaceTabDropPosition::Before,
        WorkspaceTabDropPosition::After,
        WorkspaceTabDropPosition::InsideGroup,
        WorkspaceTabDropPosition::NewGroup,
    ];

    bar.apply_action(WorkspaceTabBarAction::SelectTab {
        tab_id: WorkspaceTabId::new("one"),
    });
    bar.apply_action(WorkspaceTabBarAction::PinTab {
        tab_id: WorkspaceTabId::new("two"),
    });
    bar.apply_action(WorkspaceTabBarAction::UnpinTab {
        tab_id: WorkspaceTabId::new("two"),
    });
    bar.apply_action(WorkspaceTabBarAction::MoveToGroup {
        tab_id: WorkspaceTabId::new("one"),
        target: WorkspaceTabGroupTarget::Ungrouped,
    });

    assert_eq!(44, default_options.overflow_trigger_width);
    assert_eq!(4, all_drop_positions.len());
    assert!(!bar.event_log().is_empty());
}
