mod scroll_area;
mod split_pane;
mod split_pane_actions;
mod split_pane_contract;
mod split_pane_event_actions;
mod split_pane_ratio;
mod split_pane_slots;
mod types;

use crate::render_model::{
    UiCommonProps, UiDisplay, UiInteractionState, UiNode, UiNodeKind, UiStateId,
};
pub use scroll_area::{
    ScrollArea, ScrollAreaAction, ScrollAreaEvent, ScrollAxis, ScrollEdge, ScrollRejectionReason,
    ScrollbarPlacement, ScrollbarVisibility,
};
pub use split_pane::{SplitPane, SplitPaneAxis, SplitPaneResizeMode};
pub use split_pane_contract::{
    SplitPaneAction, SplitPaneEvent, SplitPaneOptions, SplitPaneRejectionReason,
    SplitPaneResizeSource,
};
pub use types::{AlignHorizontal, AlignVertical, Alignment, EdgeInsets, Length, SizePolicy};

macro_rules! layout_model {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            state_id: UiStateId,
            children: Vec<UiNode>,
            gap: Length,
            alignment: Alignment,
            interaction: UiInteractionState,
        }

        impl $name {
            #[must_use]
            pub fn new() -> Self {
                Self {
                    state_id: UiStateId::next_for($kind),
                    children: Vec::new(),
                    gap: Length::Px(0.0),
                    alignment: Alignment::Start,
                    interaction: UiInteractionState::default(),
                }
            }

            #[must_use]
            pub fn child(mut self, child: impl Into<UiNode>) -> Self {
                self.children.push(child.into());
                self
            }

            #[must_use]
            pub fn gap(mut self, gap: Length) -> Self {
                self.gap = gap;
                self
            }

            #[must_use]
            pub fn align(mut self, alignment: Alignment) -> Self {
                self.alignment = alignment;
                self
            }

            #[must_use]
            pub fn value(mut self, value: impl Into<String>) -> Self {
                self.interaction.value = value.into();
                self
            }

            #[must_use]
            pub fn children(&self) -> &[UiNode] {
                &self.children
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl From<$name> for UiNode {
            fn from(value: $name) -> Self {
                let mut node = UiNode::from_state($kind, stringify!($name), value.state_id)
                    .interaction(value.interaction);
                for child in value.children {
                    node = node.child(child);
                }
                node
            }
        }
    };
}

layout_model!(Row, UiNodeKind::Row);
layout_model!(Column, UiNodeKind::Column);
layout_model!(Stack, UiNodeKind::Stack);
layout_model!(Grid, UiNodeKind::Grid);
layout_model!(AlignCenter, UiNodeKind::AlignCenter);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlignNode {
    state_id: UiStateId,
    horizontal: AlignHorizontal,
    vertical: AlignVertical,
    children: Vec<UiNode>,
}

impl AlignNode {
    #[must_use]
    pub fn new(horizontal: AlignHorizontal, vertical: AlignVertical) -> Self {
        Self {
            state_id: UiStateId::next_for(UiNodeKind::AlignNode),
            horizontal,
            vertical,
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn center() -> Self {
        Self::new(AlignHorizontal::Center, AlignVertical::Center)
    }

    #[must_use]
    pub fn left_center() -> Self {
        Self::new(AlignHorizontal::Start, AlignVertical::Center)
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }
}

impl From<AlignNode> for UiNode {
    fn from(value: AlignNode) -> Self {
        let common = UiCommonProps::default()
            .display(UiDisplay::Flex)
            .align_items(value.vertical.to_items())
            .justify_content(value.horizontal.to_justify());
        let mut node =
            UiNode::from_state(UiNodeKind::AlignNode, "AlignNode", value.state_id).common(common);
        for child in value.children {
            node = node.child(child);
        }
        node
    }
}

#[cfg(test)]
mod tests {
    use super::{AlignNode, Column, Length, Row};
    use crate::atom::Text;
    use crate::render_model::{UiAlignItems, UiJustifyContent, UiNodeKind, UiTree};

    #[test]
    fn layout_serializes_to_tree_shape() {
        let row = Row::new().gap(Length::px(8.0)).child(Text::new("A"));
        let tree = UiTree::new(Column::new().child(row));
        assert_eq!(1, tree.root().children().len());
    }

    #[test]
    fn align_node_maps_to_common_layout_contract() {
        let tree = UiTree::new(AlignNode::left_center().child(Text::new("Label")));

        assert_eq!(UiNodeKind::AlignNode, tree.root().kind());
        assert_eq!(UiAlignItems::Center, tree.root().props().common.align_items);
        assert_eq!(
            UiJustifyContent::Start,
            tree.root().props().common.justify_content
        );
        assert_eq!(1, tree.root().children().len());
    }
}
