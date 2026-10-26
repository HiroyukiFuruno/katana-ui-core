#![cfg(feature = "egui")]

#[path = "support/tab_host_effects.rs"]
mod tab_host_effects;

use std::sync::{Arc, Mutex};

use katana_ui_core::atom::TextArea;
use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfacePresentation, EguiTextCommandSurfaceRootFactory,
    TabStripContextMenuPresentation, TabStripCorrelation, TabStripGroupCapabilities,
    TabStripGroupDescriptor, TabStripGroupPopupPresentation, TabStripGroupTarget,
    TabStripMenuEntry, TabStripMenuOperation, TabStripProjection, TabStripProjectionLease,
    TabStripProposal, TabStripProposalPort, TabStripProposalPortError, TabStripTabCapabilities,
    TabStripTabDescriptor, TabStripTabTarget, TabStripText, TextCommandSurfaceStyle,
};
use katana_ui_core::render_model::{UiStateId, UiTextSpan};
use katana_ui_core::text_surface::{
    TextSurface, TextSurfacePresentation, TextSurfaceProps, TextSurfaceViewport,
};
use tab_host_effects::{
    HostPort, button_bounds, button_bounds_all, frame, input, lease_for, lease_with_port,
    projection, projection_with_draggable_tabs, projection_with_group_actions,
    projection_with_group_menu, projection_with_popup, projection_with_popup_named, root_with_host,
    root_with_port,
};

#[test]
fn public_root_actions_update_host_state_for_distinct_opaque_same_label_tabs() {
    let tab_a = TabStripTabTarget::from_opaque_bytes(b"tab-a");
    let tab_b = TabStripTabTarget::from_opaque_bytes(b"tab-b");
    let group = TabStripGroupTarget::from_opaque_bytes(b"group");
    let host = Arc::new(Mutex::new(tab_host_effects::HostState::default()));
    let mut root = root_with_host(
        projection(1, tab_a.clone(), tab_b.clone(), group),
        Arc::clone(&host),
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    let same_label_nodes = initial
        .platform_output
        .accesskit_update
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .filter(|(_, node)| {
            node.label() == Some("同じ名前") && node.role() == egui::accesskit::Role::Button
        })
        .count();
    assert_eq!(same_label_nodes, 2);

    let target = button_bounds(&initial, "同じ名前");
    let point = target.center();
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(point),
            egui::Event::PointerButton {
                pos: point,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
        ],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        }],
    );
    let state = host.lock().unwrap();
    assert!(state.selected == Some(tab_a) || state.selected == Some(tab_b));
    assert_eq!(state.operations, 1);
}

#[test]
fn public_root_close_control_updates_host_state_for_the_second_same_label_tab() {
    let tab_a = TabStripTabTarget::from_opaque_bytes(b"tab-a-close");
    let tab_b = TabStripTabTarget::from_opaque_bytes(b"tab-b-close");
    let group = TabStripGroupTarget::from_opaque_bytes(b"group-close");
    let host = Arc::new(Mutex::new(tab_host_effects::HostState::default()));
    host.lock().unwrap().tab_order = vec![
        TabStripTabTarget::from_opaque_bytes(b"tab-a-close"),
        tab_b.clone(),
    ];
    let mut root = root_with_host(
        projection(1, tab_a, tab_b.clone(), group),
        Arc::clone(&host),
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    let close_buttons = button_bounds_all(&initial, "閉じる");
    assert_eq!(close_buttons.len(), 2);
    let point = close_buttons[1].center();
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(point),
            egui::Event::PointerButton {
                pos: point,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: point,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ],
    );
    let state = host.lock().unwrap();
    assert_eq!(state.closed, vec![tab_b]);
    assert_eq!(
        state.tab_order,
        vec![TabStripTabTarget::from_opaque_bytes(b"tab-a-close")]
    );
    assert_eq!(state.operations, 1);
}

