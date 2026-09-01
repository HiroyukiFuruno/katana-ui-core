#[test]
fn raw_input_close_emits_one_opaque_intent_and_waits_for_next_projection() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory.retain(input_with_tabs(1)).expect("retain succeeds");
    let context = egui::Context::default();

    let mut first = None;
    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                first = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    let first = first.expect("first frame exists");
    let select_target = first
        .tab_rects()
        .iter()
        .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
        .map(|(_, rect)| rect.center())
        .expect("tab response exists");
    let close_target = first
        .tab_close_rects()
        .iter()
        .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
        .map(|(_, rect)| rect.center())
        .expect("close response exists");

    let mut root_only_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    let root_only_receipt = first
        .forward_events_once(&mut root_only_forwarder)
        .expect("root-only forwarding succeeds");

    let _ = run_root_frame(&context, &mut root, pointer_button(select_target, true));
    let selected = run_root_frame(&context, &mut root, pointer_button(select_target, false));
    let mut select_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    let select_receipt = selected
        .forward_events_once(&mut select_forwarder)
        .expect("select forwarding succeeds");

    let _ = run_root_frame(&context, &mut root, pointer_button(close_target, true));
    let close_frame = run_root_frame(&context, &mut root, pointer_button(close_target, false));
    assert_eq!(close_frame.tab_closed_event_count(), 1);
    let mut close_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    let close_receipt = close_frame
        .forward_events_once(&mut close_forwarder)
        .expect("close forwarding succeeds");

    assert_eq!(root_only_forwarder.calls, 1);
    assert_eq!(select_forwarder.calls, 1);
    assert_eq!(close_forwarder.calls, 1);
    assert_ne!(
        root_only_receipt.event_batch_fingerprint(),
        select_receipt.event_batch_fingerprint()
    );
    assert_ne!(
        select_receipt.event_batch_fingerprint(),
        close_receipt.event_batch_fingerprint()
    );
    assert_ne!(
        root_only_receipt.correlation_fingerprint(),
        select_receipt.correlation_fingerprint()
    );
    assert_ne!(
        select_receipt.correlation_fingerprint(),
        close_receipt.correlation_fingerprint()
    );
    assert_eq!(close_receipt.event_cardinality(), 1);

    let close_debug = format!("{close_receipt:?}");
    for forbidden in ["次の文書", "sanitized-tab-0-1", "opaque payload"] {
        assert!(!close_debug.contains(forbidden));
    }
    let transport_debug = close_forwarder
        .transport_debug
        .as_deref()
        .expect("close transport debug exists");
    for forbidden in ["次の文書", "sanitized-tab-0-1", "opaque payload"] {
        assert!(!transport_debug.contains(forbidden));
    }

    let retained = run_root_frame(
        &context,
        &mut root,
        egui::Event::PointerMoved(egui::Pos2::new(0.0, 0.0)),
    );
    assert!(retained
        .tab_close_rects()
        .iter()
        .any(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0));
    assert_eq!(
        close_frame.forward_events_once(&mut close_forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(close_forwarder.calls, 1);

    root.synchronize(input_with_one_tab(2))
        .expect("new projection synchronizes");
    let synchronized = run_root_frame(
        &context,
        &mut root,
        egui::Event::PointerMoved(egui::Pos2::new(0.0, 0.0)),
    );
    assert!(synchronized.tab_close_rects().is_empty());
}

#[test]
fn accesskit_click_from_previous_frame_button_update_emits_one_close_intent() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory.retain(input_with_tabs(1)).expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();

    let mut first_frame = None;
    let mut first_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                first_frame = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    first_output.textures_delta.clear();
    let first_frame = first_frame.expect("first frame exists");
    let update = first_output
        .platform_output
        .accesskit_update
        .expect("first frame emits AccessKit update");
    let (close_node, close_node_count) = update
        .nodes
        .iter()
        .filter(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some("次の文書を閉じる")
        })
        .map(|(node_id, _)| (*node_id, 1usize))
        .fold((None, 0usize), |(_, count), (node_id, _)| {
            (Some(node_id), count + 1)
        });
    assert_eq!(
        close_node_count, 1,
        "the supplied close label must identify one button"
    );
    let close_node = close_node.expect("close button node exists");

    let mut accesskit_frame = None;
    let mut accesskit_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events: vec![egui::Event::AccessKitActionRequest(
                egui::accesskit::ActionRequest {
                    action: egui::accesskit::Action::Click,
                    target_tree: egui::accesskit::TreeId::ROOT,
                    target_node: close_node,
                    data: None,
                },
            )],
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                accesskit_frame = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    accesskit_output.textures_delta.clear();
    let accesskit_frame = accesskit_frame.expect("AccessKit frame exists");
    assert_eq!(accesskit_frame.tab_activation_event_count(), 0);
    assert_eq!(accesskit_frame.tab_close_request_event_count(), 1);
    assert_eq!(accesskit_frame.tab_closed_event_count(), 1);
    assert!(accesskit_frame
        .tab_close_rects()
        .iter()
        .any(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0));

    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    let receipt = accesskit_frame
        .forward_events_once(&mut forwarder)
        .expect("AccessKit close forwarding succeeds");
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(forwarder.calls, 1);
    assert_eq!(
        accesskit_frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);

    let frame_debug = format!("{accesskit_frame:?}");
    for forbidden in ["次の文書", "sanitized-tab-0-1"] {
        assert!(!frame_debug.contains(forbidden));
    }
    let transport_debug = forwarder
        .transport_debug
        .as_deref()
        .expect("transport debug exists");
    for forbidden in ["次の文書", "sanitized-tab-0-1", "opaque payload"] {
        assert!(!transport_debug.contains(forbidden));
    }
    assert!(transport_debug.contains("<opaque>"));
    assert!(accesskit_output.platform_output.accesskit_update.is_some());
    drop(first_frame);
}
