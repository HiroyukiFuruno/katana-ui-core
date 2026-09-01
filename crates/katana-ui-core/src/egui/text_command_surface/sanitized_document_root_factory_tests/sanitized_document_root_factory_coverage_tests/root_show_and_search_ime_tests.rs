#[test]
fn show_returns_a_closed_record_and_forwards_events_only_once() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input(1, b"document", "日本語 ⭐️"))
        .expect("retain succeeds");
    let context = egui::Context::default();
    let mut frame = None;
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
                frame = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    let frame = frame.expect("frame exists");

    assert_eq!(frame.record().revision(), 1);
    assert!(!frame.record().identity().is_empty());
    assert!(frame.record().dimensions().width() > 0);
    assert!(frame.record().dimensions().height() > 0);
    assert_eq!(frame.record().rgba_hash().len(), 64);
    assert_eq!(frame.record().accessibility_snapshot_hash().len(), 64);

    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    let receipt = frame
        .forward_events_once(&mut forwarder)
        .expect("first forwarding succeeds");
    assert_eq!(forwarder.calls, 1);
    assert!(receipt.consumed_once());
    assert!(!receipt.root_identity().is_empty());
    assert_eq!(receipt.state_revision(), frame.record().state_revision());
    assert!(!receipt.correlation_fingerprint().is_empty());
    assert!(!receipt.event_batch_fingerprint().is_empty());
    assert_eq!(receipt.event_cardinality(), 0);
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
}

#[test]
fn raw_ime_search_at_current_root_renders_and_forwards_one_opaque_event() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_search(1))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();

    let (initial_output, initial_frame) = run_search_root_frame(&context, &mut root, Vec::new());
    let query_bounds = initial_output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(_, node)| {
                (node.role() == egui::accesskit::Role::TextInput).then(|| node.bounds())
            })
        })
        .flatten()
        .expect("the localized query input has an actual AccessKit bounds");
    assert!(query_bounds.x1 > query_bounds.x0);
    assert!(query_bounds.y1 > query_bounds.y0);

    let no_search_context = egui::Context::default();
    let mut no_search_root = factory
        .retain(input(1, b"document", "本文 ⭐️"))
        .expect("retain without search succeeds");
    let (_, no_search_frame) =
        run_search_root_frame(&no_search_context, &mut no_search_root, Vec::new());
    assert_ne!(
        initial_frame.record().record_hash(),
        no_search_frame.record().record_hash()
    );

    let query = egui::pos2(
        ((query_bounds.x0 + query_bounds.x1) / 2.0) as f32,
        ((query_bounds.y0 + query_bounds.y1) / 2.0) as f32,
    );
    let (_, pressed) = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: query,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        }],
    );
    let (_, focused) = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: query,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        }],
    );
    assert_eq!(pressed.output.events().event_cardinality(), 0);
    assert_eq!(focused.output.events().event_cardinality(), 0);

    let (_, preedit) = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "かな".to_string(),
            active_range_chars: None,
        })],
    );
    assert_eq!(preedit.output.events().event_cardinality(), 0);
    assert_ne!(
        focused.record().record_hash(),
        preedit.record().record_hash()
    );

    let (_, committed) = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
    );
    assert_eq!(committed.output.events().event_cardinality(), 0);
    assert_eq!(
        committed
            .search_events
            .borrow()
            .as_ref()
            .map_or(0, Vec::len),
        1
    );

    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    let receipt = committed
        .forward_events_once(&mut forwarder)
        .expect("one-shot search forwarding succeeds");
    assert_eq!(forwarder.calls, 1);
    assert_eq!(receipt.event_cardinality(), 1);

    let frame_debug = format!("{committed:?}");
    for forbidden in ["検索語 ⭐️", "かな", "payload"] {
        assert!(!frame_debug.contains(forbidden));
    }
    let transport_debug = forwarder
        .transport_debug
        .as_deref()
        .expect("transport debug exists");
    for forbidden in ["検索語 ⭐️", "かな", "opaque payload"] {
        assert!(!transport_debug.contains(forbidden));
    }
    assert!(transport_debug.contains("<opaque>"));
    let receipt_debug = format!("{receipt:?}");
    for forbidden in ["検索語 ⭐️", "かな", "opaque payload"] {
        assert!(!receipt_debug.contains(forbidden));
    }
    assert_eq!(
        committed.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
}
