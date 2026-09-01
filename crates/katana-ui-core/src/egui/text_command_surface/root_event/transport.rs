use super::super::super::source_address_projection_lease::SourceAddressSubmissionPortError;
#[cfg(test)]
use super::super::super::source_address_projection_lease::SourceAddressSubmissionPortHandle;
use super::{
    EguiTextCommandSurfaceRootEventBatchDispatchError, EguiTextCommandSurfaceRootEventChildClass,
    EguiTextCommandSurfaceRootEventClassDispatch, EguiTextCommandSurfaceRootEventDispatchReceipt,
    EguiTextCommandSurfaceRootEventTransport, KucOpaqueHostEffectBatch,
    KucRootEventBatchDispatcher, RootEventPayload,
};

impl EguiTextCommandSurfaceRootEventTransport {
    #[cfg(test)]
    #[must_use]
    pub(crate) fn with_source_address_submission_port(
        mut self,
        port: Option<SourceAddressSubmissionPortHandle>,
    ) -> Self {
        self.source_address_submission_port = port;
        self
    }

    /// Attaches a host-owned opaque effect batch to this one-shot transport.
    #[must_use]
    pub fn with_opaque_host_effect_batch(mut self, effect_batch: KucOpaqueHostEffectBatch) -> Self {
        self.opaque_host_effect_batch = Some(effect_batch);
        self
    }

    pub fn dispatch_once<Dispatcher>(
        self,
        dispatcher: &mut Dispatcher,
    ) -> Result<
        EguiTextCommandSurfaceRootEventDispatchReceipt,
        EguiTextCommandSurfaceRootEventBatchDispatchError<Dispatcher::Error>,
    >
    where
        Dispatcher: KucRootEventBatchDispatcher,
    {
        let RootEventPayload {
            text,
            toolbar,
            floating,
            search,
            context_menu,
            source_address_submissions,
            status_bar,
            diagnostics_list,
        } = self.payload;
        let opaque_host_effect_batch = self.opaque_host_effect_batch;
        let source_address_submission_port = self.source_address_submission_port;
        let toolbar = toolbar.unwrap_or_default();
        let floating = floating.unwrap_or_default();
        let search = search.unwrap_or_default();
        let context_menu = context_menu.unwrap_or_default();
        let status_bar = status_bar.unwrap_or_default();
        let diagnostics_list = diagnostics_list.unwrap_or_default();
        let receipt = EguiTextCommandSurfaceRootEventDispatchReceipt {
            class_dispatches: [
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::Text,
                    event_count: text.len(),
                },
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::Toolbar,
                    event_count: toolbar.len(),
                },
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::Floating,
                    event_count: floating.len(),
                },
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::Search,
                    event_count: search.len(),
                },
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::ContextMenu,
                    event_count: context_menu.len(),
                },
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::StatusBar,
                    event_count: status_bar.len(),
                },
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::DiagnosticsList,
                    event_count: diagnostics_list.len(),
                },
            ],
        };

        dispatcher
            .dispatch_text_events(text)
            .map_err(EguiTextCommandSurfaceRootEventBatchDispatchError::Dispatcher)?;
        dispatcher
            .dispatch_toolbar_events(toolbar)
            .map_err(EguiTextCommandSurfaceRootEventBatchDispatchError::Dispatcher)?;
        dispatcher
            .dispatch_floating_events(floating)
            .map_err(EguiTextCommandSurfaceRootEventBatchDispatchError::Dispatcher)?;
        dispatcher
            .dispatch_search_events(search)
            .map_err(EguiTextCommandSurfaceRootEventBatchDispatchError::Dispatcher)?;
        dispatcher
            .dispatch_context_menu_events(context_menu)
            .map_err(EguiTextCommandSurfaceRootEventBatchDispatchError::Dispatcher)?;
        dispatcher
            .dispatch_status_bar_events(status_bar)
            .map_err(EguiTextCommandSurfaceRootEventBatchDispatchError::Dispatcher)?;
        dispatcher
            .dispatch_diagnostics_list_events(diagnostics_list)
            .map_err(EguiTextCommandSurfaceRootEventBatchDispatchError::Dispatcher)?;
        for submission in source_address_submissions {
            let Some(port) = source_address_submission_port.as_ref() else {
                return Err(
                    EguiTextCommandSurfaceRootEventBatchDispatchError::SourceAddressPort(
                        SourceAddressSubmissionPortError::Rejected,
                    ),
                );
            };
            port.forward_submission(submission)
                .map_err(EguiTextCommandSurfaceRootEventBatchDispatchError::SourceAddressPort)?;
        }
        if let Some(effect_batch) = opaque_host_effect_batch {
            dispatcher
                .consume_opaque_host_effect_batch(effect_batch)
                .map_err(|_| EguiTextCommandSurfaceRootEventBatchDispatchError::OpaqueHostEffect)?;
        }

        Ok(receipt)
    }
}

impl std::fmt::Debug for EguiTextCommandSurfaceRootEventTransport {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("EguiTextCommandSurfaceRootEventTransport(..)")
    }
}
