use super::{UiNode, UiNodeId, UiNodeKind, UiVisualRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiTree {
    root: UiNode,
}

impl UiTree {
    #[must_use]
    pub fn new(root: impl Into<UiNode>) -> Self {
        Self { root: root.into() }
    }

    #[must_use]
    pub fn root(&self) -> &UiNode {
        &self.root
    }

    #[must_use]
    pub fn with_hovered_node_id(&self, hovered_node_id: Option<&UiNodeId>) -> Self {
        let Some(hovered_node_id) = hovered_node_id else {
            return self.clone();
        };
        Self {
            root: hovered_node(self.root.clone(), hovered_node_id),
        }
    }

    #[must_use]
    pub fn with_hover_surface_for_node_id(&self, hovered_node_id: Option<&UiNodeId>) -> Self {
        let Some(hovered_node_id) = hovered_node_id else {
            return self.clone();
        };
        Self {
            root: hover_surface_node(self.root.clone(), hovered_node_id),
        }
    }

    #[must_use]
    pub fn with_animation_phase(&self, phase: u16) -> Self {
        Self {
            root: animation_phase_node(self.root.clone(), phase),
        }
    }

    #[must_use]
    pub fn with_scroll_area_offset_y(&self, offset_y: u32) -> Self {
        Self {
            root: scroll_area_offset_y_node(self.root.clone(), offset_y),
        }
    }
}

fn hovered_node(mut node: UiNode, hovered_node_id: &UiNodeId) -> UiNode {
    node.props.interaction.hovered = node.id() == hovered_node_id;
    node.children = node
        .children
        .into_iter()
        .map(|child| hovered_node(child, hovered_node_id))
        .collect();
    node
}

fn hover_surface_node(mut node: UiNode, hovered_node_id: &UiNodeId) -> UiNode {
    if node_matches_hover_surface(&node, hovered_node_id) {
        let mut surface = UiNode::new(UiNodeKind::Stack, "");
        surface.props.common = node.props.common.clone();
        surface.props.visual_role = UiVisualRole::HoverSurface;
        surface.children.push(node);
        return surface;
    }
    node.children = node
        .children
        .into_iter()
        .map(|child| hover_surface_node(child, hovered_node_id))
        .collect();
    node
}

fn node_matches_hover_surface(node: &UiNode, hovered_node_id: &UiNodeId) -> bool {
    if node.id() == hovered_node_id {
        return true;
    }
    let semantic_node_id = node.props.common.semantic_node_id.trim();
    !semantic_node_id.is_empty() && semantic_node_id == hovered_node_id.as_str()
}

fn animation_phase_node(mut node: UiNode, phase: u16) -> UiNode {
    node.props.interaction.animation_phase = phase;
    node.children = node
        .children
        .into_iter()
        .map(|child| animation_phase_node(child, phase))
        .collect();
    node
}

fn scroll_area_offset_y_node(mut node: UiNode, offset_y: u32) -> UiNode {
    if node.kind() == UiNodeKind::ScrollArea {
        node.props.scroll_area.offset_y = offset_y;
    }
    node.children = node
        .children
        .into_iter()
        .map(|child| scroll_area_offset_y_node(child, offset_y))
        .collect();
    node
}

#[cfg(test)]
mod tests {
    use super::UiTree;
    use crate::render_model::{UiCommonProps, UiDimension, UiNode, UiNodeKind, UiVisualRole};

    #[test]
    fn with_hovered_node_id_marks_only_matching_node() {
        let target = UiNode::new(UiNodeKind::Button, "target").stable_node_id("target-node");
        let other = UiNode::new(UiNodeKind::Button, "other").stable_node_id("other-node");
        let tree = UiTree::new(UiNode::new(UiNodeKind::Row, "").child(target).child(other));

        let hovered = tree.with_hovered_node_id(Some(&"target-node".into()));

        assert!(hovered.root().children()[0].props().interaction.hovered);
        assert!(!hovered.root().children()[1].props().interaction.hovered);
    }

    #[test]
    fn missing_hover_targets_preserve_the_tree() {
        let tree = UiTree::new(UiNode::new(UiNodeKind::Button, "target"));

        assert_eq!(tree, tree.with_hovered_node_id(None));
        assert_eq!(tree, tree.with_hover_surface_for_node_id(None));
    }

    #[test]
    fn with_hovered_node_id_clears_stale_hover_on_non_matching_nodes() {
        let stale = UiNode::new(UiNodeKind::Button, "stale")
            .stable_node_id("stale-node")
            .interaction(crate::render_model::UiInteractionState {
                hovered: true,
                ..crate::render_model::UiInteractionState::default()
            });
        let tree = UiTree::new(UiNode::new(UiNodeKind::Row, "").child(stale));

        let hovered = tree.with_hovered_node_id(Some(&"missing-node".into()));

        assert!(!hovered.root().children()[0].props().interaction.hovered);
    }

    #[test]
    fn with_hover_surface_for_node_id_wraps_matching_node_with_same_geometry() {
        let target = UiNode::new(UiNodeKind::Text, "target")
            .stable_node_id("target-node")
            .width(UiDimension::px(240))
            .height(UiDimension::px(32));
        let tree = UiTree::new(UiNode::new(UiNodeKind::Column, "").child(target));

        let hovered = tree.with_hover_surface_for_node_id(Some(&"target-node".into()));
        let wrapper = &hovered.root().children()[0];

        assert_eq!(UiVisualRole::HoverSurface, wrapper.props().visual_role);
        assert_eq!(UiDimension::Px(240), wrapper.props().common.width);
        assert_eq!(UiDimension::Px(32), wrapper.props().common.height);
        assert_eq!("target-node", wrapper.children()[0].id().as_str());
    }

    #[test]
    fn with_hover_surface_for_node_id_prefers_semantic_wrapper_geometry() {
        let target = UiNode::new(UiNodeKind::Text, "target").stable_node_id("inner-node");
        let semantic_wrapper = UiNode::new(UiNodeKind::Stack, "")
            .common(
                UiCommonProps::default()
                    .semantic_node_id("semantic-node")
                    .width(UiDimension::px(240))
                    .height(UiDimension::px(40)),
            )
            .child(target);
        let tree = UiTree::new(UiNode::new(UiNodeKind::Column, "").child(semantic_wrapper));

        let hovered = tree.with_hover_surface_for_node_id(Some(&"semantic-node".into()));
        let wrapper = &hovered.root().children()[0];

        assert_eq!(UiVisualRole::HoverSurface, wrapper.props().visual_role);
        assert_eq!(UiDimension::Px(240), wrapper.props().common.width);
        assert_eq!(UiDimension::Px(40), wrapper.props().common.height);
        assert_eq!("semantic-node", wrapper.props().common.semantic_node_id);
        assert_eq!(UiNodeKind::Stack, wrapper.children()[0].kind());
        assert_eq!(
            "inner-node",
            wrapper.children()[0].children()[0].id().as_str()
        );
    }

    #[test]
    fn with_animation_phase_marks_loading_descendants() {
        let spinner = UiNode::new(UiNodeKind::Spinner, "loading");
        let tree = UiTree::new(UiNode::new(UiNodeKind::Row, "").child(spinner));

        let animated = tree.with_animation_phase(3);

        assert_eq!(3, animated.root().props().interaction.animation_phase);
        assert_eq!(
            3,
            animated.root().children()[0]
                .props()
                .interaction
                .animation_phase
        );
    }

    #[test]
    fn with_scroll_area_offset_y_marks_scroll_area_descendants() {
        let scroll = UiNode::new(UiNodeKind::ScrollArea, "document");
        let tree = UiTree::new(UiNode::new(UiNodeKind::Stack, "").child(scroll));

        let scrolled = tree.with_scroll_area_offset_y(240);

        assert_eq!(
            240,
            scrolled.root().children()[0].props().scroll_area.offset_y
        );
    }
}
