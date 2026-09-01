//! Closed retained root contract for KUC text-command composition.

mod interaction_locator;
mod root_event;
mod root_frame;
#[path = "root/root_types.rs"]
mod root_types;

use super::source_address_projection_lease::SourceAddressProjectionLease;
use super::status_diagnostics_projection_lease::StatusDiagnosticsProjectionLease;
use super::tab_strip_projection_lease::TabStripProjectionLease;
use super::tab_strip_retained::TabStripRetainedState;
use super::types::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceError,
    TextCommandSurfaceStyle,
};
use crate::egui::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor,
};
use crate::text_raster::{PlatformTextRasterConfig, PlatformTextRasterResources};
use root_event::build_event_batch;
use root_frame::build_frame;

pub use interaction_locator::{
    KucInteractionActionClass, KucInteractionLocator, KucInteractionLocatorError,
    KucInteractionRequestError, KucInteractionSelector, KucOpaqueClickContinuation,
    KucOpaqueClickContinuationError, KucOpaqueInteractionRequest, KucOpaqueSearchTraceContinuation,
    KucOpaqueTextSelectionContinuation, KucSearchTraceContinuationError,
    KucTextSelectionContinuationError,
};
pub use root_event::{
    EguiTextCommandSurfaceRootEventBatch, EguiTextCommandSurfaceRootEventBatchDispatchError,
    EguiTextCommandSurfaceRootEventBatchForwardError, EguiTextCommandSurfaceRootEventChildClass,
    EguiTextCommandSurfaceRootEventClassDispatch, EguiTextCommandSurfaceRootEventDispatchReceipt,
    EguiTextCommandSurfaceRootEventForwardingReceipt, EguiTextCommandSurfaceRootEventTransport,
    KucOpaqueHostEffectAttachError, KucOpaqueHostEffectBatch, KucOpaqueHostEffectError,
    KucRootEffectRouter, KucRootEventBatchContext, KucRootEventBatchDispatcher,
    KucRootEventBatchForwarder,
};
pub use root_frame::{
    EguiTextCommandSurfaceRootAccessKitReference, EguiTextCommandSurfaceRootDimensions,
    EguiTextCommandSurfaceRootFrame,
};
pub use root_types::{
    EguiTextCommandSurfaceRoot, EguiTextCommandSurfaceRootError, EguiTextCommandSurfaceRootOutput,
};

impl EguiTextCommandSurfaceRoot {
    pub(crate) fn evidence_catalog(&self) -> &crate::text_raster::PlatformFontCatalog {
        &self.adapter.catalog
    }

    /// Creates a root with an identity derived from the retained text state id.
    pub fn new(surface: EguiTextCommandSurface) -> Result<Self, EguiTextCommandSurfaceError> {
        let identity = format!(
            "kuc.text-command-root/{}",
            surface.text().state().text_area.state_id.as_str()
        );
        Self::with_identity(identity, surface)
    }

    /// Creates a root with a caller-provided opaque, stable identity.
    pub fn with_identity(
        identity: impl Into<String>,
        surface: EguiTextCommandSurface,
    ) -> Result<Self, EguiTextCommandSurfaceError> {
        Ok(Self::with_text_raster_resources(
            identity,
            surface,
            PlatformTextRasterResources::new(PlatformTextRasterConfig::default()),
        ))
    }

    /// Creates a root whose retained text children use one catalog policy.
    pub fn with_text_raster_config(
        identity: impl Into<String>,
        surface: EguiTextCommandSurface,
        config: PlatformTextRasterConfig,
    ) -> Result<Self, EguiTextCommandSurfaceError> {
        Ok(Self::with_text_raster_resources(
            identity,
            surface,
            PlatformTextRasterResources::new(config),
        ))
    }

    pub(crate) fn with_text_raster_resources(
        identity: impl Into<String>,
        surface: EguiTextCommandSurface,
        resources: PlatformTextRasterResources,
    ) -> Self {
        Self {
            surface,
            adapter: EguiTextCommandSurfaceAdapter::with_resources(resources),
            identity: identity.into(),
            state_revision: 0,
            frame_serial: 0,
            source_address_submission_port: None,
            tab_strip: None,
            status_bar: None,
            diagnostics_list: None,
            editor_viewport: None,
        }
    }

    /// Synchronizes generic controlled presentation without exposing child models.
    pub fn synchronize_presentation(
        &mut self,
        presentation: super::types::EguiTextCommandSurfacePresentation,
    ) -> bool {
        let changed = self.surface.synchronize_presentation(presentation);
        if changed {
            self.state_revision = self.state_revision.saturating_add(1);
        }
        changed
    }

    pub fn attach_source_address(&mut self, lease: SourceAddressProjectionLease) {
        let (strip, port) = lease.into_parts();
        self.surface.set_source_address(strip);
        self.source_address_submission_port = port;
    }

    /// Mounts the generic KUC status child into this retained root.
    pub(crate) fn attach_status_bar(&mut self, status_bar: crate::molecule::StatusBar) {
        self.status_bar = Some(status_bar);
    }

    /// Mounts the generic KUC diagnostics child into this retained root.
    pub(crate) fn attach_diagnostics_list(
        &mut self,
        diagnostics_list: crate::molecule::DiagnosticsList,
    ) {
        self.diagnostics_list = Some(diagnostics_list);
    }

