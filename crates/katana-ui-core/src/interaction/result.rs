use crate::interaction::UiAction;
use crate::render_model::{UiInteractionState, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiCallbackLog {
    pub target: UiStateId,
    pub action: String,
    pub before: String,
    pub after: String,
}

impl UiCallbackLog {
    #[must_use]
    pub fn new(
        target: UiStateId,
        action: impl Into<String>,
        before: impl Into<String>,
        after: impl Into<String>,
    ) -> Self {
        Self {
            target,
            action: action.into(),
            before: before.into(),
            after: after.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiActionResult {
    pub target: UiStateId,
    pub handled: bool,
    pub before: UiInteractionState,
    pub after: UiInteractionState,
    pub callback_log: Vec<UiCallbackLog>,
}

impl UiActionResult {
    #[must_use]
    pub fn ignored(target: UiStateId, state: UiInteractionState) -> Self {
        Self {
            target,
            handled: false,
            before: state.clone(),
            after: state,
            callback_log: Vec::new(),
        }
    }

    #[must_use]
    pub fn handled(
        target: UiStateId,
        action: &UiAction,
        before: UiInteractionState,
        after: UiInteractionState,
    ) -> Self {
        let callback_log = vec![UiCallbackLog::new(
            target.clone(),
            action.name(),
            before.summary(),
            after.summary(),
        )];
        Self {
            target,
            handled: true,
            before,
            after,
            callback_log,
        }
    }
}
