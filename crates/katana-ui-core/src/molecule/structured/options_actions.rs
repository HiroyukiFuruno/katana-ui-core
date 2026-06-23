use super::items::{ArrayEditorItem, CommandItem, TreeNode};
use super::model::{CommandPalette, DynamicArrayEditor, TreeView};
use crate::render_model::UiStateId;

macro_rules! structured_action_options {
    ($name:ident, $item:ty) => {
        impl $name {
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
    };
}

structured_action_options!(TreeView, TreeNode);
structured_action_options!(CommandPalette, CommandItem);
structured_action_options!(DynamicArrayEditor, ArrayEditorItem);
