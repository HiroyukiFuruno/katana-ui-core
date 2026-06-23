use super::super::{
    CLOSEABLE_TAB_DRAG_TAG, WorkspaceGroupContextCommand, WorkspaceTab, WorkspaceTabBar,
    WorkspaceTabBarAction, WorkspaceTabBarOptions, WorkspaceTabBarState, WorkspaceTabChildState,
    WorkspaceTabContextCommand, WorkspaceTabContextMenu, WorkspaceTabDropPosition,
    WorkspaceTabGroup, WorkspaceTabGroupId, WorkspaceTabGroupTarget, WorkspaceTabId,
    WorkspaceTabOverflowPlan, WorkspaceTabTone,
};
use crate::molecule::selection::{
    ContextMenuAction, ContextMenuAnchor, ContextMenuEvent, ContextMenuItem, ContextMenuItemKind,
};
use crate::render_model::{UiIconProps, UiStateId, UiSvgPaintPolicy};
use crate::render_model::{UiNode, UiNodeKind};

#[test]
fn typed_options_cover_workspace_tab_and_group_contract() {
    let tab = WorkspaceTab::new("draft", "Draft")
        .icon("<svg/>")
        .dirty(true)
        .pinned(true)
        .closeable(false)
        .groupable(false)
        .tone(WorkspaceTabTone::Warning)
        .tooltip("Unsaved")
        .group_id("docs")
        .accessibility_label("Draft modified");
    let group = WorkspaceTabGroup::new("docs", "Docs")
        .color("accent")
        .collapsed(true);

    assert_eq!("katana-ui-core/closeable-tab", CLOSEABLE_TAB_DRAG_TAG);
    assert_eq!(
        Some("<svg/>"),
        tab.icon.as_ref().map(|icon| icon.svg_source.as_str())
    );
    assert_eq!(WorkspaceTabTone::Warning, tab.tone);
    assert_eq!(
        Some("docs"),
        tab.group_id.as_ref().map(WorkspaceTabGroupId::as_str)
    );
    assert!(!tab.closeable);
    assert!(!tab.groupable);
    assert!(tab.dirty);
    assert!(tab.pinned);
    assert_eq!("accent", group.color);
    assert!(group.collapsed);
}

#[test]
fn workspace_tab_icon_accepts_external_svg_props() {
    let tab = WorkspaceTab::new("search", "Search").svg_icon(
        UiIconProps::new("<svg data-icon=\"search\"/>")
            .view_box("0 0 16 16")
            .role("search")
            .paint_policy(UiSvgPaintPolicy::CurrentColor),
    );

    assert!(tab.icon.is_some());
    if let Some(icon) = tab.icon.as_ref() {
        assert_eq!("<svg data-icon=\"search\"/>", icon.svg_source);
        assert_eq!("0 0 16 16", icon.view_box);
        assert_eq!("search", icon.role);
        assert_eq!(UiSvgPaintPolicy::CurrentColor, icon.paint_policy);
    }
}

#[test]
fn context_command_sets_match_tab_and_group_state() {
    let pinned = WorkspaceTab::new("pinned", "Pinned").pinned(true);
    let regular = WorkspaceTab::new("regular", "Regular");
    let ungroupable = WorkspaceTab::new("virtual", "Virtual").groupable(false);
    let group = WorkspaceTabGroup::new("docs", "Docs").collapsed(true);

    let pinned_commands =
        WorkspaceTabContextMenu::tab_commands(&pinned, std::slice::from_ref(&group));
    let tab_commands =
        WorkspaceTabContextMenu::tab_commands(&regular, std::slice::from_ref(&group));
    let ungroupable_commands =
        WorkspaceTabContextMenu::tab_commands(&ungroupable, std::slice::from_ref(&group));
    let group_commands = WorkspaceTabContextMenu::group_commands(&group);

    assert_eq!(
        vec![
            WorkspaceTabContextCommand::Close,
            WorkspaceTabContextCommand::CloseOthers,
            WorkspaceTabContextCommand::CloseAll,
            WorkspaceTabContextCommand::CloseToRight,
            WorkspaceTabContextCommand::CloseToLeft,
            WorkspaceTabContextCommand::Pin,
            WorkspaceTabContextCommand::MoveToNewGroup,
            WorkspaceTabContextCommand::MoveToGroup,
        ],
        tab_commands
    );
    assert!(tab_commands.contains(&WorkspaceTabContextCommand::MoveToGroup));
    assert!(pinned_commands.contains(&WorkspaceTabContextCommand::Unpin));
    assert!(!pinned_commands.contains(&WorkspaceTabContextCommand::MoveToNewGroup));
    assert!(!pinned_commands.contains(&WorkspaceTabContextCommand::MoveToGroup));
    assert!(!ungroupable_commands.contains(&WorkspaceTabContextCommand::MoveToNewGroup));
    assert!(!ungroupable_commands.contains(&WorkspaceTabContextCommand::MoveToGroup));
    assert!(group_commands.contains(&WorkspaceGroupContextCommand::Rename));
    assert!(group_commands.contains(&WorkspaceGroupContextCommand::SetColor));
    assert!(group_commands.contains(&WorkspaceGroupContextCommand::Expand));
    assert!(group_commands.contains(&WorkspaceGroupContextCommand::Ungroup));
    assert!(group_commands.contains(&WorkspaceGroupContextCommand::Close));
}

