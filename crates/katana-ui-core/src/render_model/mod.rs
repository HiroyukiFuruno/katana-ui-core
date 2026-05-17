mod command;
mod identity;
mod kind;
mod props;
mod tree;
mod tree_model;
mod typed;

pub use command::{RenderContext, UiCommand, UiTreeDiff};
pub use identity::{UiNodeId, UiStateId};
pub use kind::UiNodeKind;
pub use props::{UiInteractionState, UiProps, UiSize, UiTone, UiVariant, UiVisualRole};
pub use tree::UiNode;
pub use tree_model::UiTree;
pub use typed::{
    UiAnimationState, UiClearActionSpec, UiDismissAction, UiLoadingProps, UiProgressMode,
    UiSlotPlacement, UiSlotSpec, UiStatusProps, UiTextEntryProps,
};

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
