use super::super::artifact::EguiTextCommandSurfaceArtifactError;
use super::super::source_address_projection_lease::SourceAddressSubmissionPortHandle;
use super::super::tab_strip_retained::TabStripRetainedState;
use super::super::types::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceError,
};
use super::interaction_locator;
use super::root_event::EguiTextCommandSurfaceRootEventBatch;
use super::root_frame::EguiTextCommandSurfaceRootFrame;
use crate::artifact_compositor::{ArtifactCompositeError, ArtifactCompositeFrame};
use crate::text_surface::EguiTextSurfaceOutput;

/// KUC-owned retained root that composes the generic text-command children once.
pub struct EguiTextCommandSurfaceRoot {
    pub(super) surface: EguiTextCommandSurface,
    pub(super) adapter: EguiTextCommandSurfaceAdapter,
    pub(super) identity: String,
    pub(super) state_revision: u64,
    pub(super) frame_serial: u64,
    pub(super) source_address_submission_port: Option<SourceAddressSubmissionPortHandle>,
    pub(super) tab_strip: Option<TabStripRetainedState>,
    pub(super) status_bar: Option<katana_ui_core::molecule::StatusBar>,
    pub(super) diagnostics_list: Option<katana_ui_core::molecule::DiagnosticsList>,
    pub(super) editor_viewport: Option<super::super::EditorViewportProjectionLease>,
}

/// The only frame data exposed by the retained root.
#[derive(Debug)]
pub struct EguiTextCommandSurfaceRootOutput {
    pub(super) frame: EguiTextCommandSurfaceRootFrame,
    pub(super) events: EguiTextCommandSurfaceRootEventBatch,
    pub(crate) evidence_text: EguiTextSurfaceOutput,
    pub(crate) evidence_composite: ArtifactCompositeFrame,
    pub(super) locator: interaction_locator::KucInteractionLocator,
    pub(super) artifact_order: Vec<super::super::types::EguiTextCommandSurfaceChild>,
    #[cfg(test)]
    pub(crate) toolbar_record: Option<crate::command_chrome::EguiCommandChromeFrameRecord>,
    #[cfg(test)]
    pub(crate) context_menu_record: Option<crate::context_menu::EguiContextMenuFrameRecord>,
    #[cfg(test)]
    pub(crate) search_record: Option<crate::command_chrome::EguiCommandChromeSearchFrameRecord>,
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
