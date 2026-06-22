use super::{
    UiCursor, UiNode, UiNodeKind, UiTreeHitRect, UiTreeHostActionHit, UiTreeRenderArea,
    dimension_px,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ScrollHitClip {
    Viewport,
    Document,
}

pub(crate) const PRESERVED_WHITESPACE_WIDTH_FACTOR: f32 = 0.58;
pub(crate) const COLLAPSED_WHITESPACE_WIDTH_FACTOR: f32 = 0.30;

pub(super) fn whitespace_width(font_size: f32, preserve_whitespace: bool) -> usize {
    let factor = if preserve_whitespace {
        PRESERVED_WHITESPACE_WIDTH_FACTOR
    } else {
        COLLAPSED_WHITESPACE_WIDTH_FACTOR
    };
    (font_size * factor).ceil() as usize
}

pub(super) fn node_cursor(node: &UiNode) -> UiCursor {
    node.props().common.cursor
}

pub(super) fn duplicate_panel_label(parent: &UiNode, child: &UiNode) -> bool {
    child.kind() == UiNodeKind::Text && child.props().label == parent.props().label
}

pub(super) fn scroll_source_y(node: &UiNode, area: UiTreeRenderArea) -> usize {
    (node.props().scroll_area.offset_y as f32 + area.scroll_y.max(0.0))
        .round()
        .max(0.0) as usize
}

pub(super) fn clip_scroll_hit(
    mut hit: UiTreeHostActionHit,
    viewport_x: usize,
    viewport_y: usize,
    viewport_width: usize,
    viewport_height: usize,
    source_y: usize,
) -> Option<UiTreeHostActionHit> {
    let visible_left = hit.rect.x;
    let visible_right = hit
        .rect
        .x
        .saturating_add(hit.rect.width)
        .min(viewport_width);
    let visible_top = hit.rect.y.max(source_y);
    let visible_bottom = hit
        .rect
        .y
        .saturating_add(hit.rect.height)
        .min(source_y.saturating_add(viewport_height));
    if visible_right <= visible_left || visible_bottom <= visible_top {
        return None;
    }
    hit.rect = UiTreeHitRect {
        x: viewport_x.saturating_add(visible_left),
        y: viewport_y.saturating_add(visible_top.saturating_sub(source_y)),
        width: visible_right.saturating_sub(visible_left),
        height: visible_bottom.saturating_sub(visible_top),
    };
    Some(hit)
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ContainerPadding {
    pub(super) left: usize,
    pub(super) right: usize,
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
