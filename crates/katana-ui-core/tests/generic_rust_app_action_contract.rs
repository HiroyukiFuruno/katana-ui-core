use katana_ui_core::adapter_contract::AdapterActionBridge;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::widget::atoms::Input;
use katana_ui_core::widget::molecules::{
    CloseableTab, CloseableTabContextCommand, CloseableTabContextMenu, CloseableTabGroup,
    CloseableTabGroupContextCommand, CloseableTabGroupId, CloseableTabGroupTarget, CloseableTabId,
    CloseableTabStrip, CloseableTabStripAction, CloseableTabStripEvent, ContextMenuAnchor,
    ContextMenuItem, ContextMenuItemKind,
};

#[path = "generic_rust_app_contract/support.rs"]
mod support;

#[test]
fn generic_app_action_suite_reuses_public_props_support() {
    let search = katana_ui_core::render_model::UiNode::from(support::search_input());
    let notes = katana_ui_core::render_model::UiNode::from(support::notes_text_area());
    let tabs = katana_ui_core::render_model::UiNode::from(support::generic_tabs());

    support::assert_search_input_contract(&search);
    support::assert_text_area_contract(&notes);
    support::assert_workspace_tab_contract(&tabs);
}

#[test]
fn generic_app_input_icon_button_invokes_callback_without_mutating_text() {
    let mut input = support::search_input();
    let action = UiAction::invoke_callback(input.state_id().clone(), "generic.search.clear");

    let result = input.apply_action(&action);

    assert!(result.handled);
    assert_eq!("generic.search.clear", result.callback_log[0].action);
    assert_eq!("src", result.before.value);
    assert_eq!("src", result.after.value);
}

#[test]
fn generic_app_disabled_input_blocks_icon_button_callback() {
    let mut input = Input::new("Search").disabled(true);
    let action = UiAction::invoke_callback(input.state_id().clone(), "generic.search.clear");

    let result = input.apply_action(&action);

    assert!(!result.handled);
    assert!(result.callback_log.is_empty());
}

#[test]
fn generic_adapter_dispatch_targets_stable_state_id_after_redraw() {
    let initial = Input::new("Search")
        .stable_state_id("generic.search.input")
        .value("src");
    let action = UiAction::input_value(initial.state_id().clone(), "query");
    let mut rebuilt = Input::new("Search")
        .stable_state_id("generic.search.input")
        .value("src");
    let mut other = Input::new("Other")
        .stable_state_id("generic.other.input")
        .value("untouched");

    let accepted = AdapterActionBridge::dispatch(&mut rebuilt, &action);
    let ignored = AdapterActionBridge::dispatch(&mut other, &action);

    assert!(accepted.handled);
    assert!(!ignored.handled);
    assert_eq!("query", rebuilt.state_snapshot().interaction.value);
    assert_eq!("untouched", other.state_snapshot().interaction.value);
}

#[test]
fn generic_adapter_dispatches_closeable_tab_typed_actions() {
    let mut tabs = support::generic_tabs().stable_state_id("generic.tabs");
    let target = tabs.state().state_id.clone();

    let selected =
        AdapterActionBridge::dispatch(&mut tabs, &UiAction::tab_select(target.clone(), "preview"));
    let pinned = AdapterActionBridge::dispatch(
        &mut tabs,
        &UiAction::tab_pin(target.clone(), "editor", true),
    );
    let moved =
        AdapterActionBridge::dispatch(&mut tabs, &UiAction::tab_move(target.clone(), "editor", 0));
    let grouped = AdapterActionBridge::dispatch(
        &mut tabs,
        &UiAction::tab_move_to_group(target.clone(), "preview", "docs"),
    );
    let closed =
        AdapterActionBridge::dispatch(&mut tabs, &UiAction::tab_close(target.clone(), "preview"));

    assert!(selected.handled);
    assert_eq!("preview", selected.after.value);
    assert_eq!("tab_select", selected.callback_log[0].action);
    assert!(pinned.handled);
    assert_eq!("tab_pin", pinned.callback_log[0].action);
    assert!(moved.handled);
    assert_eq!("tab_move", moved.callback_log[0].action);
    assert!(grouped.handled);
    assert_eq!("tab_move_to_group", grouped.callback_log[0].action);
    assert!(closed.handled);
    assert_eq!("tab_close", closed.callback_log[0].action);
}

