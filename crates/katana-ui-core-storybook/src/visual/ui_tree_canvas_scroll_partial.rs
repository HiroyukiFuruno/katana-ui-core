use super::canvas::Canvas;
use super::ui_tree_canvas::UiTreeCanvasRenderer;
use super::ui_tree_canvas_hit_metrics::remaining_width;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_scroll_measure::can_render_partial_media_frame_stack;
use super::ui_tree_canvas_types::{PhysicalCanvasBlitRequest, UiTreeRenderArea};
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiVisualRole};

const HOVER_SURFACE_PARTIAL_CLIP_GUARD: usize = 20;
const HOVER_SURFACE_ALPHA: u8 = 96;

pub(super) fn draw_partially_visible_node(
    renderer: &UiTreeCanvasRenderer,
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    node_height: usize,
    source_y: f32,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
) {
    let source_y = source_y.max(0.0);
    if source_y >= node_height as f32 {
        return;
    }
    if node.kind() == UiNodeKind::ImageSurface {
        draw_partially_visible_image_node(renderer, canvas, node, x, source_y, area, palette);
        return;
    }
    if can_render_partial_hover_text_surface(node) {
        draw_partially_visible_hover_text_surface(
            renderer,
            canvas,
            node,
            x,
            source_y,
            area,
            palette,
            node_height,
        );
        return;
    }
    if can_render_partial_media_frame_stack(node) {
        draw_partially_visible_media_frame_stack(
            renderer,
            canvas,
            node,
            x,
            node_height,
            source_y,
            area,
            palette,
        );
        return;
    }
    let temp_height = partial_node_temp_height(node, node_height, area.height);
    let mut temp = Canvas::new_scaled_with_logical_phase(
        area.width,
        temp_height,
        canvas.scale_factor(),
        canvas.logical_phase_x().saturating_add(area.x),
        canvas.logical_phase_y() + area.y as f64 - f64::from(source_y),
        palette.background,
    );
    let mut temp_y = 0;
    let local_x = x.saturating_sub(area.x);
    renderer.render_node(
        &mut temp,
        node,
        local_x,
        &mut temp_y,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: area.width,
            height: temp_height,
            scroll_y: partial_node_inner_scroll_y(node, source_y),
        },
        palette,
    );
    let blit_source_y = if can_render_partial_node_in_viewport(node) {
        0
    } else {
        temp.fractional_to_physical_y(source_y)
    };
    canvas.blit_canvas_physical(
        &temp,
        PhysicalCanvasBlitRequest {
            dest_x: canvas.to_physical_x(area.x),
            dest_y: canvas.to_physical_y(area.y),
            dest_logical_x: area.x,
            dest_logical_y: area.y,
            width: canvas
                .to_physical_x(area.x.saturating_add(area.width))
                .saturating_sub(canvas.to_physical_x(area.x)),
            height: canvas
                .to_physical_y(area.y.saturating_add(area.height))
                .saturating_sub(canvas.to_physical_y(area.y)),
            source_y: blit_source_y,
            source_logical_y: if can_render_partial_node_in_viewport(node) {
                0.0
            } else {
                source_y
            },
        },
    );
}

fn draw_partially_visible_hover_text_surface(
    renderer: &UiTreeCanvasRenderer,
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    source_y: f32,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
    node_height: usize,
) {
    let Some(child) = node.children().first() else {
        return;
    };
    let visible_height = (node_height as f32 - source_y)
        .max(0.0)
        .min(area.height as f32);
    if visible_height <= 0.0 {
        return;
    }
    draw_partially_visible_node(
        renderer,
        canvas,
        child,
        x,
        node_height,
        source_y,
        area,
        palette,
    );
    canvas.blend_rect_at_logical_y(
        x,
        area.y as f32,
        hover_surface_width(node, x, area),
        visible_height,
        palette.hover_background,
        HOVER_SURFACE_ALPHA,
    );
}

fn draw_partially_visible_image_node(
    renderer: &UiTreeCanvasRenderer,
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    source_y: f32,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
) {
    let mut draw_y = area.y;
    renderer.render_node(
        canvas,
        node,
        x,
        &mut draw_y,
        UiTreeRenderArea {
            x: area.x,
            y: area.y,
            width: area.width,
            height: area.height,
            scroll_y: source_y,
        },
        palette,
    );
}

