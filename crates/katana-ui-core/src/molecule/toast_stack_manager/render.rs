use super::{ToastActionKind, ToastStackManager};
use crate::molecule::NotificationToast;
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind, UiVariant};

impl From<ToastStackManager> for UiNode {
    fn from(value: ToastStackManager) -> Self {
        let contract = value.visual_contract();
        let mut node = UiNode::from_state(
            UiNodeKind::ToastStackManager,
            "toast-stack-manager",
            value.state_id,
        )
        .interaction(UiInteractionState {
            item_count: value.state.visible.len(),
            value: format!("{:?}:{:?}", contract.position, contract.stack_direction),
            hovered: value.state.hover_count > 0,
            focused: value.state.focus_count > 0,
            active: value.state.paused,
            ..UiInteractionState::default()
        });
        for toast in value.state.visible {
            node = node.child(toast_node(toast));
        }
        node
    }
}

fn toast_node(toast: super::ActiveToast) -> UiNode {
    let payload = toast.payload;
    let mut node = UiNode::from(
        NotificationToast::new(payload.message)
            .severity(payload.severity)
            .open(true),
    );
    for action in payload.actions {
        node =
            node.child(crate::atom::Button::new(action.label).variant(action_variant(action.kind)));
    }
    node
}

fn action_variant(kind: ToastActionKind) -> UiVariant {
    match kind {
        ToastActionKind::Primary => UiVariant::Filled,
        ToastActionKind::Secondary => UiVariant::Text,
    }
}
