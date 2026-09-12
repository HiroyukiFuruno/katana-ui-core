use super::{
    Canvas, UiNode, UiTreeCanvasPalette, UiTreeCanvasRenderer, UiTreeRenderArea, dimension_px,
    remaining_width,
};
use crate::raster_host::ui_tree_canvas_row_layout::UiTreeRowLayout;

impl UiTreeCanvasRenderer {
    pub(super) fn draw_row_with_logical_cursor(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        logical_y: &mut f32,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let row_top = *logical_y;
        let requested_height = dimension_px(&node.props().common.height);
        let mut row_bottom = row_top;
        let mut draw_children = |canvas: &mut Canvas| {
            for child_layout in UiTreeRowLayout::children(node, x, area) {
                let mut child_logical_y = row_top;
                self.render_node_with_logical_cursor(
                    canvas,
                    child_layout.child,
                    child_layout.x,
                    &mut child_logical_y,
                    area,
                    palette,
                );
                row_bottom = row_bottom.max(child_logical_y);
            }
        };
        if requested_height > 0 {
            canvas.with_clip_at_logical_y(
                x,
                row_top,
                remaining_width(area, x),
                requested_height as f32,
                &mut draw_children,
            );
            *logical_y = row_top + requested_height as f32;
        } else {
            draw_children(canvas);
            *logical_y = row_bottom;
        }
    }
}
