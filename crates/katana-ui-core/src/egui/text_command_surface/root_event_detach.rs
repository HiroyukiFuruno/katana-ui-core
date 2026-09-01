use super::{
    EguiTextCommandSurfaceRootEventBatch, EguiTextCommandSurfaceRootEventCommandDetachError,
    EguiTextCommandSurfaceRootEventSearchDetachError,
    root_event_fingerprint::RootEventCorrelationFingerprint,
};
use crate::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};

impl EguiTextCommandSurfaceRootEventBatch {
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
        let events = transport.payload.search.take().unwrap_or_default();
        self.refresh_after_detach(&transport.payload)
            .map_err(|_| EguiTextCommandSurfaceRootEventSearchDetachError::Serialization)?;
        Ok(events)
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
            /* WHY: Command activation owns a RawInput frame also observed by the text child. */
            transport.payload.text.clear();
        }
        self.refresh_after_detach(&transport.payload)
            .map_err(|_| EguiTextCommandSurfaceRootEventCommandDetachError::Serialization)?;
        Ok((toolbar, floating))
    }

    pub(crate) fn detach_context_menu_events(
        &self,
    ) -> Result<
        Vec<crate::molecule::selection::ContextMenuEvent>,
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
        self.refresh_after_detach(&transport.payload)
            .map_err(|_| EguiTextCommandSurfaceRootEventCommandDetachError::Serialization)?;
        Ok(events)
    }

    fn refresh_after_detach(
        &self,
        payload: &super::root_event_payload::RootEventPayload,
    ) -> Result<(), serde_json::Error> {
        self.event_cardinality.set(payload.event_cardinality());
        *self.event_batch_fingerprint.borrow_mut() = payload.fingerprint()?;
        *self.correlation_fingerprint.borrow_mut() = RootEventCorrelationFingerprint::compose(
            &self.root_identity,
            self.state_revision,
            &self.event_batch_fingerprint.borrow(),
        );
        Ok(())
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
