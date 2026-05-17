use super::types::DisclosureTypedModel;
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::molecule::state::MoleculeState;
use crate::render_model::{UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

macro_rules! disclosure_molecule {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            label: String,
            pub(super) state: MoleculeState,
            pub(super) model: DisclosureTypedModel,
            children: Vec<UiNode>,
        }

        impl $name {
            #[must_use]
            pub fn new(label: impl Into<String>) -> Self {
                Self {
                    label: label.into(),
                    state: MoleculeState::new($kind),
                    model: DisclosureTypedModel::default(),
                    children: Vec::new(),
                }
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

            #[must_use]
            pub fn disabled(mut self, value: bool) -> Self {
                self.state.disabled = value;
                self
            }

            #[must_use]
            pub fn state_id(&self) -> &UiStateId {
                &self.state.state_id
            }
        }

        impl $name {
            #[must_use]
            pub fn placement(mut self, value: impl Into<String>) -> Self {
                self.model.placement = value.into();
                self
            }

            #[must_use]
            pub fn offset(mut self, x: i16, y: i16) -> Self {
                self.model.offset = (x, y);
                self
            }

            #[must_use]
            pub fn outside_click_dismiss(mut self, value: bool) -> Self {
                self.model.outside_click_dismiss = value;
                self
            }

            #[must_use]
            pub fn escape_dismiss(mut self, value: bool) -> Self {
                self.model.escape_dismiss = value;
                self
            }

            #[must_use]
            pub fn anchor_summary(mut self, value: impl Into<String>) -> Self {
                self.model.anchor_summary = value.into();
                self
            }

            #[must_use]
            pub fn backdrop(mut self, value: impl Into<String>) -> Self {
                self.model.backdrop = value.into();
                self
            }

            #[must_use]
            pub fn focus_return(mut self, value: impl Into<String>) -> Self {
                self.model.focus_return = value.into();
                self
            }

            #[must_use]
            pub fn dismiss_policy(mut self, value: impl Into<String>) -> Self {
                self.model.dismiss_policy = value.into();
                self
            }
        }

        impl $name {
            #[must_use]
            pub fn controlled(mut self, value: bool) -> Self {
                self.model.controlled = value;
                self
            }

            #[must_use]
            pub fn multiple(mut self, value: bool) -> Self {
                self.model.multiple = value;
                self
            }

            #[must_use]
            pub fn indicator_position(mut self, value: impl Into<String>) -> Self {
                self.model.indicator_position = value.into();
                self
            }

            #[must_use]
            pub fn tree_mode(mut self, value: bool) -> Self {
                self.model.tree_mode = value;
                self
            }

            #[must_use]
            pub fn range(mut self, minimum: i32, maximum: i32, step: i32) -> Self {
                self.model.minimum = minimum;
                self.model.maximum = maximum;
                self.model.step = step;
                self
            }

            #[must_use]
            pub fn binding(mut self, value: impl Into<String>) -> Self {
                self.model.binding = value.into();
                self
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

disclosure_molecule!(Accordion, UiNodeKind::Accordion);
disclosure_molecule!(Modal, UiNodeKind::Modal);
disclosure_molecule!(ModalOverlay, UiNodeKind::ModalOverlay);
disclosure_molecule!(Popover, UiNodeKind::Popover);
disclosure_molecule!(Tooltip, UiNodeKind::Tooltip);
disclosure_molecule!(NotificationToast, UiNodeKind::NotificationToast);
disclosure_molecule!(SearchBox, UiNodeKind::SearchBox);
disclosure_molecule!(SegmentedToggle, UiNodeKind::SegmentedToggle);
disclosure_molecule!(SlideControl, UiNodeKind::SlideControl);
