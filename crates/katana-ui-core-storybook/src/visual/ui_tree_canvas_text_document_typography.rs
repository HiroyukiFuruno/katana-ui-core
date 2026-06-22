use super::{BODY_FONT_SIZE, CODE_FONT_SIZE, DOCUMENT_BODY_FONT_ROLE, DOCUMENT_CODE_FONT_ROLE};
use katana_ui_core::theme::ThemeSnapshot;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::visual) struct UiTreeDocumentTypography {
    pub(in crate::visual) body_font_size: f32,
    pub(in crate::visual) code_font_size: f32,
}

impl UiTreeDocumentTypography {
    pub(in crate::visual) fn from_theme(theme: &ThemeSnapshot) -> Self {
        Self {
            body_font_size: theme
                .font(DOCUMENT_BODY_FONT_ROLE)
                .map_or(BODY_FONT_SIZE, |font| font.size),
            code_font_size: theme
                .font(DOCUMENT_CODE_FONT_ROLE)
                .map_or(CODE_FONT_SIZE, |font| font.size),
        }
    }
}

impl Default for UiTreeDocumentTypography {
    fn default() -> Self {
        Self {
            body_font_size: BODY_FONT_SIZE,
            code_font_size: CODE_FONT_SIZE,
        }
    }
}
