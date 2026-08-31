#[test]
fn synchronize_maps_process_errors_to_the_public_error_contract() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input(3, b"one", "a"))
        .expect("retain succeeds");

    assert_eq!(
        root.synchronize(input(4, b"two", "b")),
        Err(SanitizedDocumentRootFactoryError::IdentityChanged)
    );
    assert_eq!(
        root.synchronize(input(2, b"one", "b")),
        Err(SanitizedDocumentRootFactoryError::StaleRevision {
            current: 3,
            received: 2,
        })
    );
    assert_eq!(
        root.synchronize(input(3, b"one", "b")),
        Err(SanitizedDocumentRootFactoryError::RevisionConflict { revision: 3 })
    );
}

#[test]
fn readonly_is_revisioned_and_exposed_to_accesskit() {
    let factory = SanitizedDocumentRootFactory::new();
    let mut root = factory
        .retain(input(1, b"readonly", "本文 ⭐️"))
        .expect("retain succeeds");
    assert_eq!(
        root.synchronize(input(1, b"readonly", "本文 ⭐️").with_readonly(true)),
        Err(SanitizedDocumentRootFactoryError::RevisionConflict { revision: 1 })
    );

    assert!(root
        .synchronize(input(2, b"readonly", "本文 ⭐️").with_readonly(true))
        .expect("new revision synchronizes"));
    let context = egui::Context::default();
    context.enable_accesskit();
    let (output, frame) = run_root_frame_events(&context, &mut root, Vec::new());
    assert!(
        frame
            .output
            .evidence_text
            .record
            .frame
            .accessibility
            .root
            .readonly
    );
    let node = output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(_, node)| {
                (node.role() == egui::accesskit::Role::TextInput
                    || node.role() == egui::accesskit::Role::MultilineTextInput)
                    .then_some(node)
            })
        })
        .expect("readonly text input is exposed to AccessKit");
    assert!(node.is_read_only());
}

#[test]
fn readonly_raw_text_and_ime_do_not_mutate_but_pointer_selection_remains_available() {
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(input(1, b"readonly-input", "本文 ⭐️").with_readonly(true))
        .expect("retain succeeds");
    let context = egui::Context::default();

    let (_, initial) = run_root_frame_events(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(egui::Pos2::ZERO)],
    );
    let content = initial.output.evidence_text.record.frame.content_bounds;
    let start = egui::pos2(
        content.x as f32 + FLOATING_SURFACE_HORIZONTAL_OFFSET,
        content.y as f32 + content.height as f32 / 2.0,
    );
    let end = egui::pos2(
        content.x as f32 + content.width as f32 - FLOATING_SURFACE_HORIZONTAL_OFFSET,
        start.y,
    );
    let midpoint = egui::pos2(content.x as f32 + content.width as f32 / 2.0, start.y);

    let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(start, true)]);
    let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(start, false)]);
    let (_, focused) = run_root_frame_events(&context, &mut root, Vec::new());
    assert!(
        focused
            .output
            .evidence_text
            .record
            .frame
            .accessibility
            .root
            .focused
    );
    for events in [
        vec![egui::Event::Text("追加入力 ⭐️".to_string())],
        vec![egui::Event::Ime(egui::ImeEvent::Preedit {
            text: "下書き ⭐️".to_string(),
            active_range_chars: None,
        })],
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            "確定 ⭐️".to_string(),
        ))],
    ] {
        let (_, frame) = run_root_frame_events(&context, &mut root, events);
        assert_eq!(root.process.input.snapshot, "本文 ⭐️");
        assert!(frame.output.evidence_text.events.is_empty());
    }

    let _ = run_root_frame_events(&context, &mut root, vec![pointer_button(start, true)]);
    let _ = run_root_frame_events(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(midpoint)],
    );
    let _ = run_root_frame_events(&context, &mut root, vec![egui::Event::PointerMoved(end)]);
    let (_, selected) =
        run_root_frame_events(&context, &mut root, vec![pointer_button(end, false)]);
    let range = selected
        .output
        .evidence_text
        .record
        .frame
        .selection
        .range
        .ordered();
    assert!(range.start < range.end);
    assert_eq!(root.process.input.snapshot, "本文 ⭐️");
}

#[test]
fn readonly_does_not_assign_semantics_to_an_enabled_opaque_command() {
    let calls = Rc::new(RefCell::new(0));
    let projection =
        super::super::SanitizedCommandProjection::new([super::super::SanitizedCommandGroup::new(
            1, "generic",
        )
        .item(
            super::super::SanitizedCommandItem::new(
                super::super::SanitizedCommandTarget::from_opaque_bytes([7]).with_unit_capability(
                    {
                        let calls = calls.clone();
                        move || {
                            *calls.borrow_mut() += 1;
                            Ok::<(), ()>(())
                        }
                    },
                ),
                1,
                "opaque action",
            )
            .with_icon(katana_ui_core::render_model::UiIconProps::new("<svg/>")),
        )]);
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(
            input(1, b"readonly-command", "本文 ⭐️")
                .with_readonly(true)
                .with_command_projection(projection),
        )
        .expect("retain succeeds");
    let context = egui::Context::default();
    context.enable_accesskit();
    let (output, _) = run_command_root_frame(&context, &mut root, Vec::new());
    let (node, _) = command_node(&output, "opaque action");
    let (_, released) = run_command_root_frame(&context, &mut root, vec![accesskit_click(node)]);
    assert_eq!(
        released
            .command_events
            .borrow()
            .as_ref()
            .map_or(0, Vec::len),
        1
    );
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    released
        .forward_events_once(&mut forwarder)
        .expect("opaque command forwards");
    assert_eq!(*calls.borrow(), 1);
    assert_eq!(root.process.input.snapshot, "本文 ⭐️");
}
