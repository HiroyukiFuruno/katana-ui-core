use super::{BODY_FONT_SIZE, CODE_FONT_SIZE, DOCUMENT_BODY_FONT_ROLE, DOCUMENT_CODE_FONT_ROLE};
use crate::raster_host::document_typography::UiTreeDocumentTypography as UiTreeDocumentTypographyOverrides;
use katana_ui_core::theme::ThemeSnapshot;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::raster_host) struct UiTreeDocumentTypography {
    pub(in crate::raster_host) body_font_size: f32,
    pub(in crate::raster_host) code_font_size: f32,
    pub(in crate::raster_host) document_typography: UiTreeDocumentTypographyOverrides,
}

impl UiTreeDocumentTypography {
    pub(in crate::raster_host) fn from_theme(theme: &ThemeSnapshot) -> Self {
        Self {
            body_font_size: theme
                .font(DOCUMENT_BODY_FONT_ROLE)
                .map_or(BODY_FONT_SIZE, |font| font.size),
            code_font_size: theme
                .font(DOCUMENT_CODE_FONT_ROLE)
                .map_or(CODE_FONT_SIZE, |font| font.size),
            document_typography: UiTreeDocumentTypographyOverrides::default(),
        }
    }

    pub(in crate::raster_host) fn from_theme_with_document_typography(
        theme: &ThemeSnapshot,
        document_typography: UiTreeDocumentTypographyOverrides,
    ) -> Self {
        let mut typography = Self::from_theme(theme);
        typography.document_typography = document_typography;
        typography
    }
}

impl Default for UiTreeDocumentTypography {
    fn default() -> Self {
        Self {
            body_font_size: BODY_FONT_SIZE,
            code_font_size: CODE_FONT_SIZE,
            document_typography: UiTreeDocumentTypographyOverrides::default(),
        }
    }
}
