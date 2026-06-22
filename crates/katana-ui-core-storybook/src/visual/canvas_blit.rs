use super::canvas::Canvas;
use super::ui_tree_canvas_types::CanvasBlitRequest;

impl Canvas {
    pub(crate) fn copy_unclipped_canvas_row(
        &mut self,
        source: &Self,
        request: CanvasBlitRequest,
        dest_y_offset: usize,
        source_y: usize,
    ) -> bool {
        let dest_y = request.dest_y.saturating_add(dest_y_offset);
        if dest_y >= self.height || source_y >= source.height {
            return true;
        }
        let copy_width = request
            .width
            .min(source.width)
            .min(self.width.saturating_sub(request.dest_x));
        if copy_width == 0 {
            return true;
        }
        if !self.row_copy_fully_visible(request.dest_x, dest_y, copy_width) {
            return false;
        }
        let source_start = source_y.saturating_mul(source.width);
        let source_end = source_start.saturating_add(copy_width);
        let dest_start = dest_y
            .saturating_mul(self.width)
            .saturating_add(request.dest_x);
        let dest_end = dest_start.saturating_add(copy_width);
        self.pixels[dest_start..dest_end].copy_from_slice(&source.pixels[source_start..source_end]);
        true
    }

    fn row_copy_fully_visible(&self, dest_x: usize, dest_y: usize, width: usize) -> bool {
        let Some(clip) = self.clip else {
            return true;
        };
        dest_x >= clip.x
            && dest_y >= clip.y
            && dest_y < clip.bottom()
            && dest_x.saturating_add(width) <= clip.right()
    }
}
