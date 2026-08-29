//! KUC-owned root composition for a text surface and its generic command chrome.

pub(crate) mod accesskit_evidence;
mod artifact;
mod composition;
mod context_menu;
mod editor_viewport_projection_lease;
mod editor_viewport_render;
mod host_root;
mod model;
#[path = "text_command_surface/root.rs"]
mod root;
mod sanitized_document_root;
mod scenario;
mod scenario_session;
mod source_address_projection_lease;
mod status_diagnostics_projection_lease;
mod synchronization;
mod tab_strip_projection_lease;
mod tab_strip_proposal_port;
mod tab_strip_retained;
mod tab_strip_route_table;
mod tab_strip_text_raster;
mod text_command_surface_style_factory;
mod types;
mod unicode_evidence;

pub use artifact::EguiTextCommandSurfaceArtifactError;
pub use editor_viewport_projection_lease::{
    EditorViewportProjectionError, EditorViewportProjectionLease,
};
pub use host_root::{
    EguiTextCommandSurfaceCommandFamilyProjection, EguiTextCommandSurfaceHostProjectionEncoder,
    EguiTextCommandSurfaceHostProjectionLease, EguiTextCommandSurfaceHostRoot,
    EguiTextCommandSurfaceHostRootFrame, EguiTextCommandSurfaceHostRootRecord,
    EguiTextCommandSurfaceHostRootRecordDimensions, EguiTextCommandSurfaceHostTargetToken,
    EguiTextCommandSurfacePresentationToken, EguiTextCommandSurfaceRootFactory,
    EguiTextCommandSurfaceRootFactoryError,
};
/// A generic, one-shot host effect deferred until root event dispatch completes.
pub use root::KucOpaqueHostEffectBatch;
/// An opaque error returned by a generic host effect boundary.
pub use root::KucOpaqueHostEffectError;
/// Routes generic root event context into an optional opaque host effect.
pub use root::KucRootEffectRouter;
/// A snapshot of generic root event metadata and child event payloads.
pub use root::KucRootEventBatchContext;
#[doc(hidden)]
pub use root::{
    EguiTextCommandSurfaceRoot, EguiTextCommandSurfaceRootAccessKitReference,
    EguiTextCommandSurfaceRootDimensions, EguiTextCommandSurfaceRootError,
    EguiTextCommandSurfaceRootEventBatch, EguiTextCommandSurfaceRootEventBatchDispatchError,
    EguiTextCommandSurfaceRootEventBatchForwardError, EguiTextCommandSurfaceRootEventChildClass,
    EguiTextCommandSurfaceRootEventClassDispatch, EguiTextCommandSurfaceRootEventDispatchReceipt,
    EguiTextCommandSurfaceRootEventForwardingReceipt, EguiTextCommandSurfaceRootEventTransport,
    EguiTextCommandSurfaceRootFrame, EguiTextCommandSurfaceRootOutput,
    KucOpaqueHostEffectAttachError, KucRootEventBatchDispatcher, KucRootEventBatchForwarder,
};
pub use root::{
    KucInteractionActionClass, KucInteractionLocator, KucInteractionLocatorError,
    KucInteractionRequestError, KucInteractionSelector, KucOpaqueClickContinuation,
    KucOpaqueClickContinuationError, KucOpaqueInteractionRequest, KucOpaqueSearchTraceContinuation,
    KucOpaqueTextSelectionContinuation, KucSearchTraceContinuationError,
    KucTextSelectionContinuationError,
};
pub use sanitized_document_root::{
    SanitizedCommandDropdownItem, SanitizedCommandGroup, SanitizedCommandItem,
    SanitizedCommandProjection, SanitizedCommandTarget, SanitizedContextMenuCapabilityRejection,
    SanitizedContextMenuItem, SanitizedContextMenuProjection,
    SanitizedContextMenuProjectionBuilder, SanitizedContextMenuTarget, SanitizedDocumentRoot,
    SanitizedDocumentRootEventDispatchError, SanitizedDocumentRootEventForwardError,
    SanitizedDocumentRootEventForwarder, SanitizedDocumentRootEventForwardingReceipt,
    SanitizedDocumentRootEventTransport, SanitizedDocumentRootFactory,
    SanitizedDocumentRootFactoryError, SanitizedDocumentRootFrame, SanitizedDocumentRootIdentity,
    SanitizedDocumentRootInput, SanitizedDocumentRootRecord, SanitizedDocumentRootRecordDimensions,
    SanitizedDocumentRootStyleKey, SanitizedSearchCapabilityRejection,
    SanitizedSearchControlPresentation, SanitizedSearchLocalizedPresentation,
    SanitizedSearchOperationPresentation, SanitizedSearchOperationSlot, SanitizedSearchProjection,
    SanitizedSearchProjectionBuildError, SanitizedSearchProjectionBuilder,
    SanitizedSearchResultSummaryPresentation, SanitizedSearchTarget, SanitizedSearchTextOperation,
    SanitizedSearchTextPresentation, SanitizedSearchUnavailablePresentation,
    SanitizedSearchUnitOperation, SanitizedTab, SanitizedTabCapabilities,
    SanitizedTabClosePresentation, SanitizedTabGroup, SanitizedTabProjection, SanitizedTabTarget,
};
pub use scenario::{
    FullTextCommandSurfaceMotionFrame, FullTextCommandSurfaceMotionPlan,
    FullTextCommandSurfaceMotionPlanError, FullTextCommandSurfaceRawInputStage,
    FullTextCommandSurfaceScenario, FullTextCommandSurfaceScenarioError,
    FullTextCommandSurfaceScenarioFactory, FullTextCommandSurfaceScenarioId,
    KucOpaqueMotionContinuation, KucOpaqueMotionContinuationError,
};
pub use scenario_session::FullTextCommandSurfaceScenarioSession;
pub use source_address_projection_lease::{
    SourceAddressProjectionLease, SourceAddressSubmissionPort, SourceAddressSubmissionPortError,
};
pub use status_diagnostics_projection_lease::StatusDiagnosticsProjectionLease;
pub use tab_strip_projection_lease::{
    TabStripContextMenuPresentation, TabStripControlPresentation, TabStripCorrelation,
    TabStripGroupCapabilities, TabStripGroupDescriptor, TabStripGroupPopupPresentation,
    TabStripGroupTarget, TabStripMenuEntry, TabStripMenuOperation, TabStripNavigationPresentation,
    TabStripProjection, TabStripProjectionLease, TabStripScrollPresentation,
    TabStripSurfaceCapabilities, TabStripSwatchDescriptor, TabStripSwatchTarget,
    TabStripTabCapabilities, TabStripTabDescriptor, TabStripTabTarget, TabStripText,
};
pub use tab_strip_proposal_port::{
    TabStripGroupPlacement, TabStripProposal, TabStripProposalOperation, TabStripProposalPort,
    TabStripProposalPortError, TabStripProposalPortHandle, TabStripTabPlacement,
};
pub use tab_strip_text_raster::{TabStripTextRaster, TabStripTextRasterizer};
#[doc(hidden)]
pub use types::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceChild,
    EguiTextCommandSurfaceError, EguiTextCommandSurfaceFloatingPresentation,
    EguiTextCommandSurfaceOutput, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceSearchPresentation, TextCommandSurfaceStyle,
};
pub use unicode_evidence::{
    CONTROL_STAR_TEXT, IME_COMMIT_TEXT, IME_PREEDIT_TEXT, KucAccessKitNodeObservation, KucBounds,
    KucCaretObservation, KucHitTestObservation, KucImeTraceEvidence, KucRgbaCropEvidence,
    KucUnicodeColorGlyphEvidence, KucUnicodeColorGlyphEvidenceBuilder,
    KucUnicodeColorGlyphEvidenceCapture, KucUnicodeColorGlyphEvidenceError,
    KucUnicodeColorGlyphEvidenceInput, KucUnicodeColorGlyphEvidenceOptions,
    KucUnicodeColorGlyphEvidenceProfile, STAR_TEXT, UNICODE_EVIDENCE_SCHEMA,
    UNICODE_EVIDENCE_SCHEMA_VERSION, ZWJ_TEXT,
};
