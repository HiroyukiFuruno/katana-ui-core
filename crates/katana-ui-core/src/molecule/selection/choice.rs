use super::types::{ChoiceItem, SelectionTypedModel};
use crate::molecule::state::MoleculeState;
use crate::render_model::{UiNode, UiNodeKind};
use serde::{Deserialize, Serialize};

macro_rules! choice_molecule {
    ($name:ident, $kind:expr, $close_on_select:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            pub(super) label: String,
            pub(super) state: MoleculeState,
            pub(super) items: Vec<ChoiceItem>,
            pub(super) model: SelectionTypedModel,
            pub(super) children: Vec<UiNode>,
        }

        impl $name {
            #[must_use]
            pub fn new(label: impl Into<String>) -> Self {
                Self {
                    label: label.into(),
                    state: MoleculeState::new($kind),
                    items: Vec::new(),
                    model: SelectionTypedModel::default(),
                    children: Vec::new(),
                }
            }

            #[must_use]
            pub fn item(mut self, item: ChoiceItem) -> Self {
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
                self.model.selected_option = self.items.get(value).cloned();
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

            #[must_use]
            pub fn placeholder(mut self, value: impl Into<String>) -> Self {
                self.state.placeholder = value.into();
                self
            }
        }

        impl $name {
            #[must_use]
            pub fn disabled(mut self, value: bool) -> Self {
                self.state.disabled = value;
                self
            }

            #[must_use]
            pub fn readonly(mut self, value: bool) -> Self {
                self.state.readonly = value;
                self
            }

            #[must_use]
            pub fn input_value(mut self, value: impl Into<String>) -> Self {
                self.model.input_value = value.into();
                self
            }

            #[must_use]
            pub fn filter_result(mut self, item: ChoiceItem) -> Self {
                self.model.filter_results.push(item);
                self
            }

            #[must_use]
            pub fn free_input(mut self, value: bool) -> Self {
                self.model.free_input = value;
                self
            }

            #[must_use]
            pub fn keyboard_navigation(mut self, value: impl Into<String>) -> Self {
                self.model.keyboard_navigation_summary = value.into();
                self
            }

            #[must_use]
            pub fn placement(mut self, value: impl Into<String>) -> Self {
                self.model.placement = value.into();
                self
            }

            #[must_use]
            pub fn highlighted_index(mut self, value: usize) -> Self {
                self.model.highlighted_index = value;
                self
            }

            #[must_use]
            pub fn long_list(mut self, value: bool) -> Self {
                self.model.long_list = value;
                self
            }

            #[must_use]
            pub fn outside_click_dismiss(mut self, value: bool) -> Self {
                self.model.outside_click_dismiss = value;
                self
            }

            #[must_use]
            pub fn framed(mut self, value: bool) -> Self {
                self.model.framed = value;
                self
            }

            #[must_use]
            pub fn trigger_summary(mut self, value: impl Into<String>) -> Self {
                self.model.trigger_summary = value.into();
                self
            }

            #[must_use]
            pub fn select_action(mut self, value: impl Into<String>) -> Self {
                self.model.select_action = value.into();
                self
            }

            #[must_use]
            pub fn crumb_action(mut self, value: impl Into<String>) -> Self {
                self.model.crumb_action = value.into();
                self
            }
        }
    };
}

choice_molecule!(SelectBox, UiNodeKind::SelectBox, true);
choice_molecule!(ComboBox, UiNodeKind::ComboBox, true);
choice_molecule!(MenuButton, UiNodeKind::MenuButton, true);
choice_molecule!(SelectionList, UiNodeKind::SelectionList, false);
choice_molecule!(SideMenu, UiNodeKind::SideMenu, false);
choice_molecule!(Tabs, UiNodeKind::Tabs, false);
choice_molecule!(Breadcrumb, UiNodeKind::Breadcrumb, false);
