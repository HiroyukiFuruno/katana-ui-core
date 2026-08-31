use super::super::super::source_address_projection_lease::SourceAddressSubmissionPortHandle;
use super::dispatcher::RootEventFingerprint;
use super::{
    EguiTextCommandSurfaceRootEventBatch, EguiTextCommandSurfaceRootEventBatchForwardError,
    EguiTextCommandSurfaceRootEventCommandDetachError,
    EguiTextCommandSurfaceRootEventForwardingReceipt,
    EguiTextCommandSurfaceRootEventSearchDetachError, EguiTextCommandSurfaceRootEventTransport,
    KucRootEventBatchForwarder, RootEventPayload,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use std::cell::{Cell, RefCell};

impl EguiTextCommandSurfaceRootEventBatch {
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
        let _ = &transport.payload;
        let receipt = self.receipt();
        forwarder
            .forward_root_event_batch(transport)
            .map_err(EguiTextCommandSurfaceRootEventBatchForwardError::Forwarder)?;
        Ok(receipt)
    }

    #[cfg(test)]
    pub(super) fn new(payload: RootEventPayload, event_batch_fingerprint: String) -> Self {
        Self::with_source_address_port(payload, event_batch_fingerprint, None)
    }

    pub(super) fn with_source_address_port(
        payload: RootEventPayload,
        event_batch_fingerprint: String,
        source_address_submission_port: Option<SourceAddressSubmissionPortHandle>,
    ) -> Self {
        let event_cardinality = payload.event_cardinality();
        let correlation_fingerprint =
            RootEventFingerprint::correlation_fingerprint("", 0, &event_batch_fingerprint);
        Self {
            transport: std::cell::RefCell::new(Some(EguiTextCommandSurfaceRootEventTransport {
                payload,
                opaque_host_effect_batch: None,
                source_address_submission_port,
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

    pub(crate) fn detach_search_events(
        &self,
    ) -> Result<Vec<CommandChromeSearchEvent>, EguiTextCommandSurfaceRootEventSearchDetachError>
    {
        let mut transport = self.transport.borrow_mut();
        let transport = transport
            .as_mut()
            .ok_or(EguiTextCommandSurfaceRootEventSearchDetachError::AlreadyConsumed)?;
        if self.search_detached.replace(true) {
            return Err(EguiTextCommandSurfaceRootEventSearchDetachError::AlreadyDetached);
        }
        let search_events = transport.payload.search.take().unwrap_or_default();
        self.event_cardinality
            .set(transport.payload.event_cardinality());
        *self.event_batch_fingerprint.borrow_mut() =
            RootEventFingerprint::fingerprint_payload(&transport.payload)
                .map_err(|_| EguiTextCommandSurfaceRootEventSearchDetachError::Serialization)?;
        *self.correlation_fingerprint.borrow_mut() = RootEventFingerprint::correlation_fingerprint(
            &self.root_identity,
            self.state_revision,
            &self.event_batch_fingerprint.borrow(),
        );
        Ok(search_events)
    }

    pub(crate) fn detach_search_events_exclusively(
        &self,
    ) -> Result<Vec<CommandChromeSearchEvent>, EguiTextCommandSurfaceRootEventSearchDetachError>
    {
        let search_events = self.detach_search_events()?;
        if search_events.is_empty() {
            return Ok(search_events);
        }
        let mut transport = self.transport.borrow_mut();
        let transport = transport
            .as_mut()
            .ok_or(EguiTextCommandSurfaceRootEventSearchDetachError::AlreadyConsumed)?;
        /* WHY: A sanitized search control and the text child can observe the same
         * RawInput frame; the search capability owns that physical input exclusively. */
        transport.payload.text.clear();
        self.event_cardinality
            .set(transport.payload.event_cardinality());
        *self.event_batch_fingerprint.borrow_mut() =
            RootEventFingerprint::fingerprint_payload(&transport.payload)
                .map_err(|_| EguiTextCommandSurfaceRootEventSearchDetachError::Serialization)?;
        *self.correlation_fingerprint.borrow_mut() = RootEventFingerprint::correlation_fingerprint(
            &self.root_identity,
            self.state_revision,
            &self.event_batch_fingerprint.borrow(),
        );
        Ok(search_events)
    }

    pub(crate) fn detach_command_events(
        &self,
    ) -> Result<
        (
            Vec<CommandChromeToolbarEvent>,
            Vec<FloatingCommandToolbarEvent>,
        ),
        EguiTextCommandSurfaceRootEventCommandDetachError,
    > {
        let mut transport = self.transport.borrow_mut();
        let transport = transport
            .as_mut()
            .ok_or(EguiTextCommandSurfaceRootEventCommandDetachError::AlreadyConsumed)?;
        if self.command_detached.replace(true) {
            return Err(EguiTextCommandSurfaceRootEventCommandDetachError::AlreadyDetached);
        }
        let toolbar = transport.payload.toolbar.take().unwrap_or_default();
        let floating = transport.payload.floating.take().unwrap_or_default();
        if toolbar.iter().any(command_activation_event)
            || floating.iter().any(|event| {
                matches!(
                    event,
                    FloatingCommandToolbarEvent::Toolbar { event }
                        if command_activation_event(event)
                )
            })
        {
            /* WHY: A focused command control and the text child can observe the same
             * RawInput frame; activation owns that input at the sanitized root boundary. */
            transport.payload.text.clear();
        }
        self.event_cardinality
            .set(transport.payload.event_cardinality());
        *self.event_batch_fingerprint.borrow_mut() =
            RootEventFingerprint::fingerprint_payload(&transport.payload)
                .map_err(|_| EguiTextCommandSurfaceRootEventCommandDetachError::Serialization)?;
        *self.correlation_fingerprint.borrow_mut() = RootEventFingerprint::correlation_fingerprint(
            &self.root_identity,
            self.state_revision,
            &self.event_batch_fingerprint.borrow(),
        );
        Ok((toolbar, floating))
    }

    pub(crate) fn detach_context_menu_events(
        &self,
    ) -> Result<
        Vec<katana_ui_core::molecule::selection::ContextMenuEvent>,
        EguiTextCommandSurfaceRootEventCommandDetachError,
    > {
        let mut transport = self.transport.borrow_mut();
        let transport = transport
            .as_mut()
            .ok_or(EguiTextCommandSurfaceRootEventCommandDetachError::AlreadyConsumed)?;
        if self.context_menu_detached.replace(true) {
            return Err(EguiTextCommandSurfaceRootEventCommandDetachError::AlreadyDetached);
        }
        let events = transport.payload.context_menu.take().unwrap_or_default();
        self.event_cardinality
            .set(transport.payload.event_cardinality());
        *self.event_batch_fingerprint.borrow_mut() =
            RootEventFingerprint::fingerprint_payload(&transport.payload)
                .map_err(|_| EguiTextCommandSurfaceRootEventCommandDetachError::Serialization)?;
        *self.correlation_fingerprint.borrow_mut() = RootEventFingerprint::correlation_fingerprint(
            &self.root_identity,
            self.state_revision,
            &self.event_batch_fingerprint.borrow(),
        );
        Ok(events)
    }

    pub(crate) fn set_root_metadata(&mut self, root_identity: &str, state_revision: u64) {
        self.root_identity = root_identity.to_owned();
        self.state_revision = state_revision;
        *self.correlation_fingerprint.borrow_mut() = RootEventFingerprint::correlation_fingerprint(
            root_identity,
            state_revision,
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

fn command_activation_event(event: &CommandChromeToolbarEvent) -> bool {
    matches!(
        event,
        CommandChromeToolbarEvent::CommandActivated { .. }
            | CommandChromeToolbarEvent::AcceleratorTriggered { .. }
            | CommandChromeToolbarEvent::DropdownItemActivated { .. }
    )
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