#[test]
fn generic_adapter_dispatches_closeable_tab_context_bulk_actions() {
    let mut right = support::generic_tabs().stable_state_id("generic.tabs.bulk.right");
    let right_target = right.state().state_id.clone();
    let closed_right = AdapterActionBridge::dispatch(
        &mut right,
        &UiAction::tab_close_to_right(right_target, "editor"),
    );
    let mut others = support::generic_tabs().stable_state_id("generic.tabs.bulk.others");
    let others_target = others.state().state_id.clone();
    let closed_others = AdapterActionBridge::dispatch(
        &mut others,
        &UiAction::tab_close_others(others_target, "editor"),
    );
    let mut left = support::generic_tabs().stable_state_id("generic.tabs.bulk.left");
    let left_target = left.state().state_id.clone();
    let closed_left = AdapterActionBridge::dispatch(
        &mut left,
        &UiAction::tab_close_to_left(left_target, "preview"),
    );
    let mut all = support::generic_tabs().stable_state_id("generic.tabs.bulk.all");
    let all_target = all.state().state_id.clone();
    let closed_all = AdapterActionBridge::dispatch(&mut all, &UiAction::tab_close_all(all_target));

    assert!(closed_right.handled);
    assert_eq!(2, closed_right.after.item_count);
    assert_eq!("tab_close_to_right", closed_right.callback_log[0].action);
    assert!(closed_others.handled);
    assert_eq!(2, closed_others.after.item_count);
    assert_eq!("tab_close_others", closed_others.callback_log[0].action);
    assert!(closed_left.handled);
    assert_eq!(2, closed_left.after.item_count);
    assert_eq!("tab_close_to_left", closed_left.callback_log[0].action);
    assert!(closed_all.handled);
    assert_eq!(1, closed_all.after.item_count);
    assert_eq!("tab_close_all", closed_all.callback_log[0].action);
}

#[test]
fn generic_adapter_dispatches_closeable_tab_add_and_group_actions() {
    let mut tabs = CloseableTabStrip::new("workspace")
        .stable_state_id("generic.tabs.group")
        .group(CloseableTabGroup::new("docs", "Docs"))
        .group(CloseableTabGroup::new("review", "Review"))
        .tab(CloseableTab::new("editor", "Editor").group_id("docs"))
        .tab(CloseableTab::new("preview", "Preview").group_id("review"));
    let target = tabs.state().state_id.clone();

    let added = AdapterActionBridge::dispatch(
        &mut tabs,
        &UiAction::tab_add(target.clone(), "logs", "Logs", true),
    );
    let grouped = AdapterActionBridge::dispatch(
        &mut tabs,
        &UiAction::tab_move_to_new_group(target.clone(), "logs", "notes", "Notes"),
    );
    let collapsed = AdapterActionBridge::dispatch(
        &mut tabs,
        &UiAction::tab_toggle_group_collapse(target.clone(), "notes"),
    );
    let renamed = AdapterActionBridge::dispatch(
        &mut tabs,
        &UiAction::tab_rename_group(target.clone(), "notes", "Reference"),
    );
    let recolored = AdapterActionBridge::dispatch(
        &mut tabs,
        &UiAction::tab_set_group_color(target.clone(), "notes", "#5aa65a"),
    );
    let moved_group = AdapterActionBridge::dispatch(
        &mut tabs,
        &UiAction::tab_move_group(target.clone(), "notes", 0),
    );
    let ungrouped =
        AdapterActionBridge::dispatch(&mut tabs, &UiAction::tab_ungroup(target, "notes"));

    assert!(added.handled);
    assert_eq!("logs", added.after.value);
    assert_eq!("tab_add", added.callback_log[0].action);
    assert!(grouped.handled);
    assert_eq!("tab_move_to_new_group", grouped.callback_log[0].action);
    assert!(collapsed.handled);
    assert_eq!(
        "tab_toggle_group_collapse",
        collapsed.callback_log[0].action
    );
    assert!(renamed.handled);
    assert_eq!("tab_rename_group", renamed.callback_log[0].action);
    assert!(recolored.handled);
    assert_eq!("tab_set_group_color", recolored.callback_log[0].action);
    assert!(moved_group.handled);
    assert_eq!("tab_move_group", moved_group.callback_log[0].action);
    assert!(ungrouped.handled);
    assert_eq!("tab_ungroup", ungrouped.callback_log[0].action);
    assert!(
        tabs.options()
            .groups
            .iter()
            .all(|group| group.id.as_str() != "notes")
    );
}

