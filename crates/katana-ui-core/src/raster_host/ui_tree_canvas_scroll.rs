use super::canvas::Canvas;
use super::ui_tree_canvas::UiTreeCanvasRenderer;
use super::ui_tree_canvas_hit_metrics::child_container_x;
use super::ui_tree_canvas_hit_metrics::dimension_px;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_scroll_measure::{
    ContainerPadding, can_render_children_incrementally, child_render_area, container_gap,
};
use super::ui_tree_canvas_scroll_partial::draw_partially_visible_node;
use super::ui_tree_canvas_text::UiTreeTextContext;
use super::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::{UiNode, UiScrollAreaProps};

pub(super) fn draw_scroll_area(
    renderer: &UiTreeCanvasRenderer,
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    y: &mut usize,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
) {
    let scroll_area = &node.props().scroll_area;
    let viewport = scroll_viewport(scroll_area, x, *y, area);
    let scroll_y = scroll_area.offset_y as f32 + area.scroll_y.max(0.0);
    draw_offset_scroll_area(renderer, canvas, node, viewport, palette, scroll_y);
    *y = y.saturating_add(viewport.height);
}

fn scroll_viewport(
    scroll_area: &UiScrollAreaProps,
    x: usize,
    y: usize,
    area: UiTreeRenderArea,
) -> UiTreeRenderArea {
    let height = (scroll_area.viewport_height as usize)
        .min(area.height.saturating_sub(y.saturating_sub(area.y)))
        .max(1);
    let width = (scroll_area.viewport_width as usize)
        .min(area.width.saturating_sub(x.saturating_sub(area.x)))
        .max(1);
    UiTreeRenderArea {
        x,
        y,
        width,
        height,
        scroll_y: scroll_area.offset_y as f32,
    }
}

fn draw_offset_scroll_area(
    renderer: &UiTreeCanvasRenderer,
    canvas: &mut Canvas,
    node: &UiNode,
    viewport: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
    scroll_y: f32,
) {
    let source_y = scroll_y.round().max(0.0) as usize;
    let text_context = renderer.text_context(palette);
    let mut logical_y = 0;
    canvas.with_clip(
        viewport.x,
        viewport.y,
        viewport.width,
        viewport.height,
        &mut |canvas| {
            for child in node.children() {
                if logical_y < source_y.saturating_add(viewport.height) {
                    draw_visible_node(
                        renderer,
                        canvas,
                        child,
                        viewport.x,
                        &mut logical_y,
                        source_y,
                        viewport,
                        palette,
                        text_context,
                    );
                }
            }
        },
    );
}

fn draw_visible_node(
    renderer: &UiTreeCanvasRenderer,
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    logical_y: &mut usize,
    source_y: usize,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
    text_context: UiTreeTextContext<'_>,
) {
    if can_render_children_incrementally(node) {
        let node_top = *logical_y;
        let node_height = renderer
            .measured_scroll_node_height(node, text_context, x, area)
            .max(1);
        let node_bottom = node_top.saturating_add(node_height);
        if node_bottom <= source_y || node_top >= source_y.saturating_add(area.height) {
            *logical_y = node_bottom;
            return;
        }
        draw_visible_children(
            renderer,
            canvas,
            node,
            x,
            logical_y,
            source_y,
            area,
            palette,
            text_context,
        );
        let requested_height = dimension_px(&node.props().common.height);
        if requested_height > 0 {
            *logical_y = (*logical_y).max(node_top.saturating_add(requested_height));
        }
        return;
    }
    let node_top = *logical_y;
    let node_height = renderer
        .measured_scroll_node_height(node, text_context, x, area)
        .max(1);
    let node_bottom = node_top.saturating_add(node_height);
    *logical_y = node_bottom;
    if node_bottom <= source_y || node_top >= source_y.saturating_add(area.height) {
        return;
    }
    if node_top >= source_y {
        let mut draw_y = area.y.saturating_add(node_top.saturating_sub(source_y));
        renderer.render_node(canvas, node, x, &mut draw_y, area, palette);
        return;
    }
    draw_partially_visible_node(
        renderer,
        canvas,
        node,
        x,
        node_height,
        source_y.saturating_sub(node_top),
        area,
        palette,
    );
}

