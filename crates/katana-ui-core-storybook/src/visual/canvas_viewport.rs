use super::canvas_model::Canvas;

impl Canvas {
    #[must_use]
    pub fn viewport_y(&self, offset_y: usize, height: usize, fill: u32) -> Self {
        let mut viewport = Self::new_scaled(self.logical_width, height, self.scale_factor, fill);
        if self.logical_height == 0 || self.logical_width == 0 || height == 0 {
            return viewport;
        }
        let physical_offset_y = self.to_physical_y(offset_y);
        for target_y in 0..viewport.height {
            let source_y = physical_offset_y.saturating_add(target_y);
            if source_y >= self.height {
                break;
            }
            let source_start = source_y * self.width;
            let target_start = target_y * viewport.width;
            let copy_end = target_start + viewport.width;
            let source_end = source_start + self.width;
            viewport.pixels[target_start..copy_end]
                .copy_from_slice(&self.pixels[source_start..source_end]);
        }
        viewport
    }
}
