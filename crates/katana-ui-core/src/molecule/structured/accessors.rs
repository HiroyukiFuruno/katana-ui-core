use super::CommandItem;
use super::model::{CommandPalette, DynamicArrayEditor, TreeView};
use super::types::TreeLineStyle;
use crate::molecule::DisclosureTriggerArea;

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
            pub fn line_style_model(&self) -> TreeLineStyle {
                self.model.line_style
            }

            #[must_use]
            pub fn line_width_model(&self) -> u8 {
                self.model.line_width
            }

            #[must_use]
            pub fn icons_visible_model(&self) -> bool {
                self.model.icons_visible
            }

            #[must_use]
            pub fn directory_icon_model(&self) -> &str {
                &self.model.directory_icon
            }

            #[must_use]
            pub fn file_icon_model(&self) -> &str {
                &self.model.file_icon
            }

            #[must_use]
            pub fn tree_font_role_model(&self) -> &str {
                &self.model.font_role
            }

            #[must_use]
            pub fn tree_theme_id_model(&self) -> &str {
                &self.model.theme_id
            }

            #[must_use]
            pub fn empty_area_context_menu_model(&self) -> bool {
                self.model.empty_area_context_menu
            }

            #[must_use]
            pub fn default_open_model(&self) -> bool {
                self.model.default_open
            }

            #[must_use]
            pub fn toggle_icon_model(&self) -> &str {
                &self.model.toggle_icon
            }

            #[must_use]
            pub fn toggle_trigger_area_model(&self) -> DisclosureTriggerArea {
                self.model.toggle_trigger_area
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
