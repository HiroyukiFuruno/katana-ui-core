use super::hover_card::HoverCard;
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind};

impl From<HoverCard> for UiNode {
    fn from(value: HoverCard) -> Self {
        let mut node = UiNode::new(UiNodeKind::HoverCard, value.label)
            .interaction(UiInteractionState {
                open: value.open,
                ..UiInteractionState::default()
            })
            .child(UiNode::new(UiNodeKind::Button, "Anchor"));
        if value.slots.heading.is_empty()
            && value.slots.body.is_empty()
            && value.slots.footer.is_empty()
        {
            node = node.child(UiNode::new(UiNodeKind::Text, "Rich preview"));
        }
        node = add_slot_text(node, "Heading", &value.slots.heading);
        node = add_slot_text(node, "Body", &value.slots.body);
        node = add_slot_text(node, "Footer", &value.slots.footer);
        for action in value.slots.actions {
            node = node.child(UiNode::new(UiNodeKind::Button, action.label));
        }
        for action in value.slot_actions {
            node = node.child(UiNode::new(UiNodeKind::Button, action.label));
        }
        node
    }
}

fn add_slot_text(node: UiNode, label: &str, value: &str) -> UiNode {
    if value.is_empty() {
        return node;
    }
    node.child(UiNode::new(UiNodeKind::Text, format!("{label}: {value}")))
}
