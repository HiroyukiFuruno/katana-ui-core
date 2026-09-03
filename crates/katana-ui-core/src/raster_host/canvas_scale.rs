pub(super) fn normalized_scale(scale: f32) -> f32 {
    if scale.is_finite() && scale >= 1.0 {
        scale
    } else {
        1.0
    }
}

pub(super) fn physical_size(size: usize, scale: f32) -> usize {
    (size as f64 * f64::from(scale)).round() as usize
}
