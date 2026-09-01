use super::{
    EguiTextCommandSurfaceRootEventBatchDispatchError, EguiTextCommandSurfaceRootEventChildClass,
    EguiTextCommandSurfaceRootEventClassDispatch, EguiTextCommandSurfaceRootEventDispatchReceipt,
    EguiTextCommandSurfaceRootEventTransport, KucRootEventBatchDispatcher,
};

impl EguiTextCommandSurfaceRootEventTransport {
    pub(crate) fn dispatch<Dispatcher>(
        self,
        dispatcher: &mut Dispatcher,
    ) -> Result<
        EguiTextCommandSurfaceRootEventDispatchReceipt,
        EguiTextCommandSurfaceRootEventBatchDispatchError<Dispatcher::Error>,
    >
    where
        Dispatcher: KucRootEventBatchDispatcher,
    {
        let super::EguiTextCommandSurfaceRootEventTransport {
            payload:
                super::root_event_payload::RootEventPayload {
                    text,
                    toolbar,
                    floating,
                    search,
                    context_menu,
                },
            opaque_host_effect_batch,
        } = self;
        let toolbar = toolbar.unwrap_or_default();
        let floating = floating.unwrap_or_default();
        let search = search.unwrap_or_default();
        let context_menu = context_menu.unwrap_or_default();
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
        if let Some(effect_batch) = opaque_host_effect_batch {
            dispatcher
                .consume_opaque_host_effect_batch(effect_batch)
                .map_err(|_| EguiTextCommandSurfaceRootEventBatchDispatchError::OpaqueHostEffect)?;
        }

        Ok(receipt)
    }
}
