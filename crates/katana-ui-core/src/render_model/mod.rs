mod command;
mod identity;
mod kind;
mod tree;

pub use command::{RenderContext, UiCommand, UiTreeDiff};
pub use identity::{UiNodeId, UiStateId};
pub use kind::UiNodeKind;
pub use tree::{UiInteractionState, UiNode, UiProps, UiTree};

#[cfg(test)]
mod tests {
    use super::{UiNode, UiNodeKind, UiTree};

    #[test]
    fn tree_keeps_children_order() {
        let tree = UiTree::new(
            UiNode::new(UiNodeKind::Row, "row")
                .child(UiNode::new(UiNodeKind::Text, "a"))
                .child(UiNode::new(UiNodeKind::Text, "b")),
        );
        assert_eq!(2, tree.root().children().len());
    }

    #[test]
    fn duplicate_components_get_unique_state_ids() {
        let tree = UiTree::new(
            UiNode::new(UiNodeKind::Row, "row")
                .child(UiNode::new(UiNodeKind::Button, "Save"))
                .child(UiNode::new(UiNodeKind::Button, "Save")),
        );
        let first = &tree.root().children()[0];
        let second = &tree.root().children()[1];

        assert_ne!(first.id(), second.id());
        assert_ne!(first.props().state_id, second.props().state_id);
    }
}
