#[test]
fn current_context_returns_empty_payloads_after_transport_consumption() {
    let batch = EguiTextCommandSurfaceRootEventBatch::new(full_payload(), String::new());
    assert!(batch.transport.borrow_mut().take().is_some());
    let context = batch.current_context();
    assert_eq!(context.root_identity(), "");
    assert_eq!(context.state_revision(), 0);
    assert_eq!(context.source_address_submission_count(), 0);
    assert!(context.text_events().is_empty());
    assert!(context.toolbar_events().is_empty());
    assert!(context.floating_events().is_empty());
    assert!(context.search_events().is_empty());
    assert!(context.context_menu_events().is_empty());
    assert!(context.status_bar_events().is_empty());
    assert!(context.diagnostics_list_events().is_empty());
    assert!(
        context
            .class_dispatches()
            .iter()
            .all(|dispatch| dispatch.event_count == 0)
    );
}

#[test]
fn command_detach_clears_text_only_when_command_activation_is_present() {
    let payload = RootEventPayload {
        text: vec![TextSurfaceEvent::TextArea(
            crate::atom::TextAreaEvent::Change("activated".into()),
        )],
        toolbar: Some(vec![CommandChromeToolbarEvent::CommandActivated {
            action_id: "kuc.toolbar".into(),
        }]),
        floating: Some(vec![FloatingCommandToolbarEvent::Toolbar {
            event: CommandChromeToolbarEvent::CommandActivated {
                action_id: "kuc.toolbar".into(),
            },
        }]),
        search: None,
        context_menu: None,
        status_bar: None,
        diagnostics_list: None,
        source_address_submissions: Vec::new(),
    };
    let batch = EguiTextCommandSurfaceRootEventBatch::new(payload, String::new());
    let _ = batch
        .detach_command_events()
        .expect("command detach succeeds");
    assert_eq!(batch.current_context().text_events(), &[]);
    assert_eq!(
        batch.detach_command_events(),
        Err(EguiTextCommandSurfaceRootEventCommandDetachError::AlreadyDetached)
    );
}

#[test]
fn context_menu_detach_is_one_shot_with_expected_duplicate_error() {
    let payload = RootEventPayload {
        text: Vec::new(),
        toolbar: None,
        floating: None,
        search: None,
        context_menu: Some(vec![
            crate::molecule::selection::ContextMenuEvent::Closed {
                reason: crate::molecule::selection::ContextMenuCloseReason::Escape,
            },
        ]),
        status_bar: None,
        diagnostics_list: None,
        source_address_submissions: Vec::new(),
    };
    let batch = EguiTextCommandSurfaceRootEventBatch::new(payload, String::new());
    assert_eq!(
        batch
            .detach_context_menu_events()
            .expect("context-menu detach succeeds")
            .len(),
        1
    );
    assert_eq!(
        batch.detach_context_menu_events(),
        Err(EguiTextCommandSurfaceRootEventCommandDetachError::AlreadyDetached)
    );
}

#[test]
fn attach_opaque_host_effect_batch_reports_consumed_and_already_attached() {
    let batch = EguiTextCommandSurfaceRootEventBatch::new(full_payload(), String::new());
    assert!(
        batch
            .attach_opaque_host_effect_batch(KucOpaqueHostEffectBatch::from_handler(|| Ok(())))
            .is_ok()
    );
    assert!(matches!(
        batch.attach_opaque_host_effect_batch(KucOpaqueHostEffectBatch::from_handler(|| Ok(()))),
        Err(KucOpaqueHostEffectAttachError::AlreadyAttached)
    ));

    let batch = EguiTextCommandSurfaceRootEventBatch::new(full_payload(), String::new());
    batch.transport.borrow_mut().take();
    assert!(matches!(
        batch.attach_opaque_host_effect_batch(KucOpaqueHostEffectBatch::from_handler(|| Ok(()))),
        Err(KucOpaqueHostEffectAttachError::AlreadyConsumed)
    ));
}

#[test]
fn event_envelope_serialization_fails_closed_for_unsupported_json_map_keys() {
    let unsupported_json_key = std::collections::BTreeMap::from([((1_u8, 2_u8), 3_u8)]);

    let error = dispatcher::serialize_value(&unsupported_json_key)
        .expect_err("JSON object keys must not silently accept tuple semantics");

    assert!(error.contains("key must be a string"));
}
