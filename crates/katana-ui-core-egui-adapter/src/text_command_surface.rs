//! KUC-owned root composition for a text surface and its generic command chrome.

pub(crate) mod accesskit_evidence;
mod artifact;
mod composition;
mod context_menu;
mod host_root;
mod model;
#[path = "text_command_surface/root.rs"]
mod root;
mod sanitized_document_root;
mod synchronization;
mod text_command_surface_style_factory;
mod types;
mod unicode_evidence;

pub use artifact::EguiTextCommandSurfaceArtifactError;
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
    EguiTextCommandSurfaceRootFrame, EguiTextCommandSurfaceRootOutput, KucRootEventBatchDispatcher,
    KucRootEventBatchForwarder,
};
pub use root::{
    KucInteractionActionClass, KucInteractionLocator, KucInteractionLocatorError,
    KucInteractionRequestError, KucInteractionSelector, KucOpaqueInteractionRequest,
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
#[doc(hidden)]
pub use types::{
    EguiTextCommandSurface, EguiTextCommandSurfaceAdapter, EguiTextCommandSurfaceChild,
    EguiTextCommandSurfaceError, EguiTextCommandSurfaceFloatingPresentation,
    EguiTextCommandSurfaceOutput, EguiTextCommandSurfacePresentation,
    EguiTextCommandSurfaceSearchPresentation, TextCommandSurfaceStyle,
};
pub use unicode_evidence::{
    CONTROL_STAR_TEXT, IME_COMMIT_TEXT, IME_PREEDIT_TEXT, KucBounds, KucCaretObservation,
    KucHitTestObservation, KucImeTraceEvidence, KucRgbaCropEvidence, KucUnicodeColorGlyphEvidence,
    KucUnicodeColorGlyphEvidenceBuilder, KucUnicodeColorGlyphEvidenceCapture,
    KucUnicodeColorGlyphEvidenceError, KucUnicodeColorGlyphEvidenceInput,
    KucUnicodeColorGlyphEvidenceOptions, KucUnicodeColorGlyphEvidenceProfile, STAR_TEXT,
    UNICODE_EVIDENCE_SCHEMA, UNICODE_EVIDENCE_SCHEMA_VERSION, ZWJ_TEXT,
};
