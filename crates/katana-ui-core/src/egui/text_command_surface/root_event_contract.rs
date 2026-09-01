use super::{EguiTextCommandSurfaceRootEventTransport, root_event_types::KucRootEventBatchContext};
use crate::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use crate::text_surface::TextSurfaceEvent;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KucOpaqueHostEffectError;

impl KucOpaqueHostEffectBatch {
    /// Creates an opaque batch from a host-owned one-shot handler.
    #[must_use]
    pub fn from_handler<F>(handler: F) -> Self
    where
        F: FnOnce() -> Result<(), KucOpaqueHostEffectError> + 'static,
    {
        Self {
            effect: Some(Box::new(handler)),
        }
    }

    /// Consumes this batch exactly once without exposing its handler or payload.
    pub fn consume_once(mut self) -> Result<(), KucOpaqueHostEffectError> {
        self.effect.take().map_or(Ok(()), |effect| effect())
    }
}

impl std::fmt::Debug for KucOpaqueHostEffectBatch {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("KucOpaqueHostEffectBatch(..)")
    }
}

/// Generic KUC router for a non-wire host effect.
pub trait KucRootEffectRouter {
    fn route(
        &mut self,
        context: KucRootEventBatchContext,
    ) -> Result<Option<KucOpaqueHostEffectBatch>, KucOpaqueHostEffectError>;
}

impl<F> KucRootEffectRouter for F
where
    F: FnMut(
            KucRootEventBatchContext,
        ) -> Result<Option<KucOpaqueHostEffectBatch>, KucOpaqueHostEffectError>
        + 'static,
{
    fn route(
        &mut self,
        context: KucRootEventBatchContext,
    ) -> Result<Option<KucOpaqueHostEffectBatch>, KucOpaqueHostEffectError> {
        self(context)
    }
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
        events: Vec<crate::molecule::selection::ContextMenuEvent>,
    ) -> Result<(), Self::Error>;

    fn consume_opaque_host_effect_batch(
        &mut self,
        effect_batch: KucOpaqueHostEffectBatch,
    ) -> Result<(), KucOpaqueHostEffectError> {
        effect_batch.consume_once()
    }
}
