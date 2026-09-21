use super::canvas_model::{Canvas, CanvasImageSurfaceExtentMode};
use super::canvas_scale::{normalized_scale, physical_size};

impl Canvas {
    #[must_use]
    pub fn new(width: usize, height: usize, color: u32) -> Self {
        Self::new_scaled(width, height, 1.0, color)
    }

    #[must_use]
    pub fn new_scaled(width: usize, height: usize, scale: f32, color: u32) -> Self {
        Self::new_scaled_with_raster_scale(width, height, scale, scale, color)
    }

    #[must_use]
    pub fn new_scaled_with_raster_scale(
        width: usize,
        height: usize,
        scale: f32,
        raster_scale: f32,
        color: u32,
    ) -> Self {
        Self::new_scaled_with_logical_phase_and_raster_scale(
            width,
            height,
            scale,
            raster_scale,
            0,
            0.0,
            color,
        )
    }

    #[must_use]
    pub(super) fn new_scaled_with_logical_phase(
        width: usize,
        height: usize,
        scale: f32,
        phase_x: usize,
        phase_y: f64,
        color: u32,
    ) -> Self {
        Self::new_scaled_with_logical_phase_and_raster_scale(
            width, height, scale, scale, phase_x, phase_y, color,
        )
    }

    fn new_scaled_with_logical_phase_and_raster_scale(
        width: usize,
        height: usize,
        scale: f32,
        raster_scale: f32,
        phase_x: usize,
        phase_y: f64,
        color: u32,
    ) -> Self {
        let scale = normalized_scale(scale);
        let raster_scale = normalized_scale(raster_scale);
        let physical_width = physical_size(phase_x.saturating_add(width), scale)
            .saturating_sub(physical_size(phase_x, scale));
        let physical_height = physical_height_at_logical_phase(phase_y, height, scale);
        Self {
            width: physical_width,
            height: physical_height,
            logical_width: width,
            logical_height: height,
            logical_phase_x: phase_x,
            logical_phase_y: phase_y,
            fractional_y_origin: 0.0,
            scale_factor: scale,
            raster_scale_factor: raster_scale,
            image_surface_extent_mode: CanvasImageSurfaceExtentMode::LogicalDisplay,
            pixels: vec![color; physical_width * physical_height],
            clip: None,
            text_runs: Vec::new(),
            physical_y_offset: 0,
        }
    }
}

fn physical_height_at_logical_phase(phase: f64, height: usize, scale: f32) -> usize {
    let scale = f64::from(scale);
    (((phase + height as f64) * scale).round() - (phase * scale).round()).max(0.0) as usize
}
