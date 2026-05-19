use crate::event::EventRoute;
use crate::render_model::{
    UiCommonProps, UiDismissAction, UiNode, UiNodeId, UiNodeKind, UiStateId, UiStatusProps, UiTone,
    UiVariant,
};
use serde::{Deserialize, Serialize};

macro_rules! molecule_model {
    ($name:ident, $kind:expr) => {
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
molecule_model!(Menu, UiNodeKind::Menu);
molecule_model!(Toolbar, UiNodeKind::Toolbar);
molecule_model!(FormField, UiNodeKind::FormField);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBar {
    label: String,
    state_id: UiStateId,
    common: UiCommonProps,
    status: UiStatusProps,
    children: Vec<UiNode>,
}

impl StatusBar {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::StatusBar),
            common: UiCommonProps::default(),
            status: UiStatusProps::default(),
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn severity(mut self, value: UiTone) -> Self {
        self.status.severity = value;
        self
    }

    #[must_use]
    pub fn variant(mut self, value: UiVariant) -> Self {
        self.status.variant = value;
        self
    }

    #[must_use]
    pub fn dismiss_action(mut self, value: UiDismissAction) -> Self {
        self.status.dismiss_action = value;
        self
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
}

impl From<StatusBar> for UiNode {
    fn from(value: StatusBar) -> Self {
        let mut node = UiNode::from_state(UiNodeKind::StatusBar, value.label, value.state_id)
            .common(value.common)
            .status(value.status);
        for child in value.children {
            node = node.child(child);
        }
        node
    }
}

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