#[test]
fn generic_adapter_dispatches_closeable_tab_typed_event_log() {
    let mut tabs = CloseableTabStrip::new("workspace")
        .stable_state_id("generic.tabs.events")
        .group(CloseableTabGroup::new("docs", "Docs"))
        .tab(CloseableTab::new("editor", "Editor").group_id("docs"));
    let target = tabs.state().state_id.clone();

    let added = AdapterActionBridge::dispatch(
        &mut tabs,
        &UiAction::tab_add(target.clone(), "logs", "Logs", true),
    );
    let grouped = AdapterActionBridge::dispatch(
        &mut tabs,
        &UiAction::tab_move_to_new_group(target.clone(), "logs", "notes", "Notes"),
    );
    let collapsed = AdapterActionBridge::dispatch(
        &mut tabs,
        &UiAction::tab_toggle_group_collapse(target, "notes"),
    );
    let event_names: Vec<&str> = tabs
        .event_log()
        .iter()
        .map(CloseableTabStripEvent::name)
        .collect();

    assert_eq!(
        vec![
            "closeable_tab_added",
            "closeable_tab_group_created",
            "closeable_tab_group_changed",
            "closeable_tab_group_collapse_changed",
        ],
        event_names
    );
    assert!(added.handled);
    assert!(grouped.handled);
    assert!(collapsed.handled);
}

#[test]
fn generic_adapter_dispatches_closeable_tab_visual_index_selection() {
    let mut tabs = support::generic_tabs().stable_state_id("generic.tabs");
    let target = tabs.state().state_id.clone();

    let result = AdapterActionBridge::dispatch(&mut tabs, &UiAction::set_selected_index(target, 2));

    assert!(result.handled);
    assert_eq!("preview", result.after.value);
    assert_eq!("set_selected_index", result.callback_log[0].action);
}

#[test]
fn generic_app_tabs_support_bulk_context_actions_from_public_api() {
    let mut tabs = support::generic_tabs();

    let closed_right = tabs.apply_action(CloseableTabStripAction::CloseToRight {
        tab_id: CloseableTabId::new("editor"),
    });
    tabs.apply_action(CloseableTabStripAction::AddTab {
        tab: CloseableTab::new("logs", "Logs"),
        activate: true,
    });
    let closed_left = tabs.apply_action(CloseableTabStripAction::CloseToLeft {
        tab_id: CloseableTabId::new("logs"),
    });
    tabs.apply_action(CloseableTabStripAction::AddTab {
        tab: CloseableTab::new("preview", "Preview"),
        activate: true,
    });
    let closed_others = tabs.apply_action(CloseableTabStripAction::CloseOthers {
        tab_id: CloseableTabId::new("preview"),
    });
    let closed_all = tabs.apply_action(CloseableTabStripAction::CloseAll);

    assert_eq!(
        vec![CloseableTabStripEvent::TabClosed {
            tab_id: CloseableTabId::new("preview")
        }],
        closed_right
    );
    assert_eq!(1, closed_left.len());
    assert_eq!(1, closed_others.len());
    assert_eq!(1, closed_all.len());
    assert_eq!(
        Some(&CloseableTabId::new("home")),
        tabs.state().active_tab_id.as_ref()
    );
}

