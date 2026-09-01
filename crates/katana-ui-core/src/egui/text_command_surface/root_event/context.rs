use super::ROOT_EVENT_CLASS_COUNT;
use super::{
    EguiTextCommandSurfaceRootEventBatch, EguiTextCommandSurfaceRootEventChildClass,
    EguiTextCommandSurfaceRootEventClassDispatch, KucOpaqueHostEffectAttachError,
    KucOpaqueHostEffectBatch, KucRootEventBatchContext,
};

impl EguiTextCommandSurfaceRootEventBatch {
    pub(crate) fn current_context(&self) -> KucRootEventBatchContext {
        let transport = self.transport.borrow();
        let (
            text_events,
            toolbar_events,
            floating_events,
            search_events,
            context_menu_events,
            status_bar_events,
            diagnostics_list_events,
        ) = transport.as_ref().map_or_else(
            || {
                (
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                )
            },
            |transport| {
                (
                    transport.payload.text.clone(),
                    transport.payload.toolbar.clone().unwrap_or_default(),
                    transport.payload.floating.clone().unwrap_or_default(),
                    transport.payload.search.clone().unwrap_or_default(),
                    transport.payload.context_menu.clone().unwrap_or_default(),
                    transport.payload.status_bar.clone().unwrap_or_default(),
                    transport
                        .payload
                        .diagnostics_list
                        .clone()
                        .unwrap_or_default(),
                )
            },
        );
        let source_address_submission_count = transport
            .as_ref()
            .map_or(0, |value| value.payload.source_address_submissions.len());
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
            status_bar_events,
            diagnostics_list_events,
            source_address_submission_count,
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
    ) -> [EguiTextCommandSurfaceRootEventClassDispatch; ROOT_EVENT_CLASS_COUNT] {
        let transport = self.transport.borrow();
        let Some(transport) = transport.as_ref() else {
            return [
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
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::StatusBar,
                    event_count: 0,
                },
                EguiTextCommandSurfaceRootEventClassDispatch {
                    child_class: EguiTextCommandSurfaceRootEventChildClass::DiagnosticsList,
                    event_count: 0,
                },
            ];
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
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::StatusBar,
                event_count: transport.payload.status_bar.as_ref().map_or(0, Vec::len),
            },
            EguiTextCommandSurfaceRootEventClassDispatch {
                child_class: EguiTextCommandSurfaceRootEventChildClass::DiagnosticsList,
                event_count: transport
                    .payload
                    .diagnostics_list
                    .as_ref()
                    .map_or(0, Vec::len),
            },
        ]
    }
}