fn draw_partially_visible_media_frame_stack(
    renderer: &UiTreeCanvasRenderer,
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    node_height: usize,
    source_y: f32,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
) {
    let temp_height = node_height.max(1);
    let mut temp = Canvas::new_scaled_with_logical_phase(
        area.width,
        temp_height,
        canvas.scale_factor(),
        canvas.logical_phase_x().saturating_add(area.x),
        canvas.logical_phase_y() + area.y as f64 - f64::from(source_y),
        palette.background,
    );
    let local_x = x.saturating_sub(area.x);
    let mut draw_y = 0;
    renderer.render_node(
        &mut temp,
        node,
        local_x,
        &mut draw_y,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: area.width,
            height: temp_height,
            scroll_y: 0.0,
        },
        palette,
    );
    canvas.blit_canvas_physical(
        &temp,
        PhysicalCanvasBlitRequest {
            dest_x: canvas.to_physical_x(area.x),
            dest_y: canvas.to_physical_y(area.y),
            dest_logical_x: area.x,
            dest_logical_y: area.y,
            width: canvas
                .to_physical_x(area.x.saturating_add(area.width))
                .saturating_sub(canvas.to_physical_x(area.x)),
            height: canvas
                .to_physical_y(area.y.saturating_add(area.height))
                .saturating_sub(canvas.to_physical_y(area.y)),
            source_y: temp.fractional_to_physical_y(source_y),
            source_logical_y: source_y,
        },
    );
}

fn partial_node_temp_height(node: &UiNode, node_height: usize, viewport_height: usize) -> usize {
    if can_render_partial_node_in_viewport(node) {
        return viewport_height.max(1);
    }
    if node.props().visual_role == UiVisualRole::HoverSurface {
        return node_height
            .saturating_add(HOVER_SURFACE_PARTIAL_CLIP_GUARD)
            .max(1);
    }
    node_height.max(1)
}

fn partial_node_inner_scroll_y(node: &UiNode, source_y: f32) -> f32 {
    if can_render_partial_node_in_viewport(node) {
        return source_y;
    }
    0.0
}

fn can_render_partial_node_in_viewport(node: &UiNode) -> bool {
    matches!(node.kind(), UiNodeKind::Text | UiNodeKind::ImageSurface)
        || can_render_partial_media_frame_stack(node)
}

fn can_render_partial_hover_text_surface(node: &UiNode) -> bool {
    node.kind() == UiNodeKind::Stack
        && node.props().visual_role == UiVisualRole::HoverSurface
        && node.children().len() == 1
        && node
            .children()
            .first()
            .is_some_and(|child| child.kind() == UiNodeKind::Text)
}

