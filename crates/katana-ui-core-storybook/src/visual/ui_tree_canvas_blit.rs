use super::canvas::Canvas;
use super::ui_tree_canvas_types::{CanvasBlitRequest, PhysicalCanvasBlitRequest};

impl Canvas {
    pub fn blit_canvas(&mut self, source: &Canvas, request: CanvasBlitRequest) {
        let dest_x = self.to_physical_x(request.dest_x);
        let dest_y = self.to_physical_y(request.dest_y);
        let width = self
            .to_physical_x(request.dest_x.saturating_add(request.width))
            .saturating_sub(dest_x);
        let height = self
            .to_physical_y(request.dest_y.saturating_add(request.height))
            .saturating_sub(dest_y);
        self.blit_canvas_physical(
            source,
            PhysicalCanvasBlitRequest {
                dest_x,
                dest_y,
                width,
                height,
                source_y: request.source_y,
                source_logical_y: logical_source_y(request.source_y, source.scale_factor()),
            },
        );
    }

    pub(super) fn blit_canvas_physical(
        &mut self,
        source: &Canvas,
        request: PhysicalCanvasBlitRequest,
    ) {
        for y in 0..request.height {
            let source_y = request.source_y.saturating_add(y);
            if source_y >= source.height() {
                break;
            }
            self.blit_canvas_row(source, request, y, source_y);
        }
        self.blit_canvas_text_runs(source, request);
    }

    fn blit_canvas_row(
        &mut self,
        source: &Canvas,
        request: PhysicalCanvasBlitRequest,
        dest_y_offset: usize,
        source_y: usize,
    ) {
        if self.copy_unclipped_canvas_row(source, request, dest_y_offset, source_y) {
            return;
        }
        let dest_y = request.dest_y.saturating_add(dest_y_offset);
        let copy_width = request
            .width
            .min(source.width())
            .min(self.width().saturating_sub(request.dest_x))
            .saturating_mul(usize::from(dest_y < self.height()));
        for x in 0..copy_width {
            let dest_x = request.dest_x.saturating_add(x);
            let color = source.pixels()[source_y * source.width() + x];
            self.set_physical(dest_x, dest_y, color);
        }
    }

    fn blit_canvas_text_runs(&mut self, source: &Canvas, request: PhysicalCanvasBlitRequest) {
        let source_bottom = request.source_y.saturating_add(request.height);
        let destination_scale = self.scale_factor();
        let destination_x = logical_blit_coordinate(request.dest_x, destination_scale);
        let destination_y = logical_blit_coordinate(request.dest_y, destination_scale);
        let source_y = request.source_logical_y;
        for run in source.text_runs() {
            let rect = run.rect();
            let physical_rect_top = source.to_physical_y(rect.y);
            let physical_rect_bottom = source.to_physical_y(rect.bottom());
            if physical_rect_bottom <= request.source_y || physical_rect_top >= source_bottom {
                continue;
            }
            let target_x = destination_x.saturating_add(rect.x);
            let target_y =
                destination_y.saturating_add((rect.y as f32 - source_y).round().max(0.0) as usize);
            self.record_text_run(run.text(), target_x, target_y, rect.width, rect.height);
        }
    }
}

fn logical_blit_coordinate(physical_coordinate: usize, scale_factor: f32) -> usize {
    (physical_coordinate as f64 / f64::from(scale_factor)).round() as usize
}

fn logical_source_y(physical_coordinate: usize, scale_factor: f32) -> f32 {
    (physical_coordinate as f32 / scale_factor).floor()
}
