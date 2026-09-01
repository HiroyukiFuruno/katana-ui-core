use super::super::EditorViewportProjectionLease;
use super::super::root::EguiTextCommandSurfaceRootOutput;
use super::super::source_address_projection_lease::SourceAddressProjectionLease;
use super::super::status_diagnostics_projection_lease::StatusDiagnosticsProjectionLease;
use super::super::tab_strip_projection_lease::TabStripProjectionLease;
use super::super::types::{EguiTextCommandSurfacePresentation, TextCommandSurfaceStyle};
use super::host_root_process::HostRootProcess;
use crate::molecule::command_chrome::CommandChromeFamilyId;
use serde::{Deserialize, Serialize};

pub(super) type HostProjectionParts = (
    EguiTextCommandSurfacePresentationToken,
    Box<dyn super::super::root::KucRootEffectRouter>,
    Option<SourceAddressProjectionLease>,
    Option<TabStripProjectionLease>,
    Option<StatusDiagnosticsProjectionLease>,
    Option<EditorViewportProjectionLease>,
);

/// Opaque host target token. Its payload cannot be inspected by a consumer.
pub struct EguiTextCommandSurfaceHostTargetToken {
    pub(super) payload: Box<[u8]>,
}

/// Opaque, revisioned presentation token accepted by the retained root.
pub struct EguiTextCommandSurfacePresentationToken {
    pub(super) revision: u64,
    pub(super) target: EguiTextCommandSurfaceHostTargetToken,
    pub(super) payload: Box<[u8]>,
}

/// Non-wire host projection lease carrying a private router for one retained root.
pub struct EguiTextCommandSurfaceHostProjectionLease {
    pub(super) token: EguiTextCommandSurfacePresentationToken,
    pub(super) router: Box<dyn super::super::root::KucRootEffectRouter>,
    pub(super) source_address: Option<SourceAddressProjectionLease>,
    pub(super) tab_strip: Option<TabStripProjectionLease>,
    pub(super) status_diagnostics: Option<StatusDiagnosticsProjectionLease>,
    pub(super) editor_viewport: Option<EditorViewportProjectionLease>,
}

/// Encodes host-projected generic presentation data into an opaque root token.
pub struct EguiTextCommandSurfaceHostProjectionEncoder;

/// Additive host projection for the opaque identities of the two command-chrome slots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiTextCommandSurfaceCommandFamilyProjection {
    pub(super) primary: Option<CommandChromeFamilyId>,
    pub(super) floating: Option<CommandChromeFamilyId>,
}

/// Factory for KUC-owned retained roots and their platform text catalog policy.
pub struct EguiTextCommandSurfaceRootFactory;

/// KUC-retained root. Consumers can inject tokens and call one root `show`.
pub struct EguiTextCommandSurfaceHostRoot {
    pub(super) process: HostRootProcess,
}

/// Closed root record returned by the consumer-safe facade.
/// One root frame. The event batch is intentionally inaccessible except through forwarding.
pub struct EguiTextCommandSurfaceHostRootFrame {
    pub(super) output: EguiTextCommandSurfaceRootOutput,
    pub(super) record: super::host_root_record::EguiTextCommandSurfaceHostRootRecord,
}

/// Errors raised while retaining or synchronizing the facade root.
#[derive(Debug)]
pub enum EguiTextCommandSurfaceRootFactoryError {
    InvalidToken(&'static str),
    IdentityChanged,
    StaleRevision { current: u64, received: u64 },
    RevisionConflict { revision: u64 },
    Decode(String),
    Root(String),
    OpaqueHostEffect,
    OpaqueHostEffectRejected,
    DuplicateLease { revision: u64 },
}

#[derive(Deserialize, Serialize)]
pub(super) struct RootPresentationWire {
    pub(super) presentation: EguiTextCommandSurfacePresentation,
    pub(super) style: TextCommandSurfaceStyle,
}

#[derive(Deserialize, Serialize)]
pub(super) struct RootPresentationWireWithCommandFamilies {
    pub(super) version: u8,
    pub(super) presentation: EguiTextCommandSurfacePresentation,
    pub(super) style: TextCommandSurfaceStyle,
    pub(super) command_families: EguiTextCommandSurfaceCommandFamilyProjection,
}
