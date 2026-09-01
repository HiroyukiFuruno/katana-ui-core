use super::{EguiTextCommandSurfaceHostTargetToken, EguiTextCommandSurfacePresentationToken};

impl EguiTextCommandSurfaceHostTargetToken {
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
