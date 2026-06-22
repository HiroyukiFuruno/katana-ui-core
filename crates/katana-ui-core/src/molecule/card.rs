use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{
    UiInteractionState, UiNode, UiNodeKind, UiSize, UiStateId, UiVariant, UiVisualRole,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    label: String,
    state_id: UiStateId,
    header: Option<UiNode>,
    footer: Option<UiNode>,
    children: Vec<UiNode>,
    variant: UiVariant,
    padding: UiSize,
    interactive: bool,
    interaction: UiInteractionState,
}

impl Card {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::Card),
            header: None,
            footer: None,
            children: Vec::new(),
            variant: UiVariant::Plain,
            padding: UiSize::Medium,
            interactive: false,
            interaction: UiInteractionState::default(),
        }
    }

    #[must_use]
    pub fn header(mut self, header: impl Into<UiNode>) -> Self {
        self.header = Some(header.into());
        self
    }

    #[must_use]
    pub fn footer(mut self, footer: impl Into<UiNode>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    #[must_use]
    pub fn variant(mut self, variant: UiVariant) -> Self {
        self.variant = variant;
        self
    }

    #[must_use]
    pub fn padding(mut self, padding: UiSize) -> Self {
        self.padding = padding;
        self
    }

    #[must_use]
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn children(&self) -> &[UiNode] {
        &self.children
    }
}

impl ComponentAction for Card {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = self.interaction.clone();
        if action.target() != &self.state_id || !self.interactive {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        if !matches!(action, UiAction::Press { .. }) {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        self.interaction.has_selection = true;
        UiActionResult::handled(
            self.state_id.clone(),
            action,
            before,
            self.interaction.clone(),
        )
    }
}

impl From<Card> for UiNode {
    fn from(value: Card) -> Self {
        let mut node = UiNode::from_state(UiNodeKind::Card, value.label, value.state_id)
            .visual_role(UiVisualRole::Content)
            .variant(value.variant)
            .size(value.padding)
            .focusable(value.interactive)
            .interaction(value.interaction);
        if let Some(header) = value.header {
            node = node.child(header);
        }
        for child in value.children {
            node = node.child(child);
        }
        if let Some(footer) = value.footer {
            node = node.child(footer);
        }
        node
    }
}
