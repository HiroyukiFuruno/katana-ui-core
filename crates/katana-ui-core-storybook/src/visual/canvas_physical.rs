use super::canvas_clip::CanvasClip;
use super::canvas_model::Canvas;

impl Canvas {
    pub(super) fn logical_to_physical_position(&self, logical: usize) -> usize {
        (logical as f64 * f64::from(self.scale_factor())).round() as usize
    }

    pub(super) fn physical_span_x(&self, x: usize) -> Option<(usize, usize)> {
        let left = self.to_physical_x(x);
        if left >= self.width() {
            return None;
        }
        let mut right = self
            .to_physical_x(x.saturating_add(1))
            .saturating_sub(left)
            .max(1)
            .saturating_add(left);
        if right > self.width() {
            right = self.width();
        }
        Some((left, right))
    }

    pub(super) fn physical_span_y(&self, y: usize) -> Option<(usize, usize)> {
        let top = self.to_physical_y(y);
        if top >= self.height() {
            return None;
        }
        let mut bottom = self
            .to_physical_y(y.saturating_add(1))
            .saturating_sub(top)
            .max(1)
            .saturating_add(top);
        if bottom > self.height() {
            bottom = self.height();
        }
        Some((top, bottom))
    }

    pub(super) fn to_physical_x(&self, x: usize) -> usize {
        self.logical_to_physical_position(x).min(self.width())
    }

    pub(super) fn to_physical_y(&self, y: usize) -> usize {
        self.logical_to_physical_position(y).min(self.height())
    }

    pub(super) fn logical_scale(&self, value: usize) -> usize {
        self.logical_to_physical_position(value)
    }

    pub(super) fn to_physical_clip(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Option<CanvasClip> {
        let rect = self.visible_logical_span(x, y, width, height)?;
        if rect.0 >= rect.2 || rect.1 >= rect.3 {
            return None;
        }
        CanvasClip::from_rect(
            rect.0,
            rect.1,
            rect.2 - rect.0,
            rect.3 - rect.1,
            self.width(),
            self.height(),
        )
    }

    fn visible_logical_span(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize, usize, usize)> {
        if width == 0 || height == 0 {
            return None;
        }
        let left = self.logical_to_physical_position(x).min(self.width());
        let top = self.logical_to_physical_position(y).min(self.height());
        if left >= self.width() || top >= self.height() {
            return None;
        }
        let right = self
            .logical_to_physical_position(x.saturating_add(width))
            .min(self.width())
            .max(left + 1);
        let bottom = self
            .logical_to_physical_position(y.saturating_add(height))
            .min(self.height())
            .max(top + 1);
        Some((left, top, right, bottom))
    }
}
