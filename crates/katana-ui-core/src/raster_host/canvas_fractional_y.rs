use super::canvas_clip::CanvasClip;
use super::canvas_model::Canvas;

fn physical_fractional_position(logical: f32, scale_factor: f32) -> usize {
    (f64::from(logical.max(0.0)) * f64::from(scale_factor)).round() as usize
}

impl Canvas {
    /// 小数の論理原点を、既存の整数描画 API に渡す直前まで保持する。
    pub(crate) fn with_fractional_y_origin<T>(
        &mut self,
        logical_origin: f32,
        integer_origin: usize,
        draw: impl FnOnce(&mut Self) -> T,
    ) -> T {
        let physical_origin = physical_fractional_position(logical_origin, self.scale_factor());
        let integer_physical_origin = self.logical_to_physical_position(integer_origin);
        let previous_offset = self.physical_y_offset;
        self.physical_y_offset =
            previous_offset.saturating_add(physical_origin.saturating_sub(integer_physical_origin));
        let result = draw(self);
        self.physical_y_offset = previous_offset;
        result
    }

    pub(crate) fn translate_physical_y(&self, y: usize) -> usize {
        y.saturating_add(self.physical_y_offset)
    }

    pub(super) fn visible_rect_at_logical_y(
        &self,
        x: usize,
        y: f32,
        width: usize,
        height: f32,
    ) -> Option<CanvasClip> {
        if !y.is_finite() || !height.is_finite() || height <= 0.0 || width == 0 {
            return None;
        }
        let left = self.to_physical_x(x);
        let top = self
            .translate_physical_y(physical_fractional_position(y, self.scale_factor()))
            .min(self.height());
        let bottom = self
            .translate_physical_y(physical_fractional_position(
                y + height,
                self.scale_factor(),
            ))
            .min(self.height());
        if left >= self.width() || top >= self.height() {
            return None;
        }
        let right = self.to_physical_x(x.saturating_add(width)).max(left + 1);
        if bottom <= top {
            return None;
        }
        let rect = CanvasClip {
            x: left,
            y: top,
            width: right - left,
            height: bottom - top,
        };
        match self.clip {
            Some(clip) => rect.intersect(clip),
            None => Some(rect),
        }
    }
}
