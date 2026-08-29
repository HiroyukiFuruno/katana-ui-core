//! Opaque event-batch lifecycle for the retained root.

use super::{
    EguiTextCommandSurfaceRootEventTransport,
    root_event_contract::{KucOpaqueHostEffectBatch, KucRootEventBatchForwarder},
    root_event_fingerprint::RootEventCorrelationFingerprint,
    root_event_payload::RootEventPayload,
    root_event_types::{
        EguiTextCommandSurfaceRootEventBatch, EguiTextCommandSurfaceRootEventBatchForwardError,
        EguiTextCommandSurfaceRootEventChildClass, EguiTextCommandSurfaceRootEventClassDispatch,
        EguiTextCommandSurfaceRootEventForwardingReceipt, KucOpaqueHostEffectAttachError,
        KucRootEventBatchContext, ROOT_EVENT_CHILD_CLASS_COUNT,
    },
};
use std::cell::{Cell, RefCell};

impl EguiTextCommandSurfaceRootEventBatch {
    pub(crate) fn current_context(&self) -> KucRootEventBatchContext {
        let transport = self.transport.borrow();
        let (text_events, toolbar_events, floating_events, search_events, context_menu_events) =
            transport.as_ref().map_or_else(
                || (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
                |transport| {
                    (
                        transport.payload.text.clone(),
                        transport.payload.toolbar.clone().unwrap_or_default(),
                        transport.payload.floating.clone().unwrap_or_default(),
                        transport.payload.search.clone().unwrap_or_default(),
                        transport.payload.context_menu.clone().unwrap_or_default(),
                    )
                },
            );
        drop(transport);
        KucRootEventBatchContext {
            root_identity: self.root_identity.clone(),
            state_revision: self.state_revision,
            correlation_fingerprint: self.correlation_fingerprint.borrow().clone(),
            class_dispatches: self.class_dispatches(),
            text_events,
            toolbar_events,
            floating_events,
            search_events,
            context_menu_events,
        }
    }

    pub(crate) fn attach_opaque_host_effect_batch(
        &self,
        effect_batch: KucOpaqueHostEffectBatch,
    ) -> Result<(), KucOpaqueHostEffectAttachError> {
        let mut transport = self.transport.borrow_mut();
        let transport = transport
            .as_mut()
            .ok_or(KucOpaqueHostEffectAttachError::AlreadyConsumed)?;
        if transport.opaque_host_effect_batch.is_some() {
            return Err(KucOpaqueHostEffectAttachError::AlreadyAttached);
        }
        transport.opaque_host_effect_batch = Some(effect_batch);
        Ok(())
    }

    fn class_dispatches(
        &self,
    ) -> [EguiTextCommandSurfaceRootEventClassDispatch; ROOT_EVENT_CHILD_CLASS_COUNT] {
        let transport = self.transport.borrow();
        let Some(transport) = transport.as_ref() else {
            return Self::empty_class_dispatches();
        };
        [
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::Text,
                event_count: transport.payload.text.len(),
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::Toolbar,
                event_count: transport.payload.toolbar.as_ref().map_or(0, Vec::len),
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::Floating,
                event_count: transport.payload.floating.as_ref().map_or(0, Vec::len),
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::Search,
                event_count: transport.payload.search.as_ref().map_or(0, Vec::len),
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::ContextMenu,
                event_count: transport.payload.context_menu.as_ref().map_or(0, Vec::len),
            },
        ]
    }

