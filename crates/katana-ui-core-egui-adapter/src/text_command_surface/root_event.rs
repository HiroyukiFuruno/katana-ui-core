//! Opaque event-batch projection for the retained root.

mod api;
mod batch;
mod context;
mod dispatcher;
mod receipt;
mod transport;

const ROOT_EVENT_CLASS_COUNT: usize = 7;
const ROOT_EVENT_SEARCH_INDEX: usize = 3;
const ROOT_EVENT_CONTEXT_MENU_INDEX: usize = 4;
const ROOT_EVENT_STATUS_BAR_INDEX: usize = 5;
const ROOT_EVENT_DIAGNOSTICS_INDEX: usize = 6;

pub(super) fn build_event_batch(
    output: &mut super::super::types::EguiTextCommandSurfaceOutput,
    source_address_submission_port: Option<
        super::super::source_address_projection_lease::SourceAddressSubmissionPortHandle,
    >,
) -> Result<EguiTextCommandSurfaceRootEventBatch, String> {
    dispatcher::RootEventDispatcher::build_event_batch(output, source_address_submission_port)
}

use super::super::source_address_projection_lease::{
    SourceAddressSubmissionPortError, SourceAddressSubmissionPortHandle,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::molecule::structured::source_address_strip::SourceAddressSubmission;
use katana_ui_core::molecule::{DiagnosticsListEvent, StatusBarEvent};
use katana_ui_core::text_surface::TextSurfaceEvent;
use std::cell::{Cell, RefCell};

/// Generic KUC callback used to forward one opaque root event transport.
pub trait KucRootEventBatchForwarder {
    type Error;

    fn forward_root_event_batch(
        &mut self,
        transport: EguiTextCommandSurfaceRootEventTransport,
    ) -> Result<(), Self::Error>;
}

/// Host-owned opaque one-shot effect batch.
pub struct KucOpaqueHostEffectBatch {
    effect: Option<Box<dyn FnOnce() -> Result<(), KucOpaqueHostEffectError>>>,
}

/// Generic current-root event information supplied to a host effect router.
/// Event payloads and child models remain private to KUC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KucRootEventBatchContext {
    root_identity: String,
    state_revision: u64,
    correlation_fingerprint: String,
    class_dispatches: [EguiTextCommandSurfaceRootEventClassDispatch; ROOT_EVENT_CLASS_COUNT],
    text_events: Vec<TextSurfaceEvent>,
    toolbar_events: Vec<CommandChromeToolbarEvent>,
    floating_events: Vec<FloatingCommandToolbarEvent>,
    search_events: Vec<CommandChromeSearchEvent>,
    context_menu_events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
    source_address_submission_count: usize,
    status_bar_events: Vec<StatusBarEvent>,
    diagnostics_list_events: Vec<DiagnosticsListEvent>,
}

/// Generic KUC router for a non-wire host effect.
pub trait KucRootEffectRouter {
    fn route(
        &mut self,
        context: KucRootEventBatchContext,
    ) -> Result<Option<KucOpaqueHostEffectBatch>, KucOpaqueHostEffectError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KucOpaqueHostEffectError;

struct RootEventPayload {
    text: Vec<TextSurfaceEvent>,
    toolbar: Option<Vec<CommandChromeToolbarEvent>>,
    floating: Option<Vec<FloatingCommandToolbarEvent>>,
    search: Option<Vec<CommandChromeSearchEvent>>,
    context_menu: Option<Vec<katana_ui_core::molecule::selection::ContextMenuEvent>>,
    status_bar: Option<Vec<StatusBarEvent>>,
    diagnostics_list: Option<Vec<DiagnosticsListEvent>>,
    source_address_submissions: Vec<SourceAddressSubmission>,
}

/// Opaque one-shot transport token. It has no consumer-visible semantic accessors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KucOpaqueHostEffectAttachError {
    AlreadyConsumed,
    AlreadyAttached,
}

