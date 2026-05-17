use super::items::{ArrayEditorItem, CommandItem, TreeNode};
use super::types::StructuredTypedModel;
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::molecule::state::MoleculeState;
use crate::render_model::{UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

macro_rules! structured_molecule {
    ($name:ident, $item:ty, $kind:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            label: String,
            state: MoleculeState,
            items: Vec<$item>,
            pub(super) model: StructuredTypedModel,
            children: Vec<UiNode>,
        }

        impl $name {
            #[must_use]
            pub fn new(label: impl Into<String>) -> Self {
                Self {
                    label: label.into(),
                    state: MoleculeState::new($kind),
                    items: Vec::new(),
                    model: StructuredTypedModel::default(),
                    children: Vec::new(),
                }
            }

            #[must_use]
            pub fn item(mut self, item: $item) -> Self {
                self.items.push(item);
                self.state.item_count = self.items.len();
                self
            }

            #[must_use]
            pub fn child(mut self, child: impl Into<UiNode>) -> Self {
                self.children.push(child.into());
                self
            }

            #[must_use]
            pub fn open(mut self, value: bool) -> Self {
                self.state.open = value;
                self
            }

            #[must_use]
            pub fn selected_index(mut self, value: usize) -> Self {
                self.state.has_selection = true;
                self.state.selected_index = value;
                self
            }

            #[must_use]
            pub fn item_count(mut self, value: usize) -> Self {
                self.state.item_count = value;
                self
            }

            #[must_use]
            pub fn value(mut self, value: impl Into<String>) -> Self {
                self.state.value = value.into();
                self
            }
        }

        impl $name {
            #[must_use]
            pub fn active(mut self, value: impl Into<String>) -> Self {
                self.model.active_id = value.into();
                self
            }

            #[must_use]
            pub fn line_display(mut self, value: bool) -> Self {
                self.model.line_display = value;
                self
            }

            #[must_use]
            pub fn query(mut self, value: impl Into<String>) -> Self {
                self.model.query = value.into();
                self
            }

            #[must_use]
            pub fn filtered_action(mut self, value: CommandItem) -> Self {
                self.model.filtered_actions.push(value);
                self
            }

            #[must_use]
            pub fn keyboard_action(mut self, value: impl Into<String>) -> Self {
                self.model.keyboard_action = value.into();
                self
            }

            #[must_use]
            pub fn add_action(mut self, value: impl Into<String>) -> Self {
                self.model.add_action = value.into();
                self
            }

            #[must_use]
            pub fn delete_action(mut self, value: impl Into<String>) -> Self {
                self.model.delete_action = value.into();
                self
            }
        }

        impl $name {
            #[must_use]
            pub fn reorder_action(mut self, value: impl Into<String>) -> Self {
                self.model.reorder_action = value.into();
                self
            }

            #[must_use]
            pub fn edit_action(mut self, value: impl Into<String>) -> Self {
                self.model.edit_action = value.into();
                self
            }

            #[must_use]
            pub fn empty_state(mut self, value: impl Into<String>) -> Self {
                self.model.empty_state = value.into();
                self
            }

            #[must_use]
            pub fn items(&self) -> &[$item] {
                &self.items
            }

            #[must_use]
            pub fn state_id(&self) -> &UiStateId {
                &self.state.state_id
            }
        }

        impl ComponentAction for $name {
            fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
                self.state.apply_action(action, false)
            }
        }

        impl From<$name> for UiNode {
            fn from(value: $name) -> Self {
                let mut node = value.state.node($kind, value.label);
                for child in value.children {
                    node = node.child(child);
                }
                node
            }
        }
    };
}

structured_molecule!(TreeView, TreeNode, UiNodeKind::TreeView);
structured_molecule!(CommandPalette, CommandItem, UiNodeKind::CommandPalette);
structured_molecule!(
    DynamicArrayEditor,
    ArrayEditorItem,
    UiNodeKind::DynamicArrayEditor
);