fn draw_visible_children(
    renderer: &UiTreeCanvasRenderer,
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    logical_y: &mut usize,
    source_y: usize,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
    text_context: UiTreeTextContext<'_>,
) {
    let padding = ContainerPadding::from_node(node);
    *logical_y = logical_y.saturating_add(padding.top);
    let child_x = child_container_x(node, x).saturating_add(padding.left);
    let child_area = child_render_area(area, node, child_x, padding);
    let gap = container_gap(node);
    for (index, child) in node.children().iter().enumerate() {
        if index > 0 {
            *logical_y = logical_y.saturating_add(gap);
        }
        if *logical_y >= source_y.saturating_add(area.height) {
            break;
        }
        draw_visible_node(
            renderer,
            canvas,
            child,
            child_x,
            logical_y,
            source_y,
            child_area,
            palette,
            text_context,
        );
    }
    *logical_y = logical_y.saturating_add(padding.bottom);
}

#[cfg(test)]
mod tests {
    use super::{draw_scroll_area, draw_visible_node};
    use crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette;
    use crate::raster_host::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
    use crate::render_model::{
        UiDimension, UiGridCell, UiGridCellAppearance, UiGridProps, UiGridViewport, UiNode,
        UiNodeKind, UiRect, UiScrollAreaProps,
    };
    use crate::theme::ThemeSnapshot;

    #[test]
    fn incremental_scroll_container_reserves_its_explicit_height_after_short_children() {
        let theme = ThemeSnapshot::dark();
        let renderer = UiTreeCanvasRenderer::new(theme.clone());
        let palette = UiTreeCanvasPalette::from_theme(&theme);
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 120,
            height: 24,
            scroll_y: 0.0,
        };
        let node = UiNode::new(UiNodeKind::Column, "")
            .height(UiDimension::Px(60))
            .child(UiNode::new(UiNodeKind::Text, "short"));
        let mut canvas = Canvas::new(area.width, area.height, palette.background);
        let mut logical_y = 0;
        let text_context = renderer.text_context(palette);

        draw_visible_node(
            &renderer,
            &mut canvas,
            &node,
            0,
            &mut logical_y,
            0,
            area,
            palette,
            text_context,
        );

        assert_eq!(60, logical_y);
    }

    #[test]
    fn scroll_area_renders_direct_grids_and_reserves_each_grid_height() {
        let theme = ThemeSnapshot::dark();
        let renderer = UiTreeCanvasRenderer::new(theme.clone());
        let palette = UiTreeCanvasPalette::from_theme(&theme);
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 24,
            height: 32,
            scroll_y: 0.0,
        };
        let node = UiNode::new(UiNodeKind::ScrollArea, "")
            .scroll_area(UiScrollAreaProps {
                viewport_width: 24,
                viewport_height: 32,
                content_width: 24,
                content_height: 32,
                ..UiScrollAreaProps::default()
            })
            .child(colored_grid("#AA0000"))
            .child(colored_grid("#00AA00"));
        let mut canvas = Canvas::new(area.width, area.height, palette.background);
        let mut y = 0;

        draw_scroll_area(&renderer, &mut canvas, &node, 0, &mut y, area, palette);

        assert_eq!(32, y);
        let first_grid_pixel = pixel_at(&canvas, 1, 1);
        let second_grid_pixel = pixel_at(&canvas, 1, 17);
        assert_ne!(palette.background, first_grid_pixel);
        assert_ne!(palette.background, second_grid_pixel);
        assert_ne!(first_grid_pixel, second_grid_pixel);
    }

    fn colored_grid(fill_color: &str) -> UiNode {
        UiNode::new(UiNodeKind::Grid, "").grid(UiGridProps {
            total_width: 24,
            total_height: 16,
            viewport: UiGridViewport::new(24, 16),
            show_grid_lines: false,
            cells: vec![UiGridCell {
                bounds: UiRect::new(0, 0, 24, 16),
                clipped_bounds: UiRect::new(0, 0, 24, 16),
                appearance: UiGridCellAppearance {
                    fill_color: Some(fill_color.to_owned()),
                    ..UiGridCellAppearance::default()
                },
                ..UiGridCell::default()
            }],
            ..UiGridProps::default()
        })
    }

    fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> u32 {
        canvas.pixels()[y * canvas.width() + x]
    }
}