fn click_menu_action(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
    tab_bounds: egui::Rect,
    action_label: &str,
) {
    let point = tab_bounds.center();
    frame(
        context,
        root,
        vec![
            egui::Event::PointerMoved(point),
            egui::Event::PointerButton {
                pos: point,
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
        ],
    );
    frame(
        context,
        root,
        vec![egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Secondary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        }],
    );
    let menu = frame(context, root, Vec::new());
    let action = button_bounds(&menu, action_label).center();
    frame(
        context,
        root,
        vec![
            egui::Event::PointerMoved(action),
            egui::Event::PointerButton {
                pos: action,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: action,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ],
    );
}

#[test]
fn public_root_menu_pin_group_and_remove_actions_update_host_state_stepwise() {
    let tab = TabStripTabTarget::from_opaque_bytes(b"menu-tab");
    let group = TabStripGroupTarget::from_opaque_bytes(b"menu-group");
    let host = Arc::new(Mutex::new(tab_host_effects::HostState::default()));
    let mut root = root_with_host(
        projection_with_group_menu(1, tab.clone(), group.clone()),
        Arc::clone(&host),
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    let tab_bounds = button_bounds(&initial, "Menu target");

    click_menu_action(&context, &mut root, tab_bounds, "Pin");
    assert_eq!(host.lock().unwrap().pinned, vec![tab.clone()]);
    click_menu_action(&context, &mut root, tab_bounds, "Unpin");
    assert!(host.lock().unwrap().pinned.is_empty());
    click_menu_action(&context, &mut root, tab_bounds, "Create group");
    assert_eq!(host.lock().unwrap().created_groups, vec![tab.clone()]);
    click_menu_action(&context, &mut root, tab_bounds, "Move to group");
    assert_eq!(
        host.lock().unwrap().moved_tabs,
        vec![(tab.clone(), group.clone())]
    );
    assert_eq!(
        host.lock().unwrap().group_membership,
        vec![(tab.clone(), group.clone())]
    );
    click_menu_action(&context, &mut root, tab_bounds, "Remove from group");
    assert_eq!(host.lock().unwrap().removed_tabs, vec![tab]);
    assert!(host.lock().unwrap().group_membership.is_empty());
}

#[test]
fn public_root_rehomes_open_tab_menu_by_opaque_target_after_reorder() {
    let tab_a = TabStripTabTarget::from_opaque_bytes(b"menu-reorder-a");
    let tab_b = TabStripTabTarget::from_opaque_bytes(b"menu-reorder-b");
    let group = TabStripGroupTarget::from_opaque_bytes(b"menu-reorder-group");
    let host = Arc::new(Mutex::new(tab_host_effects::HostState::default()));
    let menu = || {
        TabStripContextMenuPresentation::new().entry(TabStripMenuEntry::action(
            TabStripText::new("Pin"),
            TabStripText::new("Pin"),
            TabStripMenuOperation::SetPinned(true),
        ))
    };
    let projection = |revision, first, second| {
        TabStripProjection::new(
            revision,
            TabStripCorrelation::from_opaque_bytes(b"menu-reorder"),
        )
        .tab(TabStripTabDescriptor::new(first, TabStripText::new("同じ名前")).context_menu(menu()))
        .tab(TabStripTabDescriptor::new(second, TabStripText::new("同じ名前")).context_menu(menu()))
        .group(TabStripGroupDescriptor::new(
            group.clone(),
            TabStripText::new("group"),
        ))
    };
    let mut root = root_with_host(
        projection(1, tab_a.clone(), tab_b.clone()),
        Arc::clone(&host),
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    let mut initial_tabs = button_bounds_all(&initial, "同じ名前");
    assert_eq!(initial_tabs.len(), 2);
    initial_tabs.sort_by(|left, right| left.min.x.total_cmp(&right.min.x));
    let tab = initial_tabs[0];
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(tab.center()),
            egui::Event::PointerButton {
                pos: tab.center(),
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
        ],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: tab.center(),
            button: egui::PointerButton::Secondary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        }],
    );
    let opened = frame(&context, &mut root, Vec::new());
    let _opened_pin = button_bounds(&opened, "Pin");

    root.synchronize_with_lease(lease_for(
        2,
        projection(2, tab_b.clone(), tab_a.clone()),
        HostPort(Arc::clone(&host)),
    ))
    .expect("reordered lease should synchronize");
    let reordered = frame(&context, &mut root, Vec::new());
    let pin = button_bounds(&reordered, "Pin");
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(pin.center()),
            egui::Event::PointerButton {
                pos: pin.center(),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: pin.center(),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ],
    );
    assert_eq!(host.lock().unwrap().pinned, vec![tab_a.clone()]);

    let after_pin = frame(&context, &mut root, Vec::new());
    let mut reordered_tabs = button_bounds_all(&after_pin, "同じ名前");
    assert_eq!(reordered_tabs.len(), 2);
    reordered_tabs.sort_by(|left, right| left.min.x.total_cmp(&right.min.x));
    let reordered_a = reordered_tabs[1];
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(reordered_a.center()),
            egui::Event::PointerButton {
                pos: reordered_a.center(),
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
        ],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: reordered_a.center(),
            button: egui::PointerButton::Secondary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        }],
    );
    let reopened = frame(&context, &mut root, Vec::new());
    let old_pin = button_bounds(&reopened, "Pin");
    let operations_before_remove = host.lock().unwrap().operations;

    let removed_target_projection =
        TabStripProjection::new(3, TabStripCorrelation::from_opaque_bytes(b"menu-reorder"))
            .tab(
                TabStripTabDescriptor::new(tab_b, TabStripText::new("同じ名前"))
                    .context_menu(menu()),
            )
            .group(TabStripGroupDescriptor::new(
                group,
                TabStripText::new("group"),
            ));
    root.synchronize_with_lease(lease_for(
        3,
        removed_target_projection,
        HostPort(Arc::clone(&host)),
    ))
    .expect("removed target lease should synchronize");
    let closed = frame(&context, &mut root, Vec::new());
    assert!(button_bounds_all(&closed, "Pin").is_empty());
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(old_pin.center()),
            egui::Event::PointerButton {
                pos: old_pin.center(),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: old_pin.center(),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ],
    );
    let host = host.lock().unwrap();
    assert_eq!(host.operations, operations_before_remove);
    assert_eq!(host.pinned, vec![tab_a]);
}