#[test]
fn context_commands_map_to_public_tab_actions() {
    let tab_id = WorkspaceTabId::new("draft");

    assert_eq!(
        Some(WorkspaceTabBarAction::CloseTab {
            tab_id: tab_id.clone()
        }),
        WorkspaceTabContextCommand::Close.to_tab_action(tab_id.clone())
    );
    assert_eq!(
        Some(WorkspaceTabBarAction::CloseOthers {
            tab_id: tab_id.clone()
        }),
        WorkspaceTabContextCommand::CloseOthers.to_tab_action(tab_id.clone())
    );
    assert_eq!(
        Some(WorkspaceTabBarAction::CloseAll),
        WorkspaceTabContextCommand::CloseAll.to_tab_action(tab_id.clone())
    );
    assert_eq!(
        Some(WorkspaceTabBarAction::PinTab {
            tab_id: tab_id.clone()
        }),
        WorkspaceTabContextCommand::Pin.to_tab_action(tab_id.clone())
    );
    assert_eq!(
        None,
        WorkspaceTabContextCommand::MoveToGroup.to_tab_action(tab_id.clone())
    );
    assert_eq!(
        WorkspaceTabBarAction::MoveToGroup {
            tab_id: tab_id.clone(),
            target: WorkspaceTabGroupTarget::Existing(WorkspaceTabGroupId::new("docs")),
        },
        WorkspaceTabContextCommand::move_to_existing_group_action(
            tab_id.clone(),
            WorkspaceTabGroupId::new("docs"),
        )
    );
    assert_eq!(
        WorkspaceTabBarAction::MoveToGroup {
            tab_id,
            target: WorkspaceTabGroupTarget::NewGroup(WorkspaceTabGroup::new("notes", "Notes")),
        },
        WorkspaceTabContextCommand::move_to_new_group_action(
            WorkspaceTabId::new("draft"),
            WorkspaceTabGroup::new("notes", "Notes"),
        )
    );
    assert_eq!(
        "move-to-group:docs",
        WorkspaceTabContextCommand::move_to_group_item_id(&WorkspaceTabGroupId::new("docs"))
    );
    assert_eq!(
        Some(WorkspaceTabGroupId::new("docs")),
        WorkspaceTabContextCommand::move_to_group_id_from_item_id("move-to-group:docs")
    );
    assert_eq!(
        None,
        WorkspaceTabContextCommand::move_to_group_id_from_item_id("move-to-group")
    );
}

#[test]
fn all_tab_context_command_ids_round_trip_to_public_actions() {
    let tab_id = WorkspaceTabId::new("draft");
    let cases = [
        (
            WorkspaceTabContextCommand::Close,
            Some(WorkspaceTabBarAction::CloseTab {
                tab_id: tab_id.clone(),
            }),
        ),
        (
            WorkspaceTabContextCommand::CloseOthers,
            Some(WorkspaceTabBarAction::CloseOthers {
                tab_id: tab_id.clone(),
            }),
        ),
        (
            WorkspaceTabContextCommand::CloseAll,
            Some(WorkspaceTabBarAction::CloseAll),
        ),
        (
            WorkspaceTabContextCommand::CloseToRight,
            Some(WorkspaceTabBarAction::CloseToRight {
                tab_id: tab_id.clone(),
            }),
        ),
        (
            WorkspaceTabContextCommand::CloseToLeft,
            Some(WorkspaceTabBarAction::CloseToLeft {
                tab_id: tab_id.clone(),
            }),
        ),
        (
            WorkspaceTabContextCommand::RestoreClosed,
            Some(WorkspaceTabBarAction::RestoreClosedTab),
        ),
        (
            WorkspaceTabContextCommand::Pin,
            Some(WorkspaceTabBarAction::PinTab {
                tab_id: tab_id.clone(),
            }),
        ),
        (
            WorkspaceTabContextCommand::Unpin,
            Some(WorkspaceTabBarAction::UnpinTab {
                tab_id: tab_id.clone(),
            }),
        ),
        (WorkspaceTabContextCommand::MoveToNewGroup, None),
        (WorkspaceTabContextCommand::MoveToGroup, None),
    ];

    for (command, expected_action) in cases {
        assert_eq!(
            Some(command),
            WorkspaceTabContextCommand::from_id(command.id())
        );
        assert_eq!(expected_action, command.to_tab_action(tab_id.clone()));
    }
}

