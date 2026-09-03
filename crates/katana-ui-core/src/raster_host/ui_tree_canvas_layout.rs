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
                &mut |canvas| {
                    let _ = draw_children(canvas);
                },
            );
            *y = start_y.saturating_add(requested_height);
        } else {
            *y = draw_children(canvas);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UiTreeLayoutRenderer;
    use crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette;
    use crate::raster_host::ui_tree_canvas_types::UiTreeRenderArea;
    use crate::raster_host::{Canvas, UiTreeCanvasRenderer};
    use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind};
    use katana_ui_core::theme::ThemeSnapshot;

    #[test]
    fn explicit_height_row_clips_children_and_preserves_row_advance() {
        let theme = ThemeSnapshot::dark();
        let palette = UiTreeCanvasPalette::from_theme(&theme);
        let renderer = UiTreeCanvasRenderer::new(theme);
        let row = UiNode::new(UiNodeKind::Row, "")
            .height(UiDimension::Px(12))
            .child(UiNode::new(UiNodeKind::Text, "first"))
            .child(UiNode::new(UiNodeKind::Text, "second"));
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 120,
            height: 30,
            scroll_y: 0.0,
        };
        let mut canvas = Canvas::new(120, 30, palette.background);
        let mut y = 3;

        UiTreeLayoutRenderer::draw_row(&renderer, &mut canvas, &row, 0, &mut y, area, palette);

        assert_eq!(15, y);
        assert!(canvas.non_background_pixels(palette.background) > 0);
    }
}
