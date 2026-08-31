#[test]
fn physical_context_pointer_nested_keyboard_and_accesskit_are_one_shot() {
    for route in ["pointer", "nested", "keyboard", "accesskit"] {
        let calls = Rc::new(RefCell::new(0));
        let mut root = SanitizedDocumentRootFactory::new()
            .retain(context_input(
                1,
                calls.clone(),
                route == "nested",
                true,
                true,
                true,
                false,
            ))
            .expect("retain succeeds");
        let context = egui::Context::default();
        context.enable_accesskit();
        let (initial_output, initial) = run_root_frame_events(
            &context,
            &mut root,
            vec![egui::Event::PointerMoved(egui::Pos2::ZERO)],
        );
        let viewport = initial.output.evidence_text.record.frame.viewport_bounds;
        let point = egui::pos2(viewport.x as f32 + 20.0, viewport.y as f32 + 12.0);
        let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(point)]);
        let _ = run_root_frame_events(&context, &mut root, vec![secondary_button(point, true)]);
        let (opened_output, opened) =
            run_root_frame_events(&context, &mut root, vec![secondary_button(point, false)]);
        let frame = if route == "nested" {
            let item = opened
                .output
                .context_menu_record
                .as_ref()
                .expect("opened context menu record")
                .items
                .first()
                .expect("submenu item bounds");
            let p = egui::pos2(
                item.bounds.x as f32 + item.bounds.width as f32 / 2.0,
                item.bounds.y as f32 + item.bounds.height as f32 / 2.0,
            );
            let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(p)]);
            let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(p, true)]);
            let (_, submenu) =
                run_root_frame_events(&context, &mut root, vec![pointer_button(p, false)]);
            let child = submenu
                .output
                .context_menu_record
                .as_ref()
                .expect("submenu context menu record")
                .items
                .first()
                .expect("submenu leaf bounds");
            let cp = egui::pos2(
                child.bounds.x as f32 + child.bounds.width as f32 / 2.0,
                child.bounds.y as f32 + child.bounds.height as f32 / 2.0,
            );
            let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(cp)]);
            let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(cp, true)]);
            run_root_frame_events(&context, &mut root, vec![pointer_button(cp, false)]).1
        } else if route == "keyboard" {
            let _ =
                run_root_frame_events(&context, &mut root, vec![key_press(egui::Key::ArrowDown)]);
            run_root_frame_events(&context, &mut root, vec![key_press(egui::Key::Enter)]).1
        } else if route == "accesskit" {
            let node = context_menu_node(&opened_output, "葉 日本語 ⭐️👩‍💻").0;
            run_root_frame_events(
                &context,
                &mut root,
                vec![egui::Event::AccessKitActionRequest(
                    egui::accesskit::ActionRequest {
                        action: egui::accesskit::Action::Click,
                        target_tree: egui::accesskit::TreeId::ROOT,
                        target_node: node,
                        data: None,
                    },
                )],
            )
            .1
        } else {
            let item = opened
                .output
                .context_menu_record
                .as_ref()
                .expect("opened context menu record")
                .items
                .first()
                .expect("leaf item bounds");
            let p = egui::pos2(
                item.bounds.x as f32 + item.bounds.width as f32 / 2.0,
                item.bounds.y as f32 + item.bounds.height as f32 / 2.0,
            );
            let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(p)]);
            let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(p, true)]);
            run_root_frame_events(&context, &mut root, vec![pointer_button(p, false)]).1
        };
        let mut forwarder = RecordingForwarder {
            calls: 0,
            transport_debug: None,
        };
        let context_event_count = frame
            .context_menu_events
            .borrow()
            .as_ref()
            .map_or(0, Vec::len);
        let receipt = frame
            .forward_events_once(&mut forwarder)
            .expect("context forwarding succeeds");
        assert_eq!(
            *calls.borrow(),
            1,
            "route={route} context_events={context_event_count}"
        );
        assert_eq!(receipt.event_cardinality(), 1);
        assert_eq!(forwarder.calls, 1);
        assert_eq!(frame.output.events().event_cardinality(), 0);
        assert_eq!(
            frame.forward_events_once(&mut forwarder),
            Err(SanitizedDocumentRootEventForwardError::AlreadyConsumed)
        );
        assert_eq!(forwarder.calls, 1);
        assert!(!format!("{frame:?}").contains("context-leaf-secret"));
        assert!(!forwarder.transport_debug.unwrap().contains("葉 日本語"));
        let _ = initial_output;
    }
}
