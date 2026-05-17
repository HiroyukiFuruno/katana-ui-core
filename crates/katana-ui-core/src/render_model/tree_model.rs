use super::UiNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
}
