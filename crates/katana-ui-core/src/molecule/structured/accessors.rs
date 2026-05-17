use super::CommandItem;
use super::model::{CommandPalette, DynamicArrayEditor, TreeView};

macro_rules! structured_accessors {
    ($name:ident) => {
        impl $name {
            #[must_use]
            pub fn active_model(&self) -> &str {
                &self.model.active_id
            }

            #[must_use]
            pub fn line_display_model(&self) -> bool {
                self.model.line_display
            }

            #[must_use]
            pub fn query_model(&self) -> &str {
                &self.model.query
            }

            #[must_use]
            pub fn filtered_actions(&self) -> &[CommandItem] {
                &self.model.filtered_actions
            }

            #[must_use]
            pub fn keyboard_action_model(&self) -> &str {
                &self.model.keyboard_action
            }

            #[must_use]
            pub fn add_action_model(&self) -> &str {
                &self.model.add_action
            }

            #[must_use]
            pub fn delete_action_model(&self) -> &str {
                &self.model.delete_action
            }

            #[must_use]
            pub fn reorder_action_model(&self) -> &str {
                &self.model.reorder_action
            }

            #[must_use]
            pub fn edit_action_model(&self) -> &str {
                &self.model.edit_action
            }

            #[must_use]
            pub fn empty_state_model(&self) -> &str {
                &self.model.empty_state
            }
        }
    };
}

structured_accessors!(CommandPalette);
structured_accessors!(DynamicArrayEditor);
structured_accessors!(TreeView);
