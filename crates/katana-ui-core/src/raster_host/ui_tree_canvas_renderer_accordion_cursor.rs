use super::{
    Canvas, UiNode, UiTreeCanvasPalette, UiTreeCanvasRenderer, UiTreeRenderArea, dimension_px,
    remaining_width,
};

impl UiTreeCanvasRenderer {
    pub(super) fn draw_sized_accordion_with_logical_cursor(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        logical_y: &mut f32,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let requested_height = dimension_px(&node.props().common.height);
        let logical_start = *logical_y;
        let mut draw = |canvas: &mut Canvas| {
            self.draw_accordion_with_logical_cursor(canvas, node, x, logical_y, area, palette);
        };
        if requested_height > 0 {
            canvas.with_clip_at_logical_y(
                x,
                logical_start,
                remaining_width(area, x),
                requested_height as f32,
                &mut draw,
            );
            *logical_y = logical_start + requested_height as f32;
        } else {
            draw(canvas);
        }
    }
}
