#[cfg(any(test, feature = "storybook-artifacts"))]
use super::super::root::EguiTextCommandSurfaceRootOutput;
use super::super::root::{
    EguiTextCommandSurfaceRootEventBatchForwardError,
    EguiTextCommandSurfaceRootEventForwardingReceipt, KucRootEventBatchForwarder,
};
use super::host_root_record::EguiTextCommandSurfaceHostRootRecord;
use super::{
    EguiTextCommandSurfaceHostProjectionLease, EguiTextCommandSurfaceHostRoot,
    EguiTextCommandSurfaceHostRootFrame, EguiTextCommandSurfacePresentationToken,
    EguiTextCommandSurfaceRootFactoryError,
};

impl EguiTextCommandSurfaceHostRoot {
    #[cfg(test)]
    pub(crate) fn show_output_for_test(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootFactoryError> {
        self.process.show(ui)
    }

    /// Applies a newer opaque token without exposing child models or layout facts.
    pub fn synchronize(
        &mut self,
        token: EguiTextCommandSurfacePresentationToken,
    ) -> Result<bool, EguiTextCommandSurfaceRootFactoryError> {
        self.process.synchronize(
            token.revision,
            super::host_root_token_codec::decode_token(&token)?,
        )
    }

    /// Applies a newer token and replaces the private host effect router.
    pub fn synchronize_with_lease(
        &mut self,
        lease: EguiTextCommandSurfaceHostProjectionLease,
    ) -> Result<bool, EguiTextCommandSurfaceRootFactoryError> {
        let (token, router, source_address, tab_strip, status_diagnostics, editor_viewport) =
            lease.into_parts();
        self.process.synchronize_with_router(
            token.revision,
            super::host_root_token_codec::decode_token(&token)?,
            router,
            source_address,
            tab_strip,
            status_diagnostics,
            editor_viewport,
        )
    }

    /// Shows the complete retained root once and returns only the closed facade frame.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
    ) -> Result<EguiTextCommandSurfaceHostRootFrame, EguiTextCommandSurfaceRootFactoryError> {
        let output = self.process.show(ui)?;
        let record = EguiTextCommandSurfaceHostRootRecord::from_output(
            self.process.identity(),
            self.process.presentation_revision(),
            &output,
        );
        Ok(EguiTextCommandSurfaceHostRootFrame { output, record })
    }
}

impl EguiTextCommandSurfaceHostRootFrame {
    #[cfg(feature = "storybook-artifacts")]
    pub(crate) const fn artifact_output(&self) -> &EguiTextCommandSurfaceRootOutput {
        &self.output
    }

    #[must_use]
    pub const fn record(&self) -> &EguiTextCommandSurfaceHostRootRecord {
        &self.record
    }

    /// Returns the current KUC-owned generic interaction locator.
    #[must_use]
    pub const fn interaction_locator(&self) -> &super::super::root::KucInteractionLocator {
        self.output.interaction_locator()
    }

    /// Forwards the closed event transport exactly once.
    pub fn forward_events_once<Forwarder>(
        &self,
        forwarder: &mut Forwarder,
    ) -> Result<
        EguiTextCommandSurfaceRootEventForwardingReceipt,
        EguiTextCommandSurfaceRootEventBatchForwardError<Forwarder::Error>,
    >
    where
        Forwarder: KucRootEventBatchForwarder,
    {
        self.output.events().forward_once(forwarder)
    }
}

impl std::fmt::Debug for EguiTextCommandSurfaceHostRootFrame {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EguiTextCommandSurfaceHostRootFrame")
            .field("record", &self.record)
            .finish_non_exhaustive()
    }
}
