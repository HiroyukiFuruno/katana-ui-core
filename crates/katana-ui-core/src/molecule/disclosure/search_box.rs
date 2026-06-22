use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::molecule::state::MoleculeState;
use crate::render_model::{UiClearActionSpec, UiNode, UiNodeKind, UiStateId, UiTextEntryProps};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchBox {
    label: String,
    state: MoleculeState,
    clear_action: Option<UiClearActionSpec>,
    submit_on_enter: bool,
    case_sensitive: bool,
    children: Vec<UiNode>,
}

impl SearchBox {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state: MoleculeState::new(UiNodeKind::SearchBox),
            clear_action: None,
            submit_on_enter: false,
            case_sensitive: false,
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn placeholder(mut self, value: impl Into<String>) -> Self {
        self.state.placeholder = value.into();
        self
    }

    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.state.value = value.into();
        self
    }

    #[must_use]
    pub fn stable_state_id(mut self, value: impl Into<UiStateId>) -> Self {
        self.state.state_id = value.into();
        self
    }

    #[must_use]
    pub fn clear_action(mut self, label: impl Into<String>) -> Self {
        self.clear_action = Some(UiClearActionSpec::new(label));
        self
    }

    #[must_use]
    pub fn submit_on_enter(mut self, value: bool) -> Self {
        self.submit_on_enter = value;
        self
    }

    #[must_use]
    pub fn case_sensitive(mut self, value: bool) -> Self {
        self.case_sensitive = value;
        self
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state.state_id
    }

    #[must_use]
    pub const fn submits_on_enter(&self) -> bool {
        self.submit_on_enter
    }

    #[must_use]
    pub const fn is_case_sensitive(&self) -> bool {
        self.case_sensitive
    }
}

impl ComponentAction for SearchBox {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        self.state.apply_action(action, false)
    }
}

impl From<SearchBox> for UiNode {
    fn from(value: SearchBox) -> Self {
        let mut node = value
            .state
            .node(UiNodeKind::SearchBox, value.label)
            .text_entry(text_entry_props(value.clear_action, value.submit_on_enter));
        for child in value.children {
            node = node.child(child);
        }
        node
    }
}

fn text_entry_props(
    clear_action: Option<UiClearActionSpec>,
    submit_on_enter: bool,
) -> UiTextEntryProps {
    UiTextEntryProps {
        clear_action,
        submit_on_enter,
        ..UiTextEntryProps::default()
    }
}
