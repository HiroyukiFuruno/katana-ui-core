use super::{EguiTextCommandSurfaceRootEventTransport, KucRootEventBatchContext};

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
    pub(super) effect: Option<Box<dyn FnOnce() -> Result<(), KucOpaqueHostEffectError>>>,
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