#[test]
fn generic_app_tabs_keep_endpoint_close_noops_and_non_closeable_tabs() {
    let mut right_tabs = CloseableTabStrip::new("Workspace")
        .tab(CloseableTab::new("home", "Home"))
        .tab(CloseableTab::new("editor", "Editor"));
    let right_events = right_tabs.apply_action(CloseableTabStripAction::CloseToRight {
        tab_id: CloseableTabId::new("editor"),
    });

    let mut left_tabs = CloseableTabStrip::new("Workspace")
        .tab(CloseableTab::new("home", "Home"))
        .tab(CloseableTab::new("editor", "Editor"));
    let left_events = left_tabs.apply_action(CloseableTabStripAction::CloseToLeft {
        tab_id: CloseableTabId::new("home"),
    });

    let mut protected_tabs = CloseableTabStrip::new("Workspace")
        .tab(CloseableTab::new("home", "Home").pinned(true))
        .tab(CloseableTab::new("fixed", "Fixed").closeable(false))
        .tab(CloseableTab::new("editor", "Editor"));
    let close_all_events = protected_tabs.apply_action(CloseableTabStripAction::CloseAll);
    let remaining_ids: Vec<&str> = protected_tabs
        .options()
        .tabs
        .iter()
        .map(|tab| tab.id.as_str())
        .collect();

    assert!(right_events.is_empty());
    assert!(left_events.is_empty());
    assert_eq!(2, right_tabs.options().tabs.len());
    assert_eq!(2, left_tabs.options().tabs.len());
    assert_eq!(vec!["home", "fixed"], remaining_ids);
    assert_eq!(
        vec![CloseableTabStripEvent::TabClosed {
            tab_id: CloseableTabId::new("editor")
        }],
        close_all_events
    );
}

