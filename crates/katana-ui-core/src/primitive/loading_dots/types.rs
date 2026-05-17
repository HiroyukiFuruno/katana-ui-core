use crate::theme::color::Color;

pub(super) const ACTIVE_SCALE: f32 = 1.35;
pub(super) const INACTIVE_SCALE: f32 = 0.75;
pub(super) const ACTIVE_ALPHA: f32 = 1.0;
pub(super) const INACTIVE_ALPHA: f32 = 0.35;
pub(super) const DEFAULT_COUNT: usize = 3;
pub(super) const DEFAULT_SIZE: f32 = 6.0;
pub(super) const DEFAULT_GAP: f32 = 6.0;
pub(super) const DEFAULT_SPEED_MS: u64 = 260;
pub(super) const MIN_SIZE: f32 = 2.0;

/// Properties for the `LoadingDots` primitive.
#[derive(Debug, Clone)]
pub struct LoadingDotsProps {
    /// Number of dots to render.
    pub dot_count: usize,
    /// Dot diameter in logical pixels.
    pub dot_size: f32,
    /// Horizontal gap between dots.
    pub dot_gap: f32,
    /// Base color of active dots.
    pub color_override: Option<Color>,
    /// Whether the animation is running.
    pub active: bool,
    /// Interval for advancing the active dot in milliseconds.
    pub animation_speed_ms: u64,
    /// Optional trailing label.
    pub label: Option<String>,
}

impl Default for LoadingDotsProps {
    fn default() -> Self {
        Self {
            dot_count: DEFAULT_COUNT,
            dot_size: DEFAULT_SIZE,
            dot_gap: DEFAULT_GAP,
            color_override: None,
            active: true,
            animation_speed_ms: DEFAULT_SPEED_MS,
            label: None,
        }
    }
}
