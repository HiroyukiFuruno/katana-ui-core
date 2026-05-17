use crate::render_model::{UiInteractionState, UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Length {
    Px(f32),
    Fill,
    Fit,
}

impl Length {
    #[must_use]
    pub const fn px(value: f32) -> Self {
        Self::Px(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EdgeInsets {
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SizePolicy {
    pub width: Length,
    pub height: Length,
}

macro_rules! layout_model {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
layout_model!(ScrollArea, UiNodeKind::ScrollArea);
layout_model!(SplitPane, UiNodeKind::SplitPane);
layout_model!(AlignCenter, UiNodeKind::AlignCenter);

#[cfg(test)]
mod tests {
    use super::{Column, Length, Row};
    use crate::atom::Text;
    use crate::render_model::UiTree;

    #[test]
    fn layout_serializes_to_tree_shape() {
        let row = Row::new().gap(Length::px(8.0)).child(Text::new("A"));
        let tree = UiTree::new(Column::new().child(row));
        assert_eq!(1, tree.root().children().len());
    }
}
