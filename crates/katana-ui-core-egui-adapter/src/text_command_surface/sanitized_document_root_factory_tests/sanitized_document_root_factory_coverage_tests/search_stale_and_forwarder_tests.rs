#[test]
fn sanitized_physical_search_frame_is_stale_after_newer_same_identity_sync() {
    let text_calls = Rc::new(RefCell::new(0));
    let unit_calls = Rc::new(RefCell::new(0));
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_rejecting_recorders(
            1,
            text_calls.clone(),
            unit_calls.clone(),
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (initial_output, _) = run_search_root_frame(&context, &mut root, Vec::new());
    let bounds = accesskit_bounds(
        &initial_output,
        egui::accesskit::Role::TextInput,
        "検索語 ⭐️",
    );
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(bounds.center(), true)],
    );
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(bounds.center(), false)],
    );
    let (_, frame) = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "日本語 ⭐️👩‍💻".to_string(),
        ))],
    );
    assert_eq!(frame.output.events().event_cardinality(), 0);
    assert_eq!(frame.search_events.borrow().as_ref().map_or(0, Vec::len), 1);

    assert!(root
        .synchronize(input_with_rejecting_recorders(
            2,
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
        ))
        .expect("newer same-identity synchronization succeeds"));
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::StaleFrame)
    );
    assert_eq!(*text_calls.borrow(), 0);
    assert_eq!(*unit_calls.borrow(), 0);
    assert_eq!(forwarder.calls, 0);
    assert!(frame.search_events.borrow().is_some());
    assert!(!format!("{frame:?}").contains("日本語 ⭐️👩‍💻"));
    assert!(!format!("{frame:?}").contains("payload"));
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::StaleFrame)
    );
    assert_eq!(forwarder.calls, 0);
}

#[test]
fn forwarder_error_consumes_root_tab_and_search_batches() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_search(1))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();

    let (initial_output, _) = run_search_root_frame(&context, &mut root, Vec::new());
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
        .expect("query bounds exist");
    let query = egui::pos2(
        ((query_bounds.x0 + query_bounds.x1) / 2.0) as f32,
        ((query_bounds.y0 + query_bounds.y1) / 2.0) as f32,
    );
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: query,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        }],
    );
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::PointerButton {
            pos: query,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        }],
    );
    let (_, frame) = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
    );
    assert_eq!(frame.search_events.borrow().as_ref().map_or(0, Vec::len), 1);
    assert!(frame.tab_closed_events.borrow().is_some());

    let mut forwarder = FailingForwarder { calls: 0 };
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::Forwarder(()))
    );
    assert_eq!(forwarder.calls, 1);
    assert!(frame.tab_closed_events.borrow().is_none());
    assert!(frame.search_events.borrow().is_none());
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
}
