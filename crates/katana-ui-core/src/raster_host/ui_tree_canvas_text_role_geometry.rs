use super::{LIST_DEPTH_INDENT, QUOTE_INDENT, UiDimension, UiNode, UiTreeRenderArea};

pub(super) fn remaining_width(area: UiTreeRenderArea, x: usize) -> usize {
    area.width.saturating_sub(x.saturating_sub(area.x)).max(1)
}

pub(super) fn code_text_padding_left(node: &UiNode) -> usize {
    dimension_px(&node.props().common.padding.left)
}

pub(super) fn quote_text_padding_left(node: &UiNode) -> usize {
    dimension_px(&node.props().common.padding.left)
}

pub(super) fn quote_depth(node: &UiNode) -> usize {
    dimension_px(&node.props().common.margin.left) / QUOTE_INDENT
}

pub(super) fn dimension_px(value: &UiDimension) -> usize {
    match value {
        UiDimension::Px(value) => usize::from(*value),
        _ => 0,
    }
}

pub(super) fn list_depth(node: &UiNode) -> usize {
    dimension_px(&node.props().common.margin.left) / LIST_DEPTH_INDENT
}
