use super::SplitPane;
use crate::render_model::{UiNode, UiNodeKind};

impl SplitPane {
    #[must_use]
    pub fn first(mut self, child: impl Into<UiNode>) -> Self {
        self.set_slot(0, child.into());
        self
    }

    #[must_use]
    pub fn second(mut self, child: impl Into<UiNode>) -> Self {
        self.set_slot(1, child.into());
        self
    }

    fn set_slot(&mut self, index: usize, child: UiNode) {
        while self.children.len() <= index {
            self.children
                .push(UiNode::new(UiNodeKind::Spacer, "EmptyPane"));
        }
        self.children[index] = child;
    }
}