    fn empty_class_dispatches()
    -> [EguiTextCommandSurfaceRootEventClassDispatch; ROOT_EVENT_CHILD_CLASS_COUNT] {
        [
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::Text,
                event_count: 0,
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::Toolbar,
                event_count: 0,
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::Floating,
                event_count: 0,
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::Search,
                event_count: 0,
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::ContextMenu,
                event_count: 0,
            },
        ]
    }

    /// Forwards this batch once to a generic KUC callback.
    pub fn forward_once<Forwarder>(
        &self,
        forwarder: &mut Forwarder,
    ) -> Result<
        EguiTextCommandSurfaceRootEventForwardingReceipt,
        EguiTextCommandSurfaceRootEventBatchForwardError<Forwarder::Error>,
    >
    where
        Forwarder: KucRootEventBatchForwarder,
    {
        let transport = self
            .transport
            .borrow_mut()
            .take()
            .ok_or(EguiTextCommandSurfaceRootEventBatchForwardError::AlreadyConsumed)?;
        let receipt = self.receipt();
        forwarder
            .forward_root_event_batch(transport)
            .map_err(EguiTextCommandSurfaceRootEventBatchForwardError::Forwarder)?;
        Ok(receipt)
    }

    pub(crate) fn new(payload: RootEventPayload, event_batch_fingerprint: String) -> Self {
        let event_cardinality = payload.event_cardinality();
        let correlation_fingerprint =
            RootEventCorrelationFingerprint::compose("", 0, &event_batch_fingerprint);
        Self {
            transport: RefCell::new(Some(EguiTextCommandSurfaceRootEventTransport {
                payload,
                opaque_host_effect_batch: None,
            })),
            root_identity: String::new(),
            state_revision: 0,
            correlation_fingerprint: RefCell::new(correlation_fingerprint),
            event_batch_fingerprint: RefCell::new(event_batch_fingerprint),
            event_cardinality: Cell::new(event_cardinality),
            search_detached: Cell::new(false),
            command_detached: Cell::new(false),
            context_menu_detached: Cell::new(false),
        }
    }

    pub(crate) fn has_events(&self) -> bool {
        self.event_cardinality.get() != 0
    }

    #[cfg(test)]
    pub(crate) fn event_cardinality(&self) -> usize {
        self.event_cardinality.get()
    }

    pub(crate) fn set_root_metadata(&mut self, root_identity: &str, state_revision: u64) {
        self.root_identity = root_identity.to_owned();
        self.state_revision = state_revision;
        self.refresh_correlation_fingerprint();
    }

    pub(crate) fn refresh_correlation_fingerprint(&self) {
        *self.correlation_fingerprint.borrow_mut() = RootEventCorrelationFingerprint::compose(
            &self.root_identity,
            self.state_revision,
            &self.event_batch_fingerprint.borrow(),
        );
    }

    fn receipt(&self) -> EguiTextCommandSurfaceRootEventForwardingReceipt {
        EguiTextCommandSurfaceRootEventForwardingReceipt {
            root_identity: self.root_identity.clone(),
            state_revision: self.state_revision,
            correlation_fingerprint: self.correlation_fingerprint.borrow().clone(),
            event_batch_fingerprint: self.event_batch_fingerprint.borrow().clone(),
            consumed_once: true,
            event_cardinality: self.event_cardinality.get(),
        }
    }
}

impl std::fmt::Debug for EguiTextCommandSurfaceRootEventBatch {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EguiTextCommandSurfaceRootEventBatch")
            .field("root_identity", &self.root_identity)
            .field("state_revision", &self.state_revision)
            .field(
                "correlation_fingerprint",
                &self.correlation_fingerprint.borrow(),
            )
            .field(
                "event_batch_fingerprint",
                &self.event_batch_fingerprint.borrow(),
            )
            .field("consumed_once", &self.transport.borrow().is_none())
            .field("event_cardinality", &self.event_cardinality.get())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::{EguiTextCommandSurfaceRootEventBatchForwardError, KucOpaqueHostEffectAttachError};
    use crate::text_command_surface::root::root_event::EguiTextCommandSurfaceRootEventBatch;
    use crate::text_command_surface::root::root_event::root_event_payload::RootEventPayload;
    use crate::text_command_surface::{KucOpaqueHostEffectBatch, KucOpaqueHostEffectError};

    fn successful_effect() -> Result<(), KucOpaqueHostEffectError> {
        Ok(())
    }

    #[test]
    fn debug_reports_only_closed_batch_metadata() {
        let mut batch = EguiTextCommandSurfaceRootEventBatch::new(
            RootEventPayload {
                text: Vec::new(),
                toolbar: None,
                floating: None,
                search: None,
                context_menu: None,
            },
            "opaque-batch-fingerprint".into(),
        );
        batch.set_root_metadata("opaque-root", 7);
        let debug = format!("{batch:?}");
        assert!(debug.contains("opaque-root"));
        assert!(debug.contains("state_revision: 7"));
        assert!(debug.contains("event_cardinality: 0"));
        assert!(!debug.contains("RootEventPayload"));
    }

