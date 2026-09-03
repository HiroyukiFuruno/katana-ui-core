use super::canvas_model::Canvas;
use super::canvas_round_rect;

impl Canvas {
    pub fn fill_round_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        radius: usize,
        color: u32,
    ) {
        let physical_x = self.to_physical_x(x);
        let physical_y = self.to_physical_y(y);
        let width = self
            .to_physical_x(x.saturating_add(width))
            .saturating_sub(physical_x);
        let height = self
            .to_physical_y(y.saturating_add(height))
            .saturating_sub(physical_y);
        let radius = self.logical_scale(radius);
        canvas_round_rect::fill_physical(
            self, physical_x, physical_y, width, height, radius, color,
        );
    }
}
