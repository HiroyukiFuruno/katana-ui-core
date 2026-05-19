mod button_layout;
mod command;
mod common;
mod common_builder;
mod common_types;
mod context_menu_props;
mod identity;
mod kind;
mod props;
mod tree;
mod tree_builder;
mod tree_model;
mod tree_props;
mod typed;

pub use button_layout::{
    UiButtonLayoutDto, UiButtonLayoutPatchDto, UiButtonLayoutPreset, UiButtonLayoutSpec,
};
pub use command::{RenderContext, UiCommand, UiTreeDiff};
pub use common::UiCommonProps;
pub use common_types::{
    UiAlignItems, UiBorder, UiCursor, UiDimension, UiDisplay, UiEdgeInsets, UiJustifyContent,
    UiPointerEvents, UiPosition, UiZIndex,
};
pub use context_menu_props::{
    UiContextMenuAnchor, UiContextMenuItem, UiContextMenuItemKind, UiContextMenuPlacement,
    UiContextMenuProps, UiContextMenuRect,
};
pub use identity::{UiNodeId, UiStateId};
pub use kind::UiNodeKind;
pub use props::{UiInteractionState, UiProps, UiSize, UiTone, UiVariant, UiVisualRole};
pub use tree::UiNode;
pub use tree_model::UiTree;
pub use tree_props::{
    UiTreeLineStyle, UiTreeNodeKind, UiTreeNodeProps, UiTreeProps, UiTreeToggleTriggerArea,
};
pub use typed::{
    UiAnimationState, UiButtonProps, UiClearActionSpec, UiColorSwatchProps, UiDismissAction,
    UiIconProps, UiLoadingProps, UiPanelProps, UiProgressMode, UiRect, UiScrollbarDragState,
    UiScrollbarModel, UiScrollbarPlacement, UiScrollbarVisibility, UiShortcutProps,
    UiSlotPlacement, UiSlotSpec, UiStatusProps, UiSvgPaintPolicy, UiTextEntryProps, UiTextProps,
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
