use crate::render_model::{UiInteractionState, UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ComplexMoleculeState {
    state_id: UiStateId,
    open: bool,
    has_selection: bool,
    selected_index: usize,
    item_count: usize,
    value: String,
}

impl ComplexMoleculeState {
    fn new(kind: UiNodeKind) -> Self {
        Self {
            state_id: UiStateId::next_for(kind),
            open: false,
            has_selection: false,
            selected_index: 0,
            item_count: 0,
            value: String::new(),
        }
    }

    fn interaction(&self) -> UiInteractionState {
        UiInteractionState {
            open: self.open,
            has_selection: self.has_selection,
            selected_index: self.selected_index,
            item_count: self.item_count,
            value: self.value.clone(),
        }
    }
}

macro_rules! complex_molecule_model {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            label: String,
            state: ComplexMoleculeState,
            children: Vec<UiNode>,
        }

        impl $name {
            #[must_use]
            pub fn new(label: impl Into<String>) -> Self {
                Self {
                    label: label.into(),
                    state: ComplexMoleculeState::new($kind),
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
        }

        impl From<$name> for UiNode {
            fn from(value: $name) -> Self {
                let interaction = value.state.interaction();
                let mut node = UiNode::from_state($kind, value.label, value.state.state_id)
                    .interaction(interaction);
                for child in value.children {
                    node = node.child(child);
                }
                node
            }
        }
    };
}

complex_molecule_model!(Accordion, UiNodeKind::Accordion);
complex_molecule_model!(CodeDiff, UiNodeKind::CodeDiff);
complex_molecule_model!(ColorPicker, UiNodeKind::ColorPicker);
complex_molecule_model!(ComboBox, UiNodeKind::ComboBox);
complex_molecule_model!(CommandPalette, UiNodeKind::CommandPalette);
complex_molecule_model!(DynamicArrayEditor, UiNodeKind::DynamicArrayEditor);
complex_molecule_model!(MenuButton, UiNodeKind::MenuButton);
complex_molecule_model!(Modal, UiNodeKind::Modal);
complex_molecule_model!(ModalOverlay, UiNodeKind::ModalOverlay);
complex_molecule_model!(NotificationToast, UiNodeKind::NotificationToast);
complex_molecule_model!(Popover, UiNodeKind::Popover);
complex_molecule_model!(SearchBox, UiNodeKind::SearchBox);
complex_molecule_model!(SegmentedToggle, UiNodeKind::SegmentedToggle);
complex_molecule_model!(SelectBox, UiNodeKind::SelectBox);
complex_molecule_model!(Tooltip, UiNodeKind::Tooltip);
complex_molecule_model!(TreeView, UiNodeKind::TreeView);
