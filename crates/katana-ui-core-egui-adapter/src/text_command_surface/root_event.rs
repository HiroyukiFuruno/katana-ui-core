//! Opaque event-batch projection for the retained root.

mod root_event_contract;
mod root_event_core;
mod root_event_detach;
mod root_event_fingerprint;
mod root_event_payload;
mod root_event_transport;
mod root_event_types;

#[cfg(test)]
mod root_event_tests;

impl EguiTextCommandSurfaceRootEventBatch {
    fn from_output(
        output: &crate::text_command_surface::types::EguiTextCommandSurfaceOutput,
    ) -> Result<Self, String> {
        let payload = root_event_payload::RootEventPayload::from_output(output);
        let event_batch_fingerprint = payload.fingerprint().map_err(json_error_to_string)?;
        Ok(Self::new(payload, event_batch_fingerprint))
    }
}

fn json_error_to_string(error: serde_json::Error) -> String {
    error.to_string()
}

#[cfg(test)]
mod serialization_error_tests {
    use super::json_error_to_string;

    #[test]
    fn event_serialization_errors_retain_their_message() {
        let result = serde_json::from_slice::<serde_json::Value>(b"").map_err(json_error_to_string);
        assert!(result.is_err());
    }
}

pub(crate) const ROOT_EVENT_BATCH_BUILDER: fn(
    &crate::text_command_surface::types::EguiTextCommandSurfaceOutput,
) -> Result<
    EguiTextCommandSurfaceRootEventBatch,
    String,
> = EguiTextCommandSurfaceRootEventBatch::from_output;
pub(crate) use ROOT_EVENT_BATCH_BUILDER as build_event_batch;

/// Opaque one-shot transport token. It has no consumer-visible semantic accessors.
pub struct EguiTextCommandSurfaceRootEventTransport {
    pub(crate) payload: root_event_payload::RootEventPayload,
    opaque_host_effect_batch: Option<KucOpaqueHostEffectBatch>,
}

impl EguiTextCommandSurfaceRootEventTransport {
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
        self.dispatch(dispatcher)
    }
}

impl std::fmt::Debug for EguiTextCommandSurfaceRootEventTransport {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("EguiTextCommandSurfaceRootEventTransport(..)")
    }
}

pub use root_event_contract::{
    KucOpaqueHostEffectBatch, KucOpaqueHostEffectError, KucRootEffectRouter,
    KucRootEventBatchDispatcher, KucRootEventBatchForwarder,
};
#[cfg(test)]
pub(crate) use root_event_payload::RootEventPayload;
pub use root_event_types::{
    EguiTextCommandSurfaceRootEventBatch, EguiTextCommandSurfaceRootEventBatchDispatchError,
    EguiTextCommandSurfaceRootEventBatchForwardError, EguiTextCommandSurfaceRootEventChildClass,
    EguiTextCommandSurfaceRootEventClassDispatch, EguiTextCommandSurfaceRootEventDispatchReceipt,
    EguiTextCommandSurfaceRootEventForwardingReceipt, KucRootEventBatchContext,
};
pub(crate) use root_event_types::{
    EguiTextCommandSurfaceRootEventCommandDetachError,
    EguiTextCommandSurfaceRootEventSearchDetachError, KucOpaqueHostEffectAttachError,
};
