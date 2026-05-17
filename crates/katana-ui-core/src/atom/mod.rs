use crate::render_model::{UiInteractionState, UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AtomState {
    state_id: UiStateId,
    disabled: bool,
    focusable: bool,
    accessibility_label: String,
    interaction: UiInteractionState,
}

impl AtomState {
    #[must_use]
    pub fn enabled(kind: UiNodeKind) -> Self {
        Self {
            state_id: UiStateId::next_for(kind),
            disabled: false,
            focusable: false,
            accessibility_label: String::new(),
            interaction: UiInteractionState::default(),
        }
    }
}

macro_rules! atom_model {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            label: String,
            state: AtomState,
        }

        impl $name {
            #[must_use]
            pub fn new(label: impl Into<String>) -> Self {
                Self {
                    label: label.into(),
                    state: AtomState::enabled($kind),
                }
            }

            #[must_use]
            pub fn disabled(mut self, value: bool) -> Self {
                self.state.disabled = value;
                self
            }

            #[must_use]
            pub fn focusable(mut self, value: bool) -> Self {
                self.state.focusable = value;
                self
            }

            #[must_use]
            pub fn accessibility_label(mut self, value: impl Into<String>) -> Self {
                self.state.accessibility_label = value.into();
                self
            }

            #[must_use]
            pub fn selected(mut self, value: bool) -> Self {
                self.state.interaction.has_selection = value;
                self.state.interaction.selected_index = usize::from(value);
                self
            }

            #[must_use]
            pub fn value(mut self, value: impl Into<String>) -> Self {
                self.state.interaction.value = value.into();
                self
            }
        }

        impl From<$name> for UiNode {
            fn from(value: $name) -> Self {
                UiNode::from_state($kind, value.label, value.state.state_id)
                    .disabled(value.state.disabled)
                    .focusable(value.state.focusable)
                    .accessibility_label(value.state.accessibility_label)
                    .interaction(value.state.interaction)
            }
        }
    };
}

atom_model!(Text, UiNodeKind::Text);
atom_model!(Icon, UiNodeKind::Icon);
atom_model!(Button, UiNodeKind::Button);
atom_model!(Input, UiNodeKind::Input);
atom_model!(Checkbox, UiNodeKind::Checkbox);
atom_model!(Radio, UiNodeKind::Radio);
atom_model!(Badge, UiNodeKind::Badge);
atom_model!(Divider, UiNodeKind::Divider);
atom_model!(Spacer, UiNodeKind::Spacer);
atom_model!(KeyCap, UiNodeKind::KeyCap);
atom_model!(LoadingDots, UiNodeKind::LoadingDots);
atom_model!(Spinner, UiNodeKind::Spinner);
atom_model!(ProgressBar, UiNodeKind::ProgressBar);
atom_model!(ColorSwatch, UiNodeKind::ColorSwatch);
atom_model!(Toggle, UiNodeKind::Toggle);
atom_model!(SlideControl, UiNodeKind::SlideControl);
atom_model!(SvgButton, UiNodeKind::SvgButton);
atom_model!(TextButton, UiNodeKind::TextButton);
atom_model!(IconTextButton, UiNodeKind::IconTextButton);

#[cfg(test)]
mod tests {
    use super::{Button, Text};
    use crate::render_model::{UiNodeKind, UiTree};

    #[test]
    fn atom_snapshot_uses_neutral_node_kind() {
        let tree = UiTree::new(Button::new("Save"));
        assert_eq!(UiNodeKind::Button, tree.root().kind());
    }

    #[test]
    fn text_atom_can_be_tree_root() {
        let tree = UiTree::new(Text::new("Title"));
        assert_eq!(UiNodeKind::Text, tree.root().kind());
    }
}
