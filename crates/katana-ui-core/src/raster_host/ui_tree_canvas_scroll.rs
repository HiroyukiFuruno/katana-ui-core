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
    let source_y = scroll_y.max(0.0);
    let mut logical_y = 0.0_f32;
    let mut render_y = viewport.y as f32 - source_y;
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
                if logical_y < source_y + viewport.height as f32 {
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
    source_y: f32,
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
        if node_bottom <= source_y || node_top >= source_y + area.height as f32 {
            *logical_y = node_bottom;
            *render_y += node_height;
            return;
        }
        let requested_height = dimension_px(&node.props().common.height);
        if requested_height > 0 {
            let child_logical_top = *logical_y;
            let child_render_top = *render_y;
            let renderer = context.renderer;
            let source_y = context.source_y;
            let child_area = context.area;
            let palette = context.palette;
            let text_context = context.text_context;
            context.canvas.with_clip_at_logical_y(
                x,
                child_render_top,
                child_area
                    .width
                    .saturating_sub(x.saturating_sub(child_area.x)),
                requested_height as f32,
                &mut |canvas| {
                    let mut clipped = ScrollDrawContext {
                        renderer,
                        canvas,
                        source_y,
                        area: child_area,
                        palette,
                        text_context,
                    };
                    draw_visible_children(&mut clipped, node, x, logical_y, render_y);
                },
            );
            let requested_bottom = node_top + requested_height as f32;
            *render_y += requested_bottom - *logical_y;
            *logical_y = requested_bottom;
            debug_assert!(*logical_y >= child_logical_top);
        } else {
            draw_visible_children(context, node, x, logical_y, render_y);
        }
        return;
    }
    let node_top = *logical_y;
    let node_height = logical_node_height(renderer, text_context, node, x, area).max(1.0);
    let node_bottom = node_top + node_height;
    *logical_y = node_bottom;
    if node_bottom <= source_y || node_top >= source_y + area.height as f32 {
        *render_y += node_height;
        return;
    }
    if node_top >= source_y {
        let render_start = *render_y;
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
            renderer.render_node_with_logical_cursor(
                context.canvas,
                node,
                x,
                render_y,
                area,
                palette,
            );
            *logical_y = node_top + (*render_y - render_start);
        }
        return;
    }
    draw_partially_visible_node(
        renderer,
        context.canvas,
        node,
        x,
        node_height.ceil() as usize,
        partial_scroll_source_offset(source_y, node_top),
        area,
        palette,
    );
    *render_y += node_height;
}

fn partial_scroll_source_offset(source_y: f32, node_top: f32) -> f32 {
    (source_y - node_top).max(0.0)
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
        if *logical_y >= source_y + area.height as f32 {
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
    use super::{
        ScrollDrawContext, draw_scroll_area, draw_visible_node, partial_scroll_source_offset,
    };
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
            source_y: 0.0,
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
    fn scroll_area_preserves_fractional_text_origin_for_following_non_text_child() {
        let theme = ThemeSnapshot::dark();
        let palette = UiTreeCanvasPalette::from_theme(&theme);
        let renderer = UiTreeCanvasRenderer::with_document_typography(
            theme,
            UiTreeDocumentTypography::new()
                .with_body_baseline(UiTreeTextRoleBaselineTypography::new(16.0, 31.5, 18.5)),
        );
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 180,
            height: 80,
            scroll_y: 0.0,
        };
        let node = UiNode::new(UiNodeKind::ScrollArea, "")
            .scroll_area(UiScrollAreaProps {
                viewport_width: 180,
                viewport_height: 80,
                content_width: 180,
                content_height: 80,
                ..UiScrollAreaProps::default()
            })
            .child(Text::new("first").text_role("body"))
            .child(UiNode::new(UiNodeKind::Button, "button").height(UiDimension::px(20)));
        let mut canvas = Canvas::new_scaled(180, 80, 2.0, palette.background);
        let mut y = 0;

        draw_scroll_area(&renderer, &mut canvas, &node, 0, &mut y, area, palette);

        let physical_width = canvas.width();
        assert_ne!(
            palette.selection,
            canvas.pixels()[62 * physical_width],
            "the following non-Text child must not floor 31.5px to physical row 62"
        );
        assert_eq!(
            palette.selection,
            canvas.pixels()[63 * physical_width],
            "the following non-Text child must begin at physical row 63 at scale 2"
        );
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
                source_y: 0.0,
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

    #[test]
    fn partial_scroll_offset_keeps_fractional_logical_coordinate() {
        let node_top = 31.5;
        let source_y = 32.0;

        assert_eq!(0.5, partial_scroll_source_offset(source_y, node_top));
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
