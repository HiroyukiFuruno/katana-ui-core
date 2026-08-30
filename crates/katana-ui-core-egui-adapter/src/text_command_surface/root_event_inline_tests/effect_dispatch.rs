#[test]
fn transport_fails_closed_at_each_child_dispatch_stage() {
    let stages = [
        EguiTextCommandSurfaceRootEventChildClass::Text,
        EguiTextCommandSurfaceRootEventChildClass::Toolbar,
        EguiTextCommandSurfaceRootEventChildClass::Floating,
        EguiTextCommandSurfaceRootEventChildClass::Search,
        EguiTextCommandSurfaceRootEventChildClass::ContextMenu,
        EguiTextCommandSurfaceRootEventChildClass::StatusBar,
        EguiTextCommandSurfaceRootEventChildClass::DiagnosticsList,
    ];

    for (index, fail_at) in stages.into_iter().enumerate() {
        let batch = EguiTextCommandSurfaceRootEventBatch::new(full_payload(), String::new());
        let transport = batch
            .transport
            .borrow_mut()
            .take()
            .expect("transport exists");
        let mut dispatcher = StageFailingDispatcher {
            fail_at,
            calls: Vec::new(),
        };

        assert_eq!(
            transport.dispatch_once(&mut dispatcher),
            Err(EguiTextCommandSurfaceRootEventBatchDispatchError::Dispatcher(fail_at))
        );
        assert_eq!(dispatcher.calls, stages[..=index]);
    }
}

#[test]
fn opaque_host_effect_runs_once_after_all_child_dispatches() {
    let effect_calls = Rc::new(Cell::new(0));
    let effect_calls_for_handler = Rc::clone(&effect_calls);
    let child_dispatch_complete = Rc::new(Cell::new(false));
    let child_dispatch_complete_for_handler = Rc::clone(&child_dispatch_complete);
    let batch = EguiTextCommandSurfaceRootEventBatch::new(full_payload(), String::new());
    let transport = match batch.transport.borrow_mut().take() {
        Some(transport) => transport,
        None => panic!("transport exists"),
    }
    .with_opaque_host_effect_batch(KucOpaqueHostEffectBatch::from_handler(move || {
        assert!(child_dispatch_complete_for_handler.get());
        effect_calls_for_handler.set(effect_calls_for_handler.get() + 1);
        Ok(())
    }));
    let mut dispatcher = OrderRecorder {
        calls: Vec::new(),
        context_menu_dispatch_complete: child_dispatch_complete,
    };

    assert!(transport.dispatch_once(&mut dispatcher).is_ok());

    assert_eq!(effect_calls.get(), 1);
    assert_eq!(
        dispatcher.calls,
        vec!["text", "toolbar", "floating", "search", "context-menu"]
    );
}

#[test]
fn opaque_host_effect_failure_is_a_dedicated_dispatch_failure() {
    let batch = EguiTextCommandSurfaceRootEventBatch::new(full_payload(), String::new());
    let transport = match batch.transport.borrow_mut().take() {
        Some(transport) => transport,
        None => panic!("transport exists"),
    }
    .with_opaque_host_effect_batch(KucOpaqueHostEffectBatch::from_handler(|| {
        Err(KucOpaqueHostEffectError)
    }));
    let mut dispatcher = OrderRecorder {
        calls: Vec::new(),
        context_menu_dispatch_complete: Rc::new(Cell::new(false)),
    };

    assert_eq!(
        transport.dispatch_once(&mut dispatcher),
        Err(EguiTextCommandSurfaceRootEventBatchDispatchError::OpaqueHostEffect)
    );
    assert_eq!(
        dispatcher.calls,
        vec!["text", "toolbar", "floating", "search", "context-menu"]
    );
}

#[test]
fn child_dispatch_failure_prevents_opaque_host_effect() {
    let effect_calls = Rc::new(Cell::new(0));
    let effect_calls_for_handler = Rc::clone(&effect_calls);
    let batch = EguiTextCommandSurfaceRootEventBatch::new(full_payload(), String::new());
    let transport = match batch.transport.borrow_mut().take() {
        Some(transport) => transport,
        None => panic!("transport exists"),
    }
    .with_opaque_host_effect_batch(KucOpaqueHostEffectBatch::from_handler(move || {
        effect_calls_for_handler.set(effect_calls_for_handler.get() + 1);
        Ok(())
    }));

    assert_eq!(
        transport.dispatch_once(&mut DispatcherError),
        Err(EguiTextCommandSurfaceRootEventBatchDispatchError::Dispatcher(3))
    );
    assert_eq!(effect_calls.get(), 0);
}

#[test]
fn opaque_effect_batch_debug_is_fixed_and_has_no_readback() {
    let batch = KucOpaqueHostEffectBatch::from_handler(|| Ok(()));
    let debug = format!("{batch:?}");
    assert_eq!(debug, "KucOpaqueHostEffectBatch(..)");
    assert!(!debug.contains("handler"));
    assert!(!debug.contains("payload"));
}

#[test]
fn root_event_transport_debug_is_fixed_and_hides_child_payloads() {
    let batch = EguiTextCommandSurfaceRootEventBatch::new(full_payload(), String::new());
    let transport = batch
        .transport
        .borrow_mut()
        .take()
        .expect("transport exists");
    let debug = format!("{transport:?}");
    assert_eq!(debug, "EguiTextCommandSurfaceRootEventTransport(..)");
    assert!(!debug.contains("toolbar-action"));
    assert!(!debug.contains("FocusChanged"));
}

#[test]
fn root_event_batch_debug_exposes_metadata_without_child_payloads() {
    let mut batch = EguiTextCommandSurfaceRootEventBatch::new(
        full_payload(),
        String::from("event-fingerprint"),
    );
    batch.set_root_metadata("opaque-root", 7);

    let debug = format!("{batch:?}");
    assert!(debug.contains("EguiTextCommandSurfaceRootEventBatch"));
    assert!(debug.contains("opaque-root"));
    assert!(debug.contains("event-fingerprint"));
    assert!(debug.contains("event_cardinality"));
    assert!(!debug.contains("toolbar-action"));
    assert!(!debug.contains("FocusChanged"));
}

#[test]
fn dispatch_contract_keeps_transport_opaque_and_neutral() {
    let source = include_str!("../root_event.rs");
    let transport_debug_impl = "impl std::fmt::Debug for EguiTextCommandSurfaceRootEventTransport";
    let public_block = source
        .split_once("pub struct EguiTextCommandSurfaceRootEventTransport")
        .expect("transport declaration exists")
        .1;
    assert!(
        public_block.contains(transport_debug_impl)
            || include_str!("../root_event/transport.rs").contains(transport_debug_impl),
        "transport debug impl exists"
    );
    let public_block =
        if let Some((public_block, _)) = public_block.split_once(transport_debug_impl) {
            public_block
        } else {
            public_block
        };
    for forbidden in [
        "pub fn payload",
        "pub fn text_events",
        "pub fn toolbar_events",
        "pub fn floating_events",
        "pub fn search_events",
        "pub fn context_menu_events",
        "Katana",
        "KLE",
        "AppAction",
        "Document",
    ] {
        assert!(
            !public_block.contains(forbidden),
            "transport should not expose forbidden public accessor or KatanA term: {forbidden}"
        );
    }
}
