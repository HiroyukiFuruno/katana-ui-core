#[test]
fn each_enabled_search_leaf_has_individual_raw_input_accesskit_evidence() {
    {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let bounds = accesskit_bounds(&output, egui::accesskit::Role::TextInput, "検索語 ⭐️");
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
        let frame = run_search_root_frame(
            &context,
            &mut root,
            vec![egui::Event::Ime(egui::ImeEvent::Commit(
                "日本語 ⭐️👩‍💻".to_string(),
            ))],
        )
        .1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
        };
        assert_search_leaf_forwarded_once("query IME commit", &frame, &mut forwarder);
        assert_eq!(
            text_events.borrow().as_slice(),
            &[(
                SanitizedSearchTextOperation::Query,
                "日本語 ⭐️👩‍💻".to_string()
            )],
            "query IME commit callback"
        );
        assert!(unit_events.borrow().is_empty(), "query unit callbacks");
    }

    {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let bounds = accesskit_bounds(&output, egui::accesskit::Role::TextInput, "置換 ⭐️");
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
        let frame = run_search_root_frame(
            &context,
            &mut root,
            vec![egui::Event::Ime(egui::ImeEvent::Commit(
                "日本語 ⭐️👩‍💻".to_string(),
            ))],
        )
        .1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
        };
        assert_search_leaf_forwarded_once("replacement IME commit", &frame, &mut forwarder);
        assert_eq!(
            text_events.borrow().as_slice(),
            &[(
                SanitizedSearchTextOperation::Replacement,
                "日本語 ⭐️👩‍💻".to_string(),
            )],
            "replacement IME commit callback"
        );
        assert!(
            unit_events.borrow().is_empty(),
            "replacement unit callbacks"
        );
    }

    {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let (node, _) = accesskit_button(&output, "置換 ⭐️");
        let frame = run_search_root_frame(&context, &mut root, vec![accesskit_click(node)]).1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
        };
        assert_search_leaf_forwarded_once("replace-one", &frame, &mut forwarder);
        assert_eq!(
            text_events.borrow().as_slice(),
            &[(SanitizedSearchTextOperation::Replace, String::new())],
            "replace-one callback"
        );
        assert!(
            unit_events.borrow().is_empty(),
            "replace-one unit callbacks"
        );
    }

    {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let (node, _) = accesskit_button(&output, "すべて置換 ⭐️");
        let frame = run_search_root_frame(&context, &mut root, vec![accesskit_click(node)]).1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
        };
        assert_search_leaf_forwarded_once("replace-all", &frame, &mut forwarder);
        assert_eq!(
            text_events.borrow().as_slice(),
            &[(SanitizedSearchTextOperation::ReplaceAll, String::new())],
            "replace-all callback"
        );
        assert!(
            unit_events.borrow().is_empty(),
            "replace-all unit callbacks"
        );
    }

    {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let bounds = accesskit_bounds(&output, egui::accesskit::Role::TextInput, "検索語 ⭐️");
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
        let frame =
            run_search_root_frame(&context, &mut root, vec![key_press(egui::Key::ArrowDown)]).1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
        };
        assert_search_leaf_forwarded_once("next", &frame, &mut forwarder);
        assert!(text_events.borrow().is_empty(), "next text callbacks");
        assert_eq!(
            unit_events.borrow().as_slice(),
            &[SanitizedSearchUnitOperation::Next],
            "next callback"
        );
    }

    {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let bounds = accesskit_bounds(&output, egui::accesskit::Role::TextInput, "検索語 ⭐️");
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
        let frame =
            run_search_root_frame(&context, &mut root, vec![key_press(egui::Key::ArrowUp)]).1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
        };
        assert_search_leaf_forwarded_once("previous", &frame, &mut forwarder);
        assert!(text_events.borrow().is_empty(), "previous text callbacks");
        assert_eq!(
            unit_events.borrow().as_slice(),
            &[SanitizedSearchUnitOperation::Previous],
            "previous callback"
        );
    }

    for (leaf, label, operation) in [
        (
            "match-case",
            "大文字小文字 ⭐️",
            SanitizedSearchUnitOperation::MatchCase(true),
        ),
        (
            "whole-word",
            "単語 ⭐️",
            SanitizedSearchUnitOperation::WholeWord(true),
        ),
        (
            "regex",
            "正規表現 ⭐️",
            SanitizedSearchUnitOperation::Regex(true),
        ),
    ] {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let (node, _) = accesskit_button(&output, label);
        let frame = run_search_root_frame(&context, &mut root, vec![accesskit_click(node)]).1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
        };
        assert_search_leaf_forwarded_once(leaf, &frame, &mut forwarder);
        assert!(text_events.borrow().is_empty(), "{leaf} text callbacks");
        assert_eq!(
            unit_events.borrow().as_slice(),
            &[operation],
            "{leaf} callback"
        );
    }

    {
        let (context, mut root, text_events, unit_events) = recorded_search_case();
        let (output, _) = run_search_root_frame(&context, &mut root, Vec::new());
        let (node, _) = accesskit_button(&output, "閉じる ⭐️");
        let frame = run_search_root_frame(&context, &mut root, vec![accesskit_click(node)]).1;
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
        };
        assert_search_leaf_forwarded_once("close", &frame, &mut forwarder);
        assert!(text_events.borrow().is_empty(), "close text callbacks");
        assert_eq!(
            unit_events.borrow().as_slice(),
            &[SanitizedSearchUnitOperation::Close],
            "close callback"
        );
    }
}
