use super::EguiTextCommandSurfaceRootFactoryError;

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
            Self::RevisionConflict { revision } => write!(
                formatter,
                "root presentation revision {revision} was already retained"
            ),
            Self::Decode(error) => {
                write!(formatter, "root presentation token decode failed: {error}")
            }
            Self::Root(error) => error.fmt(formatter),
            Self::OpaqueHostEffect => formatter.write_str("opaque host effect router failed"),
            Self::OpaqueHostEffectRejected => {
                formatter.write_str("opaque host effect batch was rejected")
            }
            Self::DuplicateLease { revision } => write!(
                formatter,
                "root lease revision {revision} was already consumed"
            ),
        }
    }
}

impl std::error::Error for EguiTextCommandSurfaceRootFactoryError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_factory_error_display_covers_every_variant() {
        let cases = [
            (
                EguiTextCommandSurfaceRootFactoryError::InvalidToken("reason"),
                "invalid root token: reason".to_owned(),
            ),
            (
                EguiTextCommandSurfaceRootFactoryError::IdentityChanged,
                "root identity cannot change while retained".to_owned(),
            ),
            (
                EguiTextCommandSurfaceRootFactoryError::StaleRevision {
                    current: 4,
                    received: 3,
                },
                "stale root presentation revision 3; current is 4".to_owned(),
            ),
            (
                EguiTextCommandSurfaceRootFactoryError::RevisionConflict { revision: 5 },
                "root presentation revision 5 was already retained".to_owned(),
            ),
            (
                EguiTextCommandSurfaceRootFactoryError::Decode("decode".into()),
                "root presentation token decode failed: decode".to_owned(),
            ),
            (
                EguiTextCommandSurfaceRootFactoryError::Root("root".into()),
                "root".to_owned(),
            ),
            (
                EguiTextCommandSurfaceRootFactoryError::OpaqueHostEffect,
                "opaque host effect router failed".to_owned(),
            ),
            (
                EguiTextCommandSurfaceRootFactoryError::OpaqueHostEffectRejected,
                "opaque host effect batch was rejected".to_owned(),
            ),
            (
                EguiTextCommandSurfaceRootFactoryError::DuplicateLease { revision: 6 },
                "root lease revision 6 was already consumed".to_owned(),
            ),
        ];

        for (error, expected) in cases {
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn root_factory_error_implements_error() {
        let error: &dyn std::error::Error =
            &EguiTextCommandSurfaceRootFactoryError::IdentityChanged;
        assert_eq!(
            error.to_string(),
            "root identity cannot change while retained"
        );
    }
}
