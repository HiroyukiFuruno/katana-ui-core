use crate::event::EventRoute;
use crate::interaction::{
    VirtualRange, VirtualizationConfig,
    placement::{PlacementConsumer, PlacementEngine, PlacementRequest, PlacementResult},
};
use crate::molecule::virtualization::MoleculeVirtualization;
use crate::render_model::{UiCommonProps, UiNode, UiNodeId, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct List {
    label: String,
    state_id: UiStateId,
    common: UiCommonProps,
    virtualization: Option<VirtualizationConfig>,
    children: Vec<UiNode>,
}

impl List {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::List),
            common: UiCommonProps::default(),
            virtualization: None,
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

    #[must_use]
    pub fn virtualization(mut self, value: VirtualizationConfig) -> Self {
        self.virtualization = Some(value);
        self
    }

    #[must_use]
    pub fn virtual_range_model(&self) -> Option<VirtualRange> {
        MoleculeVirtualization::range(&self.virtualization, self.children.len())
    }
}

impl From<List> for UiNode {
    fn from(value: List) -> Self {
        let range = value.virtual_range_model();
        let interaction = MoleculeVirtualization::interaction(
            crate::render_model::UiInteractionState {
                item_count: value.children.len(),
                ..crate::render_model::UiInteractionState::default()
            },
            range.as_ref(),
        );
        let mut node = UiNode::from_state(UiNodeKind::List, value.label, value.state_id)
            .common(value.common)
            .interaction(interaction);
        for child in MoleculeVirtualization::slice_by_range(value.children, range.as_ref()) {
            node = node.child(child);
        }
        node
    }
}

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

impl Menu {
    #[must_use]
    pub fn resolve_panel_placement(&self, request: &PlacementRequest) -> PlacementResult {
        PlacementEngine::resolve_for(PlacementConsumer::Menu, request)
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
