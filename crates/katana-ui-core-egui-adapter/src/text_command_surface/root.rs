//! Closed retained root contract for KUC text-command composition.

#[path = "interaction_locator.rs"]
mod interaction_locator;
#[path = "root_event.rs"]
mod root_event;
#[path = "root_frame.rs"]
mod root_frame;

use super::artifact::EguiTextCommandSurfaceArtifactError;
use super::types::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceError,
    TextCommandSurfaceStyle,
};
use crate::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeError, ArtifactCompositeFrame, ArtifactCompositeRequest,
    ArtifactCompositor,
};
use crate::text_surface::EguiTextSurfaceOutput;
use root_event::build_event_batch;
pub(crate) use root_event::{
    EguiTextCommandSurfaceRootEventCommandDetachError,
    EguiTextCommandSurfaceRootEventSearchDetachError, KucOpaqueHostEffectAttachError,
};
use root_frame::build_frame;

pub use interaction_locator::{
    KucInteractionActionClass, KucInteractionLocator, KucInteractionLocatorError,
    KucInteractionRequestError, KucInteractionSelector, KucOpaqueInteractionRequest,
};
pub use root_event::{
    EguiTextCommandSurfaceRootEventBatch, EguiTextCommandSurfaceRootEventBatchDispatchError,
    EguiTextCommandSurfaceRootEventBatchForwardError, EguiTextCommandSurfaceRootEventChildClass,
    EguiTextCommandSurfaceRootEventClassDispatch, EguiTextCommandSurfaceRootEventDispatchReceipt,
    EguiTextCommandSurfaceRootEventForwardingReceipt, EguiTextCommandSurfaceRootEventTransport,
    KucOpaqueHostEffectBatch, KucOpaqueHostEffectError, KucRootEffectRouter,
    KucRootEventBatchContext, KucRootEventBatchDispatcher, KucRootEventBatchForwarder,
};
pub use root_frame::{
    EguiTextCommandSurfaceRootAccessKitReference, EguiTextCommandSurfaceRootDimensions,
    EguiTextCommandSurfaceRootFrame,
};

/// KUC-owned retained root that composes the generic text-command children once.
pub struct EguiTextCommandSurfaceRoot {
    surface: EguiTextCommandSurface,
    adapter: EguiTextCommandSurfaceAdapter,
    identity: String,
    state_revision: u64,
}

/// The only frame data exposed by the retained root.
#[derive(Debug)]
pub struct EguiTextCommandSurfaceRootOutput {
    frame: EguiTextCommandSurfaceRootFrame,
    events: EguiTextCommandSurfaceRootEventBatch,
    pub(crate) evidence_text: EguiTextSurfaceOutput,
    pub(crate) evidence_composite: ArtifactCompositeFrame,
    locator: interaction_locator::KucInteractionLocator,
    #[cfg(test)]
    pub(crate) toolbar_record: Option<crate::command_chrome::EguiCommandChromeFrameRecord>,
    #[cfg(test)]
    pub(crate) context_menu_record: Option<crate::context_menu::EguiContextMenuFrameRecord>,
    #[cfg(test)]
    pub(crate) floating: Option<crate::command_chrome::EguiCommandChromeFloatingOutput>,
}

/// Failure while producing the closed root frame or event batch.
#[derive(Debug)]
pub enum EguiTextCommandSurfaceRootError {
    Surface(EguiTextCommandSurfaceError),
    Artifact(EguiTextCommandSurfaceArtifactError),
    Composite(ArtifactCompositeError),
    Serialization(String),
}

impl EguiTextCommandSurfaceRoot {
    pub(crate) fn evidence_catalog(&self) -> &katana_ui_core_text_raster::PlatformFontCatalog {
        &self.adapter.catalog
    }

    /// Creates a root with an identity derived from the retained text state id.
    #[must_use]
    pub fn new(surface: EguiTextCommandSurface) -> Self {
        let identity = format!(
            "kuc.text-command-root/{}",
            surface.text().state().text_area.state_id.as_str()
        );
        Self::with_identity(identity, surface)
    }

    /// Creates a root with a caller-provided opaque, stable identity.
    #[must_use]
    pub fn with_identity(identity: impl Into<String>, surface: EguiTextCommandSurface) -> Self {
        Self {
            surface,
            adapter: EguiTextCommandSurfaceAdapter::default(),
            identity: identity.into(),
            state_revision: 0,
        }
    }

    /// Creates a root whose retained text children use one catalog policy.
    #[must_use]
    pub fn with_text_raster_config(
        identity: impl Into<String>,
        surface: EguiTextCommandSurface,
        config: katana_ui_core_text_raster::PlatformTextRasterConfig,
    ) -> Self {
        Self {
            surface,
            adapter: EguiTextCommandSurfaceAdapter::with_text_raster_config(config),
            identity: identity.into(),
            state_revision: 0,
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

    pub(crate) fn apply_command_family_projection(
        &mut self,
        projection: &super::host_root::EguiTextCommandSurfaceCommandFamilyProjection,
    ) {
        self.surface.apply_command_family_projection(projection);
    }

    /// Renders one actual root frame and returns only its closed frame/event contracts.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        style: &TextCommandSurfaceStyle,
    ) -> Result<EguiTextCommandSurfaceRootOutput, EguiTextCommandSurfaceRootError> {
        let output = self.adapter.show(ui, &mut self.surface, style)?;
        let mut events =
            build_event_batch(&output).map_err(EguiTextCommandSurfaceRootError::Serialization)?;
        if events.has_events() {
            self.state_revision = self.state_revision.saturating_add(1);
        }
        events.set_root_metadata(&self.identity, self.state_revision);
        let context = events.current_context();
        let bound_evidence = super::accesskit_evidence::AccessKitEvidenceLedger::bind_frame(
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
            &output,
            &bound_evidence,
        );
        Ok(EguiTextCommandSurfaceRootOutput {
            evidence_text: output.text,
            evidence_composite: composite,
            locator,
            frame,
            events,
            #[cfg(test)]
            toolbar_record: output.toolbar.map(|value| value.record),
            #[cfg(test)]
            floating: output.floating,
            #[cfg(test)]
            context_menu_record: output.context_menu.and_then(|value| value.record),
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
}

impl From<EguiTextCommandSurfaceError> for EguiTextCommandSurfaceRootError {
    fn from(value: EguiTextCommandSurfaceError) -> Self {
        Self::Surface(value)
    }
}

impl From<EguiTextCommandSurfaceArtifactError> for EguiTextCommandSurfaceRootError {
    fn from(value: EguiTextCommandSurfaceArtifactError) -> Self {
        Self::Artifact(value)
    }
}

impl From<ArtifactCompositeError> for EguiTextCommandSurfaceRootError {
    fn from(value: ArtifactCompositeError) -> Self {
        Self::Composite(value)
    }
}

impl std::fmt::Display for EguiTextCommandSurfaceRootError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Surface(error) => write!(formatter, "text-command root surface failed: {error}"),
            Self::Artifact(error) => {
                write!(formatter, "text-command root artifact failed: {error}")
            }
            Self::Composite(error) => {
                write!(formatter, "text-command root composition failed: {error}")
            }
            Self::Serialization(error) => {
                write!(formatter, "text-command root serialization failed: {error}")
            }
        }
    }
}

impl std::error::Error for EguiTextCommandSurfaceRootError {}

#[path = "root_tests.rs"]
#[cfg(test)]
mod root_tests;
