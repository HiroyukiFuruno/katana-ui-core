use super::canvas::Canvas;
use super::ui_tree_canvas::UiTreeCanvasRenderer;
use super::ui_tree_canvas_hit_metrics::child_container_x;
use super::ui_tree_canvas_hit_metrics::dimension_px;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_scroll_measure::{
    ContainerPadding, can_render_children_incrementally, child_render_area, container_gap,
};
use super::ui_tree_canvas_scroll_partial::draw_partially_visible_node;
use super::ui_tree_canvas_text::{UiTreeTextContext, UiTreeTextRenderer};
use super::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
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
    let mut logical_y = 0.0_f32;
    let mut render_y = viewport.y as f32 - source_y as f32;
    canvas.with_clip(
        viewport.x,
        viewport.y,
        viewport.width,
        viewport.height,
        &mut |canvas| {
            let mut context = ScrollDrawContext {
                renderer,
                canvas,
                source_y,
                area: viewport,
                palette,
                text_context: renderer.text_context(palette),
            };
            for child in node.children() {
                if logical_y < source_y.saturating_add(viewport.height) as f32 {
                    draw_visible_node(
                        &mut context,
                        child,
                        viewport.x,
                        &mut logical_y,
                        &mut render_y,
                    );
                }
            }
        },
    );
}

struct ScrollDrawContext<'renderer, 'canvas> {
    renderer: &'renderer UiTreeCanvasRenderer,
    canvas: &'canvas mut Canvas,
    source_y: usize,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
    text_context: UiTreeTextContext<'renderer>,
}

fn draw_visible_node(
    context: &mut ScrollDrawContext<'_, '_>,
    node: &UiNode,
    x: usize,
    logical_y: &mut f32,
    render_y: &mut f32,
) {
    let renderer = context.renderer;
    let source_y = context.source_y;
    let area = context.area;
    let palette = context.palette;
    let text_context = context.text_context;
    if can_render_children_incrementally(node) {
        let node_top = *logical_y;
        let node_height = logical_node_height(renderer, text_context, node, x, area).max(1.0);
        let node_bottom = node_top + node_height;
        if node_bottom <= source_y as f32 || node_top >= source_y.saturating_add(area.height) as f32
        {
            *logical_y = node_bottom;
            *render_y += node_height;
            return;
        }
        draw_visible_children(context, node, x, logical_y, render_y);
        let requested_height = dimension_px(&node.props().common.height);
        if requested_height > 0 {
            let requested_bottom = node_top + requested_height as f32;
            if requested_bottom > *logical_y {
                *render_y += requested_bottom - *logical_y;
                *logical_y = requested_bottom;
            }
        }
        return;
    }
    let node_top = *logical_y;
    let node_height = logical_node_height(renderer, text_context, node, x, area).max(1.0);
    let node_bottom = node_top + node_height;
    *logical_y = node_bottom;
    if node_bottom <= source_y as f32 || node_top >= source_y.saturating_add(area.height) as f32 {
        *render_y += node_height;
        return;
    }
    if node_top >= source_y as f32 {
        let mut draw_y = render_y.max(0.0).floor() as usize;
        if node.kind() == katana_ui_core::render_model::UiNodeKind::Text {
            UiTreeTextRenderer::draw_node_at_logical_y(
                context.canvas,
                text_context,
                node,
                x,
                *render_y,
                area,
            );
            *render_y += logical_text_height(renderer, text_context, node, x, area);
        } else {
            renderer.render_node(context.canvas, node, x, &mut draw_y, area, palette);
            *render_y += draw_y.saturating_sub(render_y.max(0.0).floor() as usize) as f32;
        }
        return;
    }
    draw_partially_visible_node(
        renderer,
        context.canvas,
        node,
        x,
        node_height.ceil() as usize,
        source_y.saturating_sub(node_top.floor() as usize),
        area,
        palette,
    );
    *render_y += node_height;
}

fn logical_node_height(
    renderer: &UiTreeCanvasRenderer,
    context: UiTreeTextContext<'_>,
    node: &UiNode,
    x: usize,
    area: UiTreeRenderArea,
) -> f32 {
    if node.kind() == katana_ui_core::render_model::UiNodeKind::Text {
        return logical_text_height(renderer, context, node, x, area);
    }
    renderer.measured_scroll_node_height(node, context, x, area) as f32
}

