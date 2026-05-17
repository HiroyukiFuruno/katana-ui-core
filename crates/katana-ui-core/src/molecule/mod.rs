use crate::event::EventRoute;
mod complex;

use crate::render_model::UiNodeId;
use crate::render_model::{UiNode, UiNodeKind, UiStateId};
pub use complex::{
    Accordion, CodeDiff, ColorPicker, ComboBox, CommandPalette, DynamicArrayEditor, MenuButton,
    Modal, ModalOverlay, NotificationToast, Popover, SearchBox, SegmentedToggle, SelectBox,
    Tooltip, TreeView,
};
use serde::{Deserialize, Serialize};

macro_rules! molecule_model {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            label: String,
            state_id: UiStateId,
            children: Vec<UiNode>,
        }

        impl $name {
            #[must_use]
            pub fn new(label: impl Into<String>) -> Self {
                Self {
                    label: label.into(),
                    state_id: UiStateId::next_for($kind),
                    children: Vec::new(),
                }
            }

            #[must_use]
            pub fn child(mut self, child: impl Into<UiNode>) -> Self {
                self.children.push(child.into());
                self
            }

            #[must_use]
            pub fn children(&self) -> &[UiNode] {
                &self.children
            }
        }

        impl From<$name> for UiNode {
            fn from(value: $name) -> Self {
                let mut node = UiNode::from_state($kind, value.label, value.state_id);
                for child in value.children {
                    node = node.child(child);
                }
                node
            }
        }
    };
}

molecule_model!(Card, UiNodeKind::Card);
molecule_model!(List, UiNodeKind::List);
molecule_model!(Menu, UiNodeKind::Menu);
molecule_model!(Tabs, UiNodeKind::Tabs);
molecule_model!(Toolbar, UiNodeKind::Toolbar);
molecule_model!(FormField, UiNodeKind::FormField);
molecule_model!(Breadcrumb, UiNodeKind::Breadcrumb);
molecule_model!(SelectionList, UiNodeKind::SelectionList);
molecule_model!(SideMenu, UiNodeKind::SideMenu);
molecule_model!(StatusBar, UiNodeKind::StatusBar);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoleculeEventRouting;

impl MoleculeEventRouting {
    #[must_use]
    pub fn bubble_nested(
        target: UiNodeId,
        molecule: UiNodeId,
        root: UiNodeId,
        disabled: bool,
    ) -> EventRoute {
        EventRoute::bubble(target, vec![molecule, root], disabled)
    }
}

#[cfg(test)]
mod tests {
    use super::MoleculeEventRouting;
    use super::{Card, Toolbar};
    use crate::atom::Button;
    use crate::render_model::{UiNodeId, UiNodeKind, UiTree};

    #[test]
    fn molecule_snapshot_keeps_children() {
        let tree = UiTree::new(Toolbar::new("main").child(Button::new("Save")));
        assert_eq!(1, tree.root().children().len());
    }

    #[test]
    fn card_uses_molecule_kind() {
        let tree = UiTree::new(Card::new("summary"));
        assert_eq!(UiNodeKind::Card, tree.root().kind());
    }

    #[test]
    fn molecule_event_routing_visits_nested_target_then_parents() {
        let route = MoleculeEventRouting::bubble_nested(
            UiNodeId::new("button"),
            UiNodeId::new("toolbar"),
            UiNodeId::new("root"),
            false,
        );
        let actual: Vec<&str> = route.order().iter().map(UiNodeId::as_str).collect();
        assert_eq!(["button", "toolbar", "root"], actual.as_slice());
    }
}
