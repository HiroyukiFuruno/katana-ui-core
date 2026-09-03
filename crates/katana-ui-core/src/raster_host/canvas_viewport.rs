use super::canvas_model::Canvas;

impl Canvas {
    #[must_use]
    pub fn viewport_y(&self, offset_y: usize, height: usize, fill: u32) -> Self {
        let mut viewport = Self::new_scaled_with_raster_scale(
            self.logical_width,
            height,
            self.scale_factor,
            self.raster_scale_factor,
            fill,
        );
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
        for run in &self.text_runs {
            let rect = run.rect();
            if rect.bottom() <= offset_y || rect.y >= offset_y.saturating_add(height) {
                continue;
            }
            viewport.record_text_run(
                run.text(),
                rect.x,
                rect.y.saturating_sub(offset_y),
                rect.width,
                rect.height,
            );
        }
        viewport
    }
}

#[cfg(test)]
mod tests {
    use super::Canvas;

    #[test]
    fn viewport_copies_visible_pixels_and_rebases_intersecting_text_runs() {
        let mut canvas = Canvas::new(4, 6, 0);
        canvas.fill_rect(0, 2, 4, 2, 0x112233);
        canvas.record_text_run("above", 0, 0, 1, 1);
        canvas.record_text_run("visible", 1, 2, 2, 2);
        canvas.record_text_run("below", 0, 5, 1, 1);

        let viewport = canvas.viewport_y(2, 3, 0x445566);

        assert_eq!(4, viewport.width());
        assert_eq!(3, viewport.height());
        assert_eq!(0x112233, viewport.pixels()[0]);
        assert_eq!(0, viewport.pixels()[2 * viewport.width()]);
        assert_eq!(1, viewport.text_runs().len());
        assert_eq!("visible", viewport.text_runs()[0].text());
        assert_eq!(0, viewport.text_runs()[0].y());

        let padded = canvas.viewport_y(5, 3, 0x445566);
        assert_eq!(0x445566, padded.pixels()[padded.width()]);
    }

    #[test]
    fn viewport_handles_empty_source_dimensions_without_copying() {
        let canvas = Canvas::new(0, 4, 0x112233);
        let viewport = canvas.viewport_y(1, 2, 0x445566);

        assert_eq!(0, viewport.width());
        assert_eq!(2, viewport.height());
        assert!(viewport.text_runs().is_empty());
    }
}
