use super::canvas::Canvas;
use super::ui_tree_canvas::UiTreeCanvasRenderer;
use super::ui_tree_canvas_hit_metrics::remaining_width;
use super::ui_tree_canvas_palette::UiTreeCanvasPalette;
use super::ui_tree_canvas_scroll_measure::can_render_partial_media_frame_stack;
use super::ui_tree_canvas_types::{CanvasBlitRequest, UiTreeRenderArea};
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiVisualRole};

const HOVER_SURFACE_PARTIAL_CLIP_GUARD: usize = 20;
const HOVER_SURFACE_ALPHA: u8 = 96;

pub(super) fn draw_partially_visible_node(
    renderer: &UiTreeCanvasRenderer,
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    node_height: usize,
    source_y: usize,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
) {
    if source_y >= node_height {
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
    let mut temp = Canvas::new(area.width, temp_height, palette.background);
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
        source_y
    };
    canvas.blit_canvas(
        &temp,
        CanvasBlitRequest {
            dest_x: area.x,
            dest_y: area.y,
            width: area.width,
            height: area.height,
            source_y: blit_source_y,
        },
    );
}

fn draw_partially_visible_hover_text_surface(
    renderer: &UiTreeCanvasRenderer,
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    source_y: usize,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
    node_height: usize,
) {
    let Some(child) = node.children().first() else {
        return;
    };
    let visible_height = node_height.saturating_sub(source_y).min(area.height);
    if visible_height == 0 {
        return;
    }
    canvas.blend_rect(
        x,
        area.y,
        hover_surface_width(node, x, area),
        visible_height,
        palette.hover_background,
        HOVER_SURFACE_ALPHA,
    );
    let mut draw_y = area.y;
    renderer.render_node(
        canvas,
        child,
        x,
        &mut draw_y,
        UiTreeRenderArea {
            x: area.x,
            y: area.y,
            width: area.width,
            height: area.height,
            scroll_y: source_y as f32,
        },
        palette,
    );
}

fn draw_partially_visible_image_node(
    renderer: &UiTreeCanvasRenderer,
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    source_y: usize,
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
            scroll_y: source_y as f32,
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
    source_y: usize,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
) {
    let temp_height = node_height.max(1);
    let mut temp = Canvas::new(area.width, temp_height, palette.background);
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
    canvas.blit_canvas(
        &temp,
        CanvasBlitRequest {
            dest_x: area.x,
            dest_y: area.y,
            width: area.width,
            height: area.height,
            source_y,
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

fn partial_node_inner_scroll_y(node: &UiNode, source_y: usize) -> f32 {
    if can_render_partial_node_in_viewport(node) {
        return source_y as f32;
    }
    0.0
}

fn can_render_partial_node_in_viewport(node: &UiNode) -> bool {
    matches!(node.kind(), UiNodeKind::Text | UiNodeKind::ImageSurface)
        || can_render_partial_media_frame_stack(node)
}

fn can_render_partial_hover_text_surface(node: &UiNode) -> bool {
    node.props().visual_role == UiVisualRole::HoverSurface
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
    use katana_ui_core::render_model::UiPosition;
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

        draw_partially_visible_node(&renderer, &mut canvas, &text, 8, 20, 20, area, palette);
        assert!(
            canvas
                .pixels()
                .iter()
                .all(|pixel| *pixel == palette.background)
        );

        let image = UiNode::new(UiNodeKind::ImageSurface, "");
        draw_partially_visible_node(&renderer, &mut canvas, &image, 8, 20, 1, area, palette);

        let hover = UiNode::new(UiNodeKind::Stack, "")
            .visual_role(UiVisualRole::HoverSurface)
            .width(UiDimension::Px(30))
            .child(Text::new("hover"));
        draw_partially_visible_node(&renderer, &mut canvas, &hover, 8, 24, 4, area, palette);
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
            4,
            area,
            palette,
        );
        draw_partially_visible_node(&renderer, &mut canvas, &text, 8, 24, 4, area, palette);

        let empty_hover =
            UiNode::new(UiNodeKind::Stack, "").visual_role(UiVisualRole::HoverSurface);
        draw_partially_visible_hover_text_surface(
            &renderer,
            &mut canvas,
            &empty_hover,
            8,
            0,
            area,
            palette,
            0,
        );
        draw_partially_visible_hover_text_surface(
            &renderer,
            &mut canvas,
            &hover,
            8,
            0,
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
            5,
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
        assert_eq!(partial_node_inner_scroll_y(&media_frame, 7), 7.0);
        let plain = UiNode::new(UiNodeKind::Column, "");
        assert_eq!(partial_node_temp_height(&plain, 30, 40), 30);
        assert_eq!(partial_node_inner_scroll_y(&plain, 7), 0.0);
        let hover = UiNode::new(UiNodeKind::Stack, "").visual_role(UiVisualRole::HoverSurface);
        assert_eq!(partial_node_temp_height(&hover, 30, 40), 50);
        assert_eq!(hover_surface_width(&hover, 10, area), 74);
        let fixed = hover.width(UiDimension::Px(32));
        assert_eq!(hover_surface_width(&fixed, 10, area), 32);
    }
}