#[test]
fn context_menu_command_ids_round_trip_to_typed_actions() {
    let tab_id = WorkspaceTabId::new("draft");
    let command = WorkspaceTabContextCommand::from_id("close-to-right");
    let group_command = WorkspaceGroupContextCommand::from_id("collapse");

    assert_eq!(Some(WorkspaceTabContextCommand::CloseToRight), command);
    assert_eq!(
        Some(WorkspaceTabBarAction::CloseToRight {
            tab_id: tab_id.clone()
        }),
        command.and_then(|it| it.to_tab_action(tab_id))
    );
    assert_eq!(Some(WorkspaceGroupContextCommand::Collapse), group_command);
    assert_eq!(None, WorkspaceTabContextCommand::from_id("unknown"));
    assert_eq!(None, WorkspaceGroupContextCommand::from_id("unknown"));
}

#[test]
fn group_context_commands_map_to_public_actions() {
    let expanded = WorkspaceTabGroup::new("docs", "Docs").collapsed(false);
    let collapsed = WorkspaceTabGroup::new("docs", "Docs").collapsed(true);

    assert_eq!(
        Some(WorkspaceTabBarAction::ToggleGroupCollapse {
            group_id: WorkspaceTabGroupId::new("docs")
        }),
        WorkspaceGroupContextCommand::Collapse.to_group_action(&expanded)
    );
    assert_eq!(
        Some(WorkspaceTabBarAction::ToggleGroupCollapse {
            group_id: WorkspaceTabGroupId::new("docs")
        }),
        WorkspaceGroupContextCommand::Expand.to_group_action(&collapsed)
    );
    assert_eq!(
        None,
        WorkspaceGroupContextCommand::Collapse.to_group_action(&collapsed)
    );
    assert_eq!(
        None,
        WorkspaceGroupContextCommand::Rename.to_group_action(&expanded)
    );
    assert_eq!(
        None,
        WorkspaceGroupContextCommand::Move.to_group_action(&expanded)
    );
    assert_eq!(
        None,
        WorkspaceGroupContextCommand::SetColor.to_group_action(&expanded)
    );
    assert_eq!(
        Some(WorkspaceTabBarAction::Ungroup {
            group_id: WorkspaceTabGroupId::new("docs")
        }),
        WorkspaceGroupContextCommand::Ungroup.to_group_action(&expanded)
    );
    assert_eq!(
        Some(WorkspaceTabBarAction::CloseGroup {
            group_id: WorkspaceTabGroupId::new("docs")
        }),
        WorkspaceGroupContextCommand::Close.to_group_action(&expanded)
    );
    assert_eq!(
        WorkspaceTabBarAction::RenameGroup {
            group_id: WorkspaceTabGroupId::new("docs"),
            label: "Reference".to_string()
        },
        WorkspaceGroupContextCommand::rename_group_action(
            WorkspaceTabGroupId::new("docs"),
            "Reference",
        )
    );
    assert_eq!(
        WorkspaceTabBarAction::MoveGroup {
            group_id: WorkspaceTabGroupId::new("docs"),
            to_index: 1
        },
        WorkspaceGroupContextCommand::move_group_action(WorkspaceTabGroupId::new("docs"), 1)
    );
    assert_eq!(
        WorkspaceTabBarAction::SetGroupColor {
            group_id: WorkspaceTabGroupId::new("docs"),
            color: "#5aa65a".to_string()
        },
        WorkspaceGroupContextCommand::set_group_color_action(
            WorkspaceTabGroupId::new("docs"),
            "#5aa65a",
        )
    );
}

