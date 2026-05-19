use super::choice::{Breadcrumb, ComboBox, MenuButton, SelectBox, SelectionList, SideMenu, Tabs};
use super::types::ChoiceItem;

macro_rules! selection_accessors {
    ($name:ident) => {
        impl $name {
            #[must_use]
            pub fn input_model(&self) -> &str {
                &self.model.input_value
            }

            #[must_use]
            pub fn filter_results(&self) -> &[ChoiceItem] {
                &self.model.filter_results
            }

            #[must_use]
            pub fn allows_free_input(&self) -> bool {
                self.model.free_input
            }

            #[must_use]
            pub fn selected_option(&self) -> Option<&ChoiceItem> {
                self.model.selected_option.as_ref()
            }

            #[must_use]
            pub fn keyboard_navigation_summary(&self) -> &str {
                &self.model.keyboard_navigation_summary
            }

            #[must_use]
            pub fn placement_model(&self) -> &str {
                &self.model.placement
            }

            #[must_use]
            pub fn highlighted_index_model(&self) -> usize {
                self.model.highlighted_index
            }

            #[must_use]
            pub fn is_long_list(&self) -> bool {
                self.model.long_list
            }

            #[must_use]
            pub fn dismisses_on_outside_click(&self) -> bool {
                self.model.outside_click_dismiss
            }

            #[must_use]
            pub fn framed_model(&self) -> bool {
                self.model.framed
            }

            #[must_use]
            pub fn trigger_model(&self) -> &str {
                &self.model.trigger_summary
            }

            #[must_use]
            pub fn select_action_model(&self) -> &str {
                &self.model.select_action
            }

            #[must_use]
            pub fn crumb_action_model(&self) -> &str {
                &self.model.crumb_action
            }

            #[must_use]
            pub fn icon_action_model(&self) -> &str {
                &self.model.icon_action
            }

            #[must_use]
            pub fn hover_expansion_model(&self) -> bool {
                self.model.hover_expansion
            }

            #[must_use]
            pub fn section_model(&self) -> &str {
                &self.model.section
            }

            #[must_use]
            pub fn marker_model(&self) -> &str {
                &self.model.marker
            }

            #[must_use]
            pub fn has_more_row(&self) -> bool {
                self.model.more_row
            }
        }
    };
}

selection_accessors!(Breadcrumb);
selection_accessors!(ComboBox);
selection_accessors!(MenuButton);
selection_accessors!(SelectBox);
selection_accessors!(SelectionList);
selection_accessors!(SideMenu);
selection_accessors!(Tabs);