fn logical_text_height(
    renderer: &UiTreeCanvasRenderer,
    context: UiTreeTextContext<'_>,
    node: &UiNode,
    x: usize,
    area: UiTreeRenderArea,
) -> f32 {
    let requested = dimension_px(&node.props().common.height);
    if requested > 0 {
        return requested as f32;
    }
    let measured = renderer.measured_scroll_node_height(node, context, x, area) as f32;
    let metrics = UiTreeTextMetrics::for_node_with_typography(node, context.typography);
    if metrics.line_box_height.fract() == 0.0 {
        return measured;
    }
    let line_count = (measured / metrics.line_box_height).round().max(1.0);
    line_count * metrics.line_box_height
}

fn draw_visible_children(
    context: &mut ScrollDrawContext<'_, '_>,
    node: &UiNode,
    x: usize,
    logical_y: &mut f32,
    render_y: &mut f32,
) {
    let source_y = context.source_y;
    let area = context.area;
    let padding = ContainerPadding::from_node(node);
    *logical_y += padding.top as f32;
    *render_y += padding.top as f32;
    let child_x = child_container_x(node, x).saturating_add(padding.left);
    let child_area = child_render_area(area, node, child_x, padding);
    let gap = container_gap(node);
    for (index, child) in node.children().iter().enumerate() {
        if index > 0 {
            *logical_y += gap as f32;
            *render_y += gap as f32;
        }
        if *logical_y >= source_y.saturating_add(area.height) as f32 {
            break;
        }
        let parent_area = context.area;
        context.area = child_area;
        draw_visible_node(context, child, child_x, logical_y, render_y);
        context.area = parent_area;
    }
    *logical_y += padding.bottom as f32;
    *render_y += padding.bottom as f32;
}

#[cfg(test)]
mod tests {
    use super::{ScrollDrawContext, draw_scroll_area, draw_visible_node};
    use crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette;
    use crate::raster_host::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
    use crate::raster_host::{UiTreeDocumentTypography, UiTreeTextRoleBaselineTypography};
    use crate::render_model::{
        UiDimension, UiGridCell, UiGridCellAppearance, UiGridProps, UiGridViewport, UiNode,
        UiNodeKind, UiRect, UiScrollAreaProps, UiTextProps,
    };
    use crate::theme::ThemeSnapshot;
    use katana_ui_core::atom::Text;
    use katana_ui_core::render_model::{UiTextSpan, UiTextSpanStyle};

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
        let mut logical_y = 0.0;
        let mut render_y = 0.0;
        let mut context = ScrollDrawContext {
            renderer: &renderer,
            canvas: &mut canvas,
            source_y: 0,
            area,
            palette,
            text_context: renderer.text_context(palette),
        };

        draw_visible_node(&mut context, &node, 0, &mut logical_y, &mut render_y);

        assert_eq!(60.0, logical_y);
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

    #[test]
    fn scroll_area_keeps_fractional_text_cursor_between_direct_children() {
        let theme = ThemeSnapshot::dark();
        let typography = UiTreeDocumentTypography::new()
            .with_body_baseline(UiTreeTextRoleBaselineTypography::new(16.0, 31.5, 18.5));
        let renderer = UiTreeCanvasRenderer::with_document_typography(theme.clone(), typography);
        let palette = UiTreeCanvasPalette::from_theme(&theme);
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 160,
            height: 80,
            scroll_y: 0.0,
        };
        let highlighted = |label: &str| {
            UiNode::from(Text::new(label)).text(UiTextProps {
                role: "body".to_owned(),
                spans: vec![UiTextSpan {
                    text: label.into(),
                    style: UiTextSpanStyle {
                        current_highlight: true,
                        ..UiTextSpanStyle::default()
                    },
                    link_target: String::new(),
                }],
                ..UiTextProps::default()
            })
        };
        let node = UiNode::new(UiNodeKind::ScrollArea, "")
            .scroll_area(UiScrollAreaProps {
                viewport_width: 160,
                viewport_height: 80,
                content_width: 160,
                content_height: 80,
                ..UiScrollAreaProps::default()
            })
            .child(highlighted("first"))
            .child(highlighted("second"));
        let mut canvas = Canvas::new_scaled(160, 80, 2.0, palette.background);
        let mut y = 0;

        draw_scroll_area(&renderer, &mut canvas, &node, 0, &mut y, area, palette);

        let mut logical_y = 0.0;
        let mut render_y = 0.0;
        {
            let mut context = ScrollDrawContext {
                renderer: &renderer,
                canvas: &mut canvas,
                source_y: 0,
                area,
                palette,
                text_context: renderer.text_context(palette),
            };
            for child in node.children() {
                draw_visible_node(&mut context, child, 0, &mut logical_y, &mut render_y);
            }
        }
        assert_eq!(63.0, logical_y);
        assert_ne!(palette.background, canvas.pixels()[62 * canvas.width()]);
        assert_ne!(palette.background, canvas.pixels()[63 * canvas.width()]);
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
