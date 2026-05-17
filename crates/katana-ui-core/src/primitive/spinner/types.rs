use crate::theme::color::Color;

/// Spinner size in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpinnerSize {
    Pt(f32),
    Sm,
    Md,
    Lg,
    Xl,
}

impl SpinnerSize {
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

/// Properties for the Spinner primitive.
#[derive(Debug, Clone)]
pub struct SpinnerProps {
    pub size: SpinnerSize,
    pub color_override: Option<Color>,
    /// Rotation speed in revolutions per second.
    pub speed_rps: f32,
    /// When true, animation should stop (respects prefers-reduced-motion).
    pub reduced_motion: bool,
}

impl Default for SpinnerProps {
    fn default() -> Self {
        Self {
            size: SpinnerSize::Lg,
            color_override: None,
            speed_rps: 1.0,
            reduced_motion: false,
        }
    }
}
