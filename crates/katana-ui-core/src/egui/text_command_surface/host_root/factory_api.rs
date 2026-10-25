use super::host_root_process::HostRootProcess;
use super::host_root_token_codec::decode_token;
use super::host_root_types::{
    EguiTextCommandSurfaceHostProjectionLease, EguiTextCommandSurfaceHostRoot,
    EguiTextCommandSurfacePresentationToken, EguiTextCommandSurfaceRootFactory,
    EguiTextCommandSurfaceRootFactoryError,
};

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
            process: HostRootProcess::retain(decoded, token.revision())?,
        })
    }
    #[cfg(any(test, feature = "storybook-artifacts"))]
    pub(crate) fn has_same_root_identity(
        &self,
        left: &EguiTextCommandSurfacePresentationToken,
        right: &EguiTextCommandSurfacePresentationToken,
    ) -> Result<bool, EguiTextCommandSurfaceRootFactoryError> {
        Ok(decode_token(left)?.identity == decode_token(right)?.identity)
    }
    pub fn retain_with_lease(
        &self,
        lease: EguiTextCommandSurfaceHostProjectionLease,
    ) -> Result<EguiTextCommandSurfaceHostRoot, EguiTextCommandSurfaceRootFactoryError> {
        let (token, attachments) = lease.into_parts_with_attachments();
        let decoded = decode_token(&token)?;
        HostRootProcess::retain_with_router(decoded, token.revision(), attachments)
            .map(|process| EguiTextCommandSurfaceHostRoot { process })
    }
}
impl Default for EguiTextCommandSurfaceRootFactory {
    fn default() -> Self {
        Self::new()
    }
}
