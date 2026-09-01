#[test]
fn public_show_retains_tab_event_without_exposing_it_in_public_frame_data() {
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
    let mut root_only_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    let root_only_receipt = first
        .forward_events_once(&mut root_only_forwarder)
        .expect("root-only forwarding succeeds");
    assert_eq!(root_only_forwarder.calls, 1);
    assert_eq!(root_only_receipt.event_cardinality(), 0);
    let target = first
        .tab_rects()
        .iter()
        .find(|(id, rect)| id == "sanitized-tab-0-1" && rect.width() > 0.0)
        .map(|(_, rect)| rect.center())
        .expect("second tab rect exists");

    let mut pressed = None;
    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events: vec![egui::Event::PointerButton {
                pos: target,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            }],
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                pressed = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    let pressed = pressed.expect("pressed frame exists");
    assert_eq!(pressed.tab_closed_event_count(), 0);
    let mut no_event_forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    let no_event_receipt = pressed
        .forward_events_once(&mut no_event_forwarder)
        .expect("no-event forwarding succeeds");
    assert_eq!(no_event_forwarder.calls, 1);

    let mut released = None;
    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(SCREEN_WIDTH, SCREEN_HEIGHT),
            )),
            events: vec![egui::Event::PointerButton {
                pos: target,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            }],
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                released = Some(root.show(ui).expect("root show succeeds"));
            });
        },
    );
    let released = released.expect("released frame exists");
    assert_eq!(released.tab_closed_event_count(), 1);
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    let receipt = released
        .forward_events_once(&mut forwarder)
        .expect("released tab event forwarding succeeds");
    assert_eq!(forwarder.calls, 1);
    assert_eq!(receipt.event_cardinality(), 1);
    assert_ne!(
        root_only_receipt.event_batch_fingerprint(),
        receipt.event_batch_fingerprint()
    );
    assert_ne!(
        no_event_receipt.event_batch_fingerprint(),
        receipt.event_batch_fingerprint()
    );
    assert_ne!(
        root_only_receipt.correlation_fingerprint(),
        receipt.correlation_fingerprint()
    );
    assert_ne!(
        no_event_receipt.correlation_fingerprint(),
        receipt.correlation_fingerprint()
    );

    let frame_debug = format!("{released:?}");
    for forbidden in ["次の文書", "sanitized-tab-0-1", "payload"] {
        assert!(
            !frame_debug.contains(forbidden),
            "public frame Debug leaked `{forbidden}`: {frame_debug}"
        );
    }
    let transport_debug = forwarder
        .transport_debug
        .as_deref()
        .expect("forwarder recorded transport Debug");
    for forbidden in ["次の文書", "sanitized-tab-0-1", "opaque payload"] {
        assert!(
            !transport_debug.contains(forbidden),
            "transport Debug leaked `{forbidden}`: {transport_debug}"
        );
    }
    assert!(transport_debug.contains("<opaque>"));
    let receipt_debug = format!("{receipt:?}");
    for forbidden in ["次の文書", "sanitized-tab-0-1", "opaque payload"] {
        assert!(
            !receipt_debug.contains(forbidden),
            "receipt Debug leaked `{forbidden}`: {receipt_debug}"
        );
    }
    assert_eq!(
        released.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
    assert_eq!(released.record().revision(), 1);
}

#[test]
fn missing_tab_event_batch_fails_closed_without_calling_forwarder() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input(1, b"document", "本文"))
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
    let _ = frame.tab_closed_events.borrow_mut().take();
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };

    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::InconsistentTabEventBatch)
    );
    assert_eq!(forwarder.calls, 0);
}
