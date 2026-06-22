use super::ui_tree_canvas_types::UiTreeRenderArea;
use crate::visual::ui_tree_canvas_image_metrics::logical_image_height;
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiPosition, UiVisualRole};

pub(super) const TEXT_HEIGHT: usize = 20;
pub(super) const NODE_GAP: usize = 8;
pub(super) const INDENT: usize = 16;
pub(super) const BUTTON_DRAW_WIDTH: usize = 96;
pub(super) const BUTTON_SLOT_WIDTH: usize = 104;
pub(super) const TOGGLE_TRACK_WIDTH: usize = 48;
pub(super) const TOGGLE_TRACK_HEIGHT: usize = 22;
const CHECKBOX_SLOT_WIDTH: usize = 28;
const DEFAULT_SLOT_WIDTH: usize = 104;
const TEXT_ADVANCE: usize = 10;
const TEXT_SLOT_PADDING: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct AbsoluteChildRect {
    pub x: usize,
    pub y: usize,
}

pub(super) fn absolute_child_rect(
    parent_x: usize,
    parent_y: usize,
    parent_width: usize,
    parent_height: usize,
    child: &UiNode,
) -> AbsoluteChildRect {
    let margin = &child.props().common.margin;
    let width = control_row_width(child, slot_width(child));
    let height = control_row_height(child, child_height(child));
    let right = dimension_px(&margin.right);
    let left = dimension_px(&margin.left);
    let bottom = dimension_px(&margin.bottom);
    let top = dimension_px(&margin.top);
    let x = if right > 0 {
        parent_x
            .saturating_add(parent_width.saturating_sub(width))
            .saturating_sub(right)
    } else {
        parent_x.saturating_add(left)
    };
    let y = if bottom > 0 {
        parent_y
            .saturating_add(parent_height.saturating_sub(height))
            .saturating_sub(bottom)
    } else {
        parent_y.saturating_add(top)
    };
    AbsoluteChildRect { x, y }
}

pub(super) fn has_absolute_child(node: &UiNode) -> bool {
    node.children().iter().any(is_absolute)
}

pub(super) fn is_absolute(node: &UiNode) -> bool {
    node.props().common.position == UiPosition::Absolute
}

pub(super) fn frame_height(node: &UiNode) -> usize {
    let requested = dimension_px(&node.props().common.height);
    if requested > 0 {
        return requested;
    }
    node.children()
        .first()
        .map(child_height)
        .unwrap_or(TEXT_HEIGHT)
}

pub(super) fn should_draw_container_label(node: &UiNode) -> bool {
    !node.props().label.trim().is_empty() && !is_layout_container(node)
}

pub(super) fn button_dimensions(node: &UiNode) -> (usize, usize) {
    let requested_width = dimension_px(&node.props().common.width);
    let requested_height = dimension_px(&node.props().common.height);
    (
        if requested_width > 0 {
            requested_width
        } else {
            BUTTON_DRAW_WIDTH
        },
        if requested_height > 0 {
            requested_height
        } else {
            TEXT_HEIGHT
        },
    )
}

pub(super) const fn toggle_dimensions() -> (usize, usize) {
    (TOGGLE_TRACK_WIDTH, TOGGLE_TRACK_HEIGHT)
}

pub(super) fn button_slot_width(node: &UiNode) -> usize {
    let requested = dimension_px(&node.props().common.width);
    if requested > 0 {
        requested
    } else {
        BUTTON_SLOT_WIDTH
    }
}

pub(super) fn child_container_x(node: &UiNode, x: usize) -> usize {
    if is_layout_container(node) {
        return x;
    }
    x.saturating_add(INDENT)
}

pub(super) fn row_indent(node: &UiNode) -> usize {
    dimension_px(&node.props().common.margin.left)
}

pub(super) fn slot_width(node: &UiNode) -> usize {
    let requested = dimension_px(&node.props().common.width);
    if requested > 0 {
        return requested;
    }
    if node.props().visual_role == UiVisualRole::HoverSurface
        && let Some(child) = node.children().first()
    {
        return slot_width(child);
    }
    match node.kind() {
        UiNodeKind::Button | UiNodeKind::TextButton | UiNodeKind::IconTextButton => {
            button_slot_width(node)
        }
        UiNodeKind::Text => text_slot_width(&node.props().label),
        UiNodeKind::Checkbox => CHECKBOX_SLOT_WIDTH,
        _ => DEFAULT_SLOT_WIDTH,
    }
}

pub(super) fn control_row_width(node: &UiNode, fallback: usize) -> usize {
    let requested = dimension_px(&node.props().common.width);
    if requested > 0 {
        return requested;
    }
    let width = control_layout_width(node);
    if width > 0 { width } else { fallback }
}

fn control_layout_width(node: &UiNode) -> usize {
    match node.kind() {
        UiNodeKind::Row => sum_with_gaps(
            node.children().iter().map(slot_width),
            node.children().len(),
            dimension_px(&node.props().common.gap),
        ),
        UiNodeKind::Column => node
            .children()
            .iter()
            .map(control_layout_width)
            .max()
            .unwrap_or(0),
        _ => slot_width(node),
    }
}

pub(super) fn control_row_height(node: &UiNode, fallback: usize) -> usize {
    let requested = dimension_px(&node.props().common.height);
    if requested > 0 {
        return requested;
    }
    let height = control_layout_height(node);
    if height > 0 { height } else { fallback }
}

fn control_layout_height(node: &UiNode) -> usize {
    match node.kind() {
        UiNodeKind::Row => node
            .children()
            .iter()
            .map(control_layout_height)
            .max()
            .unwrap_or(0),
        UiNodeKind::Column => sum_with_gaps(
            node.children().iter().map(control_layout_height),
            node.children().len(),
            dimension_px(&node.props().common.gap),
        ),
        _ => control_item_height(node),
    }
}

fn sum_with_gaps(values: impl Iterator<Item = usize>, len: usize, gap: usize) -> usize {
    let sum = values.sum::<usize>();
    sum.saturating_add(gap.saturating_mul(len.saturating_sub(1)))
}

fn control_item_height(node: &UiNode) -> usize {
    let requested = dimension_px(&node.props().common.height);
    if requested > 0 {
        return requested;
    }
    child_height(node)
}

pub(super) fn remaining_width(area: UiTreeRenderArea, x: usize) -> usize {
    area.width.saturating_sub(x.saturating_sub(area.x)).max(1)
}

pub(super) fn dimension_px(value: &UiDimension) -> usize {
    match value {
        UiDimension::Px(value) => usize::from(*value),
        _ => 0,
    }
}

pub(super) fn child_height(node: &UiNode) -> usize {
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

pub(super) fn render_origin_y(root: &UiNode, area: UiTreeRenderArea) -> usize {
    if root.kind() == UiNodeKind::ScrollArea {
        return area.y;
    }
    area.y
        .saturating_sub(area.scroll_y.round().max(0.0) as usize)
}

fn is_layout_container(node: &UiNode) -> bool {
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

fn text_slot_width(text: &str) -> usize {
    text.chars()
        .count()
        .saturating_mul(TEXT_ADVANCE)
        .saturating_add(TEXT_SLOT_PADDING)
        .max(TEXT_SLOT_PADDING)
}
