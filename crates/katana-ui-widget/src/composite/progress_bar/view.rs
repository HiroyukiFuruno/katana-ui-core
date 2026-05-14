use crate::theme::Theme;
use crate::theme::color::Color;

use super::types::ProgressBarProps;

pub(super) const DEFAULT_TRACK_WIDTH: f32 = 260.0;
pub(super) const MIN_TRACK_WIDTH: f32 = 120.0;
pub(super) const DEFAULT_BAR_SIZE: f32 = 10.0;
pub(super) const DEFAULT_RADIUS: f32 = 5.0;
pub(super) const MIN_BAR_SIZE: f32 = 4.0;
pub(super) const SWEEP_STEPS: u64 = 40;
pub(super) const SWEEP_CYCLE: u64 = SWEEP_STEPS * 2;
pub(super) const DEFAULT_ANIMATION_SPEED_MS: u64 = 80;
pub(super) const INDETERMINATE_BAND_RATIO: f32 = 0.35;

pub(super) fn resolve_size(requested: f32) -> f32 {
    if requested > 0.0 {
        requested
    } else {
        DEFAULT_BAR_SIZE
    }
}

pub(super) fn resolve_radius(requested: f32, size: f32) -> f32 {
    if requested < 0.0 {
        DEFAULT_RADIUS.min(size)
    } else {
        requested.min(size)
    }
}

pub(super) fn resolve_track_width(requested: f32) -> f32 {
    if requested > 0.0 {
        requested
    } else {
        DEFAULT_TRACK_WIDTH
    }
}

pub(super) fn resolve_track_color(requested: Option<Color>, theme: &Theme) -> Color {
    requested.unwrap_or(theme.color.border)
}

pub(super) fn resolve_fill_color(requested: Option<Color>, theme: &Theme) -> Color {
    requested.unwrap_or(theme.color.accent)
}

pub(super) fn normalize_progress(props: &ProgressBarProps) -> f32 {
    let min = if props.min.is_finite() {
        props.min
    } else {
        0.0
    };
    let max = if props.max.is_finite() {
        props.max
    } else {
        1.0
    };
    let value = if props.value.is_finite() {
        props.value
    } else {
        min
    };
    if !props.indeterminate {
        let (low, high) = if min <= max { (min, max) } else { (max, min) };
        let span = high - low;
        if span.abs() <= f32::EPSILON {
            return 0.0;
        }
        ((value.clamp(low, high) - low) / span).clamp(0.0, 1.0)
    } else {
        0.0
    }
}

pub(super) fn percent_label(progress: f32) -> String {
    format!("{:.0}%", progress.clamp(0.0, 1.0) * 100.0)
}

pub(super) fn label_text(progress: f32, indeterminate: bool, label: &Option<String>) -> String {
    if let Some(custom) = label {
        custom.clone()
    } else if indeterminate {
        "Loading...".to_string()
    } else {
        percent_label(progress)
    }
}

pub(super) fn indeterminate_offset(frame: u64, track_width: f32, band_width: f32) -> f32 {
    let band_range = (track_width - band_width).max(0.0);
    if band_range <= 0.0 {
        return 0.0;
    }
    let step = (frame % SWEEP_CYCLE) as f32;
    let cycle_pos = if step > SWEEP_STEPS as f32 {
        SWEEP_CYCLE as f32 - step
    } else {
        step
    };
    (cycle_pos / SWEEP_STEPS as f32) * band_range
}

pub(super) fn indeterminate_band_width(track_width: f32) -> f32 {
    (track_width * INDETERMINATE_BAND_RATIO).max(MIN_BAR_SIZE)
}
