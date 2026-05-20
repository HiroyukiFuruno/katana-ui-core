use crate::event::EventRoute;
use crate::render_model::{UiCommonProps, UiNode, UiNodeId, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

macro_rules! molecule_model {
    ($(#[$meta:meta])* $name:ident, $kind:expr) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            label: String,
            state_id: UiStateId,
            common: UiCommonProps,
            children: Vec<UiNode>,
        }

        impl $name {
            #[must_use]
            pub fn new(label: impl Into<String>) -> Self {
                Self {
                    label: label.into(),
                    state_id: UiStateId::next_for($kind),
                    common: UiCommonProps::default(),
                    children: Vec::new(),
                }
            }

            #[must_use]
            pub fn common(mut self, value: UiCommonProps) -> Self {
                self.common = value;
                self
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
                let mut node =
                    UiNode::from_state($kind, value.label, value.state_id).common(value.common);
                for child in value.children {
                    node = node.child(child);
                }
                node
            }
        }
    };
}

molecule_model!(List, UiNodeKind::List);
molecule_model!(
    /// Basic inline menu container.
    ///
    /// Use `ContextMenu` for pointer / node anchored menus, submenu state,
    /// placement, focus return, and callback logs.
    Menu,
    UiNodeKind::Menu
);
molecule_model!(Toolbar, UiNodeKind::Toolbar);
molecule_model!(FormField, UiNodeKind::FormField);

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
