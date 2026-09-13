use super::{
    Canvas, UiNode, UiTreeCanvasPalette, UiTreeCanvasRenderer, UiTreeRenderArea, dimension_px,
    remaining_width,
};
use crate::raster_host::ui_tree_canvas_row_layout::UiTreeRowLayout;
use crate::raster_host::ui_tree_canvas_text::UiTreeTextRenderer;
use katana_ui_core::render_model::{UiAlignItems, UiNodeKind};

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
                let mut child_logical_y = self.row_child_logical_y(
                    node,
                    child_layout.child,
                    row_top,
                    child_layout.x,
                    requested_height,
                    area,
                    palette,
                );
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

    fn row_child_logical_y(
        &self,
        row: &UiNode,
        child: &UiNode,
        row_top: f32,
        child_x: usize,
        requested_height: usize,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) -> f32 {
        if requested_height == 0
            || !self
                .typography
                .document_typography
                .has_fractional_baseline()
            || row.props().common.align_items != UiAlignItems::Center
        {
            return row_top;
        }
        let child_height = if child.kind() == UiNodeKind::Text {
            UiTreeTextRenderer::logical_advance_height(
                self.text_context(palette),
                child,
                child_x,
                area,
            )
        } else {
            self.measured_scroll_node_height(child, self.text_context(palette), child_x, area)
                as f32
        };
        row_top + (requested_height as f32 - child_height).max(0.0) / 2.0
    }
}
