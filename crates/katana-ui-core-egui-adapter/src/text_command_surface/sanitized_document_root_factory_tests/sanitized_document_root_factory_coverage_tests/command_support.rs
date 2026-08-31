fn input_with_one_tab(revision: u64) -> SanitizedDocumentRootInput {
    input(revision, b"document", "本文 ⭐️").with_tab_projection(SanitizedTabProjection::new([
        SanitizedTabGroup::new(
            SanitizedTabGroupTarget::from_opaque_bytes([0]),
            0,
            "ドキュメント",
        )
        .tab(SanitizedTab::new(
            SanitizedTabTarget::from_opaque_bytes([1]),
            0,
            "最初",
        )),
    ]))
}

fn command_input(
    revision: u64,
    calls: Rc<RefCell<usize>>,
    enabled: bool,
    visible: bool,
    capability: bool,
    dropdown: bool,
    reject: bool,
) -> SanitizedDocumentRootInput {
    command_input_with_callbacks(
        revision,
        calls.clone(),
        calls,
        enabled,
        visible,
        capability,
        dropdown,
        reject,
    )
}

fn command_input_with_callbacks(
    revision: u64,
    direct_calls: Rc<RefCell<usize>>,
    dropdown_calls: Rc<RefCell<usize>>,
    enabled: bool,
    visible: bool,
    capability: bool,
    dropdown: bool,
    reject: bool,
) -> SanitizedDocumentRootInput {
    use crate::text_command_surface::{
        SanitizedCommandDropdownItem, SanitizedCommandGroup, SanitizedCommandItem,
        SanitizedCommandProjection, SanitizedCommandTarget,
    };

    let target = |bytes: &[u8], calls: Rc<RefCell<usize>>| {
        let target = SanitizedCommandTarget::from_opaque_bytes(bytes.to_vec());
        if capability {
            target.with_unit_capability(move || {
                *calls.borrow_mut() += 1;
                if reject {
                    Err(())
                } else {
                    Ok(())
                }
            })
        } else {
            target
        }
    };
    let item = SanitizedCommandItem::new(
        target(b"direct-target-secret", direct_calls),
        0,
        "直接 日本語 ⭐️👩‍💻",
    )
    .enabled_state(enabled)
    .visible_state(visible);
    let item = if dropdown {
        item.dropdown_item(
            SanitizedCommandDropdownItem::new(
                target(b"dropdown-target-secret", dropdown_calls),
                0,
                "選択 日本語 ⭐️👩‍💻",
            )
            .enabled_state(enabled)
            .visible_state(visible),
        )
    } else {
        item
    };
    input(revision, b"command-document", "本文 日本語 ⭐️👩‍💻").with_command_projection(
        SanitizedCommandProjection::new([
            SanitizedCommandGroup::new(0, "操作 日本語 ⭐️👩‍💻").item(item)
        ]),
    )
}

fn floating_command_input(
    revision: u64,
    calls: Rc<RefCell<usize>>,
    enabled: bool,
    visible: bool,
    capability: bool,
    reject: bool,
) -> SanitizedDocumentRootInput {
    use crate::text_command_surface::{
        SanitizedCommandGroup, SanitizedCommandItem, SanitizedCommandProjection,
        SanitizedCommandTarget,
    };

    let target = SanitizedCommandTarget::from_opaque_bytes(b"floating-target-secret".to_vec());
    let target = if capability {
        target.with_unit_capability(move || {
            *calls.borrow_mut() += 1;
            if reject {
                Err(())
            } else {
                Ok(())
            }
        })
    } else {
        target
    };
    let projection =
        SanitizedCommandProjection::new([SanitizedCommandGroup::new(0, "浮遊操作 日本語 ⭐️")
            .item(
                SanitizedCommandItem::new(target, 0, "太字 日本語 ⭐️")
                    .enabled_state(enabled)
                    .visible_state(visible),
            )]);
    input(revision, b"floating-document", "本文 日本語 ⭐️👩‍💻")
        .with_floating_command_projection(projection)
}

fn command_node(output: &egui::FullOutput, label: &str) -> (egui::accesskit::NodeId, egui::Rect) {
    accesskit_node(output, label, egui::accesskit::Role::Button)
}

fn context_menu_node(
    output: &egui::FullOutput,
    label: &str,
) -> (egui::accesskit::NodeId, egui::Rect) {
    accesskit_node(output, label, egui::accesskit::Role::MenuItem)
}

fn accesskit_node(
    output: &egui::FullOutput,
    label: &str,
    role: egui::accesskit::Role,
) -> (egui::accesskit::NodeId, egui::Rect) {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(node_id, node)| {
                (node.role() == role && node.label() == Some(label))
                    .then(|| {
                        node.bounds().map(|bounds| {
                            (
                                *node_id,
                                egui::Rect::from_min_max(
                                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                                ),
                            )
                        })
                    })
                    .flatten()
            })
        })
        .expect("the requested AccessKit node must be present")
}

fn run_command_root_frame(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
    events: Vec<egui::Event>,
) -> (egui::FullOutput, super::SanitizedDocumentRootFrame) {
    let (output, frame) = run_command_root_frame_result(context, root, events);
    (output, frame.expect("command frame exists"))
}

fn run_command_root_frame_result(
    context: &egui::Context,
    root: &mut super::SanitizedDocumentRoot,
    events: Vec<egui::Event>,
) -> (
    egui::FullOutput,
    Result<super::SanitizedDocumentRootFrame, SanitizedDocumentRootFactoryError>,
) {
    let mut frame = None;
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                frame = Some(root.show(ui));
            });
        },
    );
    output.textures_delta.clear();
    (output, frame.expect("command frame result exists"))
}

fn assert_command_forwarded_once(
    frame: &super::SanitizedDocumentRootFrame,
    calls: &Rc<RefCell<usize>>,
    forwarder: &mut RecordingForwarder,
) {
    assert_eq!(frame.output.events().event_cardinality(), 0);
    assert_eq!(
        frame.command_events.borrow().as_ref().map_or(0, Vec::len),
        1
    );
    let receipt = frame
        .forward_events_once(forwarder)
        .expect("command forwards");
    assert_eq!(*calls.borrow(), 1);
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(forwarder.calls, 1);
    for (name, debug) in [
        ("frame", format!("{frame:?}")),
        (
            "transport",
            forwarder.transport_debug.clone().expect("transport debug"),
        ),
        ("receipt", format!("{receipt:?}")),
    ] {
        for forbidden in [
            "直接 日本語",
            "選択 日本語",
            "⭐️",
            "👩‍💻",
            "direct-target-secret",
            "dropdown-target-secret",
        ] {
            assert!(
                !debug.contains(forbidden),
                "{name} leaked `{forbidden}`: {debug}"
            );
        }
    }
    assert_eq!(
        frame.forward_events_once(forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(*calls.borrow(), 1);
    assert_eq!(forwarder.calls, 1);
}