#[test]
fn public_root_group_collapse_and_expand_survive_host_reprojection() {
    let group = TabStripGroupTarget::from_opaque_bytes(b"collapse-group");
    let host = Arc::new(Mutex::new(tab_host_effects::HostState::default()));
    let mut root = root_with_host(
        projection_with_popup(1, group.clone(), false, false),
        Arc::clone(&host),
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let first = frame(&context, &mut root, Vec::new());
    let point = button_bounds(&first, "グループ").center();
    let click = |point| {
        vec![
            egui::Event::PointerMoved(point),
            egui::Event::PointerButton {
                pos: point,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: point,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ]
    };
    frame(&context, &mut root, click(point));
    assert_eq!(
        host.lock().unwrap().collapsed_groups,
        vec![(group.clone(), true)]
    );
    root.synchronize_with_lease(lease_for(
        2,
        projection_with_popup(2, group.clone(), true, false),
        HostPort(Arc::clone(&host)),
    ))
    .unwrap();
    let collapsed = frame(&context, &mut root, Vec::new());
    frame(
        &context,
        &mut root,
        click(button_bounds(&collapsed, "グループ").center()),
    );
    assert_eq!(host.lock().unwrap().collapsed_groups, vec![(group, false)]);
}

#[test]
fn public_root_rename_editor_closes_when_lease_revokes_rename_contract() {
    let group = TabStripGroupTarget::from_opaque_bytes(b"rename-revoked");
    for (revision, renamable, with_placeholder) in [(2, false, true), (3, true, false)] {
        let host = Arc::new(Mutex::new(tab_host_effects::HostState::default()));
        let mut root = root_with_host(
            projection_with_popup(1, group.clone(), false, true),
            Arc::clone(&host),
        );
        let context = egui::Context::default();
        context.enable_accesskit();
        let initial = frame(&context, &mut root, Vec::new());
        assert!(button_bounds(&initial, "グループ名").is_positive());
        frame(&context, &mut root, Vec::new());
        frame(
            &context,
            &mut root,
            vec![egui::Event::Text("draft".to_owned())],
        );

        let popup = if with_placeholder {
            TabStripGroupPopupPresentation::new().rename_placeholder(TabStripText::new("Rename"))
        } else {
            TabStripGroupPopupPresentation::new()
        };
        let revoked = TabStripProjection::new(
            revision,
            TabStripCorrelation::from_opaque_bytes(b"rename-revoked"),
        )
        .group(
            TabStripGroupDescriptor::new(group.clone(), TabStripText::new("グループ"))
                .capabilities(TabStripGroupCapabilities::new().renamable(renamable))
                .popup(popup),
        );
        root.synchronize_with_lease(lease_for(revision, revoked, HostPort(Arc::clone(&host))))
            .unwrap();

        let mut result = None;
        let mut output = context.run_ui(
            input(vec![egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }]),
            |ui| result = Some(root.show(ui)),
        );
        output.textures_delta.clear();
        let failure = result
            .expect("root show result")
            .err()
            .map(|error| error.to_string());
        assert!(
            failure.is_none(),
            "root render failed after rename revocation: {failure:?}"
        );
        assert!(host.lock().unwrap().renamed_groups.is_empty());
    }
}

#[test]
fn public_root_inline_rename_escape_and_swatch_update_host_state() {
    let group = TabStripGroupTarget::from_opaque_bytes(b"rename-group");
    let host = Arc::new(Mutex::new(tab_host_effects::HostState::default()));
    let mut root = root_with_host(
        projection_with_popup(1, group.clone(), false, true),
        Arc::clone(&host),
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    assert!(button_bounds(&initial, "グループ名").is_positive());
    frame(&context, &mut root, Vec::new());
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::A,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND,
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::A,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND,
        }],
    );
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::Text("日本語⭐️".to_owned()),
            egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    assert_eq!(
        host.lock().unwrap().renamed_groups,
        vec![(group.clone(), "日本語⭐️".to_owned())]
    );
    root.synchronize_with_lease(lease_for(
        2,
        projection_with_popup_named(2, group.clone(), false, false, "日本語⭐️"),
        HostPort(Arc::clone(&host)),
    ))
    .unwrap();
    let renamed = frame(&context, &mut root, Vec::new());
    assert!(button_bounds(&renamed, "日本語⭐️").is_positive());

    root.synchronize_with_lease(lease_for(
        3,
        projection_with_popup_named(3, group.clone(), false, true, "日本語⭐️"),
        HostPort(Arc::clone(&host)),
    ))
    .unwrap();
    frame(&context, &mut root, Vec::new());
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    assert_eq!(host.lock().unwrap().renamed_groups.len(), 1);

    root.synchronize_with_lease(lease_for(
        4,
        projection_with_popup_named(4, group.clone(), false, false, "日本語⭐️"),
        HostPort(Arc::clone(&host)),
    ))
    .unwrap();
    let group_frame = frame(&context, &mut root, Vec::new());
    let group_point = button_bounds(&group_frame, "日本語⭐️").center();
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(group_point),
            egui::Event::PointerButton {
                pos: group_point,
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: group_point,
            button: egui::PointerButton::Secondary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    let popup = frame(&context, &mut root, Vec::new());
    let swatch = button_bounds(&popup, "青").center();
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: swatch,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: swatch,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    assert_eq!(host.lock().unwrap().recolored_groups.len(), 1);
}

