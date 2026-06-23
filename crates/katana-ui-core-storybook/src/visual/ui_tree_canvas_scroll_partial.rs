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
