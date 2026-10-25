//! Closed retained root contract for KUC text-command composition.

mod interaction_locator;
#[path = "root/modal_input.rs"]
mod modal_input;
#[path = "root/projections.rs"]
mod projections;
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
    pub(crate) fn synchronize_selection_dialog(
        &mut self,
        lease: super::SelectionDialogProjectionLease,
    ) -> Result<bool, super::SelectionDialogProjectionError> {
        if let Some(dialog) = self.selection_dialog.as_mut() {
            return dialog.synchronize(lease);
        }
        self.selection_dialog = Some(super::selection_dialog::SelectionDialogRetainedState::new(
            lease,
            &self.adapter.text_raster_resources,
        ));
        Ok(true)
    }

    pub(crate) const fn selection_frame_serial(&self) -> u64 {
        self.frame_serial
    }

    pub(crate) fn selection_surface(&self) -> &crate::text_surface::TextSurface {
        &self.surface.text
    }

    pub(crate) fn restore_selection_surface(
        &mut self,
        value: &crate::text_surface::TextSurfaceUiSelection,
    ) {
        self.surface.text.restore_ui_selection(value);
        self.state_revision = self.state_revision.saturating_add(1);
    }

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
            selection_dialog: None,
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

    /// Renders one actual root frame and returns only its closed frame/event contracts.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        style: &TextCommandSurfaceStyle,
    ) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootError> {
        ui.ctx().enable_accesskit();
        self.frame_serial = self.frame_serial.saturating_add(1);
        if let Some(dialog) = self.selection_dialog.as_mut() {
            dialog.prepare_input(ui);
        }
        let dialog_open = self
            .selection_dialog
            .as_ref()
            .is_some_and(|dialog| dialog.is_open());
        if dialog_open {
            if let Some(dialog) = self.selection_dialog.as_mut() {
                dialog.capture_restore_focus(ui.ctx());
            }
            modal_input::suspend(self)?;
        }
        let modal_events =
            dialog_open.then(|| ui.input_mut(|input| std::mem::take(&mut input.events)));
        let output_result = ui
            .add_enabled_ui(!dialog_open, |ui| {
                self.adapter.show_with_tab_strip(
                    ui,
                    &mut self.surface,
                    style,
                    self.tab_strip.as_mut(),
                    self.status_bar.as_mut(),
                    self.diagnostics_list.as_mut(),
                    self.editor_viewport.as_mut(),
                )
            })
            .inner;
        if let Some(events) = modal_events {
            ui.input_mut(|input| input.events = events);
        }
        let output = output_result?;
        let dialog = if let Some(dialog) = self.selection_dialog.as_mut() {
            dialog
                .show(ui.ctx(), style)
                .map_err(EguiTextCommandSurfaceRootError::Serialization)?
        } else {
            None
        };
        let mut output = output.with_selection_dialog(dialog);
        let accesskit = super::accesskit_evidence::finish_frame(ui.ctx());
        output.accesskit_evidence = accesskit.entries;
        output.accesskit_text_input_nodes = accesskit.text_input_nodes;
        let mut events =
            build_event_batch(&mut output, self.source_address_submission_port.clone())
                .map_err(EguiTextCommandSurfaceRootError::Serialization)?;
        if events.has_events() {
            self.state_revision = self.state_revision.saturating_add(1);
        }
        events.set_root_metadata(&self.identity, self.state_revision);
        events.set_root_frame_serial(self.frame_serial);
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
        #[cfg(any(test, feature = "storybook-artifacts"))]
        let context_menu_visible = output
            .context_menu
            .as_ref()
            .is_some_and(|value| value.record.is_some() && value.artifact.is_some());
        Ok(EguiTextCommandSurfaceRootOutput {
            evidence_text: output.text,
            evidence_composite: composite,
            accesskit_evidence: output.accesskit_evidence,
            accesskit_text_input_nodes: output.accesskit_text_input_nodes,
            locator,
            artifact_order,
            #[cfg(any(test, feature = "storybook-artifacts"))]
            context_menu_visible,
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

    #[must_use]
    #[cfg(any(test, feature = "storybook-artifacts"))]
    pub(crate) const fn context_menu_is_visible(&self) -> bool {
        self.context_menu_visible
    }
}

#[cfg(test)]
#[path = "root_inline_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "root_tests.rs"]
mod retained_root_tests;