fn hover_surface_width(node: &UiNode, x: usize, area: UiTreeRenderArea) -> usize {
    match node.props().common.width {
        UiDimension::Px(width) if width > 0 => usize::from(width),
        _ => remaining_width(area, x),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_ui_core::atom::Text;
    use katana_ui_core::render_model::{UiPosition, UiScrollAreaProps};
    use katana_ui_core::theme::ThemeSnapshot;

    fn render_context() -> (UiTreeCanvasRenderer, UiTreeCanvasPalette, UiTreeRenderArea) {
        let theme = ThemeSnapshot::dark();
        (
            UiTreeCanvasRenderer::new(theme.clone()),
            UiTreeCanvasPalette::from_theme(&theme),
            UiTreeRenderArea {
                x: 4,
                y: 3,
                width: 80,
                height: 40,
                scroll_y: 0.0,
            },
        )
    }

    #[test]
    fn partial_renderer_covers_noop_image_hover_and_generic_nodes() {
        let (renderer, palette, area) = render_context();
        let mut canvas = Canvas::new(96, 48, palette.background);
        let text: UiNode = Text::new("partial text").into();

        draw_partially_visible_node(&renderer, &mut canvas, &text, 8, 20, 20.0, area, palette);
        assert!(
            canvas
                .pixels()
                .iter()
                .all(|pixel| *pixel == palette.background)
        );

        let image = UiNode::new(UiNodeKind::ImageSurface, "");
        draw_partially_visible_node(&renderer, &mut canvas, &image, 8, 20, 1.0, area, palette);

        let hover = UiNode::new(UiNodeKind::Stack, "")
            .visual_role(UiVisualRole::HoverSurface)
            .width(UiDimension::Px(30))
            .child(Text::new("hover"));
        draw_partially_visible_node(&renderer, &mut canvas, &hover, 8, 24, 4.0, area, palette);
        assert!(
            canvas
                .pixels()
                .iter()
                .any(|pixel| *pixel != palette.background)
        );

        let generic_hover = UiNode::new(UiNodeKind::Stack, "")
            .visual_role(UiVisualRole::HoverSurface)
            .child(UiNode::new(UiNodeKind::Divider, ""));
        draw_partially_visible_node(
            &renderer,
            &mut canvas,
            &generic_hover,
            8,
            24,
            4.0,
            area,
            palette,
        );
        draw_partially_visible_node(&renderer, &mut canvas, &text, 8, 24, 4.0, area, palette);

        let empty_hover =
            UiNode::new(UiNodeKind::Stack, "").visual_role(UiVisualRole::HoverSurface);
        draw_partially_visible_hover_text_surface(
            &renderer,
            &mut canvas,
            &empty_hover,
            8,
            0.0,
            area,
            palette,
            0,
        );
        draw_partially_visible_hover_text_surface(
            &renderer,
            &mut canvas,
            &hover,
            8,
            0.0,
            area,
            palette,
            0,
        );
    }

    #[test]
    fn partial_renderer_covers_absolute_media_frame_and_helpers() {
        let (renderer, palette, area) = render_context();
        let mut canvas = Canvas::new(96, 48, palette.background);
        let overlay = UiNode::new(UiNodeKind::Button, "overlay")
            .position(UiPosition::Absolute)
            .width(UiDimension::Px(10))
            .height(UiDimension::Px(10));
        let media_frame = UiNode::new(UiNodeKind::Stack, "")
            .visual_role(UiVisualRole::MediaFrame)
            .child(Text::new("body"))
            .child(overlay);

        assert!(can_render_partial_node_in_viewport(&media_frame));
        draw_partially_visible_node(
            &renderer,
            &mut canvas,
            &media_frame,
            8,
            30,
            5.0,
            area,
            palette,
        );
        assert!(
            canvas
                .pixels()
                .iter()
                .any(|pixel| *pixel != palette.background)
        );

        assert_eq!(partial_node_temp_height(&media_frame, 30, 40), 40);
        assert_eq!(partial_node_inner_scroll_y(&media_frame, 7.0), 7.0);
        let plain = UiNode::new(UiNodeKind::Column, "");
        assert_eq!(partial_node_temp_height(&plain, 30, 40), 30);
        assert_eq!(partial_node_inner_scroll_y(&plain, 7.0), 0.0);
        let hover = UiNode::new(UiNodeKind::Stack, "").visual_role(UiVisualRole::HoverSurface);
        assert_eq!(partial_node_temp_height(&hover, 30, 40), 50);
        assert_eq!(hover_surface_width(&hover, 10, area), 74);
        let fixed = hover.width(UiDimension::Px(32));
        assert_eq!(hover_surface_width(&fixed, 10, area), 32);
    }

    #[test]
    fn fractional_scale_partial_text_matches_the_full_physical_crop_at_nonzero_phase() {
        let (renderer, palette, area) = render_context();
        let text: UiNode = Text::new("phase-aware partial text").into();
        let source_y = 0.75;
        let mut full = Canvas::new_scaled_with_logical_phase(
            area.width,
            area.height,
            1.25,
            area.x,
            area.y as f64 - f64::from(source_y),
            palette.background,
        );
        let mut full_y = 0;
        renderer.render_node(
            &mut full,
            &text,
            4,
            &mut full_y,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: area.width,
                height: area.height,
                scroll_y: source_y,
            },
            palette,
        );
        let mut partial = Canvas::new_scaled(96, 48, 1.25, palette.background);
        draw_partially_visible_node(
            &renderer,
            &mut partial,
            &text,
            8,
            30,
            source_y,
            area,
            palette,
        );

        let physical_dest_x = partial.to_physical_x(area.x);
        let physical_dest_y = partial.to_physical_y(area.y);
        for y in 0..full.height() {
            for x in 0..full.width() {
                assert_eq!(
                    full.pixels()[y * full.width() + x],
                    partial.pixels()[(physical_dest_y + y) * partial.width() + physical_dest_x + x],
                    "partial text differs from the full physical crop at x={x}, y={y}"
                );
            }
        }
    }

    #[test]
    fn nested_scroll_partial_matches_full_crop_with_destination_logical_phase() {
        let (renderer, palette, area) = render_context();
        let source_y = 0.75;
        let inner = UiNode::new(UiNodeKind::ScrollArea, "")
            .scroll_area(UiScrollAreaProps {
                viewport_width: area.width as u32,
                viewport_height: area.height as u32,
                offset_y: 1,
                ..UiScrollAreaProps::default()
            })
            .child(Text::new("nested phase-aware scroll content"));
        let outer = UiNode::new(UiNodeKind::ScrollArea, "")
            .scroll_area(UiScrollAreaProps {
                viewport_width: area.width as u32,
                viewport_height: area.height as u32,
                ..UiScrollAreaProps::default()
            })
            .child(inner);
        let target_phase_x = 3_usize;
        let target_phase_y = 2.5;
        let mut source = Canvas::new_scaled_with_logical_phase(
            area.width,
            30,
            1.25,
            target_phase_x.saturating_add(area.x),
            target_phase_y + area.y as f64 - f64::from(source_y),
            palette.background,
        );
        let mut source_y_cursor = 0;
        renderer.render_node(
            &mut source,
            &outer,
            4,
            &mut source_y_cursor,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: area.width,
                height: 30,
                scroll_y: 0.0,
            },
            palette,
        );
        let mut expected = Canvas::new_scaled_with_logical_phase(
            96,
            48,
            1.25,
            target_phase_x,
            target_phase_y,
            palette.background,
        );
        let expected_dest_x = expected.to_physical_x(area.x);
        let expected_dest_y = expected.to_physical_y(area.y);
        expected.blit_canvas_physical(
            &source,
            PhysicalCanvasBlitRequest {
                dest_x: expected_dest_x,
                dest_y: expected_dest_y,
                dest_logical_x: area.x,
                dest_logical_y: area.y,
                width: expected
                    .to_physical_x(area.x.saturating_add(area.width))
                    .saturating_sub(expected_dest_x),
                height: expected
                    .to_physical_y(area.y.saturating_add(area.height))
                    .saturating_sub(expected_dest_y),
                source_y: source.fractional_to_physical_y(source_y),
                source_logical_y: source_y,
            },
        );
        let mut partial = Canvas::new_scaled_with_logical_phase(
            96,
            48,
            1.25,
            target_phase_x,
            target_phase_y,
            palette.background,
        );
        draw_partially_visible_node(
            &renderer,
            &mut partial,
            &outer,
            8,
            30,
            source_y,
            area,
            palette,
        );

        assert_eq!(
            expected.pixels(),
            partial.pixels(),
            "nested partial scroll must use the destination canvas logical phase"
        );

        assert_eq!(
            expected.text_runs(),
            partial.text_runs(),
            "nested partial scroll must preserve the full crop's selectable text bounds"
        );
        let expected_run = expected
            .text_runs()
            .iter()
            .find(|run| run.text() == "nested phase-aware scroll content")
            .expect("full crop must expose the nested text as a selectable run");
        let partial_run = partial
            .text_runs()
            .iter()
            .find(|run| run.text() == "nested phase-aware scroll content")
            .expect("partial crop must expose the nested text as a selectable run");
        assert_eq!(
            (
                expected_run.x(),
                expected_run.y(),
                expected_run.width(),
                expected_run.height()
            ),
            (
                partial_run.x(),
                partial_run.y(),
                partial_run.width(),
                partial_run.height()
            ),
            "nested partial scroll must retain the logical text-run hit bounds"
        );
        assert_eq!(
            expected.copy_text_in_selection(
                Some((expected_run.x(), expected_run.y())),
                Some((expected_run.right(), expected_run.bottom())),
            ),
            partial.copy_text_in_selection(
                Some((partial_run.x(), partial_run.y())),
                Some((partial_run.right(), partial_run.bottom())),
            ),
            "nested partial scroll must retain text selection hit metadata"
        );
    }
}
