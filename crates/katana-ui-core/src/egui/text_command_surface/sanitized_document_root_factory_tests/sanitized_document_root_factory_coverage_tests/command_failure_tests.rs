#[test]
fn command_disabled_hidden_and_missing_capability_never_forward_callbacks() {
    for (enabled, visible, capability) in [
        (false, true, true),
        (true, false, true),
        (true, true, false),
    ] {
        let calls = Rc::new(RefCell::new(0));
        let mut root = SanitizedDocumentRootFactory::new()
            .retain(command_input(
                1,
                calls.clone(),
                enabled,
                visible,
                capability,
                false,
                false,
            ))
            .expect("retain succeeds");
        let context = egui::Context::default();
        context.enable_accesskit();
        let (output, _) = run_command_root_frame(&context, &mut root, Vec::new());
        if !visible {
            assert!(output
                .platform_output
                .accesskit_update
                .as_ref()
                .is_none_or(|update| {
                    !update
                        .nodes
                        .iter()
                        .any(|(_, node)| node.label() == Some("直接 日本語 ⭐️👩‍💻"))
                }));
            continue;
        }
        let (node, _) = command_node(&output, "直接 日本語 ⭐️👩‍💻");
        if !capability {
            let (_, result) =
                run_command_root_frame_result(&context, &mut root, vec![accesskit_click(node)]);
            assert!(matches!(
                result,
                Err(SanitizedDocumentRootFactoryError::CommandCapability(
                    SanitizedCommandCapabilityRejection::Missing,
                ))
            ));
            assert_eq!(*calls.borrow(), 0);
            continue;
        }
        let (_, frame) = run_command_root_frame(&context, &mut root, vec![accesskit_click(node)]);
        let forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
        };
        assert!(!enabled);
        assert_eq!(
            frame.command_events.borrow().as_ref().map_or(0, Vec::len),
            0
        );
        assert_eq!(*calls.borrow(), 0);
        assert_eq!(forwarder.calls, 0);
    }
}

#[test]
fn command_callback_rejection_is_typed_opaque_and_consumed() {
    let calls = Rc::new(RefCell::new(0));
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(command_input(
            1,
            calls.clone(),
            true,
            true,
            true,
            false,
            true,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (output, _) = run_command_root_frame(&context, &mut root, Vec::new());
    let (node, _) = command_node(&output, "直接 日本語 ⭐️👩‍💻");
    let (_, frame) = run_command_root_frame(&context, &mut root, vec![accesskit_click(node)]);
    let mut forwarder = RetainingForwarder {
        calls: 0,
        transport_debug: None,
        transport: None,
    };
    let result = frame.forward_events_once(&mut forwarder);
    assert!(result.is_ok());
    assert_eq!(*calls.borrow(), 0);
    assert_eq!(forwarder.calls, 1);
    assert_eq!(
        forwarder.dispatch_root_once(),
        Err(SanitizedDocumentRootEventDispatchError::OpaqueHostEffect)
    );
    assert_eq!(*calls.borrow(), 1);
    assert_eq!(
        forwarder.dispatch_root_once(),
        Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
    );
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
}

#[test]
fn newer_revision_rejects_physical_command_frame_as_stale_before_callback() {
    let calls = Rc::new(RefCell::new(0));
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(command_input(
            1,
            calls.clone(),
            true,
            true,
            true,
            false,
            false,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (output, _) = run_command_root_frame(&context, &mut root, Vec::new());
    let (node, _) = command_node(&output, "直接 日本語 ⭐️👩‍💻");
    let (_, frame) = run_command_root_frame(&context, &mut root, vec![accesskit_click(node)]);
    root.synchronize(command_input(
        2,
        calls.clone(),
        true,
        true,
        true,
        false,
        false,
    ))
    .expect("new revision synchronizes");
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::StaleFrame)
    );
    assert_eq!(*calls.borrow(), 0);
    assert_eq!(forwarder.calls, 0);
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::StaleFrame)
    );
}