#[test]
fn public_root_ungroup_menu_removes_real_group_membership() {
    let group = TabStripGroupTarget::from_opaque_bytes(b"ungroup-group");
    let member = TabStripTabTarget::from_opaque_bytes(b"ungroup-member");
    let host = Arc::new(Mutex::new(tab_host_effects::HostState::default()));
    host.lock().unwrap().group_order.push(group.clone());
    host.lock()
        .unwrap()
        .group_membership
        .push((member, group.clone()));
    let mut root = root_with_host(
        projection_with_group_actions(
            1,
            group.clone(),
            TabStripTabTarget::from_opaque_bytes(b"unused"),
        ),
        Arc::clone(&host),
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    click_menu_action(
        &context,
        &mut root,
        button_bounds(&initial, "操作グループ"),
        "Ungroup",
    );
    let state = host.lock().unwrap();
    assert_eq!(state.ungrouped, vec![group]);
    assert!(state.group_membership.is_empty());
}

#[test]
fn public_root_close_group_menu_closes_real_member_tab() {
    let group = TabStripGroupTarget::from_opaque_bytes(b"close-group");
    let member = TabStripTabTarget::from_opaque_bytes(b"close-member");
    let host = Arc::new(Mutex::new(tab_host_effects::HostState::default()));
    host.lock().unwrap().group_order.push(group.clone());
    host.lock().unwrap().tab_order = vec![
        member.clone(),
        TabStripTabTarget::from_opaque_bytes(b"unrelated"),
    ];
    host.lock()
        .unwrap()
        .group_membership
        .push((member.clone(), group.clone()));
    let mut root = root_with_host(
        projection_with_group_actions(1, group.clone(), member.clone()),
        Arc::clone(&host),
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    click_menu_action(
        &context,
        &mut root,
        button_bounds(&initial, "操作グループ"),
        "Close group",
    );
    let state = host.lock().unwrap();
    assert_eq!(state.closed_groups, vec![group]);
    assert_eq!(state.closed, vec![member]);
    assert_eq!(
        state.tab_order,
        vec![TabStripTabTarget::from_opaque_bytes(b"unrelated")]
    );
    assert!(state.group_order.is_empty());
    assert!(state.group_membership.is_empty());
}

#[test]
fn public_root_tab_drag_updates_host_order_once_and_reprojects_bounds() {
    let first = TabStripTabTarget::from_opaque_bytes(b"drag-first");
    let second = TabStripTabTarget::from_opaque_bytes(b"drag-second");
    let host = Arc::new(Mutex::new(tab_host_effects::HostState::default()));
    host.lock().unwrap().tab_order = vec![first.clone(), second.clone()];
    let mut root = root_with_host(
        projection_with_draggable_tabs(1, first.clone(), second.clone()),
        Arc::clone(&host),
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    let source = button_bounds(&initial, "移動先").center();
    let destination = button_bounds(&initial, "ドラッグタブ");
    let drop = egui::pos2(destination.min.x + 1.0, destination.center().y);
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(source),
            egui::Event::PointerButton {
                pos: source,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    frame(&context, &mut root, vec![egui::Event::PointerMoved(drop)]);
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: drop,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    assert_eq!(
        host.lock().unwrap().tab_order,
        vec![second.clone(), first.clone()]
    );
    assert_eq!(host.lock().unwrap().operations, 2);
    root.synchronize_with_lease(lease_for(
        2,
        projection_with_draggable_tabs(2, second.clone(), first.clone()),
        HostPort(Arc::clone(&host)),
    ))
    .unwrap();
    let reordered = frame(&context, &mut root, Vec::new());
    let reordered_destination = button_bounds(&reordered, "移動先");
    let reordered_source = button_bounds(&reordered, "ドラッグタブ");
    assert!(
        reordered_destination.min.x < reordered_source.min.x,
        "destination={reordered_destination:?} source={reordered_source:?}"
    );
}

#[test]
fn public_root_removed_tab_drag_clears_before_release_and_allows_next_drag() {
    assert_invalid_tab_drag_clears_and_allows_next_drag(false);
    assert_invalid_tab_drag_clears_and_allows_next_drag(true);
}

fn assert_invalid_tab_drag_clears_and_allows_next_drag(disable_source: bool) {
    let source = TabStripTabTarget::from_opaque_bytes(b"drag-removed-source");
    let destination = TabStripTabTarget::from_opaque_bytes(b"drag-removed-destination");
    let replacement = TabStripTabTarget::from_opaque_bytes(b"drag-replacement");
    let host = Arc::new(Mutex::new(tab_host_effects::HostState {
        tab_order: vec![source.clone(), destination.clone()],
        ..Default::default()
    }));
    let draggable = |target: TabStripTabTarget, label: &str| {
        TabStripTabDescriptor::new(target, TabStripText::new(label)).capabilities(
            TabStripTabCapabilities::new()
                .draggable(true)
                .accepts_tab_drop(true),
        )
    };
    let mut root = root_with_host(
        TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"drag-remove"))
            .tab(draggable(source.clone(), "source"))
            .tab(draggable(destination.clone(), "destination")),
        Arc::clone(&host),
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    let source_bounds = button_bounds(&initial, "source");
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(source_bounds.center()),
            egui::Event::PointerButton {
                pos: source_bounds.center(),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(egui::pos2(
            source_bounds.center().x + 10.0,
            source_bounds.center().y,
        ))],
    );
    assert_eq!(host.lock().unwrap().operations, 1);

    let mut next_projection =
        TabStripProjection::new(2, TabStripCorrelation::from_opaque_bytes(b"drag-remove"))
            .tab(draggable(destination.clone(), "destination"));
    if disable_source {
        next_projection = next_projection.tab(TabStripTabDescriptor::new(
            source.clone(),
            TabStripText::new("source"),
        ));
    }
    root.synchronize_with_lease(lease_for(2, next_projection, HostPort(Arc::clone(&host))))
        .unwrap();
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: source_bounds.center(),
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    assert_eq!(host.lock().unwrap().operations, 2);

    host.lock().unwrap().tab_order = vec![replacement.clone(), destination.clone()];
    root.synchronize_with_lease(lease_for(
        3,
        TabStripProjection::new(3, TabStripCorrelation::from_opaque_bytes(b"drag-remove"))
            .tab(draggable(replacement.clone(), "replacement"))
            .tab(draggable(destination.clone(), "destination")),
        HostPort(Arc::clone(&host)),
    ))
    .unwrap();
    let next = frame(&context, &mut root, Vec::new());
    let replacement_bounds = button_bounds(&next, "replacement");
    let destination_bounds = button_bounds(&next, "destination");
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(replacement_bounds.center()),
            egui::Event::PointerButton {
                pos: replacement_bounds.center(),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    let drop = egui::pos2(
        destination_bounds.max.x - 1.0,
        destination_bounds.center().y,
    );
    frame(&context, &mut root, vec![egui::Event::PointerMoved(drop)]);
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: drop,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    let host = host.lock().unwrap();
    assert_eq!(host.operations, 4);
    assert!(host.active_drag.is_none());
    assert_eq!(host.tab_order, vec![destination, replacement]);
}

fn ax_button_id(output: &egui::FullOutput, label: &str) -> egui::accesskit::NodeId {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .find_map(|(id, node)| {
            (node.role() == egui::accesskit::Role::Button && node.label() == Some(label))
                .then_some(*id)
        })
        .expect("AX button")
}

fn ax_activate(target_node: egui::accesskit::NodeId) -> Vec<egui::Event> {
    vec![egui::Event::AccessKitActionRequest(
        egui::accesskit::ActionRequest {
            target_tree: egui::accesskit::TreeId::ROOT,
            action: egui::accesskit::Action::Click,
            target_node,
            data: None,
        },
    )]
}

#[test]
fn public_root_expired_selectable_target_ignores_old_ax_click() {
    let tab = TabStripTabTarget::from_opaque_bytes(b"expired-tab");
    let correlation = TabStripCorrelation::from_opaque_bytes(b"expired-correlation");
    let host = Arc::new(Mutex::new(tab_host_effects::HostState {
        tab_order: vec![tab.clone()],
        ..Default::default()
    }));
    let mut root = root_with_host(
        tab_host_effects::projection_with_selectable(1, correlation.clone(), tab, true),
        Arc::clone(&host),
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    let old_id = ax_button_id(&initial, "失効対象");
    root.synchronize_with_lease(lease_for(
        2,
        tab_host_effects::projection_with_selectable(
            2,
            correlation,
            TabStripTabTarget::from_opaque_bytes(b"expired-tab"),
            false,
        ),
        HostPort(Arc::clone(&host)),
    ))
    .unwrap();
    let expired = frame(&context, &mut root, Vec::new());
    let node = expired
        .platform_output
        .accesskit_update
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::Label && node.label() == Some("失効対象")
        })
        .map(|(_, node)| node)
        .expect("passive label");
    assert!(!node.supports_action(egui::accesskit::Action::Click));
    frame(&context, &mut root, ax_activate(old_id));
    let state = host.lock().unwrap();
    assert!(state.selected.is_none());
    assert_eq!(
        state.tab_order,
        vec![TabStripTabTarget::from_opaque_bytes(b"expired-tab")]
    );
    assert_eq!(state.operations, 0);
}

#[derive(Clone)]
struct CorrelatingPort {
    state: Arc<Mutex<tab_host_effects::HostState>>,
    accepted: Arc<Mutex<TabStripCorrelation>>,
    rejections: Arc<std::sync::atomic::AtomicUsize>,
}

impl TabStripProposalPort for CorrelatingPort {
    fn forward_proposal(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        if proposal.correlation() != *self.accepted.lock().unwrap() {
            self.rejections
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return Err(TabStripProposalPortError::Rejected);
        }
        let mut port = HostPort(Arc::clone(&self.state));
        port.forward_proposal(proposal)
    }
}

#[derive(Clone)]
struct RejectOncePort {
    state: Arc<Mutex<tab_host_effects::HostState>>,
    reject_next: Arc<std::sync::atomic::AtomicBool>,
    router_markers: Arc<Mutex<Vec<TabStripCorrelation>>>,
}

impl TabStripProposalPort for RejectOncePort {
    fn forward_proposal(
        &mut self,
        proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        if self
            .reject_next
            .swap(false, std::sync::atomic::Ordering::AcqRel)
        {
            return Err(TabStripProposalPortError::Rejected);
        }
        self.router_markers
            .lock()
            .unwrap()
            .push(proposal.correlation());
        let mut port = HostPort(Arc::clone(&self.state));
        port.forward_proposal(proposal)
    }
}

fn token_with_body(
    revision: u64,
    body: &str,
) -> katana_ui_core::egui::text_command_surface::EguiTextCommandSurfacePresentationToken {
    let surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("cancel-body").value(body),
        Vec::<UiTextSpan>::new(),
        TextSurfaceViewport::new(0, 0, 1280, 720),
    ));
    EguiTextCommandSurfaceHostProjectionEncoder::token(
        revision,
        b"cancel-body",
        EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("cancel-body-text")),
            text: TextSurfacePresentation::from_props(surface.props()),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
        },
        TextCommandSurfaceStyle::standard().unwrap(),
    )
    .unwrap()
}