    /// Consumes a generic child projection without exposing child models through the root API.
    pub fn attach_status_diagnostics(&mut self, lease: StatusDiagnosticsProjectionLease) {
        let (status_bar, diagnostics_list) = lease.into_parts();
        if let Some(status_bar) = status_bar {
            self.attach_status_bar(status_bar);
        }
        if let Some(diagnostics_list) = diagnostics_list {
            self.attach_diagnostics_list(diagnostics_list);
        }
    }

    pub(crate) fn attach_tab_strip(
        &mut self,
        lease: TabStripProjectionLease,
    ) -> Result<bool, EguiTextCommandSurfaceError> {
        TabStripRetainedState::from_lease(
            lease,
            std::sync::Arc::clone(&self.adapter.catalog),
            self.adapter.text_raster_config.clone(),
        )
        .map(|tab_strip| {
            self.tab_strip = Some(tab_strip);
            true
        })
        .map_err(Into::into)
    }

    pub(crate) fn clear_tab_strip(&mut self) -> bool {
        self.tab_strip.take().is_some()
    }

    pub(crate) fn clear_status_diagnostics(&mut self) -> bool {
        let changed = self.status_bar.is_some() || self.diagnostics_list.is_some();
        self.status_bar = None;
        self.diagnostics_list = None;
        changed
    }

    pub(crate) fn attach_editor_viewport(&mut self, lease: super::EditorViewportProjectionLease) {
        self.editor_viewport = Some(lease);
    }

    pub(crate) fn clear_editor_viewport(&mut self) -> bool {
        self.editor_viewport.take().is_some()
    }

    pub(crate) fn synchronize_command_families(
        &mut self,
        primary: Option<crate::molecule::command_chrome::CommandChromeFamilyId>,
        floating: Option<crate::molecule::command_chrome::CommandChromeFamilyId>,
    ) -> bool {
        self.surface.synchronize_command_families(primary, floating)
    }

    /// Renders one actual root frame and returns only its closed frame/event contracts.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        style: &TextCommandSurfaceStyle,
    ) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootError> {
        ui.ctx().enable_accesskit();
        self.frame_serial = self.frame_serial.saturating_add(1);
        let mut output = self.adapter.show_with_tab_strip(
            ui,
            &mut self.surface,
            style,
            self.tab_strip.as_mut(),
            self.status_bar.as_mut(),
            self.diagnostics_list.as_mut(),
            self.editor_viewport.as_mut(),
        )?;
        let mut events =
            build_event_batch(&mut output, self.source_address_submission_port.clone())
                .map_err(EguiTextCommandSurfaceRootError::Serialization)?;
        if events.has_events() {
            self.state_revision = self.state_revision.saturating_add(1);
        }
        events.set_root_metadata(&self.identity, self.state_revision);
        let context = events.current_context();
        let bound_evidence = super::accesskit_evidence::bind_frame(
            output.accesskit_evidence.clone(),
            &self.identity,
            &context,
        );
        let plans = output.artifact_paint_plans()?;
        let composite = ArtifactCompositor::compose(ArtifactCompositeRequest {
            canvas: ArtifactCanvasBounds::new(output.root_bounds),
            plans: &plans,
        })?;
        let frame = build_frame(&self.identity, self.state_revision, &output, &composite)
            .map_err(EguiTextCommandSurfaceRootError::Serialization)?;
        let locator = interaction_locator::KucInteractionLocator::from_output(
            &self.identity,
            &context,
            self.frame_serial,
            &output,
            &bound_evidence,
        );
        let artifact_order = output.artifact_order().to_vec();
        Ok(EguiTextCommandSurfaceRootOutput {
            evidence_text: output.text,
            evidence_composite: composite,
            locator,
            artifact_order,
            frame,
            events,
            #[cfg(test)]
            toolbar_record: output.toolbar.map(|value| value.record),
            #[cfg(test)]
            floating: output.floating,
            #[cfg(test)]
            context_menu_record: output.context_menu.and_then(|value| value.record),
            #[cfg(test)]
            search_record: output.search.map(|value| value.record),
        })
    }
}

impl EguiTextCommandSurfaceRootOutput {
    #[must_use]
    pub const fn frame(&self) -> &EguiTextCommandSurfaceRootFrame {
        &self.frame
    }

    #[must_use]
    pub const fn events(&self) -> &EguiTextCommandSurfaceRootEventBatch {
        &self.events
    }

    /// Returns the final root-owned RGBA pixels for visual artifact encoding.
    ///
    /// The returned buffer is the already-composited root frame. Child paint
    /// plans, texture handles, and child geometry remain private to KUC.
    #[must_use]
    pub fn rgba_pixels(&self) -> &[u8] {
        &self.evidence_composite.rgba_pixels
    }

    #[must_use]
    pub const fn interaction_locator(&self) -> &interaction_locator::KucInteractionLocator {
        &self.locator
    }

    #[must_use]
    pub fn artifact_order(&self) -> &[super::types::EguiTextCommandSurfaceChild] {
        &self.artifact_order
    }
}

#[cfg(test)]
#[path = "root_inline_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "root_tests.rs"]
mod retained_root_tests;
