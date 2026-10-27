//! Closed retained root contract for KUC text-command composition.

#[path = "root/global_action_render.rs"]
mod global_action_render;
#[path = "root/global_actions.rs"]
mod global_actions;
mod interaction_locator;
#[path = "root/modal_input.rs"]
mod modal_input;
#[path = "root/projections.rs"]
mod projections;
mod root_event;
mod root_frame;
#[path = "root/root_types.rs"]
mod root_types;
#[path = "root/show.rs"]
mod show;
use super::source_address_projection_lease::SourceAddressProjectionLease;
use super::status_diagnostics_projection_lease::StatusDiagnosticsProjectionLease;
use super::tab_strip_projection_lease::TabStripProjectionLease;
use super::tab_strip_retained::TabStripRetainedState;
use super::types::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceError,
    TextCommandSurfaceStyle,
};
use crate::text_raster::{PlatformTextRasterConfig, PlatformTextRasterResources};

pub use global_actions::{
    KucGlobalAction, KucGlobalActionEvent, KucGlobalActionFocusPolicy, KucGlobalActionTarget,
    KucGlobalActionsProjection, KucGlobalShortcut,
};

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
        self.global_generation
            .set(self.global_generation.get().saturating_add(1));
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
            global_actions: None,
            global_generation: std::rc::Rc::new(std::cell::Cell::new(0)),
            global_document_focus: None,
            global_ime_composing: false,
            global_renderer: None,
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