fn lease_with_body(
    revision: u64,
    body: &str,
    projection: TabStripProjection,
    port: RejectOncePort,
    root_router: Arc<Mutex<String>>,
    router_marker: &str,
) -> EguiTextCommandSurfaceHostProjectionLease {
    let router_marker = router_marker.to_owned();
    EguiTextCommandSurfaceHostProjectionLease::new(
        token_with_body(revision, body),
        move |_context| {
            *root_router.lock().unwrap() = router_marker.clone();
            Ok(None)
        },
    )
    .with_tab_strip(TabStripProjectionLease::new(projection).with_proposal_port(port))
}

fn frame_with_record(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
    events: Vec<egui::Event>,
) -> (
    katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRootFrame,
    egui::FullOutput,
) {
    let mut shown = None;
    let mut output = context.run_ui(tab_host_effects::input(events), |ui| {
        shown = Some(root.show(ui).expect("host root frame"));
    });
    output.textures_delta.clear();
    (shown.expect("host root frame"), output)
}

fn text_input_value(output: &egui::FullOutput) -> Option<String> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update
                .nodes
                .iter()
                .find(|(_, node)| node.role() == egui::accesskit::Role::MultilineTextInput)
                .and_then(|(_, node)| node.value().map(str::to_owned))
        })
}

fn draggable_projection_with_correlation(
    revision: u64,
    correlation: TabStripCorrelation,
    first: TabStripTabTarget,
    second: TabStripTabTarget,
) -> TabStripProjection {
    let first_is_drag = first == TabStripTabTarget::from_opaque_bytes(b"drag-first");
    let first_label = if first_is_drag {
        "ドラッグタブ"
    } else {
        "移動先"
    };
    let second_label = if first_is_drag {
        "移動先"
    } else {
        "ドラッグタブ"
    };
    let tab = |target, label| {
        TabStripTabDescriptor::new(target, TabStripText::new(label)).capabilities(
            TabStripTabCapabilities::new()
                .selectable(true)
                .draggable(true)
                .accepts_tab_drop(true),
        )
    };
    TabStripProjection::new(revision, correlation)
        .tab(tab(first, first_label))
        .tab(tab(second, second_label))
}