#[test]
fn tab_context_menu_is_existing_context_menu_molecule() {
    let tab = WorkspaceTab::new("draft", "Draft");
    let group = WorkspaceTabGroup::new("docs", "Docs");
    let anchor = ContextMenuAnchor::Pointer { x: 12, y: 24 };
    let mut menu = WorkspaceTabContextMenu::tab_menu(
        "Tab menu",
        &tab,
        std::slice::from_ref(&group),
        anchor.clone(),
    );

    let node = UiNode::from(menu.clone());
    let opened = menu.apply_context_action(&ContextMenuAction::Open {
        anchor: anchor.clone(),
    });
    let selected = menu.apply_context_action(&ContextMenuAction::Activate { path: vec![5] });

    assert_eq!(UiNodeKind::ContextMenu, node.kind());
    assert_eq!(anchor, node.props().context_menu.anchor);
    assert_eq!("close", node.props().context_menu.items[0].id);
    assert_eq!("Close", node.props().context_menu.items[0].label);
    assert_eq!("pin", node.props().context_menu.items[5].id);
    assert_eq!(
        ContextMenuItemKind::Submenu,
        node.props().context_menu.items[6].kind
    );
    assert_eq!("Add to Group", node.props().context_menu.items[6].label);
    assert_eq!(
        "move-to-new-group",
        node.props().context_menu.items[6].children[0].id
    );
    assert_eq!(
        "move-to-group:docs",
        node.props().context_menu.items[6].children[1].id
    );
    assert_eq!("Docs", node.props().context_menu.items[6].children[1].label);
    assert_eq!("context_menu_opened", opened.name());
    assert_eq!(
        ContextMenuEvent::ItemSelected {
            path: vec![5],
            command: "pin".to_string()
        },
        selected
    );
}

#[test]
fn tab_context_menu_without_existing_groups_uses_direct_new_group_action() {
    let tab = WorkspaceTab::new("draft", "Draft");
    let anchor = ContextMenuAnchor::Pointer { x: 12, y: 24 };
    let menu = WorkspaceTabContextMenu::tab_menu("Tab menu", &tab, &[], anchor);
    let node = UiNode::from(menu);
    let items = &node.props().context_menu.items;

    assert_eq!("move-to-new-group", items[6].id);
    assert_eq!("Move to New Group", items[6].label);
    assert_eq!(ContextMenuItemKind::Action, items[6].kind);
    assert!(items[6].children.is_empty());
}

#[test]
fn tab_context_menu_accepts_consumer_provided_items() {
    let anchor = ContextMenuAnchor::NodeId("tab-editor".to_string());
    let menu = WorkspaceTabContextMenu::menu(
        "Custom tab menu",
        anchor.clone(),
        vec![ContextMenuItem::action("split-right", "Split Right")],
    );
    let node = UiNode::from(menu);

    assert_eq!(UiNodeKind::ContextMenu, node.kind());
    assert_eq!(anchor, node.props().context_menu.anchor);
    assert_eq!("split-right", node.props().context_menu.items[0].id);
    assert_eq!("Split Right", node.props().context_menu.items[0].label);
}

#[test]
fn group_context_menu_is_existing_context_menu_molecule() {
    let group = WorkspaceTabGroup::new("docs", "Docs").collapsed(false);
    let anchor = ContextMenuAnchor::VirtualRect(crate::molecule::selection::ContextMenuRect::new(
        4, 8, 120, 28,
    ));
    let menu = WorkspaceTabContextMenu::group_menu("Group menu", &group, anchor.clone());
    let node = UiNode::from(menu);

    assert_eq!(UiNodeKind::ContextMenu, node.kind());
    assert_eq!(anchor, node.props().context_menu.anchor);
    assert_eq!("rename", node.props().context_menu.items[0].id);
    assert_eq!("set-color", node.props().context_menu.items[1].id);
    assert_eq!("collapse", node.props().context_menu.items[2].id);
    assert_eq!("move", node.props().context_menu.items[3].id);
    assert_eq!("ungroup", node.props().context_menu.items[4].id);
    assert_eq!("close-group", node.props().context_menu.items[5].id);
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
    bar.apply_action(WorkspaceTabBarAction::AddTab {
        tab: WorkspaceTab::new("three", "Three"),
        activate: true,
    });

    assert_eq!(44, default_options.overflow_trigger_width);
    assert_eq!(4, all_drop_positions.len());
    assert_eq!(3, bar.options().tabs.len());
    assert_eq!(
        Some(&WorkspaceTabId::new("three")),
        bar.state().active_tab_id.as_ref()
    );
    assert!(!bar.event_log().is_empty());
}
