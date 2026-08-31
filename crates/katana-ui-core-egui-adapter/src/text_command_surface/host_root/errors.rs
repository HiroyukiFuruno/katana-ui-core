use super::host_root_types::EguiTextCommandSurfaceRootFactoryError;

impl From<super::super::types::EguiTextCommandSurfaceError>
    for EguiTextCommandSurfaceRootFactoryError
{
    fn from(error: super::super::types::EguiTextCommandSurfaceError) -> Self {
        Self::Root(error.to_string())
    }
}

impl From<super::super::root::EguiTextCommandSurfaceRootError>
    for EguiTextCommandSurfaceRootFactoryError
{
    fn from(error: super::super::root::EguiTextCommandSurfaceRootError) -> Self {
        Self::Root(error.to_string())
    }
}

impl From<super::super::root::KucOpaqueHostEffectAttachError>
    for EguiTextCommandSurfaceRootFactoryError
{
    fn from(_: super::super::root::KucOpaqueHostEffectAttachError) -> Self {
        Self::OpaqueHostEffectRejected
    }
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
