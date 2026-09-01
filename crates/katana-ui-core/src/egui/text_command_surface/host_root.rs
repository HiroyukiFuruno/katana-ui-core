//! Consumer-safe retained root facade.

#[path = "host_root/command_families.rs"]
mod host_root_command_families;
#[path = "host_root/errors.rs"]
mod host_root_errors;
#[path = "host_root_facade.rs"]
mod host_root_facade;
#[path = "host_root/frame.rs"]
mod host_root_frame;
#[path = "host_root_process.rs"]
mod host_root_process;
#[path = "host_root_record.rs"]
mod host_root_record;
#[path = "host_root_surface.rs"]
mod host_root_surface;
#[path = "host_root_token_codec.rs"]
mod host_root_token_codec;
#[path = "host_root/types.rs"]
mod host_root_types;

use super::EditorViewportProjectionLease;
use super::root::KucRootEffectRouter;
use super::source_address_projection_lease::SourceAddressProjectionLease;
use super::status_diagnostics_projection_lease::StatusDiagnosticsProjectionLease;
use super::tab_strip_projection_lease::TabStripProjectionLease;
use super::types::{EguiTextCommandSurfacePresentation, TextCommandSurfaceStyle};
use host_root_process::HostRootProcess;
use host_root_token_codec::decode_token;
use host_root_token_codec::{encode_presentation, encode_presentation_with_command_families};
pub use host_root_types::{
    EguiTextCommandSurfaceCommandFamilyProjection, EguiTextCommandSurfaceHostProjectionEncoder,
    EguiTextCommandSurfaceHostProjectionLease, EguiTextCommandSurfaceHostRoot,
    EguiTextCommandSurfaceHostRootFrame, EguiTextCommandSurfaceHostTargetToken,
    EguiTextCommandSurfacePresentationToken, EguiTextCommandSurfaceRootFactory,
    EguiTextCommandSurfaceRootFactoryError,
};
use host_root_types::{
    HostProjectionParts, RootPresentationWire, RootPresentationWireWithCommandFamilies,
};

pub use host_root_record::{
    EguiTextCommandSurfaceHostRootRecord, EguiTextCommandSurfaceHostRootRecordDimensions,
};

impl EguiTextCommandSurfaceHostTargetToken {
    /// Creates a target token from host-owned opaque bytes.
    #[must_use]
    pub fn from_opaque_bytes(payload: impl Into<Vec<u8>>) -> Self {
        Self {
            payload: payload.into().into_boxed_slice(),
        }
    }
}

impl std::fmt::Debug for EguiTextCommandSurfaceHostTargetToken {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("EguiTextCommandSurfaceHostTargetToken(..)")
    }
}

impl EguiTextCommandSurfacePresentationToken {
    /// Creates a non-reusable presentation token from host-projected opaque bytes.
    #[must_use]
    pub fn from_opaque_bytes(
        revision: u64,
        target: EguiTextCommandSurfaceHostTargetToken,
        payload: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            revision,
            target,
            payload: payload.into().into_boxed_slice(),
        }
    }

    pub(super) fn from_encoded(
        revision: u64,
        target: EguiTextCommandSurfaceHostTargetToken,
        payload: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            revision,
            target,
            payload: payload.into().into_boxed_slice(),
        }
    }
}

impl std::fmt::Debug for EguiTextCommandSurfacePresentationToken {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EguiTextCommandSurfacePresentationToken")
            .field("revision", &self.revision)
            .field("target", &self.target)
            .field("payload", &"<opaque>")
            .finish()
    }
}

impl EguiTextCommandSurfaceHostProjectionLease {
    #[must_use]
    pub fn new<R>(token: EguiTextCommandSurfacePresentationToken, router: R) -> Self
    where
        R: KucRootEffectRouter + 'static,
    {
        Self {
            token,
            router: Box::new(router),
            source_address: None,
            tab_strip: None,
            status_diagnostics: None,
            editor_viewport: None,
        }
    }

    #[must_use]
    pub fn from_router(
        token: EguiTextCommandSurfacePresentationToken,
        router: Box<dyn KucRootEffectRouter>,
    ) -> Self {
        Self {
            token,
            router,
            source_address: None,
            tab_strip: None,
            status_diagnostics: None,
            editor_viewport: None,
        }
    }

    /// Adds one opaque source-address projection to this consuming lease.
    #[must_use]
    pub fn with_source_address(mut self, lease: SourceAddressProjectionLease) -> Self {
        self.source_address = Some(lease);
        self
    }

    /// Adds one opaque TabStrip projection to this consuming lease.
    #[must_use]
    pub fn with_tab_strip(mut self, lease: TabStripProjectionLease) -> Self {
        self.tab_strip = Some(lease);
        self
    }

    /// Adds one opaque status/diagnostics projection to this consuming lease.
    #[must_use]
    pub fn with_status_diagnostics(mut self, lease: StatusDiagnosticsProjectionLease) -> Self {
        self.status_diagnostics = Some(lease);
        self
    }

    /// Adds one generic document/preview split viewport to this consuming lease.
    #[must_use]
    pub fn with_editor_viewport(mut self, lease: EditorViewportProjectionLease) -> Self {
        self.editor_viewport = Some(lease);
        self
    }

    pub(super) fn into_parts(self) -> HostProjectionParts {
        (
            self.token,
            self.router,
            self.source_address,
            self.tab_strip,
            self.status_diagnostics,
            self.editor_viewport,
        )
    }
}

impl std::fmt::Debug for EguiTextCommandSurfaceHostProjectionLease {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("EguiTextCommandSurfaceHostProjectionLease(..)")
    }
}

impl EguiTextCommandSurfaceHostProjectionEncoder {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Creates one revisioned token without exposing the host target or payload.
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
        let (token, router, source_address, tab_strip, status_diagnostics, editor_viewport) =
            lease.into_parts();
        let decoded = decode_token(&token)?;
        HostRootProcess::retain_with_router(
            decoded,
            token.revision,
            router,
            source_address,
            tab_strip,
            status_diagnostics,
            editor_viewport,
        )
        .map(|process| EguiTextCommandSurfaceHostRoot { process })
    }
}

impl Default for EguiTextCommandSurfaceRootFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "host_root_tests.rs"]
mod tests;
