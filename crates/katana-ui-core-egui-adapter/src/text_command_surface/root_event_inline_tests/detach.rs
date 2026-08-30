#[test]
fn detach_projects_nonsearch_root_and_keeps_outer_receipt_cardinality() {
    let payload = RootEventPayload {
        text: vec![TextSurfaceEvent::FocusChanged(true)],
        toolbar: None,
        floating: None,
        search: Some(vec![CommandChromeSearchEvent::CloseRequested]),
        context_menu: None,
        source_address_submissions: Vec::new(),
        ..RootEventPayload::empty()
    };
    let expected_fingerprint = RootEventFingerprint::fingerprint_payload(&RootEventPayload {
        text: payload.text.clone(),
        toolbar: None,
        floating: None,
        search: None,
        context_menu: None,
        source_address_submissions: Vec::new(),
        ..RootEventPayload::empty()
    })
    .expect("fingerprint succeeds");

    let batch =
        EguiTextCommandSurfaceRootEventBatch::new(payload, String::from("pre-detach fingerprint"));
    let sanitized_search = batch.detach_search_events().expect("detach succeeds");
    assert_eq!(sanitized_search.len(), 1);
    assert_eq!(batch.event_cardinality(), 1);

    let mut forwarder = CountingForwarder { calls: 0 };
    let receipt = batch
        .forward_once(&mut forwarder)
        .expect("forward succeeds");
    assert_eq!(forwarder.calls, 1);
    assert_eq!(receipt.event_cardinality(), 1);
    assert_eq!(receipt.event_batch_fingerprint(), expected_fingerprint);

    let nonsearch_batch = EguiTextCommandSurfaceRootEventBatch::new(
        RootEventPayload {
            text: vec![TextSurfaceEvent::FocusChanged(true)],
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
            source_address_submissions: Vec::new(),
            ..RootEventPayload::empty()
        },
        expected_fingerprint,
    );
    let mut expected_forwarder = CountingForwarder { calls: 0 };
    let expected_receipt = nonsearch_batch
        .forward_once(&mut expected_forwarder)
        .expect("nonsearch forward succeeds");
    assert_eq!(
        receipt.correlation_fingerprint(),
        expected_receipt.correlation_fingerprint()
    );
    assert_eq!(receipt.event_cardinality() + sanitized_search.len(), 2);
}

#[test]
fn detached_search_cannot_be_retrieved_after_root_forward() {
    let batch = EguiTextCommandSurfaceRootEventBatch::new(
        RootEventPayload {
            text: Vec::new(),
            toolbar: None,
            floating: None,
            search: Some(vec![CommandChromeSearchEvent::CloseRequested]),
            context_menu: None,
            source_address_submissions: Vec::new(),
            ..RootEventPayload::empty()
        },
        String::new(),
    );
    let _ = batch.detach_search_events().expect("detach succeeds");
    let mut forwarder = CountingForwarder { calls: 0 };
    batch
        .forward_once(&mut forwarder)
        .expect("forward succeeds");

    assert_eq!(
        batch.detach_search_events(),
        Err(EguiTextCommandSurfaceRootEventSearchDetachError::AlreadyConsumed)
    );
}

#[test]
fn detached_root_envelope_excludes_search_events() {
    let payload = RootEventPayload {
        text: Vec::new(),
        toolbar: None,
        floating: None,
        search: Some(vec![CommandChromeSearchEvent::CloseRequested]),
        context_menu: None,
        source_address_submissions: Vec::new(),
        ..RootEventPayload::empty()
    };
    let batch = EguiTextCommandSurfaceRootEventBatch::new(payload, String::new());
    let _ = batch.detach_search_events().expect("detach succeeds");
    let transport = batch.transport.borrow();
    let payload = &transport
        .as_ref()
        .expect("transport remains available")
        .payload;
    let envelope = RootEventEnvelope {
        text: &payload.text,
        toolbar: payload.toolbar.as_deref(),
        floating: payload.floating.as_deref(),
        search: payload.search.as_deref(),
        context_menu: payload.context_menu.as_deref(),
        status_bar: payload.status_bar.as_deref(),
        diagnostics_list: payload.diagnostics_list.as_deref(),
    };
    let serialized = serde_json::to_vec(&envelope).expect("root envelope serializes");
    let serialized = String::from_utf8(serialized).expect("root envelope is UTF-8 JSON");

    assert!(serialized.contains("\"search\":null"));
}
