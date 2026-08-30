#[path = "sanitized_document_root_style_builder.rs"]
mod sanitized_document_root_style_builder;

use super::super::types::{EguiTextCommandSurfaceError, TextCommandSurfaceStyle};

/// Closed style selection for the first sanitized document root foundation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SanitizedDocumentRootStyleKey {
    /// KUC's default TextSurface document presentation.
    #[default]
    Default,
}

/// Resolves the closed public key inside KUC's style boundary.
pub(super) fn resolve_style(
    key: SanitizedDocumentRootStyleKey,
) -> Result<TextCommandSurfaceStyle, EguiTextCommandSurfaceError> {
    match key {
        SanitizedDocumentRootStyleKey::Default => {
            sanitized_document_root_style_builder::default_style()
        }
    }
}

#[cfg(test)]
#[path = "sanitized_document_root_style_tests.rs"]
mod tests;
