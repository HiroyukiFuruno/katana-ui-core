use super::canvas::Canvas;
use super::text::TextRenderer;
use super::ui_tree_canvas_rgba::{packed_rgb, rgba_alpha, rgba_sample};
use super::ui_tree_canvas_types::{CanvasBlitRequest, RgbaBlitRequest};
use katana_ui_core::facade::UiCoreFacade;
use std::cell::RefCell;

const TEXT_SIZE: f32 = 14.0;
const PIXEL_CENTER_OFFSET: f32 = 0.5;

thread_local! {
    static BODY_TEXT: RefCell<TextRenderer> =
        RefCell::new(TextRenderer::load(&UiCoreFacade::default(), "body"));
    static CODE_TEXT: RefCell<TextRenderer> =
        RefCell::new(TextRenderer::load(&UiCoreFacade::default(), "code"));
}

impl Canvas {
    pub fn draw_text(&mut self, x: usize, y: usize, text: &str, color: u32) {
        self.draw_text_with_role("body", x, y, text, color);
    }

    pub fn draw_text_with_role(&mut self, role: &str, x: usize, y: usize, text: &str, color: u32) {
        with_text_renderer(role, |renderer| {
            renderer.draw(self, text, x, y, TEXT_SIZE, color);
        });
    }

    pub fn text_width_with_role(&self, role: &str, text: &str) -> usize {
        with_text_renderer(role, |renderer| renderer.measure_width(text, TEXT_SIZE))
    }

