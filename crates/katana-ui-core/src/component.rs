use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{UiNode, UiTree};

pub trait Component: Sized {
    fn render(self) -> UiNode;

    #[must_use]
    fn class(self, name: impl Into<String>) -> StyledComponent<Self> {
        StyledComponent::new(self).class(name)
    }
}

pub trait ComponentAction {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult;
}

impl<T> Component for T
where
    T: Into<UiNode>,
{
    fn render(self) -> UiNode {
        self.into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyledComponent<C> {
    component: C,
    classes: Vec<String>,
}

impl<C> StyledComponent<C> {
    #[must_use]
    pub fn new(component: C) -> Self {
        Self {
            component,
            classes: Vec::new(),
        }
    }

    #[must_use]
    pub fn class(mut self, name: impl Into<String>) -> Self {
        self.classes.push(name.into());
        self
    }
}

impl<C> From<StyledComponent<C>> for UiNode
where
    C: Component,
{
    fn from(value: StyledComponent<C>) -> Self {
        let mut node = value.component.render();
        for class in value.classes {
            node = node.style_class(class);
        }
        node
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentTree {
    tree: UiTree,
}

impl ComponentTree {
    #[must_use]
    pub fn new(root: impl Component) -> Self {
        Self {
            tree: UiTree::new(root.render()),
        }
    }

    #[must_use]
    pub fn into_tree(self) -> UiTree {
        self.tree
    }
}
