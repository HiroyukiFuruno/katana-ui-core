use super::{
    Canvas, INDENT, TEXT_HEIGHT, TEXT_SIZE, TextRenderer, UiDimension, UiNode, UiNodeKind,
    UiTreeCanvasPalette, UiTreeRenderArea, UiVisualRole,
};
use crate::visual::ui_tree_canvas_image_metrics::logical_image_height;

const HOVER_SURFACE_ALPHA: u8 = 96;

pub(super) fn column_gap(node: &UiNode) -> usize {
    if node.kind() == UiNodeKind::Column {
        return dimension_px(&node.props().common.gap);
    }
    0
}

pub(super) fn gap_after_child(parent: &UiNode, child: &UiNode, next_child: &UiNode) -> usize {
    if child.kind() == UiNodeKind::ScrollArea && next_child.kind() == UiNodeKind::ScrollArea {
        return 0;
    }
    column_gap(parent)
}

pub(super) fn draw_hover_surface(
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    y: usize,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
    height: usize,
) {
    if node.props().visual_role == UiVisualRole::HoverSurface {
        let requested_width = dimension_px(&node.props().common.width);
        let width = if requested_width > 0 {
            requested_width
        } else {
            area.width.saturating_sub(x.saturating_sub(area.x))
        };
        canvas.blend_rect(
            x,
            y,
            width,
            height.max(TEXT_HEIGHT),
            palette.hover_background,
            HOVER_SURFACE_ALPHA,
        );
    }
}

pub(super) fn draw_hover_background(
    canvas: &mut Canvas,
    node: &UiNode,
    x: usize,
    y: usize,
    area: UiTreeRenderArea,
    palette: UiTreeCanvasPalette,
) {
    if !node.props().interaction.hovered || node.props().disabled {
        return;
    }
    if !matches!(node.kind(), UiNodeKind::Text | UiNodeKind::Accordion) {
        return;
    }
    let requested_width = dimension_px(&node.props().common.width);
    let width = if requested_width > 0 {
        requested_width
    } else {
        remaining_width(area, x)
    };
    canvas.fill_rect(
        x,
        y,
        width,
        dimension_px(&node.props().common.height).max(TEXT_HEIGHT),
        palette.hover_background,
    );
}

pub(in crate::visual) fn is_outside_vertical_viewport(
    y: usize,
    height: usize,
    area: UiTreeRenderArea,
) -> bool {
    let viewport_top = area.y;
    let viewport_bottom = area.y.saturating_add(area.height);
    let node_bottom = y.saturating_add(height);
    node_bottom <= viewport_top || y >= viewport_bottom
}

pub(super) fn draw_label(
    canvas: &mut Canvas,
    text: &TextRenderer,
    node: &UiNode,
    x: usize,
    y: &mut usize,
    palette: UiTreeCanvasPalette,
) {
    if !node.props().label.trim().is_empty() {
        text.draw(canvas, &node.props().label, x, *y, TEXT_SIZE, palette.text);
    }
    *y = y.saturating_add(TEXT_HEIGHT);
}

pub(super) fn remaining_width(area: UiTreeRenderArea, x: usize) -> usize {
    area.width.saturating_sub(x.saturating_sub(area.x)).max(1)
}

pub(super) fn stack_frame_height(node: &UiNode) -> usize {
    let own_height = dimension_px(&node.props().common.height);
    if own_height > 0 {
        return own_height;
    }
    node.children()
        .first()
        .map(child_media_height)
        .unwrap_or(TEXT_HEIGHT)
}

pub(super) fn child_media_height(node: &UiNode) -> usize {
    let requested = dimension_px(&node.props().common.height);
    if requested > 0 {
        return requested;
    }
    if node.kind() == UiNodeKind::ImageSurface {
        let image = &node.props().image_surface;
        return logical_image_height(image);
    }
    TEXT_HEIGHT
}

pub(super) fn should_draw_container_label(node: &UiNode) -> bool {
    if node.props().label.trim().is_empty() {
        return false;
    }
    !is_layout_container(node)
}

pub(super) fn child_container_x(node: &UiNode, x: usize) -> usize {
    if is_layout_container(node) {
        return x;
    }
    x.saturating_add(INDENT)
}

pub(super) fn is_layout_container(node: &UiNode) -> bool {
    matches!(
        node.kind(),
        UiNodeKind::AlignCenter
            | UiNodeKind::AlignNode
            | UiNodeKind::Column
            | UiNodeKind::Grid
            | UiNodeKind::Row
            | UiNodeKind::Stack
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ContainerPadding {
    pub(super) left: usize,
    right: usize,
    pub(super) top: usize,
    pub(super) bottom: usize,
}

impl ContainerPadding {
    pub(super) fn from_node(node: &UiNode) -> Self {
        Self {
            left: dimension_px(&node.props().common.padding.left),
            right: dimension_px(&node.props().common.padding.right),
            top: dimension_px(&node.props().common.padding.top),
            bottom: dimension_px(&node.props().common.padding.bottom),
        }
    }
}

pub(super) fn child_render_area(
    area: UiTreeRenderArea,
    node: &UiNode,
    child_x: usize,
    padding: ContainerPadding,
) -> UiTreeRenderArea {
    let available_width = area
        .width
        .saturating_sub(padding.left)
        .saturating_sub(padding.right);
    let requested_width = dimension_px(&node.props().common.width);
    let width = if requested_width > 0 {
        requested_width.min(available_width)
    } else {
        available_width
    };
    UiTreeRenderArea {
        x: child_x,
        y: area.y,
        width,
        height: area.height,
        scroll_y: area.scroll_y,
    }
}

pub(super) fn dimension_px(value: &UiDimension) -> usize {
    match value {
        UiDimension::Px(value) => usize::from(*value),
        _ => 0,
    }
}
