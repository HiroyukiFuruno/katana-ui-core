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
        if physical_delta == 0.0 {
            return true;
        }
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
