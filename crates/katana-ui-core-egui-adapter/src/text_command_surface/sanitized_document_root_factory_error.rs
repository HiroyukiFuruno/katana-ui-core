use crate::text_command_surface::sanitized_document_root::sanitized_document_root_process::SanitizedDocumentRootProcessError;

/// Errors reserved for the retained sanitized document root contract.
#[derive(Debug, PartialEq, Eq)]
pub enum SanitizedDocumentRootFactoryError {
    IdentityChanged,
    StaleRevision {
        current: u64,
        received: u64,
    },
    RevisionConflict {
        revision: u64,
    },
    Render(String),
    EventBatchUnavailable,
    SearchCapability(crate::text_command_surface::sanitized_document_root::sanitized_search_projection::SanitizedSearchCapabilityRejection),
    CommandCapability(crate::text_command_surface::sanitized_document_root::sanitized_command_projection::SanitizedCommandCapabilityRejection),
    ContextMenuCapability(
        crate::text_command_surface::sanitized_document_root::sanitized_context_projection::SanitizedContextMenuCapabilityRejection,
    ),
}

impl From<SanitizedDocumentRootProcessError> for SanitizedDocumentRootFactoryError {
    fn from(value: SanitizedDocumentRootProcessError) -> Self {
        match value {
            SanitizedDocumentRootProcessError::IdentityChanged => Self::IdentityChanged,
            SanitizedDocumentRootProcessError::StaleRevision { current, received } => {
                Self::StaleRevision { current, received }
            }
            SanitizedDocumentRootProcessError::RevisionConflict { revision } => {
                Self::RevisionConflict { revision }
            }
        }
    }
}

impl std::fmt::Display for SanitizedDocumentRootFactoryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdentityChanged => {
                formatter.write_str("sanitized document root identity cannot change")
            }
            Self::StaleRevision { current, received } => write!(
                formatter,
                "sanitized document root revision {received} is stale; current is {current}"
            ),
            Self::RevisionConflict { revision } => {
                write!(
                    formatter,
                    "sanitized document root revision {revision} conflicts"
                )
            }
            Self::Render(error) => {
                write!(formatter, "sanitized document root render failed: {error}")
            }
            Self::EventBatchUnavailable => {
                formatter.write_str("sanitized document root event batch is unavailable")
            }
            Self::SearchCapability(_) => {
                formatter.write_str("sanitized search capability rejected")
            }
            Self::CommandCapability(_) => {
                formatter.write_str("sanitized command capability rejected")
            }
            Self::ContextMenuCapability(_) => {
                formatter.write_str("sanitized context menu capability rejected")
            }
        }
    }
}

impl std::error::Error for SanitizedDocumentRootFactoryError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_command_surface::sanitized_document_root::{
        sanitized_command_projection::SanitizedCommandCapabilityRejection,
        sanitized_context_projection::SanitizedContextMenuCapabilityRejection,
        sanitized_search_projection::SanitizedSearchCapabilityRejection,
    };

    #[test]
    fn process_errors_convert_without_losing_revision_data() {
        let cases = [
            (
                SanitizedDocumentRootProcessError::IdentityChanged,
                SanitizedDocumentRootFactoryError::IdentityChanged,
            ),
            (
                SanitizedDocumentRootProcessError::StaleRevision {
                    current: 4,
                    received: 3,
                },
                SanitizedDocumentRootFactoryError::StaleRevision {
                    current: 4,
                    received: 3,
                },
            ),
            (
                SanitizedDocumentRootProcessError::RevisionConflict { revision: 5 },
                SanitizedDocumentRootFactoryError::RevisionConflict { revision: 5 },
            ),
        ];
        for (source, expected) in cases {
            assert_eq!(SanitizedDocumentRootFactoryError::from(source), expected);
        }
    }

    #[test]
    fn factory_error_display_covers_every_variant() {
        let cases = [
            (
                SanitizedDocumentRootFactoryError::IdentityChanged,
                "sanitized document root identity cannot change",
            ),
            (
                SanitizedDocumentRootFactoryError::StaleRevision {
                    current: 4,
                    received: 3,
                },
                "sanitized document root revision 3 is stale; current is 4",
            ),
            (
                SanitizedDocumentRootFactoryError::RevisionConflict { revision: 5 },
                "sanitized document root revision 5 conflicts",
            ),
            (
                SanitizedDocumentRootFactoryError::Render("render".into()),
                "sanitized document root render failed: render",
            ),
            (
                SanitizedDocumentRootFactoryError::SearchCapability(
                    SanitizedSearchCapabilityRejection::Missing,
                ),
                "sanitized search capability rejected",
            ),
            (
                SanitizedDocumentRootFactoryError::CommandCapability(
                    SanitizedCommandCapabilityRejection::Missing,
                ),
                "sanitized command capability rejected",
            ),
            (
                SanitizedDocumentRootFactoryError::ContextMenuCapability(
                    SanitizedContextMenuCapabilityRejection::Missing,
                ),
                "sanitized context menu capability rejected",
            ),
        ];
        for (error, expected) in cases {
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn factory_error_implements_error() {
        let error: &dyn std::error::Error = &SanitizedDocumentRootFactoryError::IdentityChanged;
        assert_eq!(
            error.to_string(),
            "sanitized document root identity cannot change"
        );
    }
}
