const RESET_RATIO: f32 = 0.5;

/// Clamps ratio within [min, max] constraints.
pub(super) fn clamp_ratio(ratio: f32, min: f32, max: f32) -> f32 {
    let (min, max) = normalized_bounds(min, max);
    ratio.clamp(min, max)
}

/// Computes new ratio from a drag delta and total size.
pub(super) fn drag_ratio(start_ratio: f32, delta_px: f32, total_px: f32) -> f32 {
    if total_px <= 0.0 {
        return start_ratio;
    }
    let updated_ratio = start_ratio + delta_px / total_px;
    clamp_ratio(updated_ratio, 0.0, 1.0)
}

/// Returns the reset ratio for double-click (50/50).
pub(super) fn reset_ratio() -> f32 {
    RESET_RATIO
}

pub(super) fn normalized_bounds(min: f32, max: f32) -> (f32, f32) {
    let mut min_ratio = min.clamp(0.0, 1.0);
    let mut max_ratio = max.clamp(0.0, 1.0);
    if min_ratio > max_ratio {
        std::mem::swap(&mut min_ratio, &mut max_ratio);
    }
    (min_ratio, max_ratio)
}
