use super::{
    UI_CODE_COPY_ACTION_ID, UI_DISCLOSURE_TOGGLE_ACTION_ID, UI_LINK_OPEN_ACTION_ID,
    UiHostActionPlan,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTextSpanAction {
    OpenLink { target: String },
    CopyCode { node_id: String },
    ToggleAccordion { node_id: String, open: bool },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiAccordionToggleAction {
    pub node_id: String,
    pub requested_open: bool,
}

impl UiHostActionPlan {
    #[must_use]
    pub fn text_span_action(&self) -> Option<UiTextSpanAction> {
        UiTextSpanAction::from_plan(self)
    }
}

impl UiTextSpanAction {
    #[must_use]
    pub fn accordion_toggle_action(&self) -> Option<UiAccordionToggleAction> {
        match self {
            Self::ToggleAccordion { node_id, open } => Some(UiAccordionToggleAction {
                node_id: node_id.clone(),
                requested_open: !*open,
            }),
            _ => None,
        }
    }

    fn from_plan(plan: &UiHostActionPlan) -> Option<Self> {
        if plan.action_id == UI_LINK_OPEN_ACTION_ID && !plan.payload.trim().is_empty() {
            return Some(Self::OpenLink {
                target: plan.payload.clone(),
            });
        }
        if plan.action_id == UI_DISCLOSURE_TOGGLE_ACTION_ID {
            return Some(Self::ToggleAccordion {
                node_id: plan.target.as_str().to_string(),
                open: accordion_open_payload(plan.payload.as_str())?,
            });
        }
        if plan.action_id == UI_CODE_COPY_ACTION_ID {
            return Some(Self::CopyCode {
                node_id: plan.payload.clone(),
            });
        }
        None
    }
}

fn accordion_open_payload(payload: &str) -> Option<bool> {
    match payload.trim() {
        "open=true" => Some(true),
        "open=false" => Some(false),
        _ => None,
    }
}
