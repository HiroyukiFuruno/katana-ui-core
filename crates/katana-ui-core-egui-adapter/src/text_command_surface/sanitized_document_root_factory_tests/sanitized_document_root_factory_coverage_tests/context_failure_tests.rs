fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

fn key_press(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }
}

#[test]
fn context_menu_failure_matrix_is_strict_and_opaque() {
    let cases = [
        ("disabled", false, true, true, false),
        ("invisible", true, false, true, false),
    ];
    for (name, enabled, visible, capability, reject) in cases {
        let calls = Rc::new(RefCell::new(0));
        let mut root = SanitizedDocumentRootFactory::new()
            .retain(context_input(
                1,
                calls.clone(),
                false,
                enabled,
                visible,
                capability,
                reject,
            ))
            .expect("retain succeeds");
        let context = egui::Context::default();
        let (_, initial) = run_root_frame_events(
            &context,
            &mut root,
            vec![egui::Event::PointerMoved(egui::Pos2::ZERO)],
        );
        let viewport = initial.output.evidence_text.record.frame.viewport_bounds;
        let point = egui::pos2(viewport.x as f32 + 20.0, viewport.y as f32 + 12.0);
        let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(point)]);
        let _ = run_root_frame_events(&context, &mut root, vec![secondary_button(point, true)]);
        let (_, frame) =
            run_root_frame_events(&context, &mut root, vec![secondary_button(point, false)]);
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
        };
        assert_eq!(
            frame
                .context_menu_events
                .borrow()
                .as_ref()
                .map_or(0, Vec::len),
            0,
            "{name} sanitized event"
        );
        let receipt = frame
            .forward_events_once(&mut forwarder)
            .expect("empty context menu batch forwards");
        assert_eq!(receipt.event_cardinality(), 1, "{name} root event batch");
        assert_eq!(forwarder.calls, 1, "{name} root batch forward");
        assert_eq!(*calls.borrow(), 0, "{name}");
    }
}

#[test]
fn context_menu_missing_and_callback_rejection_fail_at_root_frame() {
    for (capability, reject, expected) in [
        (
            false,
            false,
            SanitizedContextMenuCapabilityRejection::Missing,
        ),
        (
            true,
            true,
            SanitizedContextMenuCapabilityRejection::CallbackRejected,
        ),
    ] {
        let calls = Rc::new(RefCell::new(0));
        let mut root = SanitizedDocumentRootFactory::new()
            .retain(context_input(
                1,
                calls.clone(),
                false,
                true,
                true,
                capability,
                reject,
            ))
            .expect("retain succeeds");
        let context = egui::Context::default();
        let (_, initial) = run_root_frame_events(
            &context,
            &mut root,
            vec![egui::Event::PointerMoved(egui::Pos2::ZERO)],
        );
        let viewport = initial.output.evidence_text.record.frame.viewport_bounds;
        let point = egui::pos2(viewport.x as f32 + 20.0, viewport.y as f32 + 12.0);
        let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(point)]);
        let _ = run_root_frame_events(&context, &mut root, vec![secondary_button(point, true)]);
        let (_, opened) =
            run_root_frame_events(&context, &mut root, vec![secondary_button(point, false)]);
        let item = opened
            .output
            .context_menu_record
            .as_ref()
            .unwrap()
            .items
            .first()
            .unwrap();
        let item_point = egui::pos2(
            item.bounds.x as f32 + item.bounds.width as f32 / 2.0,
            item.bounds.y as f32 + item.bounds.height as f32 / 2.0,
        );
        let _ = run_root_frame_events(
            &context,
            &mut root,
            vec![egui::Event::PointerMoved(item_point)],
        );
        let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(item_point, true)]);
        if !capability {
            let result =
                run_root_frame_result(&context, &mut root, vec![pointer_button(item_point, false)]);
            assert!(matches!(
                result,
                Err(SanitizedDocumentRootFactoryError::ContextMenuCapability(value))
                    if value == expected
            ));
            assert_eq!(*calls.borrow(), 0, "missing capability callback");
            continue;
        }
        let frame =
            run_root_frame_result(&context, &mut root, vec![pointer_button(item_point, false)])
                .expect("callback rejection should defer to host dispatch");
        let mut forwarder = RetainingForwarder {
            calls: 0,
            transport_debug: None,
            transport: None,
        };
        assert!(frame.forward_events_once(&mut forwarder).is_ok());
        assert_eq!(*calls.borrow(), 0, "callback rejection pre-dispatch");
        assert_eq!(forwarder.calls, 1);
        assert_eq!(
            forwarder.dispatch_root_once(),
            Err(SanitizedDocumentRootEventDispatchError::OpaqueHostEffect)
        );
        assert_eq!(*calls.borrow(), 1, "callback rejection host dispatch");
        assert_eq!(
            forwarder.dispatch_root_once(),
            Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed)
        );
        assert_eq!(
            frame.forward_events_once(&mut forwarder),
            Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
        );
    }
}
