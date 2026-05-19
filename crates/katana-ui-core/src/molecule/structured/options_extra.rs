use super::items::CommandItem;
use super::model::{CommandPalette, DynamicArrayEditor, TreeView};
use crate::molecule::DisclosureTriggerArea;

macro_rules! structured_extra_options {
    ($name:ident) => {
        impl $name {
            #[must_use]
            pub fn tree_theme_id(mut self, value: impl Into<String>) -> Self {
                self.model.theme_id = value.into();
                self
            }

            #[must_use]
            pub fn empty_area_context_menu(mut self, value: bool) -> Self {
                self.model.empty_area_context_menu = value;
                self
            }

            #[must_use]
            pub fn default_open(mut self, value: bool) -> Self {
                self.model.default_open = value;
                self.state.open = value;
                self
            }

            #[must_use]
            pub fn toggle_icon(mut self, value: impl Into<String>) -> Self {
                self.model.toggle_icon = value.into();
                self
            }

            #[must_use]
            pub fn toggle_trigger_area(mut self, value: DisclosureTriggerArea) -> Self {
                self.model.toggle_trigger_area = value;
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
        }
    };
}

structured_extra_options!(TreeView);
structured_extra_options!(CommandPalette);
structured_extra_options!(DynamicArrayEditor);