    pub fn blit_canvas(&mut self, source: &Canvas, request: CanvasBlitRequest) {
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
        request: CanvasBlitRequest,
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
            self.set(dest_x, dest_y, color);
        }
    }

    fn blit_canvas_text_runs(&mut self, source: &Canvas, request: CanvasBlitRequest) {
        let source_bottom = request.source_y.saturating_add(request.height);
        for run in source.text_runs() {
            let rect = run.rect();
            if rect.bottom() <= request.source_y || rect.y >= source_bottom {
                continue;
            }
            let target_x = request.dest_x.saturating_add(rect.x);
            let target_y = request
                .dest_y
                .saturating_add(rect.y.saturating_sub(request.source_y));
            self.record_text_run(run.text(), target_x, target_y, rect.width, rect.height);
        }
    }

    pub fn blit_rgba(&mut self, source: RgbaBlitRequest<'_>) {
        if source.width == 0 || source.area.width == 0 || source.area.height == 0 {
            return;
        }
        if !source_has_retina_pixels(&source) {
            self.blit_rgba_logical(source);
            return;
        }
        let target = self.unclipped_physical_target(source.area);
        if target.width == 0 || target.height == 0 {
            return;
        }
        for y in 0..target.visible_height {
            let target_y = target.visible_y_offset.saturating_add(y);
            let source_y = source_region_sample_position(
                target_y,
                source.source.y,
                source.source.height,
                target.height,
            );
            if source_y >= source.height as f32 {
                break;
            }
            self.blit_rgba_row(
                &source,
                target.left,
                target.top,
                y,
                source_y,
                target.visible_width,
                target.width,
                target.visible_x_offset,
            );
        }
    }

    fn blit_rgba_logical(&mut self, source: RgbaBlitRequest<'_>) {
        for y in 0..source.area.height {
            let source_y = source_region_sample_position(
                y,
                source.source.y,
                source.source.height,
                source.area.height,
            );
            if source_y >= source.height as f32 {
                break;
            }
            self.blit_rgba_logical_row(&source, y, source_y);
        }
    }

    fn blit_rgba_logical_row(
        &mut self,
        source: &RgbaBlitRequest<'_>,
        dest_y_offset: usize,
        source_y: f32,
    ) {
        let dest_y = source.area.y.saturating_add(dest_y_offset);
        for x in 0..source.area.width {
            let source_x = source_region_sample_position(
                x,
                source.source.x,
                source.source.width,
                source.area.width,
            );
            if source_x >= source.width as f32 {
                break;
            }
            self.put_rgba_logical_sample(source, source.area.x + x, dest_y, source_x, source_y);
        }
    }

    fn blit_rgba_row(
        &mut self,
        source: &RgbaBlitRequest<'_>,
        left: usize,
        top: usize,
        dest_y_offset: usize,
        source_y: f32,
        visible_width: usize,
        target_width: usize,
        visible_x_offset: usize,
    ) {
        let dest_y = top.saturating_add(dest_y_offset);
        for x in 0..visible_width {
            let target_x = visible_x_offset.saturating_add(x);
            let source_x = source_region_sample_position(
                target_x,
                source.source.x,
                source.source.width,
                target_width,
            );
            if source_x >= source.width as f32 {
                break;
            }
            self.put_rgba_sample(source, left + x, dest_y, source_x, source_y);
        }
    }

    fn unclipped_physical_target(
        &self,
        area: super::ui_tree_canvas_types::UiTreeRenderArea,
    ) -> PhysicalImageTarget {
        let unclipped_left = self.logical_scale(area.x);
        let unclipped_top = self.logical_scale(area.y);
        let unclipped_right = self.logical_scale(area.x.saturating_add(area.width));
        let unclipped_bottom = self.logical_scale(area.y.saturating_add(area.height));
        if unclipped_left >= self.width
            || unclipped_top >= self.height
            || unclipped_right <= unclipped_left
            || unclipped_bottom <= unclipped_top
        {
            return PhysicalImageTarget::default();
        }
        let left = unclipped_left.min(self.width);
        let top = unclipped_top.min(self.height);
        let right = unclipped_right.min(self.width);
        let bottom = unclipped_bottom.min(self.height);
        PhysicalImageTarget {
            left,
            top,
            width: unclipped_right.saturating_sub(unclipped_left),
            height: unclipped_bottom.saturating_sub(unclipped_top),
            visible_width: right.saturating_sub(left),
            visible_height: bottom.saturating_sub(top),
            visible_x_offset: left.saturating_sub(unclipped_left),
            visible_y_offset: top.saturating_sub(unclipped_top),
        }
    }

    fn put_rgba_sample(
        &mut self,
        source: &RgbaBlitRequest<'_>,
        x: usize,
        y: usize,
        sx: f32,
        sy: f32,
    ) {
        let sample = rgba_sample(source, sx, sy);
        let alpha = rgba_alpha(sample);
        if alpha == 0 {
            return;
        }
        let color = packed_rgb(sample);
        if alpha == u8::MAX {
            self.set_physical(x, y, color);
            return;
        }
        self.blend_physical(x, y, color, alpha);
    }

    fn put_rgba_logical_sample(
        &mut self,
        source: &RgbaBlitRequest<'_>,
        x: usize,
        y: usize,
        sx: f32,
        sy: f32,
    ) {
        let sample = rgba_sample(source, sx, sy);
        let alpha = rgba_alpha(sample);
        if alpha == 0 {
            return;
        }
        let color = packed_rgb(sample);
        if alpha == u8::MAX {
            self.set(x, y, color);
            return;
        }
        self.blend(x, y, color, alpha);
    }
}

#[derive(Clone, Copy, Default)]
struct PhysicalImageTarget {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    visible_width: usize,
    visible_height: usize,
    visible_x_offset: usize,
    visible_y_offset: usize,
}

fn source_has_retina_pixels(source: &RgbaBlitRequest<'_>) -> bool {
    source.width as usize > source.area.width || source.height as usize > source.area.height
}

fn source_region_sample_position(
    target_index: usize,
    source_start: f32,
    source_extent: f32,
    target_extent: usize,
) -> f32 {
    let target_extent = target_extent.max(1) as f32;
    (source_start + (target_index as f32 + PIXEL_CENTER_OFFSET) * source_extent / target_extent
        - PIXEL_CENTER_OFFSET)
        .max(0.0)
}

fn with_text_renderer<T>(role: &str, operation: impl FnOnce(&mut TextRenderer) -> T) -> T {
    if role == "code" {
        return CODE_TEXT.with(|renderer| operation(&mut renderer.borrow_mut()));
    }
    BODY_TEXT.with(|renderer| operation(&mut renderer.borrow_mut()))
}

#[cfg(test)]
#[path = "ui_tree_canvas_extensions_tests.rs"]
mod tests;
