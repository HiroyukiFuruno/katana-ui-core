//! Root facade and factory API for retained text command surfaces.

use super::super::root::EguiTextCommandSurfaceRootOutput;
use super::super::types::{EguiTextCommandSurfacePresentation, TextCommandSurfaceStyle};
use super::host_root_process::HostRootProcess;
use super::host_root_projection::{
    EguiTextCommandSurfaceCommandFamilyProjection, EguiTextCommandSurfaceHostProjectionLease,
    EguiTextCommandSurfacePresentationToken,
};
use super::host_root_record::EguiTextCommandSurfaceHostRootRecord;
use super::host_root_token_codec::decode_token;

/// Factory for KUC-owned retained roots and their platform text catalog policy.
pub struct EguiTextCommandSurfaceRootFactory;

impl EguiTextCommandSurfaceRootFactory {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Retains a root from one opaque presentation token.
    pub fn retain(
        &self,
        token: EguiTextCommandSurfacePresentationToken,
    ) -> Result<EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfaceRootFactoryError> {
        let decoded = decode_token(&token)?;
        Ok(EguiTextCommandSurfaceHostRoot {
            process: HostRootProcess::retain(decoded, token.revision)?,
        })
    }

    /// Retains a root with a non-wire host effect lease.
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

impl EguiTextCommandSurfaceHostRootFrame {
    #[cfg(feature = "storybook-artifacts")]
    pub(crate) fn artifact_rgba(&self) -> (&[u8], u32, u32, &str) {
        let dimensions = self.record.dimensions();
        (
            self.output.rgba_pixels(),
            dimensions.width(),
            dimensions.height(),
            self.record.rgba_hash(),
        )
    }
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

impl std::fmt::Display for EguiTextCommandSurfaceRootFactoryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken(reason) => write!(formatter, "invalid root token: {reason}"),
            Self::IdentityChanged => {
                formatter.write_str("root identity cannot change while retained")
            }
            Self::StaleRevision { current, received } => write!(
                formatter,
                "stale root presentation revision {received}; current is {current}"
            ),
            Self::RevisionConflict { revision } => {
                write!(
                    formatter,
                    "root presentation revision {revision} was already retained"
                )
            }
            Self::Decode(error) => {
                write!(formatter, "root presentation token decode failed: {error}")
            }
            Self::Root(error) => error.fmt(formatter),
            Self::OpaqueHostEffect => formatter.write_str("opaque host effect router failed"),
            Self::OpaqueHostEffectRejected => {
                formatter.write_str("opaque host effect batch was rejected")
            }
            Self::DuplicateLease { revision } => {
                write!(
                    formatter,
                    "root lease revision {revision} was already consumed"
                )
            }
        }
    }
}

impl std::error::Error for EguiTextCommandSurfaceRootFactoryError {}

#[derive(serde::Deserialize, serde::Serialize)]
pub(super) struct RootPresentationWire {
    pub(super) presentation: EguiTextCommandSurfacePresentation,
    pub(super) style: TextCommandSurfaceStyle,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub(super) struct RootPresentationWireWithCommandFamilies {
    pub(super) version: u8,
    pub(super) presentation: EguiTextCommandSurfacePresentation,
    pub(super) style: TextCommandSurfaceStyle,
    pub(super) command_families: EguiTextCommandSurfaceCommandFamilyProjection,
}
