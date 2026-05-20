use super::model::Accordion;
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{UiDisclosureProps, UiInteractionState, UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccordionGroupItem {
    id: String,
    label: String,
    open: bool,
}

impl AccordionGroupItem {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            open: false,
        }
    }

    #[must_use]
    pub fn open(mut self, value: bool) -> Self {
        self.open = value;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccordionGroup {
    label: String,
    state_id: UiStateId,
    multiple: bool,
    items: Vec<AccordionGroupItem>,
}

impl AccordionGroup {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::Accordion),
            multiple: false,
            items: Vec::new(),
        }
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn multiple(mut self, value: bool) -> Self {
        self.multiple = value;
        self
    }

    #[must_use]
    pub fn item(mut self, item: AccordionGroupItem) -> Self {
        self.items.push(item);
        self
    }

    #[must_use]
    pub fn open_item_ids(&self) -> Vec<&str> {
        self.items
            .iter()
            .filter(|it| it.open)
            .map(|it| it.id.as_str())
            .collect()
    }

    fn interaction(&self) -> UiInteractionState {
        UiInteractionState {
            item_count: self.items.len(),
            selected_index: self.items.iter().position(|it| it.open).unwrap_or_default(),
            has_selection: self.items.iter().any(|it| it.open),
            value: self.open_item_ids().join(","),
            ..UiInteractionState::default()
        }
    }
}

impl ComponentAction for AccordionGroup {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = self.interaction();
        if action.target() != &self.state_id {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        let UiAction::SetSelectedIndex { selected_index, .. } = action else {
            return UiActionResult::ignored(self.state_id.clone(), before);
        };
        let Some(target) = self.items.get(*selected_index).cloned() else {
            return UiActionResult::ignored(self.state_id.clone(), before);
        };
        let closed = self.toggle_item(*selected_index);
        let mut after = self.interaction();
        after.value = format!("opened={} closed={}", target.id, closed.join(","));
        UiActionResult::handled(self.state_id.clone(), action, before, after)
    }
}

impl AccordionGroup {
    fn toggle_item(&mut self, selected_index: usize) -> Vec<String> {
        if self.multiple {
            if let Some(item) = self.items.get_mut(selected_index) {
                item.open = !item.open;
            }
            return Vec::new();
        }
        let mut closed = Vec::new();
        for (index, item) in self.items.iter_mut().enumerate() {
            if index == selected_index {
                item.open = true;
            } else if item.open {
                item.open = false;
                closed.push(item.id.clone());
            }
        }
        closed
    }
}

impl From<AccordionGroup> for UiNode {
    fn from(value: AccordionGroup) -> Self {
        let interaction = value.interaction();
        let disclosure = UiDisclosureProps {
            multiple: value.multiple,
            ..UiDisclosureProps::default()
        };
        value.items.into_iter().fold(
            UiNode::from_state(UiNodeKind::Accordion, value.label, value.state_id)
                .interaction(interaction)
                .disclosure(disclosure)
                .style_class("AccordionGroup"),
            |node, item| node.child(Accordion::new(item.label).open(item.open).value(item.id)),
        )
    }
}
