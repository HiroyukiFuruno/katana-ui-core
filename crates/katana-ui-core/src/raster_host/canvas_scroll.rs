use super::canvas::Canvas;
use super::canvas_clip::CanvasClip;

impl Canvas {
    pub fn scroll_rect_vertically(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        logical_delta_y: isize,
    ) -> bool {
        if logical_delta_y == 0 {
            return true;
        }
        let Some(rect) = self.visible_rect(x, y, width, height) else {
            return false;
        };
        let physical_delta = (logical_delta_y as f64 * f64::from(self.scale_factor)).round();
        debug_assert_ne!(
            0.0, physical_delta,
            "non-zero logical deltas scale to at least one pixel"
        );
        let physical_delta = physical_delta as isize;
        let rect_height = rect.height as isize;
        if physical_delta.abs() >= rect_height {
            return false;
        }
        if rect.x == 0 && rect.width == self.width {
            self.scroll_full_width_rect_vertically(rect, physical_delta);
            return true;
        }
        if physical_delta > 0 {
            self.scroll_rect_down(rect, physical_delta as usize);
            return true;
        }
        self.scroll_rect_up(rect, physical_delta.unsigned_abs());
        true
    }

    fn scroll_full_width_rect_vertically(&mut self, rect: CanvasClip, physical_delta: isize) {
        if physical_delta > 0 {
            let delta = physical_delta as usize;
            let source_start = rect.y.saturating_mul(self.width);
            let source_end = rect
                .bottom()
                .saturating_sub(delta)
                .saturating_mul(self.width);
            let dest_start = rect.y.saturating_add(delta).saturating_mul(self.width);
            self.pixels
                .copy_within(source_start..source_end, dest_start);
            return;
        }
        let delta = physical_delta.unsigned_abs();
        let source_start = rect.y.saturating_add(delta).saturating_mul(self.width);
        let source_end = rect.bottom().saturating_mul(self.width);
        let dest_start = rect.y.saturating_mul(self.width);
        self.pixels
            .copy_within(source_start..source_end, dest_start);
    }

    fn scroll_rect_up(&mut self, rect: CanvasClip, physical_delta: usize) {
        for source_y in rect.y.saturating_add(physical_delta)..rect.bottom() {
            self.copy_rect_row(rect, source_y, source_y - physical_delta);
        }
    }

    fn scroll_rect_down(&mut self, rect: CanvasClip, physical_delta: usize) {
        for source_y in (rect.y..rect.bottom().saturating_sub(physical_delta)).rev() {
            self.copy_rect_row(rect, source_y, source_y + physical_delta);
        }
    }

    fn copy_rect_row(&mut self, rect: CanvasClip, source_y: usize, dest_y: usize) {
        let source_start = source_y.saturating_mul(self.width).saturating_add(rect.x);
        let dest_start = dest_y.saturating_mul(self.width).saturating_add(rect.x);
        let len = rect.width;
        if source_start >= self.pixels.len()
            || dest_start >= self.pixels.len()
            || source_start.saturating_add(len) > self.pixels.len()
            || dest_start.saturating_add(len) > self.pixels.len()
        {
            return;
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.pixels.as_ptr().add(source_start),
                self.pixels.as_mut_ptr().add(dest_start),
                len,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Canvas, CanvasClip};

    #[test]
    fn vertical_scroll_covers_noop_clipping_full_width_and_partial_width() {
        let mut canvas = Canvas::new(4, 4, 0);
        canvas.fill_rect(0, 0, 4, 1, 1);
        canvas.fill_rect(0, 1, 4, 1, 2);
        canvas.fill_rect(0, 2, 4, 1, 3);
        canvas.fill_rect(0, 3, 4, 1, 4);

        assert!(canvas.scroll_rect_vertically(0, 0, 4, 4, 0));
        assert!(!canvas.scroll_rect_vertically(10, 10, 2, 2, 1));
        assert!(!canvas.scroll_rect_vertically(0, 0, 4, 4, 4));
        assert!(canvas.scroll_rect_vertically(0, 0, 4, 4, 1));
        assert!(canvas.scroll_rect_vertically(0, 0, 4, 4, -1));
        assert!(canvas.scroll_rect_vertically(1, 0, 2, 4, 1));
        assert!(canvas.scroll_rect_vertically(1, 0, 2, 4, -1));

        let mut fractional = Canvas::new_scaled(10, 10, 0.1, 0);
        assert!(fractional.scroll_rect_vertically(0, 0, 10, 10, 1));
    }

    #[test]
    fn row_copy_rejects_out_of_bounds_ranges() {
        let mut canvas = Canvas::new(2, 2, 0);
        canvas.copy_rect_row(
            CanvasClip {
                x: 3,
                y: 0,
                width: 2,
                height: 2,
            },
            0,
            1,
        );
        canvas.copy_rect_row(
            CanvasClip {
                x: 0,
                y: 3,
                width: 3,
                height: 2,
            },
            3,
            0,
        );
        assert_eq!(&[0, 0, 0, 0], canvas.pixels());
    }
}
