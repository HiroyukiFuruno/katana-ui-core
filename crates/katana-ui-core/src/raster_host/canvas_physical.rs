use super::canvas_clip::CanvasClip;
use super::canvas_model::Canvas;

impl Canvas {
    pub(super) fn logical_to_physical_x(&self, logical: usize) -> usize {
        ((self.logical_phase_x.saturating_add(logical) as f64 * f64::from(self.scale_factor()))
            .round() as usize)
            .saturating_sub(
                (self.logical_phase_x as f64 * f64::from(self.scale_factor())).round() as usize,
            )
    }

    pub(super) fn logical_to_physical_y(&self, logical: usize) -> usize {
        self.physical_y_at_logical_position(logical as f64)
    }

    pub(super) fn fractional_to_physical_y(&self, logical: f32) -> usize {
        self.physical_y_at_logical_position(f64::from(logical.max(0.0)))
    }

    fn physical_y_at_logical_position(&self, logical: f64) -> usize {
        let scale = f64::from(self.scale_factor());
        (((self.logical_phase_y + logical) * scale).round()
            - (self.logical_phase_y * scale).round())
        .max(0.0) as usize
    }

    fn logical_to_physical_position(&self, logical: usize) -> usize {
        (logical as f64 * f64::from(self.scale_factor())).round() as usize
    }

    pub(super) fn physical_span_x(&self, x: usize) -> Option<(usize, usize)> {
        let left = self.to_physical_x(x);
        if left >= self.width() {
            return None;
        }
        let right = self
            .to_physical_x(x.saturating_add(1))
            .saturating_sub(left)
            .max(1)
            .saturating_add(left);
        Some((left, right))
    }

    pub(super) fn physical_span_y(&self, y: usize) -> Option<(usize, usize)> {
        let top = self.to_physical_y(y);
        if top >= self.height() {
            return None;
        }
        let bottom = self
            .to_physical_y(y.saturating_add(1))
            .saturating_sub(top)
            .max(1)
            .saturating_add(top);
        Some((top, bottom))
    }

    pub(super) fn to_physical_x(&self, x: usize) -> usize {
        self.logical_to_physical_x(x).min(self.width())
    }

    pub(super) fn unclipped_physical_x(&self, x: usize) -> usize {
        self.logical_to_physical_x(x)
    }

    pub(super) fn to_physical_y(&self, y: usize) -> usize {
        self.translate_physical_y(self.logical_to_physical_y(y))
            .min(self.height())
    }

    pub(super) fn unclipped_physical_y(&self, y: usize) -> usize {
        self.translate_physical_y(self.logical_to_physical_y(y))
    }

    pub(super) fn physical_text_origin_x(&self, x: isize) -> isize {
        self.phase_aware_physical_position(x as f64, self.logical_phase_x as f64)
    }

    pub(super) fn physical_text_origin_y(&self, y: f32) -> isize {
        self.phase_aware_physical_position(f64::from(y), self.logical_phase_y)
    }

    fn phase_aware_physical_position(&self, position: f64, phase: f64) -> isize {
        let scale = f64::from(self.scale_factor());
        (((phase + position) * scale).round() - (phase * scale).round()) as isize
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
        let left = self.logical_to_physical_x(x).min(self.width());
        let top = self.to_physical_y(y);
        if left >= self.width() || top >= self.height() {
            return None;
        }
        let right = self
            .logical_to_physical_x(x.saturating_add(width))
            .min(self.width())
            .max(left + 1);
        let bottom = self.to_physical_y(y.saturating_add(height)).max(top + 1);
        Some((left, top, right, bottom))
    }
}
