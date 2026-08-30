#[test]
fn context_menu_parent_submenu_debug_is_opaque() {
    let projection = SanitizedContextMenuProjectionBuilder::new()
        .item(
            SanitizedContextMenuItem::new(
                SanitizedContextMenuTarget::from_opaque_bytes(b"parent-secret"),
                0,
                "親 日本語 ⭐️",
            )
            .submenu_item(SanitizedContextMenuItem::new(
                SanitizedContextMenuTarget::from_opaque_bytes(b"child-secret"),
                0,
                "子 日本語 ⭐️",
            )),
        )
        .build();
    let debug = format!("{projection:?}");
    assert!(!debug.contains("親 日本語"));
    assert!(!debug.contains("parent-secret"));
}

#[test]
fn sanitized_document_root_factory_default_and_new_are_semantically_interchangeable() {
    let default_factory = SanitizedDocumentRootFactory::default();
    let new_factory = SanitizedDocumentRootFactory::new();
    let _ = default_factory
        .retain(input(1, b"document-default", "本文 ⭐️"))
        .expect("default constructor retains");
    let _ = new_factory
        .retain(input(2, b"document-new", "本文 ⭐️"))
        .expect("new constructor retains");
}

#[test]
fn sanitized_document_root_factory_error_messages_and_debug_are_rendered_without_leaking_payload() {
    let identity_changed = SanitizedDocumentRootFactoryError::IdentityChanged;
    assert_eq!(
        identity_changed.to_string(),
        "sanitized document root identity cannot change"
    );
    assert!(format!("{identity_changed:?}").contains("IdentityChanged"));

    let stale = SanitizedDocumentRootFactoryError::StaleRevision {
        current: 7,
        received: 3,
    };
    assert_eq!(
        stale.to_string(),
        "sanitized document root revision 3 is stale; current is 7"
    );
    assert!(format!("{stale:?}").contains("StaleRevision"));

    let conflict = SanitizedDocumentRootFactoryError::RevisionConflict { revision: 9 };
    assert_eq!(
        conflict.to_string(),
        "sanitized document root revision 9 conflicts"
    );
    assert!(format!("{conflict:?}").contains("RevisionConflict"));

    let render = SanitizedDocumentRootFactoryError::Render("failure".to_string());
    assert_eq!(
        render.to_string(),
        "sanitized document root render failed: failure"
    );
    assert!(format!("{render:?}").contains("Render"));

    let search = SanitizedDocumentRootFactoryError::SearchCapability(
        SanitizedSearchCapabilityRejection::Missing,
    );
    assert_eq!(search.to_string(), "sanitized search capability rejected");
    assert!(format!("{search:?}").contains("SearchCapability"));

    let command = SanitizedDocumentRootFactoryError::CommandCapability(
        SanitizedCommandCapabilityRejection::CallbackRejected,
    );
    assert_eq!(command.to_string(), "sanitized command capability rejected");
    assert!(format!("{command:?}").contains("CommandCapability"));

    let context = SanitizedDocumentRootFactoryError::ContextMenuCapability(
        SanitizedContextMenuCapabilityRejection::Missing,
    );
    assert_eq!(
        context.to_string(),
        "sanitized context menu capability rejected"
    );
    assert!(format!("{context:?}").contains("ContextMenuCapability"));
}

#[test]
fn context_menu_frame_is_stale_and_fails_closed_when_projection_advances() {
    let calls = Rc::new(RefCell::new(0));
    let mut root = SanitizedDocumentRootFactory::new()
        .retain(context_input(
            1,
            calls.clone(),
            false,
            true,
            true,
            true,
            false,
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
        .expect("context menu opens")
        .items
        .first()
        .expect("menu leaf exists");
    let item_point = egui::pos2(
        item.bounds.x as f32 + item.bounds.width as f32 / 2.0,
        item.bounds.y as f32 + item.bounds.height as f32 / 2.0,
    );
    let _ = run_root_frame_events(
        &context,
        &mut root,
        vec![egui::Event::PointerMoved(item_point)],
    );
    let (_, frame) =
        run_root_frame_events(&context, &mut root, vec![pointer_button(item_point, false)]);
    assert_eq!(
        frame
            .context_menu_events
            .borrow()
            .as_ref()
            .map_or(0, Vec::len),
        0
    );
    let synchronize = root.synchronize(context_input(
        2,
        calls.clone(),
        false,
        true,
        true,
        true,
        false,
    ));
    assert!(
        synchronize.is_ok(),
        "expected newer same-identity input to synchronize, got: {:?}",
        synchronize.err()
    );
    let _ = synchronize.expect("newer same-identity input synchronizes");
    let mut forwarder = RecordingForwarder {
        calls: 0,
        transport_debug: None,
    };
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::StaleFrame)
    );
    assert_eq!(
        frame.forward_events_once(&mut forwarder),
        Err(SanitizedDocumentRootEventForwardError::StaleFrame)
    );
    assert_eq!(*calls.borrow(), 0);
    assert_eq!(forwarder.calls, 0);
}
