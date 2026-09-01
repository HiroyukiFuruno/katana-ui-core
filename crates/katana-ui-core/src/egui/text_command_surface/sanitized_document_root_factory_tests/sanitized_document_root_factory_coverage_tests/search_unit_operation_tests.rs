#[test]
fn current_search_unit_operations_use_physical_input_one_shot_routing() {
    let pointer_case = |label: &'static str, operation| {
        run_unit_operation_case(operation, move |context, root| {
            let (output, _) = run_search_root_frame(context, root, Vec::new());
            let (_, bounds) = accesskit_button(&output, label);
            let _ =
                run_search_root_frame(context, root, vec![pointer_button(bounds.center(), true)]);
            vec![pointer_button(bounds.center(), false)]
        });
    };
    pointer_case(
        "大文字小文字 ⭐️",
        SanitizedSearchUnitOperation::MatchCase(true),
    );
    pointer_case("単語 ⭐️", SanitizedSearchUnitOperation::WholeWord(true));
    pointer_case("正規表現 ⭐️", SanitizedSearchUnitOperation::Regex(true));
    run_unit_operation_case(SanitizedSearchUnitOperation::Close, |context, root| {
        let (output, _) = run_search_root_frame(context, root, Vec::new());
        let (node, _) = accesskit_button(&output, "閉じる ⭐️");
        vec![egui::Event::AccessKitActionRequest(
            egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target_tree: egui::accesskit::TreeId::ROOT,
                target_node: node,
                data: None,
            },
        )]
    });
}

#[test]
fn projected_option_state_renders_and_is_acknowledged_by_a_newer_revision() {
    let text_events = Rc::new(RefCell::new(Vec::new()));
    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_recorders_and_unit_targets_and_state(
            1,
            text_events.clone(),
            unit_events.clone(),
            true,
            true,
            false,
            true,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();

    let initial_options = root
        .process
        .search_options()
        .expect("search presentation exists");
    assert!(initial_options.match_case);
    assert!(!initial_options.whole_word);
    assert!(initial_options.use_regex);
    let (initial_output, _) = run_search_root_frame(&context, &mut root, Vec::new());

    let (_, match_case_bounds) = accesskit_button(&initial_output, "大文字小文字 ⭐️");
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(match_case_bounds.center(), true)],
    );
    let (_, toggled_frame) = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(match_case_bounds.center(), false)],
    );
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    let receipt = toggled_frame
        .forward_events_once(&mut forwarder)
        .expect("toggle forwards");
    assert_eq!(
        unit_events.borrow().as_slice(),
        &[SanitizedSearchUnitOperation::MatchCase(false)]
    );
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(forwarder.calls, 1);

    root.synchronize(input_with_recorders_and_unit_targets_and_state(
        2,
        text_events,
        unit_events,
        true,
        false,
        false,
        true,
    ))
    .expect("newer host acknowledgement synchronizes");
    let _ = run_search_root_frame(&context, &mut root, Vec::new());
    let acknowledged_options = root
        .process
        .search_options()
        .expect("search presentation exists");
    assert!(!acknowledged_options.match_case);
    assert!(!acknowledged_options.whole_word);
    assert!(acknowledged_options.use_regex);
}

#[test]
fn disabled_current_search_unit_operations_emit_no_callback_or_forwarded_event() {
    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_recorders_and_unit_targets(
            1,
            Rc::new(RefCell::new(Vec::new())),
            unit_events.clone(),
            false,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
    let (_, bounds) = accesskit_button(&output, "正規表現 ⭐️");
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(bounds.center(), true)],
    );
    let (_, frame) = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(bounds.center(), false)],
    );
    assert!(unit_events.borrow().is_empty());
    assert_eq!(frame.search_events.borrow().as_ref().map_or(0, Vec::len), 0);
    assert!(!format!("{frame:?}").contains("payload"));
}

#[test]
fn unsupported_current_search_unit_operations_are_disabled_without_routing() {
    let unit_events = Rc::new(RefCell::new(Vec::new()));
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input_with_recorders_and_unit_targets(
            1,
            Rc::new(RefCell::new(Vec::new())),
            unit_events.clone(),
            false,
        ))
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();

    let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
    let (_, regex_bounds) = accesskit_button(&output, "正規表現 ⭐️");
    let _ = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(regex_bounds.center(), true)],
    );
    let (_, frame) = run_search_root_frame(
        &context,
        &mut root,
        vec![pointer_button(regex_bounds.center(), false)],
    );
    assert!(unit_events.borrow().is_empty());
    assert_eq!(frame.search_events.borrow().as_ref().map_or(0, Vec::len), 0);

    for label in ["前へ ⭐️", "次へ ⭐️"] {
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let (_, bounds) = accesskit_button(&output, label);
        let _ = run_search_root_frame(
            &context,
            &mut root,
            vec![pointer_button(bounds.center(), true)],
        );
        let (_, frame) = run_search_root_frame(
            &context,
            &mut root,
            vec![pointer_button(bounds.center(), false)],
        );
        assert_eq!(frame.search_events.borrow().as_ref().map_or(0, Vec::len), 0);
    }
    assert!(unit_events.borrow().is_empty());
}
