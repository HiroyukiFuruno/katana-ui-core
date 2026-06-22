use super::{UI_TREE_ROW_ACTION_ID, UiHostActionPayload, UiHostActionPlan, UiTreeRowActionKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiTreeRowActionTarget {
    pub node_id: String,
    pub action_kind: UiTreeRowActionKind,
}

impl UiHostActionPlan {
    #[must_use]
    pub fn tree_row_action_target(&self) -> Option<UiTreeRowActionTarget> {
        UiTreeRowActionTarget::from_plan(self)
    }
}

impl UiTreeRowActionTarget {
    fn from_plan(plan: &UiHostActionPlan) -> Option<Self> {
        if plan.action_id != UI_TREE_ROW_ACTION_ID {
            return None;
        }
        let UiHostActionPayload::TreeRow(payload) = &plan.typed_payload else {
            return None;
        };
        Some(Self {
            node_id: payload.node_id.clone(),
            action_kind: payload.action_kind,
        })
    }
}