#[test]
fn generic_app_tabs_context_commands_map_to_typed_actions() {
    let tab_id = CloseableTabId::new("editor");
    let menu = CloseableTabContextMenu::menu(
        "Tab menu",
        ContextMenuAnchor::NodeId("editor-tab".to_string()),
        vec![ContextMenuItem::action("split-right", "Split Right")],
    );
    let menu_node = katana_ui_core::render_model::UiNode::from(menu);

    assert_eq!(
        Some(CloseableTabStripAction::CloseOthers {
            tab_id: tab_id.clone()
        }),
        CloseableTabContextCommand::CloseOthers.to_tab_action(tab_id.clone())
    );
    assert_eq!(
        Some(CloseableTabStripAction::PinTab {
            tab_id: tab_id.clone()
        }),
        CloseableTabContextCommand::Pin.to_tab_action(tab_id.clone())
    );
    assert_eq!(
        None,
        CloseableTabContextCommand::MoveToNewGroup.to_tab_action(tab_id.clone())
    );
    assert_eq!(
        CloseableTabStripAction::MoveToGroup {
            tab_id: tab_id.clone(),
            target: CloseableTabGroupTarget::Existing(CloseableTabGroupId::new("docs")),
        },
        CloseableTabContextCommand::move_to_existing_group_action(
            tab_id.clone(),
            CloseableTabGroupId::new("docs"),
        )
    );
    assert_eq!(
        CloseableTabStripAction::MoveToGroup {
            tab_id,
            target: CloseableTabGroupTarget::NewGroup(CloseableTabGroup::new("notes", "Notes")),
        },
        CloseableTabContextCommand::move_to_new_group_action(
            CloseableTabId::new("editor"),
            CloseableTabGroup::new("notes", "Notes"),
        )
    );
    let group_item_id =
        CloseableTabContextCommand::move_to_group_item_id(&CloseableTabGroupId::new("docs"));
    let tab = CloseableTab::new("editor", "Editor");
    let group = CloseableTabGroup::new("docs", "Docs");
    let tab_menu = CloseableTabContextMenu::tab_menu(
        "Tab menu",
        &tab,
        std::slice::from_ref(&group),
        ContextMenuAnchor::NodeId("editor-tab".to_string()),
    );
    let tab_menu_node = katana_ui_core::render_model::UiNode::from(tab_menu);
    assert_eq!("move-to-group:docs", group_item_id);
    assert_eq!(
        Some(CloseableTabGroupId::new("docs")),
        CloseableTabContextCommand::move_to_group_id_from_item_id(group_item_id.as_str())
    );
    assert_eq!(
        ContextMenuItemKind::Submenu,
        tab_menu_node.props().context_menu.items[6].kind
    );
    assert_eq!(
        "move-to-new-group",
        tab_menu_node.props().context_menu.items[6].children[0].id
    );
    assert_eq!(
        "move-to-group:docs",
        tab_menu_node.props().context_menu.items[6].children[1].id
    );
    assert_eq!(
        Some(CloseableTabContextCommand::CloseToRight),
        CloseableTabContextCommand::from_id("close-to-right")
    );
    assert_eq!(
        Some(CloseableTabStripAction::RestoreClosedTab),
        CloseableTabContextCommand::RestoreClosed.to_tab_action(CloseableTabId::new("editor"))
    );
    assert_eq!(
        Some(CloseableTabGroupContextCommand::Collapse),
        CloseableTabGroupContextCommand::from_id("collapse")
    );
    assert_eq!(
        Some(CloseableTabStripAction::ToggleGroupCollapse {
            group_id: CloseableTabGroupId::new("docs")
        }),
        CloseableTabGroupContextCommand::Collapse
            .to_group_action(&CloseableTabGroup::new("docs", "Docs"))
    );
    assert_eq!(
        CloseableTabStripAction::MoveGroup {
            group_id: CloseableTabGroupId::new("docs"),
            to_index: 1
        },
        CloseableTabGroupContextCommand::move_group_action(CloseableTabGroupId::new("docs"), 1)
    );
    assert_eq!(
        CloseableTabStripAction::RenameGroup {
            group_id: CloseableTabGroupId::new("docs"),
            label: "Reference".to_string()
        },
        CloseableTabGroupContextCommand::rename_group_action(
            CloseableTabGroupId::new("docs"),
            "Reference",
        )
    );
    assert_eq!(
        CloseableTabStripAction::SetGroupColor {
            group_id: CloseableTabGroupId::new("docs"),
            color: "#5aa65a".to_string()
        },
        CloseableTabGroupContextCommand::set_group_color_action(
            CloseableTabGroupId::new("docs"),
            "#5aa65a",
        )
    );
    assert_eq!("split-right", menu_node.props().context_menu.items[0].id);
}

#[test]
fn generic_app_tabs_emit_typed_events_for_pin_and_group_changes() {
    let mut tabs = support::generic_tabs();

    let pinned = tabs.apply_action(CloseableTabStripAction::PinTab {
        tab_id: CloseableTabId::new("editor"),
    });
    let pinned_group_rejected = tabs.apply_action(CloseableTabStripAction::MoveToGroup {
        tab_id: CloseableTabId::new("editor"),
        target: CloseableTabGroupTarget::NewGroup(CloseableTabGroup::new("blocked", "Blocked")),
    });
    let grouped = tabs.apply_action(CloseableTabStripAction::MoveToGroup {
        tab_id: CloseableTabId::new("preview"),
        target: CloseableTabGroupTarget::NewGroup(CloseableTabGroup::new("notes", "Notes")),
    });

    assert_eq!(
        vec![
            CloseableTabStripEvent::TabPinChanged {
                tab_id: CloseableTabId::new("editor"),
                pinned: true
            },
            CloseableTabStripEvent::TabGroupChanged {
                tab_id: CloseableTabId::new("editor"),
                group_id: None
            }
        ],
        pinned
    );
    assert!(pinned_group_rejected.is_empty());
    assert_eq!(
        vec![
            CloseableTabStripEvent::GroupCreated {
                group_id: CloseableTabGroupId::new("notes")
            },
            CloseableTabStripEvent::TabGroupChanged {
                tab_id: CloseableTabId::new("preview"),
                group_id: Some(CloseableTabGroupId::new("notes"))
            }
        ],
        grouped
    );
}
