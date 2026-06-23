use super::{
    ContainerPadding, TEXT_HEIGHT, UiNode, UiNodeKind, UiTreeHitRect, UiTreeHostActionHitCollector,
    UiTreeRenderArea, absolute_child_rect, child_container_x, child_render_area, dimension_px,
    frame_height, has_absolute_child, is_absolute, remaining_width, scroll_container_gap,
    should_draw_container_label,
};

impl UiTreeHostActionHitCollector<'_> {
    pub(super) fn container(&mut self, node: &UiNode, x: usize) {
        if node.kind() == UiNodeKind::Stack && has_absolute_child(node) {
            self.overlay_stack(node, x);
            return;
        }
        let container_top = self.y;
        self.push_container_action_hits(node, x, container_top, remaining_width(self.area, x));
        if should_draw_container_label(node) {
            self.y = self.y.saturating_add(TEXT_HEIGHT);
        }
        let padding = ContainerPadding::from_node(node);
        let child_x = child_container_x(node, x).saturating_add(padding.left);
        let previous_area = self.area;
        self.area = child_render_area(previous_area, node, child_x, padding);
        self.y = self.y.saturating_add(padding.top);
        let gap = scroll_container_gap(node);
        for (index, child) in node.children().iter().enumerate() {
            self.node(child, child_x);
            if index + 1 < node.children().len() {
                self.y = self.y.saturating_add(gap);
            }
        }
        self.y = self.y.saturating_add(padding.bottom);
        self.area = previous_area;
    }

    pub(super) fn overlay_stack(&mut self, node: &UiNode, x: usize) {
        let frame_top = self.y;
        let frame_width = remaining_width(self.area, x);
        let frame_height = frame_height(node).max(TEXT_HEIGHT);
        let previous_area = self.area;
        self.area = UiTreeRenderArea {
            x,
            y: frame_top,
            width: frame_width,
            height: frame_height,
            scroll_y: 0.0,
        };
        self.push_container_action_hits(node, x, frame_top, frame_width);
        for child in node.children().iter().filter(|child| !is_absolute(child)) {
            self.y = frame_top;
            self.node(child, x);
        }
        for child in node.children().iter().filter(|child| is_absolute(child)) {
            let rect = absolute_child_rect(x, frame_top, frame_width, frame_height, child);
            self.y = rect.y;
            let previous_semantic_node_id = self.semantic_node_id.take();
            self.node(child, rect.x);
            self.semantic_node_id = previous_semantic_node_id;
        }
        self.area = previous_area;
        self.y = frame_top.saturating_add(frame_height);
    }

    fn push_container_action_hits(
        &mut self,
        node: &UiNode,
        x: usize,
        y: usize,
        fallback_width: usize,
    ) {
        let actions = self.actions_for_node(node);
        if actions.is_empty() {
            return;
        }
        let requested_width = dimension_px(&node.props().common.width);
        let width = if requested_width > 0 {
            requested_width
        } else {
            fallback_width
        };
        let requested_height = dimension_px(&node.props().common.height);
        let height = if requested_height > 0 {
            requested_height
        } else {
            frame_height(node).max(TEXT_HEIGHT)
        };
        self.push_action_hits(
            node,
            actions,
            UiTreeHitRect {
                x,
                y,
                width,
                height,
            },
        );
    }
}
