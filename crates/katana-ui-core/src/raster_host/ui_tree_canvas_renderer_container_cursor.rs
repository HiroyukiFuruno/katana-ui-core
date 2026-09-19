use super::{
    Canvas, ContainerPadding, TEXT_HEIGHT, UiNode, UiTreeCanvasPalette, UiTreeCanvasRenderer,
    UiTreeRenderArea, UiVisualRole, child_container_x, child_render_area, dimension_px,
    draw_hover_surface, gap_after_child, remaining_width, should_draw_container_label,
};

#[derive(Clone, Copy)]
struct ContainerCursor {
    origin: f32,
    hover_surface_y: usize,
    requested_height: usize,
}

impl UiTreeCanvasRenderer {
    pub(super) fn draw_container_with_logical_cursor(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        logical_y: &mut f32,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let cursor = ContainerCursor {
            origin: *logical_y,
            hover_surface_y: logical_y.floor().max(0.0) as usize,
            requested_height: dimension_px(&node.props().common.height),
        };
        let hover_surface_height = if cursor.requested_height > 0 {
            cursor.requested_height
        } else if node.props().visual_role != UiVisualRole::HoverSurface {
            TEXT_HEIGHT
        } else {
            self.measured_scroll_node_height(node, self.text_context(palette), x, area)
        };
        if should_draw_container_label(node) {
            self.draw_clipped_container_label(canvas, node, x, logical_y, area, palette, cursor);
        }
        self.draw_container_children_with_logical_cursor(
            canvas, node, x, logical_y, area, palette, cursor,
        );
        canvas.with_fractional_y_origin(cursor.origin, cursor.hover_surface_y, |canvas| {
            draw_hover_surface(
                canvas,
                node,
                x,
                cursor.hover_surface_y,
                area,
                palette,
                hover_surface_height,
            );
        });
    }

    fn draw_clipped_container_label(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        logical_y: &mut f32,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
        cursor: ContainerCursor,
    ) {
        let mut label_y = cursor.hover_surface_y;
        let mut draw_label = |canvas: &mut Canvas| {
            canvas.with_fractional_y_origin(cursor.origin, cursor.hover_surface_y, |canvas| {
                super::draw_label(canvas, &self.text, node, x, &mut label_y, palette);
            });
        };
        if cursor.requested_height > 0 {
            canvas.with_clip_at_logical_y(
                x,
                cursor.origin,
                remaining_width(area, x),
                cursor.requested_height as f32,
                &mut draw_label,
            );
        } else {
            draw_label(canvas);
        }
        *logical_y += label_y.saturating_sub(cursor.hover_surface_y) as f32;
    }

    fn draw_container_children_with_logical_cursor(
        &self,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        logical_y: &mut f32,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
        cursor: ContainerCursor,
    ) {
        let padding = ContainerPadding::from_node(node);
        let child_x = child_container_x(node, x).saturating_add(padding.left);
        let child_area = child_render_area(area, node, child_x, padding);
        *logical_y += padding.top as f32;
        let child_clip_y = *logical_y;
        let mut draw_children = |canvas: &mut Canvas| {
            for (index, child) in node.children().iter().enumerate() {
                self.render_node_with_logical_cursor(
                    canvas, child, child_x, logical_y, child_area, palette,
                );
                if index + 1 < node.children().len() {
                    *logical_y += gap_after_child(node, child, &node.children()[index + 1]) as f32;
                }
            }
        };
        if cursor.requested_height > 0 {
            let container_bottom = cursor.origin + cursor.requested_height as f32;
            let clip_height = (container_bottom - child_clip_y).max(0.0);
            canvas.with_clip_at_logical_y(
                x,
                child_clip_y,
                remaining_width(area, x),
                clip_height,
                &mut draw_children,
            );
            *logical_y = container_bottom;
        } else {
            draw_children(canvas);
            *logical_y += padding.bottom as f32;
        }
    }
}
