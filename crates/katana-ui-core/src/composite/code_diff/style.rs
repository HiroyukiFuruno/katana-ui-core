use crate::floem_view::FloemColor;
use crate::theme::Theme;
use floem::peniko::Color;

use super::types::CodeDiffLineKind;

const ROW_ALPHA: u8 = 32;
#[derive(Clone, Copy)]
pub(crate) struct CodeDiffStyle {
    pub surface: Color,
    pub border: Color,
    pub text: Color,
    pub muted: Color,
    pub added_bg: Color,
    pub removed_bg: Color,
    pub added_mark: Color,
    pub removed_mark: Color,
    pub omitted_bg: Color,
}

impl CodeDiffStyle {
    pub(crate) fn from_theme(theme: &Theme) -> Self {
        Self {
            surface: FloemColor::from_token(theme.color.surface),
            border: FloemColor::from_token(theme.color.border),
            text: FloemColor::from_token(theme.color.text),
            muted: FloemColor::from_token(theme.color.text_muted),
            added_bg: alpha(theme.color.success, ROW_ALPHA),
            removed_bg: alpha(theme.color.danger, ROW_ALPHA),
            added_mark: FloemColor::from_token(theme.color.success),
            removed_mark: FloemColor::from_token(theme.color.danger),
            omitted_bg: FloemColor::from_token(theme.color.accent_muted),
        }
    }

    pub(crate) fn row_bg(self, kind: CodeDiffLineKind) -> Color {
        match kind {
            CodeDiffLineKind::Added => self.added_bg,
            CodeDiffLineKind::Removed => self.removed_bg,
            CodeDiffLineKind::Equal | CodeDiffLineKind::Placeholder => self.surface,
        }
    }

    pub(crate) fn mark(self, kind: CodeDiffLineKind) -> Color {
        match kind {
            CodeDiffLineKind::Added => self.added_mark,
            CodeDiffLineKind::Removed => self.removed_mark,
            CodeDiffLineKind::Equal | CodeDiffLineKind::Placeholder => self.muted,
        }
    }
}

fn alpha(color: crate::theme::color::Color, alpha: u8) -> Color {
    Color::rgba8(color.r, color.g, color.b, alpha)
}
