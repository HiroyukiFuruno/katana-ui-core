/// Clamps ratio within [min, max] constraints.
pub(super) fn clamp_ratio(ratio: f32, min: f32, max: f32) -> f32 {
    ratio.clamp(min, max)
}

/// Computes new ratio from a drag delta and total size.
#[cfg(test)]
pub(super) fn drag_ratio(start_ratio: f32, delta_px: f32, total_px: f32) -> f32 {
    if total_px <= 0.0 {
        return start_ratio;
    }
    start_ratio + delta_px / total_px
}

/// Returns the reset ratio for double-click (50/50).
#[cfg(test)]
pub(super) fn reset_ratio() -> f32 {
    0.5
}