    #[test]
    fn current_context_with_consumed_transport_uses_empty_class_dispatches() {
        let batch = EguiTextCommandSurfaceRootEventBatch::new(
            RootEventPayload {
                text: vec![],
                toolbar: None,
                floating: None,
                search: None,
                context_menu: None,
            },
            String::new(),
        );
        batch.transport.borrow_mut().take();

        let context = batch.current_context();
        assert_eq!(context.text_events().len(), 0);
        assert_eq!(context.toolbar_events().len(), 0);
        assert_eq!(context.class_dispatches()[0].event_count, 0);
        assert_eq!(context.class_dispatches()[4].event_count, 0);
    }

    #[test]
    fn attach_opaque_host_effect_batch_reports_already_consumed_or_attached() {
        assert!(successful_effect().is_ok());
        let batch = EguiTextCommandSurfaceRootEventBatch::new(
            RootEventPayload {
                text: Vec::new(),
                toolbar: None,
                floating: None,
                search: None,
                context_menu: None,
            },
            String::new(),
        );
        batch.transport.borrow_mut().take();

        let batch = EguiTextCommandSurfaceRootEventBatch::new(
            RootEventPayload {
                text: Vec::new(),
                toolbar: None,
                floating: None,
                search: None,
                context_menu: None,
            },
            String::new(),
        );
        let effect = KucOpaqueHostEffectBatch::from_handler(successful_effect);
        assert!(batch.attach_opaque_host_effect_batch(effect).is_ok());
        assert_eq!(
            batch.attach_opaque_host_effect_batch(KucOpaqueHostEffectBatch::from_handler(
                successful_effect,
            )),
            Err(KucOpaqueHostEffectAttachError::AlreadyAttached)
        );
    }

    #[test]
    fn attach_opaque_host_effect_batch_fails_if_transport_was_previously_consumed() {
        let batch = EguiTextCommandSurfaceRootEventBatch::new(
            RootEventPayload {
                text: Vec::new(),
                toolbar: None,
                floating: None,
                search: None,
                context_menu: None,
            },
            String::new(),
        );
        let mut forwarder = TestForwarder;
        let _ = batch.forward_once(&mut forwarder);
        assert_eq!(
            batch.attach_opaque_host_effect_batch(KucOpaqueHostEffectBatch::from_handler(
                successful_effect,
            )),
            Err(KucOpaqueHostEffectAttachError::AlreadyConsumed)
        );
    }

    #[test]
    fn set_root_metadata_updates_correlation_fingerprint() {
        let batch = EguiTextCommandSurfaceRootEventBatch::new(
            RootEventPayload {
                text: Vec::new(),
                toolbar: None,
                floating: None,
                search: None,
                context_menu: None,
            },
            String::from("batch"),
        );
        let before = batch.current_context().correlation_fingerprint().to_owned();
        let mut batch = batch;
        batch.set_root_metadata("root-id", 7);

        let after = batch.current_context().correlation_fingerprint().to_owned();
        assert_ne!(before, after);
        assert_eq!(batch.current_context().root_identity(), "root-id");
    }

    struct TestForwarder;

    impl crate::text_command_surface::KucRootEventBatchForwarder for TestForwarder {
        type Error = ();

        fn forward_root_event_batch(
            &mut self,
            transport: crate::text_command_surface::EguiTextCommandSurfaceRootEventTransport,
        ) -> Result<(), Self::Error> {
            assert_eq!(
                format!("{transport:?}"),
                "EguiTextCommandSurfaceRootEventTransport(..)"
            );
            Ok(())
        }
    }

    #[test]
    fn current_context_reflects_root_identity_and_state_revision() {
        let mut batch = EguiTextCommandSurfaceRootEventBatch::new(
            RootEventPayload {
                text: vec![],
                toolbar: None,
                floating: None,
                search: None,
                context_menu: None,
            },
            String::new(),
        );
        batch.set_root_metadata("identity", 12);
        let context = batch.current_context();
        assert_eq!(context.root_identity(), "identity");
        assert_eq!(context.state_revision(), 12);
    }

    #[test]
    fn forward_once_reports_already_consumed_after_first_send() {
        let batch = EguiTextCommandSurfaceRootEventBatch::new(
            RootEventPayload {
                text: Vec::new(),
                toolbar: None,
                floating: None,
                search: None,
                context_menu: None,
            },
            String::new(),
        );
        let mut forwarder = TestForwarder;
        assert!(batch.forward_once(&mut forwarder).is_ok());
        assert!(matches!(
            batch.forward_once(&mut forwarder),
            Err(EguiTextCommandSurfaceRootEventBatchForwardError::AlreadyConsumed)
        ));
    }
}
