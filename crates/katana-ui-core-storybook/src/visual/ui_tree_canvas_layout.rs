use super::canvas::Canvas;
use super::ui_tree_canvas::UiTreeCanvasRenderer;
use super::ui_tree_canvas_hit_metrics::dimension_px;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_row_layout::UiTreeRowLayout;
use super::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::UiNode;

pub(super) struct UiTreeLayoutRenderer;

impl UiTreeLayoutRenderer {
    pub(super) fn draw_row(
        renderer: &UiTreeCanvasRenderer,
        canvas: &mut Canvas,
        node: &UiNode,
        x: usize,
        y: &mut usize,
        area: UiTreeRenderArea,
        palette: UiTreeCanvasPalette,
    ) {
        let start_y = *y;
        let requested_height = dimension_px(&node.props().common.height);
        let draw_children = |canvas: &mut Canvas| {
            let mut row_bottom = start_y;
            for child_layout in UiTreeRowLayout::children(node, x, area) {
                let mut child_y = start_y;
                renderer.render_node(
                    canvas,
                    child_layout.child,
                    child_layout.x,
                    &mut child_y,
                    area,
                    palette,
                );
                row_bottom = row_bottom.max(child_y);
            }
            row_bottom
        };
        if requested_height > 0 {
            canvas.with_clip(
                x,
                start_y,
                area.width.saturating_sub(x.saturating_sub(area.x)),
                requested_height,
                |canvas| {
                    let _ = draw_children(canvas);
                },
            );
            *y = start_y.saturating_add(requested_height);
        } else {
            *y = draw_children(canvas);
        }
    }
}
