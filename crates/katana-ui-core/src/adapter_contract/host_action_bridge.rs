use crate::render_model::{UiHostActionPlan, UiNode, UiNodeId};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AdapterHostActionBridge;

impl AdapterHostActionBridge {
    #[must_use]
    pub fn trigger(root: &UiNode, target: &UiNodeId, action_id: &str) -> Option<UiHostActionPlan> {
        UiHostActionPlan::collect_from_root(root)
            .into_iter()
            .find(|action| {
                action.enabled && &action.target == target && action.action_id == action_id
            })
    }
}
