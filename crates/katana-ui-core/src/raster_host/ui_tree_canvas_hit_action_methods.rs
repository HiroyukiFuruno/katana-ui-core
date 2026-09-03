use super::{
    UI_LINK_OPEN_ACTION_ID, UiHostActionPlan, UiNode, UiTreeHitRect, UiTreeHostActionHit,
    UiTreeHostActionHitCollector, UiTreeNodeHit, UiTreeTextRoleRenderer, node_cursor,
    semantic_node_id,
};

impl UiTreeHostActionHitCollector<'_> {
    pub(super) fn actions_for_node(&self, node: &UiNode) -> Vec<UiHostActionPlan> {
        if self.actions.is_empty() {
            return UiHostActionPlan::collect_from_node(node);
        }
        self.actions
            .iter()
            .filter(move |action| action.target.as_str() == node.id().as_str())
            .cloned()
            .collect()
    }

    pub(super) fn tree_row_actions(
        &self,
        node: &UiNode,
        tree_node_id: &str,
    ) -> Vec<UiHostActionPlan> {
        let actions = self.actions_for_node(node);
        actions
            .iter()
            .filter(move |action| action.target.as_str() == node.id().as_str())
            .filter(|action| {
                action
                    .tree_row_action_target()
                    .is_some_and(|target| target.node_id == tree_node_id)
            })
            .cloned()
            .collect()
    }

    pub(super) fn push_node_action_hits(&mut self, node: &UiNode, rect: UiTreeHitRect) {
        self.push_node_hit(node, rect);
        self.push_action_hits(node, self.actions_for_node(node), rect);
    }

    pub(super) fn push_node_hit(&mut self, node: &UiNode, rect: UiTreeHitRect) {
        let semantic_node_id = semantic_node_id(node).or_else(|| self.semantic_node_id.clone());
        self.node_hits.push(UiTreeNodeHit {
            node_id: node.id().clone(),
            semantic_node_id,
            rect,
            cursor: node_cursor(node),
        });
    }

    pub(super) fn push_action_hits(
        &mut self,
        node: &UiNode,
        actions: impl IntoIterator<Item = UiHostActionPlan>,
        rect: UiTreeHitRect,
    ) {
        let cursor = node_cursor(node);
        for action in actions {
            self.hits.push(UiTreeHostActionHit {
                action,
                rect,
                cursor,
            });
        }
    }

    pub(super) fn push_text_link_action_hits(
        &mut self,
        node: &UiNode,
        x: usize,
        height: usize,
        actions: &[UiHostActionPlan],
    ) {
        if !actions
            .iter()
            .any(|action| action.action_id == UI_LINK_OPEN_ACTION_ID)
        {
            return;
        }
        let cursor = node_cursor(node);
        let total_width = self.text_hit_width(node);
        let mut span_x = UiTreeTextRoleRenderer::aligned_x(node, x, self.area, total_width);
        let mut link_actions = actions
            .iter()
            .filter(|action| action.action_id == UI_LINK_OPEN_ACTION_ID);
        for span in &node.props().text.spans {
            let width = self.text_span_render_width(node, span);
            if !span.link_target.trim().is_empty()
                && let Some(action) = link_actions.next()
            {
                let (visible_offset, visible_width) = self.text_span_visible_hit_bounds(node, span);
                let visible_x = span_x + visible_offset as isize;
                let rect_x = visible_x.max(0) as usize;
                let hidden_width = visible_x.unsigned_abs() * usize::from(visible_x.is_negative());
                let clipped_width = visible_width.saturating_sub(hidden_width);
                self.hits.push(UiTreeHostActionHit {
                    action: action.clone(),
                    rect: UiTreeHitRect {
                        x: rect_x,
                        y: self.y,
                        width: clipped_width,
                        height,
                    },
                    cursor,
                });
            }
            span_x += width as isize;
        }
    }
}
