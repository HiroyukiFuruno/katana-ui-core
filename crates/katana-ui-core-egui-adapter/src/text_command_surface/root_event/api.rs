use super::EguiTextCommandSurfaceRootEventClassDispatch;
use super::{
    KucOpaqueHostEffectBatch, KucOpaqueHostEffectError, KucRootEffectRouter,
    KucRootEventBatchContext,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchEvent, CommandChromeToolbarEvent, FloatingCommandToolbarEvent,
};
use katana_ui_core::molecule::{DiagnosticsListEvent, StatusBarEvent};
use katana_ui_core::text_surface::TextSurfaceEvent;

impl KucRootEventBatchContext {
    #[must_use]
    pub fn root_identity(&self) -> &str {
        &self.root_identity
    }

    #[must_use]
    pub const fn state_revision(&self) -> u64 {
        self.state_revision
    }

    #[must_use]
    pub fn correlation_fingerprint(&self) -> &str {
        &self.correlation_fingerprint
    }

    #[must_use]
    pub fn class_dispatches(&self) -> &[EguiTextCommandSurfaceRootEventClassDispatch] {
        &self.class_dispatches
    }

    #[must_use]
    pub fn text_events(&self) -> &[TextSurfaceEvent] {
        &self.text_events
    }

    #[must_use]
    pub fn toolbar_events(&self) -> &[CommandChromeToolbarEvent] {
        &self.toolbar_events
    }

    #[must_use]
    pub fn floating_events(&self) -> &[FloatingCommandToolbarEvent] {
        &self.floating_events
    }

    #[must_use]
    pub fn search_events(&self) -> &[CommandChromeSearchEvent] {
        &self.search_events
    }

    #[must_use]
    pub fn context_menu_events(&self) -> &[katana_ui_core::molecule::selection::ContextMenuEvent] {
        &self.context_menu_events
    }

    #[must_use]
    pub const fn source_address_submission_count(&self) -> usize {
        self.source_address_submission_count
    }

    #[must_use]
    pub fn status_bar_events(&self) -> &[StatusBarEvent] {
        &self.status_bar_events
    }

    #[must_use]
    pub fn diagnostics_list_events(&self) -> &[DiagnosticsListEvent] {
        &self.diagnostics_list_events
    }
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
