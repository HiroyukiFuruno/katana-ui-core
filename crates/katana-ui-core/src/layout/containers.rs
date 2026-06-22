use super::{AlignHorizontal, AlignVertical, Alignment, LayoutAxis, Length, OverflowBehavior};
use crate::render_model::{
    UiCommonProps, UiDisplay, UiInteractionState, UiNode, UiNodeKind, UiStateId,
};

macro_rules! layout_model {
    ($name:ident, $kind:expr, $axis:expr, $display:expr) => {
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            state_id: UiStateId,
            children: Vec<UiNode>,
            gap: Length,
            alignment: Alignment,
            axis: LayoutAxis,
            overflow: OverflowBehavior,
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
                    axis: $axis,
                    overflow: OverflowBehavior::Fit,
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
            pub fn axis(mut self, axis: LayoutAxis) -> Self {
                self.axis = axis;
                self
            }

            #[must_use]
            pub fn overflow(mut self, overflow: OverflowBehavior) -> Self {
                self.overflow = overflow;
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
                    .common(layout_common(
                        $display,
                        value.axis,
                        value.gap,
                        value.alignment,
                        value.overflow,
                    ))
                    .interaction(value.interaction);
                for child in value.children {
                    node = node.child(child);
                }
                node
            }
        }
    };
}

layout_model!(
    Row,
    UiNodeKind::Row,
    LayoutAxis::Horizontal,
    UiDisplay::Flex
);
layout_model!(
    Column,
    UiNodeKind::Column,
    LayoutAxis::Vertical,
    UiDisplay::Flex
);
layout_model!(
    Stack,
    UiNodeKind::Stack,
    LayoutAxis::Overlay,
    UiDisplay::Flex
);
layout_model!(Grid, UiNodeKind::Grid, LayoutAxis::Both, UiDisplay::Grid);
layout_model!(
    AlignCenter,
    UiNodeKind::AlignCenter,
    LayoutAxis::Both,
    UiDisplay::Flex
);

fn layout_common(
    display: UiDisplay,
    axis: LayoutAxis,
    gap: Length,
    alignment: Alignment,
    overflow: OverflowBehavior,
) -> UiCommonProps {
    UiCommonProps::default()
        .display(display)
        .layout_axis(axis.into())
        .gap(gap.into())
        .overflow(overflow.into())
        .align_items(alignment.into())
        .justify_content(alignment.into())
}

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