#[test]
fn public_root_rejecting_drag_cancel_keeps_old_frame_until_retry_succeeds() {
    let source = TabStripTabTarget::from_opaque_bytes(b"cancel-source");
    let destination = TabStripTabTarget::from_opaque_bytes(b"cancel-destination");
    let host = Arc::new(Mutex::new(tab_host_effects::HostState {
        tab_order: vec![source.clone(), destination.clone()],
        ..Default::default()
    }));
    let reject_next = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let router_markers = Arc::new(Mutex::new(Vec::new()));
    let active_root_router = Arc::new(Mutex::new(String::new()));
    let port = || RejectOncePort {
        state: Arc::clone(&host),
        reject_next: Arc::clone(&reject_next),
        router_markers: Arc::clone(&router_markers),
    };
    let projection = |revision, correlation, source, destination| {
        TabStripProjection::new(revision, correlation)
            .tab(
                TabStripTabDescriptor::new(source, TabStripText::new("旧本文")).capabilities(
                    TabStripTabCapabilities::new()
                        .selectable(true)
                        .draggable(true),
                ),
            )
            .tab(
                TabStripTabDescriptor::new(destination, TabStripText::new("旧相手")).capabilities(
                    TabStripTabCapabilities::new()
                        .selectable(true)
                        .draggable(true),
                ),
            )
    };
    let old_correlation = TabStripCorrelation::from_opaque_bytes(b"cancel-old");
    let new_correlation = TabStripCorrelation::from_opaque_bytes(b"cancel-new");
    let mut root = EguiTextCommandSurfaceRootFactory::new()
        .retain_with_lease(lease_with_body(
            1,
            "old body",
            projection(
                1,
                old_correlation.clone(),
                source.clone(),
                destination.clone(),
            ),
            port(),
            Arc::clone(&active_root_router),
            "old router",
        ))
        .unwrap();
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    let source_bounds = button_bounds(&initial, "旧本文");
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(source_bounds.center()),
            egui::Event::PointerButton {
                pos: source_bounds.center(),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(egui::pos2(
            source_bounds.center().x + 10.0,
            source_bounds.center().y,
        ))],
    );
    assert_eq!(host.lock().unwrap().operations, 1);
    assert_eq!(host.lock().unwrap().active_drag, Some(source.clone()));
    assert_eq!(
        *router_markers.lock().unwrap(),
        vec![old_correlation.clone()]
    );

    reject_next.store(true, std::sync::atomic::Ordering::Release);
    let replacement = || {
        TabStripProjection::new(2, new_correlation.clone()).tab(
            TabStripTabDescriptor::new(destination.clone(), TabStripText::new("新本文"))
                .capabilities(
                    TabStripTabCapabilities::new()
                        .selectable(true)
                        .draggable(true),
                ),
        )
    };
    assert!(
        root.synchronize_with_lease(lease_with_body(
            2,
            "new body",
            replacement(),
            port(),
            Arc::clone(&active_root_router),
            "new router"
        ))
        .is_err()
    );
    let (retained_record, retained) = frame_with_record(&context, &mut root, Vec::new());
    assert_eq!(retained_record.record().presentation_revision(), 1);
    assert_eq!(text_input_value(&retained), Some("old body".to_owned()));
    assert_eq!(*active_root_router.lock().unwrap(), "old router");
    assert!(button_bounds(&retained, "旧本文").is_positive());
    assert_eq!(host.lock().unwrap().active_drag, Some(source.clone()));
    assert_eq!(host.lock().unwrap().operations, 1);
    assert_eq!(
        *router_markers.lock().unwrap(),
        vec![old_correlation.clone()]
    );

    root.synchronize_with_lease(lease_with_body(
        2,
        "new body",
        replacement(),
        port(),
        Arc::clone(&active_root_router),
        "new router",
    ))
    .expect("retry with the same revision should commit after cancellation succeeds");
    let (fresh_record, fresh) = frame_with_record(&context, &mut root, Vec::new());
    assert_eq!(fresh_record.record().presentation_revision(), 2);
    assert_eq!(text_input_value(&fresh), Some("new body".to_owned()));
    assert_eq!(*active_root_router.lock().unwrap(), "new router");
    assert!(button_bounds(&fresh, "新本文").is_positive());
    assert_eq!(host.lock().unwrap().active_drag, None);
    assert_eq!(host.lock().unwrap().operations, 2);
    assert_eq!(
        *router_markers.lock().unwrap(),
        vec![old_correlation.clone(), old_correlation.clone()]
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: source_bounds.center(),
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    assert_eq!(host.lock().unwrap().operations, 2);
    assert_eq!(host.lock().unwrap().active_drag, None);
    let new_tab = button_bounds(&fresh, "新本文").center();
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(new_tab),
            egui::Event::PointerButton {
                pos: new_tab,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: new_tab,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    assert_eq!(
        router_markers.lock().unwrap().last().cloned(),
        Some(new_correlation)
    );
    assert_eq!(host.lock().unwrap().selected, Some(destination));
}

#[test]
fn public_root_rejects_old_correlation_until_fresh_lease_is_attached() {
    let tab = TabStripTabTarget::from_opaque_bytes(b"correlated-tab");
    let old = TabStripCorrelation::from_opaque_bytes(b"correlation-old");
    let fresh = TabStripCorrelation::from_opaque_bytes(b"correlation-fresh");
    let host = Arc::new(Mutex::new(tab_host_effects::HostState::default()));
    let accepted = Arc::new(Mutex::new(old.clone()));
    let rejections = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut root = root_with_port(
        tab_host_effects::projection_with_selectable(1, old, tab.clone(), true),
        CorrelatingPort {
            state: Arc::clone(&host),
            accepted: Arc::clone(&accepted),
            rejections: Arc::clone(&rejections),
        },
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    let old_id = ax_button_id(&initial, "失効対象");
    *accepted.lock().unwrap() = fresh.clone();
    let mut error = None;
    let _ = context.run_ui(tab_host_effects::input(ax_activate(old_id)), |ui| {
        error = root.show(ui).err();
    });
    assert!(matches!(error, Some(katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceRootFactoryError::Root(_))));
    assert_eq!(rejections.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert!(host.lock().unwrap().selected.is_none());
    assert_eq!(host.lock().unwrap().operations, 0);
    root.synchronize_with_lease(tab_host_effects::lease_with_port(
        2,
        tab_host_effects::projection_with_selectable(2, fresh, tab.clone(), true),
        CorrelatingPort {
            state: Arc::clone(&host),
            accepted,
            rejections: Arc::clone(&rejections),
        },
    ))
    .unwrap();
    let fresh_frame = frame(&context, &mut root, Vec::new());
    frame(
        &context,
        &mut root,
        ax_activate(ax_button_id(&fresh_frame, "失効対象")),
    );
    assert_eq!(host.lock().unwrap().selected, Some(tab));
    assert_eq!(host.lock().unwrap().operations, 1);
    assert_eq!(rejections.load(std::sync::atomic::Ordering::Relaxed), 1);
}

#[test]
fn public_root_keeps_drag_terminal_on_start_host_across_valid_lease_sync() {
    let source = TabStripTabTarget::from_opaque_bytes(b"drag-first");
    let destination = TabStripTabTarget::from_opaque_bytes(b"drag-second");
    let old_host = Arc::new(Mutex::new(tab_host_effects::HostState {
        tab_order: vec![source.clone(), destination.clone()],
        ..Default::default()
    }));
    let new_host = Arc::new(Mutex::new(tab_host_effects::HostState {
        tab_order: vec![source.clone(), destination.clone()],
        ..Default::default()
    }));
    let old_correlation = TabStripCorrelation::from_opaque_bytes(b"drag-old");
    let middle_correlation = TabStripCorrelation::from_opaque_bytes(b"drag-middle");
    let latest_correlation = TabStripCorrelation::from_opaque_bytes(b"drag-latest");
    let new_accepted = Arc::new(Mutex::new(middle_correlation.clone()));
    let mut root = root_with_port(
        draggable_projection_with_correlation(
            1,
            old_correlation.clone(),
            source.clone(),
            destination.clone(),
        ),
        CorrelatingPort {
            state: Arc::clone(&old_host),
            accepted: Arc::new(Mutex::new(old_correlation.clone())),
            rejections: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        },
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    let source_bounds = button_bounds(&initial, "ドラッグタブ");
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(source_bounds.center()),
            egui::Event::PointerButton {
                pos: source_bounds.center(),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(egui::pos2(
            source_bounds.center().x + 10.0,
            source_bounds.center().y,
        ))],
    );
    assert_eq!(old_host.lock().unwrap().active_drag, Some(source.clone()));
    assert_eq!(old_host.lock().unwrap().operations, 1);

    root.synchronize_with_lease(lease_with_port(
        2,
        draggable_projection_with_correlation(
            2,
            middle_correlation.clone(),
            source.clone(),
            destination.clone(),
        ),
        CorrelatingPort {
            state: Arc::clone(&new_host),
            accepted: Arc::clone(&new_accepted),
            rejections: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        },
    ))
    .expect("valid lease should preserve the active drag origin");
    let updated = frame(&context, &mut root, Vec::new());
    let destination_bounds = button_bounds(&updated, "移動先");
    let drop_point = egui::pos2(
        destination_bounds.max.x - 1.0,
        destination_bounds.center().y,
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(drop_point)],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: drop_point,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    let old_state = old_host.lock().unwrap();
    assert!(old_state.active_drag.is_none());
    assert_eq!(old_state.operations, 2);
    assert_eq!(
        old_state.tab_order,
        vec![destination.clone(), source.clone()]
    );
    drop(old_state);
    let new_state = new_host.lock().unwrap();
    assert!(new_state.active_drag.is_none());
    assert_eq!(new_state.operations, 0);
    drop(new_state);

    *new_accepted.lock().unwrap() = latest_correlation.clone();
    root.synchronize_with_lease(lease_with_port(
        3,
        draggable_projection_with_correlation(
            3,
            latest_correlation.clone(),
            destination.clone(),
            source.clone(),
        ),
        CorrelatingPort {
            state: Arc::clone(&new_host),
            accepted: Arc::clone(&new_accepted),
            rejections: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        },
    ))
    .expect("subsequent lease should use the current host port");
    let current = frame(&context, &mut root, Vec::new());
    let current_target = button_bounds(&current, "移動先").center();
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(current_target),
            egui::Event::PointerButton {
                pos: current_target,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: current_target,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    assert_eq!(new_host.lock().unwrap().selected, Some(destination));
    assert_eq!(new_host.lock().unwrap().operations, 1);
}

#[test]
fn public_root_escape_cancels_on_start_host_after_multiple_valid_lease_syncs() {
    let source = TabStripTabTarget::from_opaque_bytes(b"drag-first");
    let destination = TabStripTabTarget::from_opaque_bytes(b"drag-second");
    let old_host = Arc::new(Mutex::new(tab_host_effects::HostState {
        tab_order: vec![source.clone(), destination.clone()],
        ..Default::default()
    }));
    let latest_host = Arc::new(Mutex::new(tab_host_effects::HostState {
        tab_order: vec![source.clone(), destination.clone()],
        ..Default::default()
    }));
    let intermediate_host = Arc::new(Mutex::new(tab_host_effects::HostState::default()));
    let old_correlation = TabStripCorrelation::from_opaque_bytes(b"escape-old");
    let middle_correlation = TabStripCorrelation::from_opaque_bytes(b"escape-middle");
    let latest_correlation = TabStripCorrelation::from_opaque_bytes(b"escape-latest");
    let old_accepted = Arc::new(Mutex::new(old_correlation.clone()));
    let middle_accepted = Arc::new(Mutex::new(middle_correlation.clone()));
    let latest_accepted = Arc::new(Mutex::new(latest_correlation.clone()));
    let mut root = root_with_port(
        draggable_projection_with_correlation(
            1,
            old_correlation.clone(),
            source.clone(),
            destination.clone(),
        ),
        CorrelatingPort {
            state: Arc::clone(&old_host),
            accepted: old_accepted,
            rejections: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        },
    );
    let context = egui::Context::default();
    context.enable_accesskit();
    let initial = frame(&context, &mut root, Vec::new());
    let source_point = button_bounds(&initial, "ドラッグタブ").center();
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(source_point),
            egui::Event::PointerButton {
                pos: source_point,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(egui::pos2(
            source_point.x + 10.0,
            source_point.y,
        ))],
    );
    assert_eq!(old_host.lock().unwrap().operations, 1);
    root.synchronize_with_lease(lease_with_port(
        2,
        draggable_projection_with_correlation(
            2,
            middle_correlation,
            source.clone(),
            destination.clone(),
        ),
        CorrelatingPort {
            state: Arc::clone(&intermediate_host),
            accepted: middle_accepted,
            rejections: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        },
    ))
    .expect("first valid lease should retain the start host");
    root.synchronize_with_lease(lease_with_port(
        3,
        draggable_projection_with_correlation(
            3,
            latest_correlation,
            source.clone(),
            destination.clone(),
        ),
        CorrelatingPort {
            state: Arc::clone(&latest_host),
            accepted: latest_accepted,
            rejections: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        },
    ))
    .expect("second valid lease should retain the same start host");
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    frame(
        &context,
        &mut root,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: false,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    let old_state = old_host.lock().unwrap();
    assert!(old_state.active_drag.is_none());
    assert_eq!(old_state.operations, 2);
    assert_eq!(
        old_state.tab_order,
        vec![source.clone(), destination.clone()]
    );
    drop(old_state);
    assert_eq!(intermediate_host.lock().unwrap().operations, 0);
    assert_eq!(latest_host.lock().unwrap().operations, 0);
    frame(&context, &mut root, Vec::new());
    assert_eq!(old_host.lock().unwrap().operations, 2);

    let latest = frame(&context, &mut root, Vec::new());
    let source_point = button_bounds(&latest, "ドラッグタブ").center();
    frame(
        &context,
        &mut root,
        vec![
            egui::Event::PointerMoved(source_point),
            egui::Event::PointerButton {
                pos: source_point,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: source_point,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    assert_eq!(latest_host.lock().unwrap().selected, Some(source));
    assert_eq!(latest_host.lock().unwrap().operations, 1);
}

fn inline_rename_key_events(key: egui::Key, modifiers: egui::Modifiers) -> Vec<egui::Event> {
    [true, false]
        .into_iter()
        .map(|pressed| egui::Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers,
        })
        .collect()
}

fn commit_rearmed_inline_name(
    context: &egui::Context,
    root: &mut katana_ui_core::egui::text_command_surface::EguiTextCommandSurfaceHostRoot,
    name: &str,
) {
    frame(
        context,
        root,
        inline_rename_key_events(egui::Key::A, egui::Modifiers::COMMAND),
    );
    frame(context, root, vec![egui::Event::Text(name.to_owned())]);
    frame(
        context,
        root,
        inline_rename_key_events(egui::Key::Enter, egui::Modifiers::NONE),
    );
}

#[test]
fn public_root_rearms_inline_rename_only_after_request_is_cleared() {
    for confirm_first in [false, true] {
        let group = TabStripGroupTarget::from_opaque_bytes(b"inline-rearm-group");
        let host = Arc::new(Mutex::new(tab_host_effects::HostState::default()));
        let mut root = root_with_host(
            projection_with_popup_named(1, group.clone(), false, true, "グループ"),
            Arc::clone(&host),
        );
        let context = egui::Context::default();
        context.enable_accesskit();
        frame(&context, &mut root, Vec::new());
        let opened = frame(&context, &mut root, Vec::new());
        assert!(!button_bounds_all(&opened, "グループ名").is_empty());
        let current_name = if confirm_first {
            commit_rearmed_inline_name(&context, &mut root, "最初の名前");
            assert_eq!(
                host.lock().unwrap().renamed_groups,
                vec![(group.clone(), "最初の名前".to_owned())]
            );
            "最初の名前"
        } else {
            frame(
                &context,
                &mut root,
                inline_rename_key_events(egui::Key::Escape, egui::Modifiers::NONE),
            );
            assert!(host.lock().unwrap().renamed_groups.is_empty());
            "グループ"
        };
        let first_operations = usize::from(confirm_first);
        assert_eq!(host.lock().unwrap().operations, first_operations);
        for _ in 0..2 {
            let idle = frame(&context, &mut root, Vec::new());
            assert!(button_bounds_all(&idle, "グループ名").is_empty());
        }
        root.synchronize_with_lease(lease_for(
            2,
            projection_with_popup_named(2, group.clone(), false, true, current_name),
            HostPort(Arc::clone(&host)),
        ))
        .unwrap();
        let held = frame(&context, &mut root, Vec::new());
        assert!(button_bounds_all(&held, "グループ名").is_empty());
        assert_eq!(host.lock().unwrap().operations, first_operations);
        root.synchronize_with_lease(lease_for(
            3,
            projection_with_popup_named(3, group.clone(), false, false, current_name),
            HostPort(Arc::clone(&host)),
        ))
        .unwrap();
        let cleared = frame(&context, &mut root, Vec::new());
        assert!(button_bounds_all(&cleared, "グループ名").is_empty());
        root.synchronize_with_lease(lease_for(
            4,
            projection_with_popup_named(4, group.clone(), false, true, current_name),
            HostPort(Arc::clone(&host)),
        ))
        .unwrap();
        frame(&context, &mut root, Vec::new());
        let rearmed = frame(&context, &mut root, Vec::new());
        assert!(!button_bounds_all(&rearmed, "グループ名").is_empty());
        commit_rearmed_inline_name(&context, &mut root, "再要求後の名前");
        assert_eq!(
            host.lock().unwrap().renamed_groups,
            vec![(group.clone(), "再要求後の名前".to_owned())]
        );
        assert_eq!(host.lock().unwrap().operations, first_operations + 1);
        root.synchronize_with_lease(lease_for(
            5,
            projection_with_popup_named(5, group, false, true, "再要求後の名前"),
            HostPort(Arc::clone(&host)),
        ))
        .unwrap();
        let held_again = frame(&context, &mut root, Vec::new());
        assert!(button_bounds_all(&held_again, "グループ名").is_empty());
        assert_eq!(host.lock().unwrap().operations, first_operations + 1);
    }
}
