use super::modal_render::overlay_dialog_props;
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult, UiActionSource};
use crate::molecule::state::MoleculeState;
use crate::render_model::{UiModalPlacement, UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModalOverlay {
    label: String,
    state: MoleculeState,
    backdrop: String,
    focus_trap: bool,
    focus_return: String,
    dismiss_policy: String,
    escape_dismiss: bool,
    outside_click_dismiss: bool,
    placement: UiModalPlacement,
    children: Vec<UiNode>,
}

impl ModalOverlay {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state: MoleculeState::new(UiNodeKind::ModalOverlay),
            backdrop: String::new(),
            focus_trap: false,
            focus_return: String::new(),
            dismiss_policy: String::new(),
            escape_dismiss: false,
            outside_click_dismiss: false,
            placement: UiModalPlacement::Center,
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn child(mut self, child: impl Into<UiNode>) -> Self {
        self.children.push(child.into());
        self
    }

    #[must_use]
    pub fn open(mut self, value: bool) -> Self {
        self.state.open = value;
        self
    }

    #[must_use]
    pub fn backdrop(mut self, value: impl Into<String>) -> Self {
        self.backdrop = value.into();
        self
    }

    #[must_use]
    pub fn focus_trap(mut self, value: bool) -> Self {
        self.focus_trap = value;
        self
    }

    #[must_use]
    pub fn focus_return(mut self, value: impl Into<String>) -> Self {
        self.focus_return = value.into();
        self
    }

    #[must_use]
    pub fn dismiss_policy(mut self, value: impl Into<String>) -> Self {
        self.dismiss_policy = value.into();
        self
    }

    #[must_use]
    pub fn escape_dismiss(mut self, value: bool) -> Self {
        self.escape_dismiss = value;
        self
    }

    #[must_use]
    pub fn outside_click_dismiss(mut self, value: bool) -> Self {
        self.outside_click_dismiss = value;
        self
    }

    #[must_use]
    pub fn placement(mut self, value: UiModalPlacement) -> Self {
        self.placement = value;
        self
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state.state_id
    }

    #[must_use]
    pub fn backdrop_model(&self) -> &str {
        &self.backdrop
    }

    #[must_use]
    pub const fn traps_focus(&self) -> bool {
        self.focus_trap
    }

    #[must_use]
    pub fn focus_return_model(&self) -> &str {
        &self.focus_return
    }

    #[must_use]
    pub fn dismiss_policy_model(&self) -> &str {
        &self.dismiss_policy
    }

    #[must_use]
    pub const fn dismisses_on_escape(&self) -> bool {
        self.escape_dismiss
    }

    #[must_use]
    pub const fn dismisses_on_outside_click(&self) -> bool {
        self.outside_click_dismiss
    }

    #[must_use]
    pub const fn placement_model(&self) -> UiModalPlacement {
        self.placement
    }
}

impl ComponentAction for ModalOverlay {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = self.state.interaction();
        if action.target() != &self.state.state_id || self.state.disabled {
            return UiActionResult::ignored(self.state.state_id.clone(), before);
        }
        let Some(lifecycle_action) = ModalOverlayLifecycleAction::from_ui_action(action) else {
            return self.state.apply_action(action, false);
        };
        if !self.apply_lifecycle_action(lifecycle_action) {
            return UiActionResult::ignored(self.state.state_id.clone(), before);
        }
        UiActionResult::handled(
            self.state.state_id.clone(),
            action,
            before,
            self.state.interaction(),
        )
    }
}

impl From<ModalOverlay> for UiNode {
    fn from(value: ModalOverlay) -> Self {
        let modal_props = overlay_dialog_props(
            &value.backdrop,
            value.focus_trap,
            &value.focus_return,
            &value.dismiss_policy,
            value.escape_dismiss,
            value.outside_click_dismiss,
            value.placement,
        );
        let mut node = value
            .state
            .node(UiNodeKind::ModalOverlay, value.label)
            .modal(modal_props);
        for child in value.children {
            node = node.child(child);
        }
        node
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModalOverlayLifecycleAction {
    Escape,
    Backdrop,
}

impl ModalOverlayLifecycleAction {
    fn from_ui_action(action: &UiAction) -> Option<Self> {
        match action {
            UiAction::Press {
                source: UiActionSource::ModalEscape,
                ..
            } => Some(Self::Escape),
            UiAction::Press {
                source: UiActionSource::ModalBackdrop,
                ..
            } => Some(Self::Backdrop),
            _ => None,
        }
    }
}

impl ModalOverlay {
    fn apply_lifecycle_action(&mut self, action: ModalOverlayLifecycleAction) -> bool {
        match action {
            ModalOverlayLifecycleAction::Escape => self.dismiss_if_allowed(self.escape_dismiss),
            ModalOverlayLifecycleAction::Backdrop => {
                self.dismiss_if_allowed(self.outside_click_dismiss)
            }
        }
    }

    fn dismiss_if_allowed(&mut self, allowed: bool) -> bool {
        if !allowed {
            return false;
        }
        self.state.open = false;
        self.state.transient.dismiss_reason = self.dismiss_policy.clone();
        if !self.focus_return.is_empty() {
            self.state.value = format!("focus_return={}", self.focus_return);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_dispatch_rejects_unrelated_action() {
        let overlay = ModalOverlay::new("Modal");
        assert_eq!(
            None,
            ModalOverlayLifecycleAction::from_ui_action(&UiAction::focus(
                overlay.state_id().clone()
            ))
        );
    }
}
