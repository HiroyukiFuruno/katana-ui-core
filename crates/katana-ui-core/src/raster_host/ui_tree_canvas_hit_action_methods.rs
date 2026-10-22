use super::{
    UiHostActionPlan, UiNode, UiTreeHitRect, UiTreeHostActionHit, UiTreeHostActionHitCollector,
    UiTreeNodeHit, node_cursor, semantic_node_id,
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
        let rect = outset_node_hit(node, rect, self.hit_left_boundary());
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
        let rect = outset_node_hit(node, rect, self.hit_left_boundary());
        for action in actions {
            self.hits.push(UiTreeHostActionHit {
                action,
                rect,
                cursor,
            });
        }
    }

    pub(super) fn push_tree_row_action_hits(
        &mut self,
        node: &UiNode,
        actions: impl IntoIterator<Item = UiHostActionPlan>,
        rect: UiTreeHitRect,
    ) {
        let rect = outset_node_hit(node, rect, self.hit_left_boundary());
        for action in actions {
            self.hits.push(UiTreeHostActionHit {
                action,
                rect,
                cursor: node.props().tree.row_cursor,
            });
        }
    }

    fn hit_left_boundary(&self) -> usize {
        if self.scroll_clip == super::ScrollHitClip::Viewport {
            self.viewport_left_boundary
        } else {
            0
        }
    }
}

fn outset_node_hit(node: &UiNode, rect: UiTreeHitRect, left_boundary: usize) -> UiTreeHitRect {
    let outset = usize::from(node.hit_outset_left_px()).min(rect.x.saturating_sub(left_boundary));
    UiTreeHitRect {
        x: rect.x - outset,
        width: rect.width.saturating_add(outset),
        ..rect
    }
}
