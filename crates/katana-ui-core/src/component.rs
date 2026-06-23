use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{UiNode, UiTree};
use crate::state::{UiComponentState, UiStateHandle};

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

pub trait ComponentStateBinding {
    fn state_snapshot(&self) -> UiComponentState;

    fn set_state_snapshot(&mut self, state: UiComponentState);

    fn state_handle(&self) -> UiStateHandle<UiComponentState> {
        UiStateHandle::new(self.state_snapshot())
    }

    fn sync_state(&mut self, state_handle: &UiStateHandle<UiComponentState>) {
        self.set_state_snapshot(state_handle.get());
    }

    fn update_state(&mut self, update_state: impl FnOnce(&mut UiComponentState)) {
        let mut state = self.state_snapshot();
        update_state(&mut state);
        self.set_state_snapshot(state);
    }
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

#[derive(Debug, Clone, PartialEq)]
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
