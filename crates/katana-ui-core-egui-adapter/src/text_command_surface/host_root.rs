//! Consumer-safe retained root facade.

#[path = "host_root_api.rs"]
mod host_root_api;
#[path = "host_root_error.rs"]
mod host_root_error;
#[path = "host_root_facade.rs"]
mod host_root_facade;
#[path = "host_root_frame.rs"]
mod host_root_frame;
#[path = "host_root_process.rs"]
mod host_root_process;
#[path = "host_root_projection.rs"]
mod host_root_projection;
#[path = "host_root_record.rs"]
mod host_root_record;
#[path = "host_root_surface.rs"]
mod host_root_surface;
#[path = "host_root_token_codec.rs"]
mod host_root_token_codec;
use super::root::EguiTextCommandSurfaceRootOutput;
use super::types::{EguiTextCommandSurfacePresentation, TextCommandSurfaceStyle};
use host_root_process::HostRootProcess;
use host_root_token_codec::decode_token;
use host_root_token_codec::{encode_presentation, encode_presentation_with_command_families};
use serde::{Deserialize, Serialize};

pub use host_root_projection::{
    EguiTextCommandSurfaceCommandFamilyProjection, EguiTextCommandSurfaceHostProjectionLease,
};
pub use host_root_record::{
    EguiTextCommandSurfaceHostRootRecord, EguiTextCommandSurfaceHostRootRecordDimensions,
};
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

/// Encodes host-projected generic presentation data into an opaque root token.
pub struct EguiTextCommandSurfaceHostProjectionEncoder;

impl EguiTextCommandSurfaceHostProjectionEncoder {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn token(
        revision: u64,
        target: impl Into<Vec<u8>>,
        presentation: EguiTextCommandSurfacePresentation,
        style: TextCommandSurfaceStyle,
    ) -> Result<EguiTextCommandSurfacePresentationToken, serde_json::Error> {
        encode_presentation(revision, target.into(), presentation, style)
    }

    pub fn encode(
        &self,
        revision: u64,
        target: impl Into<Vec<u8>>,
        presentation: EguiTextCommandSurfacePresentation,
        style: TextCommandSurfaceStyle,
    ) -> Result<EguiTextCommandSurfacePresentationToken, serde_json::Error> {
        Self::token(revision, target, presentation, style)
    }

    pub fn token_with_command_families(
        revision: u64,
        target: impl Into<Vec<u8>>,
        presentation: EguiTextCommandSurfacePresentation,
        style: TextCommandSurfaceStyle,
        command_families: EguiTextCommandSurfaceCommandFamilyProjection,
    ) -> Result<EguiTextCommandSurfacePresentationToken, serde_json::Error> {
        encode_presentation_with_command_families(
            revision,
            target.into(),
            presentation,
            style,
            command_families,
        )
    }

    pub fn encode_with_command_families(
        &self,
        revision: u64,
        target: impl Into<Vec<u8>>,
        presentation: EguiTextCommandSurfacePresentation,
        style: TextCommandSurfaceStyle,
        command_families: EguiTextCommandSurfaceCommandFamilyProjection,
    ) -> Result<EguiTextCommandSurfacePresentationToken, serde_json::Error> {
        Self::token_with_command_families(revision, target, presentation, style, command_families)
    }
}

impl Default for EguiTextCommandSurfaceHostProjectionEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Factory for KUC-owned retained roots and their platform text catalog policy.
pub struct EguiTextCommandSurfaceRootFactory;

impl EguiTextCommandSurfaceRootFactory {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    pub fn retain(
        &self,
        token: EguiTextCommandSurfacePresentationToken,
    ) -> Result<EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfaceRootFactoryError> {
        let decoded = decode_token(&token)?;
        Ok(EguiTextCommandSurfaceHostRoot {
            process: HostRootProcess::retain(decoded, token.revision)?,
        })
    }

    pub fn retain_with_lease(
        &self,
        lease: EguiTextCommandSurfaceHostProjectionLease,
    ) -> Result<EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfaceRootFactoryError> {
        let (token, router) = lease.into_parts();
        let decoded = decode_token(&token)?;
        Ok(EguiTextCommandSurfaceHostRoot {
            process: HostRootProcess::retain_with_router(decoded, token.revision, router)?,
        })
    }
}

impl Default for EguiTextCommandSurfaceRootFactory {
    fn default() -> Self {
        Self::new()
    }
}

/// KUC-retained root. Consumers can inject tokens and call one root `show`.
pub struct EguiTextCommandSurfaceHostRoot {
    pub(super) process: HostRootProcess,
}

impl EguiTextCommandSurfaceHostRoot {}

/// Closed root record returned by the consumer-safe facade.
/// One root frame. The event batch is intentionally inaccessible except through forwarding.
pub struct EguiTextCommandSurfaceHostRootFrame {
    pub(super) output: EguiTextCommandSurfaceRootOutput,
    pub(super) record: EguiTextCommandSurfaceHostRootRecord,
}

impl EguiTextCommandSurfaceHostRootFrame {}

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

#[cfg(test)]
#[path = "host_root_tests.rs"]
mod tests;