pub struct EguiTextCommandSurfaceRootEventTransport {
    payload: RootEventPayload,
    opaque_host_effect_batch: Option<KucOpaqueHostEffectBatch>,
    source_address_submission_port: Option<SourceAddressSubmissionPortHandle>,
}

/// Deterministic receipt returned after a root event transport was forwarded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EguiTextCommandSurfaceRootEventForwardingReceipt {
    root_identity: String,
    state_revision: u64,
    correlation_fingerprint: String,
    event_batch_fingerprint: String,
    consumed_once: bool,
    event_cardinality: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EguiTextCommandSurfaceRootEventDispatchReceipt {
    class_dispatches: [EguiTextCommandSurfaceRootEventClassDispatch; ROOT_EVENT_CLASS_COUNT],
}

#[derive(Debug, PartialEq, Eq)]
pub enum EguiTextCommandSurfaceRootEventBatchDispatchError<DispatcherError> {
    AlreadyConsumed,
    Dispatcher(DispatcherError),
    OpaqueHostEffect,
    SourceAddressPort(SourceAddressSubmissionPortError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EguiTextCommandSurfaceRootEventChildClass {
    Text,
    Toolbar,
    Floating,
    Search,
    ContextMenu,
    StatusBar,
    DiagnosticsList,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EguiTextCommandSurfaceRootEventClassDispatch {
    pub child_class: EguiTextCommandSurfaceRootEventChildClass,
    pub event_count: usize,
}

/// Typed failure for one-shot root event forwarding.
#[derive(Debug)]
pub enum EguiTextCommandSurfaceRootEventBatchForwardError<ForwarderError> {
    AlreadyConsumed,
    Forwarder(ForwarderError),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum EguiTextCommandSurfaceRootEventSearchDetachError {
    AlreadyConsumed,
    AlreadyDetached,
    Serialization,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum EguiTextCommandSurfaceRootEventCommandDetachError {
    AlreadyConsumed,
    AlreadyDetached,
    Serialization,
}

/// Sealed root event batch. Its payload can leave KUC only through `forward_once`.
pub struct EguiTextCommandSurfaceRootEventBatch {
    transport: std::cell::RefCell<Option<EguiTextCommandSurfaceRootEventTransport>>,
    root_identity: String,
    state_revision: u64,
    correlation_fingerprint: RefCell<String>,
    event_batch_fingerprint: RefCell<String>,
    event_cardinality: Cell<usize>,
    search_detached: Cell<bool>,
    command_detached: Cell<bool>,
    context_menu_detached: Cell<bool>,
}

/// Generic callback used to dispatch one-time root event payload per child class.
pub trait KucRootEventBatchDispatcher {
    type Error;

    fn dispatch_text_events(&mut self, events: Vec<TextSurfaceEvent>) -> Result<(), Self::Error>;
    fn dispatch_toolbar_events(
        &mut self,
        events: Vec<CommandChromeToolbarEvent>,
    ) -> Result<(), Self::Error>;
    fn dispatch_floating_events(
        &mut self,
        events: Vec<FloatingCommandToolbarEvent>,
    ) -> Result<(), Self::Error>;
    fn dispatch_search_events(
        &mut self,
        events: Vec<CommandChromeSearchEvent>,
    ) -> Result<(), Self::Error>;
    fn dispatch_context_menu_events(
        &mut self,
        events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
    ) -> Result<(), Self::Error>;

    fn dispatch_status_bar_events(
        &mut self,
        _events: Vec<StatusBarEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn dispatch_diagnostics_list_events(
        &mut self,
        _events: Vec<DiagnosticsListEvent>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn consume_opaque_host_effect_batch(
        &mut self,
        effect_batch: KucOpaqueHostEffectBatch,
    ) -> Result<(), KucOpaqueHostEffectError> {
        effect_batch.consume_once()
    }
}

#[cfg(test)]
mod tests {
    use super::dispatcher::{RootEventEnvelope, RootEventFingerprint};
    use super::*;
    use crate::text_command_surface::source_address_projection_lease::SourceAddressSubmissionPort;
    use std::rc::Rc;

    #[test]
    fn public_facade_exposes_opaque_effect_router_contract() {
        let mut router = |_: crate::text_command_surface::KucRootEventBatchContext| {
            Ok::<
                Option<crate::text_command_surface::KucOpaqueHostEffectBatch>,
                crate::text_command_surface::KucOpaqueHostEffectError,
            >(Some(
                crate::text_command_surface::KucOpaqueHostEffectBatch::from_handler(|| Ok(())),
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

        let effect = crate::text_command_surface::KucRootEffectRouter::route(&mut router, context)
            .ok()
            .flatten();
        assert!(effect.is_some());
    }

    #[test]
    fn router_context_snapshots_all_generic_payloads_without_consuming_root_batch() {
        let search_events = vec![
            CommandChromeSearchEvent::Strip {
                event: katana_ui_core::molecule::structured::SearchControlStripEvent::SearchQueryChanged(
                    String::from("needle"),
                ),
            },
            CommandChromeSearchEvent::Strip {
                event: katana_ui_core::molecule::structured::SearchControlStripEvent::ReplaceValueChanged(
                    String::from("replacement"),
                ),
            },
            CommandChromeSearchEvent::Strip {
                event: katana_ui_core::molecule::structured::SearchControlStripEvent::SearchOptionChanged {
                    option: katana_ui_core::molecule::structured::SearchOptionKind::MatchCase,
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
                katana_ui_core::molecule::selection::ContextMenuEvent::Closed {
                    reason: katana_ui_core::molecule::selection::ContextMenuCloseReason::Escape,
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
                    katana_ui_core::molecule::selection::ContextMenuEvent::Closed {
                        reason: katana_ui_core::molecule::selection::ContextMenuCloseReason::Escape,
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

    struct CountingForwarder {
        calls: usize,
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

        let batch = EguiTextCommandSurfaceRootEventBatch::new(
            payload,
            String::from("pre-detach fingerprint"),
        );
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

    struct OrderRecorder {
        calls: Vec<&'static str>,
        context_menu_dispatch_complete: Rc<Cell<bool>>,
    }

    struct RecordingSourcePort {
        received: Rc<RefCell<Vec<String>>>,
        fail: bool,
    }

    impl SourceAddressSubmissionPort for RecordingSourcePort {
        fn forward_submission(
            &mut self,
            submission: SourceAddressSubmission,
        ) -> Result<(), SourceAddressSubmissionPortError> {
            if self.fail {
                return Err(SourceAddressSubmissionPortError::Rejected);
            }
            self.received.borrow_mut().push(submission.into_draft());
            Ok(())
        }
    }

    fn source_submission() -> SourceAddressSubmission {
        let mut strip = katana_ui_core::molecule::structured::source_address_strip::SourceAddressStrip::new(
            katana_ui_core::molecule::structured::source_address_strip::SourceAddressPresentation::new(
                "表示", "ツールチップ", "アクセシビリティ",
            ),
        );
        let _ = strip.apply_action(
            katana_ui_core::molecule::structured::source_address_strip::SourceAddressAction::SetDraft(
                "opaque-source-draft".to_owned(),
            ),
        );
        match strip.apply_action(
            katana_ui_core::molecule::structured::source_address_strip::SourceAddressAction::Submit,
        ) {
            Some(
                katana_ui_core::molecule::structured::source_address_strip::SourceAddressEvent::Submitted(
                    submission,
                ),
            ) => submission,
            _ => panic!("enabled source address should submit"),
        }
    }

    impl KucRootEventBatchDispatcher for OrderRecorder {
        type Error = ();

        fn dispatch_text_events(
            &mut self,
            events: Vec<TextSurfaceEvent>,
        ) -> Result<(), Self::Error> {
            assert_eq!(events.len(), 1);
            self.calls.push("text");
            Ok(())
        }

        fn dispatch_toolbar_events(
            &mut self,
            events: Vec<CommandChromeToolbarEvent>,
        ) -> Result<(), Self::Error> {
            assert_eq!(events.len(), 1);
            self.calls.push("toolbar");
            Ok(())
        }

        fn dispatch_floating_events(
            &mut self,
            events: Vec<FloatingCommandToolbarEvent>,
        ) -> Result<(), Self::Error> {
            assert_eq!(events.len(), 1);
            self.calls.push("floating");
            Ok(())
        }

        fn dispatch_search_events(
            &mut self,
            events: Vec<CommandChromeSearchEvent>,
        ) -> Result<(), Self::Error> {
            assert_eq!(events.len(), 1);
            self.calls.push("search");
            Ok(())
        }

        fn dispatch_context_menu_events(
            &mut self,
            events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
        ) -> Result<(), Self::Error> {
            assert_eq!(events.len(), 1);
            self.calls.push("context-menu");
            self.context_menu_dispatch_complete.set(true);
            Ok(())
        }

        fn consume_opaque_host_effect_batch(
            &mut self,
            effect_batch: KucOpaqueHostEffectBatch,
        ) -> Result<(), KucOpaqueHostEffectError> {
            effect_batch
                .consume_once()
                .map_err(|_| KucOpaqueHostEffectError)
        }
    }

    fn full_payload() -> RootEventPayload {
        RootEventPayload {
            text: vec![TextSurfaceEvent::FocusChanged(true)],
            toolbar: Some(vec![CommandChromeToolbarEvent::CommandActivated {
                action_id: "toolbar-action".into(),
            }]),
            floating: Some(vec![FloatingCommandToolbarEvent::FocusRetained]),
            search: Some(vec![CommandChromeSearchEvent::CloseRequested]),
            context_menu: Some(vec![
                katana_ui_core::molecule::selection::ContextMenuEvent::Closed {
                    reason: katana_ui_core::molecule::selection::ContextMenuCloseReason::Escape,
                },
            ]),
            source_address_submissions: Vec::new(),
            ..RootEventPayload::empty()
        }
    }

    #[test]
    fn source_submission_requires_a_dedicated_port_and_forwards_once() {
        let mut payload = full_payload();
        payload.source_address_submissions.push(source_submission());
        let batch = EguiTextCommandSurfaceRootEventBatch::new(payload, String::new());
        let transport = batch
            .transport
            .borrow_mut()
            .take()
            .expect("transport exists");
        let received = Rc::new(RefCell::new(Vec::new()));
        let mut dispatcher = OrderRecorder {
            calls: Vec::new(),
            context_menu_dispatch_complete: Rc::new(Cell::new(false)),
        };
        let receipt = transport
            .with_source_address_submission_port(Some(SourceAddressSubmissionPortHandle::new(
                RecordingSourcePort {
                    received: Rc::clone(&received),
                    fail: false,
                },
            )))
            .dispatch_once(&mut dispatcher)
            .expect("source port dispatch succeeds");
        assert_eq!(receipt.class_dispatches().len(), 7);
        assert_eq!(received.borrow().as_slice(), &["opaque-source-draft"]);
    }

    #[test]
    fn source_submission_port_handle_survives_distinct_one_shot_transports() {
        let received = Rc::new(RefCell::new(Vec::new()));
        let port = SourceAddressSubmissionPortHandle::new(RecordingSourcePort {
            received: Rc::clone(&received),
            fail: false,
        });

        for _ in 0..2 {
            let mut payload = full_payload();
            payload.source_address_submissions.push(source_submission());
            let transport = EguiTextCommandSurfaceRootEventTransport {
                payload,
                opaque_host_effect_batch: None,
                source_address_submission_port: Some(port.clone()),
            };
            let mut dispatcher = OrderRecorder {
                calls: Vec::new(),
                context_menu_dispatch_complete: Rc::new(Cell::new(false)),
            };
            transport
                .dispatch_once(&mut dispatcher)
                .expect("each one-shot transport forwards once");
        }

        assert_eq!(
            received.borrow().as_slice(),
            &["opaque-source-draft", "opaque-source-draft"]
        );
    }

    #[test]
    fn source_submission_missing_or_rejected_port_fails_closed() {
        let mut payload = full_payload();
        payload.source_address_submissions.push(source_submission());
        let transport = EguiTextCommandSurfaceRootEventTransport {
            payload,
            opaque_host_effect_batch: None,
            source_address_submission_port: None,
        };
        let mut dispatcher = OrderRecorder {
            calls: Vec::new(),
            context_menu_dispatch_complete: Rc::new(Cell::new(false)),
        };
        assert!(matches!(
            transport.dispatch_once(&mut dispatcher),
            Err(
                EguiTextCommandSurfaceRootEventBatchDispatchError::SourceAddressPort(
                    SourceAddressSubmissionPortError::Rejected
                )
            )
        ));

        let mut payload = full_payload();
        payload.source_address_submissions.push(source_submission());
        let transport = EguiTextCommandSurfaceRootEventTransport {
            payload,
            opaque_host_effect_batch: None,
            source_address_submission_port: Some(SourceAddressSubmissionPortHandle::new(
                RecordingSourcePort {
                    received: Rc::new(RefCell::new(Vec::new())),
                    fail: true,
                },
            )),
        };
        let mut dispatcher = OrderRecorder {
            calls: Vec::new(),
            context_menu_dispatch_complete: Rc::new(Cell::new(false)),
        };
        assert!(matches!(
            transport.dispatch_once(&mut dispatcher),
            Err(
                EguiTextCommandSurfaceRootEventBatchDispatchError::SourceAddressPort(
                    SourceAddressSubmissionPortError::Rejected
                )
            )
        ));
    }

    #[test]
    fn transport_dispatches_all_classes_in_fixed_order_and_returns_counts() {
        let batch = EguiTextCommandSurfaceRootEventBatch::new(full_payload(), String::new());
        let transport = batch
            .transport
            .borrow_mut()
            .take()
            .expect("transport exists");
        let mut dispatcher = OrderRecorder {
            calls: Vec::new(),
            context_menu_dispatch_complete: Rc::new(Cell::new(false)),
        };
        let receipt = transport
            .dispatch_once(&mut dispatcher)
            .expect("dispatch_once succeeds");
        assert_eq!(
            dispatcher.calls,
            vec!["text", "toolbar", "floating", "search", "context-menu"]
        );
        assert_eq!(
            receipt.class_dispatches(),
            &[
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::Text,
                    event_count: 1,
                },
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::Toolbar,
                    event_count: 1,
                },
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::Floating,
                    event_count: 1,
                },
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::Search,
                    event_count: 1,
                },
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::ContextMenu,
                    event_count: 1,
                },
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::StatusBar,
                    event_count: 0,
                },
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::DiagnosticsList,
                    event_count: 0,
                },
            ]
        );
        assert_eq!(
            receipt
                .class_dispatches()
                .iter()
                .map(|dispatch| dispatch.event_count)
                .collect::<Vec<_>>(),
            vec![
                receipt.text_count(),
                receipt.toolbar_count(),
                receipt.floating_count(),
                receipt.search_count(),
                receipt.context_menu_count(),
                receipt.status_bar_count(),
                receipt.diagnostics_list_count(),
            ]
        );
    }

    struct DispatcherError;

    impl KucRootEventBatchDispatcher for DispatcherError {
        type Error = usize;

        fn dispatch_text_events(
            &mut self,
            _events: Vec<TextSurfaceEvent>,
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        fn dispatch_toolbar_events(
            &mut self,
            _events: Vec<CommandChromeToolbarEvent>,
        ) -> Result<(), Self::Error> {
            Err(3)
        }

        fn dispatch_floating_events(
            &mut self,
            _events: Vec<FloatingCommandToolbarEvent>,
        ) -> Result<(), Self::Error> {
            panic!("must fail closed before floating dispatch")
        }

        fn dispatch_search_events(
            &mut self,
            _events: Vec<CommandChromeSearchEvent>,
        ) -> Result<(), Self::Error> {
            panic!("must fail closed before search dispatch")
        }

        fn dispatch_context_menu_events(
            &mut self,
            _events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
        ) -> Result<(), Self::Error> {
            panic!("must fail closed before context-menu dispatch")
        }

        fn consume_opaque_host_effect_batch(
            &mut self,
            _effect_batch: KucOpaqueHostEffectBatch,
        ) -> Result<(), KucOpaqueHostEffectError> {
            panic!("must not consume effect after child dispatch failure")
        }
    }

    #[test]
    fn transport_dispatch_propagates_typed_error_and_does_not_continue() {
        let batch = EguiTextCommandSurfaceRootEventBatch::new(full_payload(), String::new());
        let transport = batch
            .transport
            .borrow_mut()
            .take()
            .expect("transport exists");
        assert_eq!(
            transport.dispatch_once(&mut DispatcherError),
            Err(EguiTextCommandSurfaceRootEventBatchDispatchError::Dispatcher(3))
        );
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
    fn dispatch_contract_keeps_transport_opaque_and_neutral() {
        let source = include_str!("root_event.rs");
        let public_block = source
            .split_once("pub struct EguiTextCommandSurfaceRootEventTransport")
            .expect("transport declaration exists")
            .1
            .split_once("impl std::fmt::Debug for EguiTextCommandSurfaceRootEventTransport")
            .expect("transport debug impl exists")
            .0;
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

    struct LifecycleDispatcher {
        calls: Vec<&'static str>,
        effect_calls: usize,
        effect_failed: bool,
    }

    impl KucRootEventBatchDispatcher for LifecycleDispatcher {
        type Error = &'static str;

        fn dispatch_text_events(
            &mut self,
            _events: Vec<TextSurfaceEvent>,
        ) -> Result<(), Self::Error> {
            self.calls.push("text");
            Ok(())
        }

        fn dispatch_toolbar_events(
            &mut self,
            _events: Vec<CommandChromeToolbarEvent>,
        ) -> Result<(), Self::Error> {
            self.calls.push("toolbar");
            Ok(())
        }

        fn dispatch_floating_events(
            &mut self,
            _events: Vec<FloatingCommandToolbarEvent>,
        ) -> Result<(), Self::Error> {
            self.calls.push("floating");
            Ok(())
        }

        fn dispatch_search_events(
            &mut self,
            events: Vec<CommandChromeSearchEvent>,
        ) -> Result<(), Self::Error> {
            self.calls.push("search");
            assert!(events.iter().any(|event| matches!(
                event,
                CommandChromeSearchEvent::Strip {
                    event: katana_ui_core::molecule::structured::SearchControlStripEvent::SearchQueryChanged(value)
                } if value == "needle⭐️"
            )));
            Ok(())
        }

        fn dispatch_context_menu_events(
            &mut self,
            _events: Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
        ) -> Result<(), Self::Error> {
            self.calls.push("context-menu");
            Ok(())
        }

        fn consume_opaque_host_effect_batch(
            &mut self,
            effect_batch: KucOpaqueHostEffectBatch,
        ) -> Result<(), KucOpaqueHostEffectError> {
            self.calls.push("effect");
            self.effect_calls += 1;
            let result = effect_batch.consume_once();
            if self.effect_failed {
                return Err(KucOpaqueHostEffectError);
            }
            result
        }
    }

    impl KucRootEventBatchForwarder for LifecycleDispatcher {
        type Error = EguiTextCommandSurfaceRootEventBatchDispatchError<&'static str>;

        fn forward_root_event_batch(
            &mut self,
            transport: EguiTextCommandSurfaceRootEventTransport,
        ) -> Result<(), Self::Error> {
            transport.dispatch_once(self).map(|_| ())
        }
    }

    #[test]
    fn retained_root_routes_actual_search_payload_after_forwarding_and_consumes_effect_once()
    -> Result<(), Box<dyn std::error::Error>> {
        use std::cell::{Cell, RefCell};
        use std::rc::Rc;

        use crate::text_command_surface::{
            EguiTextCommandSurfaceCommandFamilyProjection,
            EguiTextCommandSurfaceHostProjectionEncoder, EguiTextCommandSurfaceHostProjectionLease,
            EguiTextCommandSurfaceHostRootFrame, EguiTextCommandSurfacePresentation,
            EguiTextCommandSurfaceRootFactory, EguiTextCommandSurfaceSearchPresentation,
            TextCommandSurfaceStyle,
        };
        use katana_ui_core::molecule::command_chrome::{
            CommandChromeCapability, CommandChromeSearchPresentation, CommandChromeText,
            SearchControlCapabilities, SearchControlIcons, SearchControlStrings,
            SearchResultSummaryTemplate,
        };
        use katana_ui_core::molecule::structured::{ReplaceMode, SearchOptions};
        use katana_ui_core::render_model::UiStateId;
        use katana_ui_core::text_surface::{
            TextSurfaceAccessibilityLabels, TextSurfacePresentation,
        };

        fn label(value: &str) -> CommandChromeText {
            CommandChromeText::new(value, value, value)
        }

        fn search_presentation() -> EguiTextCommandSurfaceSearchPresentation {
            EguiTextCommandSurfaceSearchPresentation {
                state_id: UiStateId::new("root-lifecycle-search"),
                label: String::from("検索と置換"),
                value: CommandChromeSearchPresentation {
                    query: String::from("needle"),
                    options: SearchOptions::default(),
                    result_count: Some(1),
                    active_index: Some(0),
                    replace_mode: ReplaceMode::Visible,
                    replace_value: String::from("replacement"),
                    strings: SearchControlStrings {
                        strip: label("検索と置換"),
                        query: label("検索語"),
                        replace: label("置換"),
                        match_case: label("大文字小文字"),
                        whole_word: label("単語"),
                        use_regex: label("正規表現"),
                        previous: label("前へ"),
                        next: label("次へ"),
                        replace_one: label("置換"),
                        replace_all: label("すべて置換"),
                        close: label("閉じる"),
                        result_summary: SearchResultSummaryTemplate {
                            empty: String::from("検索待機"),
                            zero_results: String::from("一致なし"),
                            single_result: String::from("1件"),
                            indexed_result: String::from("{active} / {count}"),
                            count_results: String::from("{count}件"),
                        },
                    },
                    capabilities: SearchControlCapabilities {
                        regex: CommandChromeCapability::available(),
                        replace: CommandChromeCapability::available(),
                        navigation: CommandChromeCapability::available(),
                        close: CommandChromeCapability::available(),
                    },
                    icons: SearchControlIcons::default(),
                },
            }
        }

        let presentation = EguiTextCommandSurfacePresentation {
            text_state_id: Some(UiStateId::new("root-lifecycle-text")),
            text: TextSurfacePresentation {
                value: String::from("body"),
                selection_start: 0,
                selection_end: 0,
                spans: Vec::new(),
                annotations: Vec::new(),
                automatic_gutter: None,
                accessibility_label: String::from("本文"),
                accessibility_actions: TextSurfaceAccessibilityLabels::new(),
                context_target_label: None,
                disabled_reason: None,
                readonly: false,
                disabled: false,
                ime_enabled: true,
                scroll_request: None,
                focus_request: None,
            },
            toolbar: None,
            floating: None,
            search: Some(search_presentation()),
            context_menu: None,
        };
        let token = EguiTextCommandSurfaceHostProjectionEncoder::token_with_command_families(
            1,
            b"root-lifecycle-target".to_vec(),
            presentation,
            TextCommandSurfaceStyle::standard()?,
            EguiTextCommandSurfaceCommandFamilyProjection::new(None, None),
        )?;

        let router_seen_query = Rc::new(RefCell::new(None::<String>));
        let host_mutations = Rc::new(Cell::new(0));
        let router_seen_query_for_router = Rc::clone(&router_seen_query);
        let host_mutations_for_effect = Rc::clone(&host_mutations);
        let lease = EguiTextCommandSurfaceHostProjectionLease::new(
            token,
            move |context: KucRootEventBatchContext| {
                let query = context.search_events().iter().find_map(|event| match event {
                CommandChromeSearchEvent::Strip {
                    event: katana_ui_core::molecule::structured::SearchControlStripEvent::SearchQueryChanged(value),
                } => Some(value.clone()),
                _ => None,
            });
                if let Some(query) = query {
                    *router_seen_query_for_router.borrow_mut() = Some(query);
                    let host_mutations = Rc::clone(&host_mutations_for_effect);
                    return Ok(Some(KucOpaqueHostEffectBatch::from_handler(move || {
                        host_mutations.set(host_mutations.get() + 1);
                        Ok(())
                    })));
                }
                Ok(None)
            },
        );
        let mut root = EguiTextCommandSurfaceRootFactory::new().retain_with_lease(lease)?;
        let context = egui::Context::default();
        let mut first_output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let _ = root.show(ui);
                });
            },
        );
        first_output.textures_delta.clear();
        let mut frame: Option<Result<EguiTextCommandSurfaceHostRootFrame, _>> = None;
        let mut output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                events: vec![
                    egui::Event::PointerButton {
                        pos: egui::pos2(100.0, 580.0),
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        modifiers: egui::Modifiers::default(),
                    },
                    egui::Event::PointerButton {
                        pos: egui::pos2(100.0, 580.0),
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        modifiers: egui::Modifiers::default(),
                    },
                    egui::Event::Text(String::from("⭐️")),
                ],
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    frame = Some(root.show(ui));
                });
            },
        );
        output.textures_delta.clear();
        let frame = frame.ok_or_else(|| "root frame missing".to_owned())??;
        assert_eq!(router_seen_query.borrow().as_deref(), Some("needle⭐️"));
        assert_eq!(host_mutations.get(), 0);

        let mut dispatcher = LifecycleDispatcher {
            calls: Vec::new(),
            effect_calls: 0,
            effect_failed: false,
        };
        assert!(frame.forward_events_once(&mut dispatcher).is_ok());
        assert_eq!(
            dispatcher.calls,
            vec![
                "text",
                "toolbar",
                "floating",
                "search",
                "context-menu",
                "effect"
            ]
        );
        assert_eq!(dispatcher.effect_calls, 1);
        assert_eq!(host_mutations.get(), 1);
        assert!(frame.forward_events_once(&mut dispatcher).is_err());
        assert_eq!(dispatcher.effect_calls, 1);
        assert_eq!(host_mutations.get(), 1);

        let failing_effect =
            KucOpaqueHostEffectBatch::from_handler(|| Err(KucOpaqueHostEffectError));
        let mut failing_dispatcher = LifecycleDispatcher {
            calls: Vec::new(),
            effect_calls: 0,
            effect_failed: true,
        };
        assert!(
            failing_dispatcher
                .consume_opaque_host_effect_batch(failing_effect)
                .is_err()
        );
        assert_eq!(failing_dispatcher.calls, vec!["effect"]);

        Ok(())
    }
}
