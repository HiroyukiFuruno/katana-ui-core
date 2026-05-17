use crate::theme::color::Color;

/// SVG source for an icon.
#[derive(Debug, Clone)]
pub enum IconSource {
    SvgBytes(&'static [u8]),
    SvgString(String),
}

impl IconSource {
    pub(super) fn to_string_lossy(&self) -> std::borrow::Cow<'_, str> {
        match self {
            Self::SvgBytes(bytes) => std::str::from_utf8(bytes)
                .map(std::borrow::Cow::Borrowed)
                .unwrap_or_else(|_| std::borrow::Cow::Borrowed("")),
            Self::SvgString(s) => std::borrow::Cow::Borrowed(s.as_str()),
        }
    }
}

/// Icon size in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconSize {
    /// Explicit size in pts.
    Pt(f32),
    /// Maps to `SpacingTokens::sm` (8 px).
    Sm,
    /// Maps to `SpacingTokens::md` (12 px).
    Md,
    /// Maps to `SpacingTokens::lg` (16 px).
    Lg,
    /// Maps to `SpacingTokens::xl` (24 px).
    Xl,
}

impl IconSize {
    #[must_use]
    pub fn resolve_px(self, spacing: &crate::theme::SpacingTokens) -> f32 {
        match self {
            Self::Pt(px) => px,
            Self::Sm => spacing.sm,
            Self::Md => spacing.md,
            Self::Lg => spacing.lg,
            Self::Xl => spacing.xl,
        }
    }
}

/// Properties for the `Icon` primitive.
#[derive(Debug, Clone)]
pub struct IconProps {
    pub size: IconSize,
    pub color_override: Option<Color>,
}

impl Default for IconProps {
    fn default() -> Self {
        Self {
            size: IconSize::Lg,
            color_override: None,
        }
    }
}
