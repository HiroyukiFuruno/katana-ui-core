use super::hover_card::HoverCard;
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind};

impl From<HoverCard> for UiNode {
    fn from(value: HoverCard) -> Self {
        let mut node = UiNode::new(UiNodeKind::HoverCard, value.label)
            .interaction(UiInteractionState {
                open: value.open,
                ..UiInteractionState::default()
            })
            .child(UiNode::new(UiNodeKind::Button, "Anchor"))
            .child(UiNode::new(UiNodeKind::Text, "Rich preview"));
        for action in value.slot_actions {
            node = node.child(UiNode::new(UiNodeKind::Button, action.label));
        }
        node
    }
}
