use super::ui_tree_canvas_hit_metrics::{dimension_px, row_indent, slot_width};
use super::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::UiNode;

#[derive(Debug, Clone, Copy)]
pub(super) struct UiTreeRowChildLayout<'a> {
    pub child: &'a UiNode,
    pub x: usize,
}

pub(super) struct UiTreeRowLayout;

impl UiTreeRowLayout {
    pub(super) fn children<'a>(
        node: &'a UiNode,
        x: usize,
        area: UiTreeRenderArea,
    ) -> Vec<UiTreeRowChildLayout<'a>> {
        let mut children = Vec::new();
        let mut cursor_x = Self::row_x(node, x);
        let gap = dimension_px(&node.props().common.gap);
        for (index, child) in node.children().iter().enumerate() {
            children.push(UiTreeRowChildLayout { child, x: cursor_x });
            cursor_x = cursor_x.saturating_add(slot_width(child));
            if index + 1 < node.children().len() {
                cursor_x = cursor_x.saturating_add(gap);
            }
            if cursor_x >= area.x.saturating_add(area.width) {
                break;
            }
        }
        children
    }

    pub(super) fn row_x(node: &UiNode, x: usize) -> usize {
        x.saturating_add(row_indent(node))
    }
}
