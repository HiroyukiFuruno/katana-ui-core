#[test]
fn physical_ime_commit_routes_exact_text_once_without_debug_leakage() {
    let text_events = Rc::new(RefCell::new(Vec::new()));
    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_recorders(1, text_events.clone(), unit_events))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();

    let (initial_output, _) = run_search_root_frame(&context, &mut root, Vec::new());
    let query_bounds = accesskit_bounds(
        &initial_output,
        egui::accesskit::Role::TextInput,
        "検索語 ⭐️",
    );
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(query_bounds.center(), true)],
    );
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(query_bounds.center(), false)],
    );
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "かな".to_string(),
            active_range_chars: None,
        })],
    );
    let (_, committed) = run_search_root_frame(
        &context,
        &mut root,
        vec![egui::Event::Ime(egui::ImeEvent::Commit("⭐️".to_string()))],
    );

    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    let receipt = committed
        .forward_events_once(&mut forwarder)
        .expect("physical IME event forwards");
    assert_eq!(
        text_events.borrow().as_slice(),
        &[(SanitizedSearchTextOperation::Query, "⭐️".to_string(),)]
    );
    assert!(committed.output.events().event_cardinality() == 0);
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(forwarder.calls, 1);
    assert!(!format!("{committed:?}").contains("⭐️"));
    assert!(!format!("{committed:?}").contains("👩‍💻"));
    assert!(!format!("{receipt:?}").contains("⭐️"));
    assert!(!format!("{receipt:?}").contains("👩‍💻"));
    assert!(!forwarder
        .transport_debug
        .as_deref()
        .expect("transport debug exists")
        .contains("⭐️"));
    assert!(!forwarder
        .transport_debug
        .as_deref()
        .expect("transport debug exists")
        .contains("👩‍💻"));
    assert_eq!(
        committed.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
    );
    assert_eq!(forwarder.calls, 1);
}

#[test]
fn sanitized_physical_search_callback_rejection_is_opaque_and_consumed() {
    for operation in ["query", "replacement", "option"] {
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
        let event = match operation {
            "query" => {
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
                vec![egui::Event::Ime(egui::ImeEvent::Commit(
                    "日本語 ⭐️👩‍💻".to_string(),
                ))]
            }
            "replacement" => {
                let bounds =
                    accesskit_bounds(&initial_output, egui::accesskit::Role::TextInput, "置換 ⭐️");
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
                vec![egui::Event::Ime(egui::ImeEvent::Commit(
                    "置換後 ⭐️👩‍💻".to_string(),
                ))]
            }
            _ => {
                let (node, _) = accesskit_button(&initial_output, "大文字小文字 ⭐️");
                vec![egui::Event::AccessKitActionRequest(
                    egui::accesskit::ActionRequest {
                        action: egui::accesskit::Action::Click,
                        target_tree: egui::accesskit::TreeId::ROOT,
                        target_node: node,
                        data: None,
                    },
                )]
            }
        };
        let (_, frame) = run_search_root_frame(&context, &mut root, event);
        assert_eq!(frame.output.events().event_cardinality(), 0, "{operation}");
        assert_eq!(frame.search_events.borrow().as_ref().map_or(0, Vec::len), 1);

        let mut forwarder = RetainingForwarder {
            calls: 0,
            transport_debug: None,
            transport: None,
        };
        let receipt = frame.forward_events_once(&mut forwarder);
        assert!(receipt.is_ok(), "{operation} outer forward");
        assert_eq!(*text_calls.borrow(), 0, "{operation} text pre-dispatch");
        assert_eq!(*unit_calls.borrow(), 0, "{operation} unit pre-dispatch");
        assert_eq!(forwarder.calls, 1, "{operation} outer forward call");
        assert!(frame.search_events.borrow().is_none());
        assert!(!format!("{frame:?}").contains("日本語 ⭐️👩‍💻"));
        assert!(!format!("{frame:?}").contains("置換後 ⭐️👩‍💻"));
        assert!(!format!("{frame:?}").contains("payload"));
        assert_eq!(
            forwarder.dispatch_root_once(),
            Err(SanitizedDocumentRootEventDispatchError::OpaqueHostEffect),
            "{operation} host dispatch rejection"
        );
        assert_eq!(
            *text_calls.borrow(),
            usize::from(operation != "option"),
            "{operation} text host dispatch"
        );
        assert_eq!(
            *unit_calls.borrow(),
            usize::from(operation == "option"),
            "{operation} unit host dispatch"
        );
        assert_eq!(
            forwarder.dispatch_root_once(),
            Err(SanitizedDocumentRootEventDispatchError::AlreadyConsumed),
            "{operation} host dispatch replay"
        );
        assert_eq!(
            frame.forward_events_once(&mut forwarder),
            Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
        );
        assert_eq!(forwarder.calls, 1);
    }
}
