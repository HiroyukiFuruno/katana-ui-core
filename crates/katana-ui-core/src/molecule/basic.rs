use crate::event::EventRoute;
use crate::interaction::placement::{
    PlacementConsumer, PlacementEngine, PlacementRequest, PlacementResult,
};
use crate::render_model::{
    UiCommonProps, UiFormFieldProps, UiHostActionSpec, UiInteractionState, UiNode, UiNodeId,
    UiNodeKind, UiStateId,
};
use serde::{Deserialize, Serialize};

macro_rules! molecule_model {
    ($(#[$meta:meta])* $name:ident, $kind:expr) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct $name {
            label: String,
            node_id: Option<UiNodeId>,
            state_id: UiStateId,
            common: UiCommonProps,
            selected_index: Option<usize>,
            invalid: bool,
            required: bool,
            control_state_id: Option<UiStateId>,
            helper_text: String,
            children: Vec<UiNode>,
        }

        impl $name {
            #[must_use]
            pub fn new(label: impl Into<String>) -> Self {
                Self {
                    label: label.into(),
                    node_id: None,
                    state_id: UiStateId::next_for($kind),
                    common: UiCommonProps::default(),
                    selected_index: None,
                    invalid: false,
                    required: false,
                    control_state_id: None,
                    helper_text: String::new(),
                    children: Vec::new(),
                }
            }

            #[must_use]
            pub fn common(mut self, value: UiCommonProps) -> Self {
                self.common = value;
                self
            }

            #[must_use]
            pub fn host_action(mut self, value: UiHostActionSpec) -> Self {
                self.common = self.common.host_action(value);
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
            pub fn selected_index(mut self, value: usize) -> Self {
                self.selected_index = Some(value);
                self
            }

            #[must_use]
            pub fn stable_node_id(mut self, value: impl Into<UiNodeId>) -> Self {
                self.node_id = Some(value.into());
                self
            }
        }

        impl From<$name> for UiNode {
            fn from(value: $name) -> Self {
                let mut node =
                    UiNode::from_state($kind, value.label, value.state_id)
                        .common(value.common)
                        .invalid(value.invalid)
                        .placeholder(value.helper_text.clone())
                        .form_field(UiFormFieldProps {
                            helper_text: value.helper_text,
                            required: value.required,
                            control_state_id: value.control_state_id,
                        })
                        .interaction(selected_interaction(value.selected_index));
                if let Some(node_id) = value.node_id {
                    node = node.stable_node_id(node_id);
                }
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

fn selected_interaction(selected_index: Option<usize>) -> UiInteractionState {
    UiInteractionState {
        has_selection: selected_index.is_some(),
        selected_index: selected_index.unwrap_or_default(),
        ..UiInteractionState::default()
    }
}

impl Menu {
    #[must_use]
    pub fn resolve_panel_placement(&self, request: &PlacementRequest) -> PlacementResult {
        PlacementEngine::resolve_for(PlacementConsumer::Menu, request)
    }
}

impl FormField {
    #[must_use]
    pub fn stable_state_id(mut self, value: impl Into<UiStateId>) -> Self {
        self.state_id = value.into();
        self
    }

    #[must_use]
    pub fn invalid(mut self, value: bool) -> Self {
        self.invalid = value;
        self
    }

    #[must_use]
    pub fn required(mut self, value: bool) -> Self {
        self.required = value;
        self
    }

    #[must_use]
    pub fn control_state_id(mut self, value: impl Into<UiStateId>) -> Self {
        self.control_state_id = Some(value.into());
        self
    }

    #[must_use]
    pub fn helper_text(mut self, value: impl Into<String>) -> Self {
        self.helper_text = value.into();
        self
    }

    #[must_use]
    pub fn invalid_model(&self) -> bool {
        self.invalid
    }

    #[must_use]
    pub fn required_model(&self) -> bool {
        self.required
    }

    #[must_use]
    pub fn control_state_id_model(&self) -> Option<&UiStateId> {
        self.control_state_id.as_ref()
    }

    #[must_use]
    pub fn helper_text_model(&self) -> &str {
        &self.helper_text
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
    use super::*;
    use crate::render_model::{UiHostActionPlan, UiHostActionSpec};

    #[test]
    fn form_field_defaults_to_no_host_action() {
        let node = UiNode::from(FormField::new("Dark"));

        assert!(UiHostActionPlan::collect_from_root(&node).is_empty());
    }

    #[test]
    fn form_field_accepts_explicit_host_action() -> Result<(), String> {
        let node = UiNode::from(
            FormField::new("Dark")
                .host_action(UiHostActionSpec::settings_field_control("Dark", "dark")),
        );
        let plan = UiHostActionPlan::collect_from_root(&node)
            .into_iter()
            .find_map(|plan| plan.settings_field_control_target());

        assert_eq!(
            "dark",
            plan.ok_or_else(|| "field action missing".to_string())?
                .field_id
        );
        Ok(())
    }
}
