#[test]
fn physical_floating_keyboard_activation_routes_only_to_floating_target_once() {
    use crate::text_command_surface::{
        SanitizedCommandGroup, SanitizedCommandItem, SanitizedCommandProjection,
        SanitizedCommandTarget,
    };

    let top_calls = Rc::new(RefCell::new(0));
    let floating_calls = Rc::new(RefCell::new(0));
    let mut input = command_input_with_callbacks(
        1,
        top_calls.clone(),
        Rc::new(RefCell::new(0)),
        false,
        true,
        true,
        false,
        false,
    );
    let floating_target =
        SanitizedCommandTarget::from_opaque_bytes(b"floating-keyboard-target-secret".to_vec())
            .with_unit_capability({
                let calls = floating_calls.clone();
                move || {
                    *calls.borrow_mut() += 1;
                    Ok::<(), ()>(())
                }
            });
    input = input.with_floating_command_projection(SanitizedCommandProjection::new([
        SanitizedCommandGroup::new(0, "浮遊操作 日本語 ⭐️").item(SanitizedCommandItem::new(
            floating_target,
            0,
            "太字 日本語 ⭐️",
        )),
    ]));

    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input)
        .expect("retain succeeds");
    let context = egui::Context::default();
    let (_, selected) = select_floating_surface(&context, &mut root);
    assert!(selected
        .output
        .floating
        .as_ref()
        .and_then(|value| value.record.as_ref())
        .is_some());
    assert_eq!(
        selected
            .command_events
            .borrow()
            .as_ref()
            .map_or(0, Vec::len),
        0,
        "selection release must not activate floating command"
    );

    let mut focused = None;
    for _ in 0..8 {
        let _ = run_root_frame_events(&context, &mut root, vec![key_press(egui::Key::Tab)]);
        let candidate =
            run_root_frame_events(&context, &mut root, vec![key_press(egui::Key::ArrowRight)]).1;
        assert_eq!(
            candidate
                .command_events
                .borrow()
                .as_ref()
                .map_or(0, Vec::len),
            0,
            "focus movement must not activate a command"
        );
        if candidate
            .output
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
            .and_then(|value| value.toolbar.focused_action_id.as_ref())
            .is_some()
        {
            focused = Some(candidate);
            break;
        }
    }
    assert!(
        focused.is_some(),
        "raw keyboard input must focus floating action"
    );

    let activated = run_root_frame_events(&context, &mut root, vec![key_press(egui::Key::Enter)]).1;
    assert_eq!(activated.output.events().event_cardinality(), 0);
    let command_event_count = activated
        .command_events
        .borrow()
        .as_ref()
        .map_or(0, Vec::len);
    let floating_event_debug = activated
        .output
        .floating
        .as_ref()
        .map(|value| format!("{:?}", value.events));
    assert_eq!(
        command_event_count, 1,
        "command_event_count={command_event_count} floating_events={floating_event_debug:?}"
    );
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    let receipt = activated
        .forward_events_once(&mut forwarder)
        .expect("floating keyboard activation forwards");
    assert_eq!(*floating_calls.borrow(), 1);
    assert_eq!(*top_calls.borrow(), 0);
    assert_eq!(forwarder.calls, 1);
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(
        activated.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
    for forbidden in [
        "floating-keyboard-target-secret",
        "浮遊操作 日本語",
        "太字 日本語",
        "panel_bounds",
        "actions",
    ] {
        assert!(!format!("{activated:?}").contains(forbidden));
        assert!(!format!("{receipt:?}").contains(forbidden));
        assert!(!forwarder
            .transport_debug
            .as_deref()
            .expect("transport debug")
            .contains(forbidden));
    }
}

#[test]
fn physical_floating_accesskit_snapshot_click_routes_one_opaque_target() {
    use crate::text_command_surface::{
        SanitizedCommandGroup, SanitizedCommandItem, SanitizedCommandProjection,
        SanitizedCommandTarget,
    };

    let calls = Rc::new(RefCell::new(0));
    let projection =
        SanitizedCommandProjection::new([SanitizedCommandGroup::new(0, "浮遊操作 日本語 ⭐️")
            .item(SanitizedCommandItem::new(
                SanitizedCommandTarget::from_opaque_bytes(
                    b"floating-accesskit-target-secret".to_vec(),
                )
                .with_unit_capability({
                    let calls = calls.clone();
                    move || {
                        *calls.borrow_mut() += 1;
                        Ok::<(), ()>(())
                    }
                }),
                0,
                "太字 日本語 ⭐️",
            ))]);
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(
            input(1, b"floating-accesskit-document", "本文 日本語 ⭐️👩‍💻")
                .with_floating_command_projection(projection),
        )
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (output, selected) = select_floating_surface(&context, &mut root);
    let floating = selected
        .output
        .floating
        .as_ref()
        .and_then(|value| value.record.as_ref())
        .expect("selection opens floating");
    let action_bounds = floating
        .toolbar
        .actions
        .first()
        .expect("floating action record")
        .bounds;
    let (node, _) = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("current AccessKit snapshot")
        .nodes
        .iter()
        .find_map(|(node_id, node)| {
            let bounds = node.bounds()?;
            let matches_action = node.role() == egui::accesskit::Role::Button
                && bounds.x0 as i32 == action_bounds.x
                && bounds.y0 as i32 == action_bounds.y
                && bounds.x1 as i32 == action_bounds.x.saturating_add_unsigned(action_bounds.width)
                && bounds.y1 as i32
                    == action_bounds
                        .y
                        .saturating_add_unsigned(action_bounds.height);
            matches_action.then_some((*node_id, bounds))
        })
        .expect("floating action node from current snapshot");

    let (_, activated) = run_root_frame_events(&context, &mut root, vec![accesskit_click(node)]);
    assert_eq!(activated.output.events().event_cardinality(), 0);
    assert_eq!(
        activated
            .command_events
            .borrow()
            .as_ref()
            .map_or(0, Vec::len),
        1
    );
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    let receipt = activated
        .forward_events_once(&mut forwarder)
        .expect("AccessKit floating activation forwards");
    assert_eq!(*calls.borrow(), 1);
    assert_eq!(forwarder.calls, 1);
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(
        activated.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
    for forbidden in [
        "floating-accesskit-target-secret",
        "浮遊操作 日本語",
        "太字 日本語",
        "panel_bounds",
        "actions",
    ] {
        assert!(!format!("{activated:?}").contains(forbidden));
        assert!(!format!("{receipt:?}").contains(forbidden));
        assert!(!forwarder
            .transport_debug
            .as_deref()
            .expect("transport debug")
            .contains(forbidden));
    }
}
