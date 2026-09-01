#[test]
fn public_facade_exposes_opaque_effect_router_contract() {
    let mut router = |_: crate::egui::text_command_surface::KucRootEventBatchContext| {
        Ok::<
            Option<crate::egui::text_command_surface::KucOpaqueHostEffectBatch>,
            crate::egui::text_command_surface::KucOpaqueHostEffectError,
        >(Some(
            crate::egui::text_command_surface::KucOpaqueHostEffectBatch::from_handler(|| Ok(())),
        ))
    };
    let context = EguiTextCommandSurfaceRootEventBatch::new(
        RootEventPayload {
            text: Vec::new(),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
            source_address_submissions: Vec::new(),
            ..RootEventPayload::empty()
        },
        String::new(),
    )
    .current_context();

    let effect = crate::egui::text_command_surface::KucRootEffectRouter::route(&mut router, context)
        .ok()
        .flatten();
    assert!(effect.is_some());
}

#[test]
fn router_context_snapshots_all_generic_payloads_without_consuming_root_batch() {
    let search_events = vec![
        CommandChromeSearchEvent::Strip {
            event:
                crate::molecule::structured::SearchControlStripEvent::SearchQueryChanged(
                    String::from("needle"),
                ),
        },
        CommandChromeSearchEvent::Strip {
            event:
                crate::molecule::structured::SearchControlStripEvent::ReplaceValueChanged(
                    String::from("replacement"),
                ),
        },
        CommandChromeSearchEvent::Strip {
            event:
                crate::molecule::structured::SearchControlStripEvent::SearchOptionChanged {
                    option: crate::molecule::structured::SearchOptionKind::MatchCase,
                    enabled: true,
                },
        },
    ];
    let payload = RootEventPayload {
        text: vec![TextSurfaceEvent::FocusChanged(true)],
        toolbar: Some(vec![CommandChromeToolbarEvent::CommandActivated {
            action_id: "toolbar-action".into(),
        }]),
        floating: Some(vec![FloatingCommandToolbarEvent::FocusRetained]),
        search: Some(search_events.clone()),
        context_menu: Some(vec![
            crate::molecule::selection::ContextMenuEvent::Closed {
                reason: crate::molecule::selection::ContextMenuCloseReason::Escape,
            },
        ]),
        source_address_submissions: Vec::new(),
        ..RootEventPayload::empty()
    };
    let batch = EguiTextCommandSurfaceRootEventBatch::new(payload, String::new());
    let before = batch.current_context();
    let mut router = move |context: KucRootEventBatchContext| {
        assert_eq!(
            context.text_events(),
            &[TextSurfaceEvent::FocusChanged(true)]
        );
        assert_eq!(
            context.toolbar_events(),
            &[CommandChromeToolbarEvent::CommandActivated {
                action_id: "toolbar-action".into(),
            }]
        );
        assert_eq!(
            context.floating_events(),
            &[FloatingCommandToolbarEvent::FocusRetained]
        );
        assert_eq!(context.search_events(), search_events.as_slice());
        assert_eq!(
            context.context_menu_events(),
            &[
                crate::molecule::selection::ContextMenuEvent::Closed {
                    reason: crate::molecule::selection::ContextMenuCloseReason::Escape,
                }
            ]
        );
        assert_eq!(
            context
                .class_dispatches()
                .iter()
                .map(|dispatch| dispatch.event_count)
                .collect::<Vec<_>>(),
            vec![1, 1, 1, 3, 1, 0, 0]
        );
        Ok::<Option<KucOpaqueHostEffectBatch>, KucOpaqueHostEffectError>(None)
    };
    assert!(router.route(before.clone()).is_ok());
    assert_eq!(batch.current_context(), before);

    let mut forwarder = CountingForwarder { calls: 0 };
    assert!(batch.forward_once(&mut forwarder).is_ok());
    assert_eq!(forwarder.calls, 1);
}

#[test]
fn router_receives_an_empty_snapshot_when_the_frame_has_no_events() {
    let batch = EguiTextCommandSurfaceRootEventBatch::new(
        RootEventPayload {
            text: Vec::new(),
            toolbar: None,
            floating: None,
            search: None,
            context_menu: None,
            source_address_submissions: Vec::new(),
            ..RootEventPayload::empty()
        },
        String::new(),
    );
    let mut router = move |context: KucRootEventBatchContext| {
        assert!(context.text_events().is_empty());
        assert!(context.toolbar_events().is_empty());
        assert!(context.floating_events().is_empty());
        assert!(context.search_events().is_empty());
        assert!(context.context_menu_events().is_empty());
        assert!(
            context
                .class_dispatches()
                .iter()
                .all(|dispatch| dispatch.event_count == 0)
        );
        Ok::<Option<KucOpaqueHostEffectBatch>, KucOpaqueHostEffectError>(None)
    };
    assert!(router.route(batch.current_context()).is_ok());
}

#[test]
fn search_event_is_detached_from_root_payload_and_counted_once() {
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
    let detached = batch.detach_search_events().expect("first detach succeeds");
    assert_eq!(detached.len(), 1);
    assert_eq!(batch.event_cardinality(), 0);
    assert_eq!(
        batch.detach_search_events(),
        Err(EguiTextCommandSurfaceRootEventSearchDetachError::AlreadyDetached)
    );
}

#[test]
fn exclusive_search_detach_removes_same_frame_text_without_changing_generic_detach() {
    let payload = || RootEventPayload {
        text: vec![TextSurfaceEvent::FocusChanged(true)],
        toolbar: None,
        floating: None,
        search: Some(vec![CommandChromeSearchEvent::CloseRequested]),
        context_menu: None,
        source_address_submissions: Vec::new(),
        ..RootEventPayload::empty()
    };
    let exclusive = EguiTextCommandSurfaceRootEventBatch::new(payload(), String::new());
    assert_eq!(
        exclusive
            .detach_search_events_exclusively()
            .expect("exclusive detach succeeds")
            .len(),
        1
    );
    assert_eq!(exclusive.event_cardinality(), 0);

    let generic = EguiTextCommandSurfaceRootEventBatch::new(payload(), String::new());
    assert_eq!(
        generic
            .detach_search_events()
            .expect("generic detach succeeds")
            .len(),
        1
    );
    assert_eq!(generic.event_cardinality(), 1);
}

pub(super) struct CountingForwarder {
    pub(super) calls: usize,
}

impl KucRootEventBatchForwarder for CountingForwarder {
    type Error = ();

    fn forward_root_event_batch(
        &mut self,
        _transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error> {
        self.calls += 1;
        Ok(())
    }
}
