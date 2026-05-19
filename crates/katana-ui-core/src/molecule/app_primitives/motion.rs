use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{UiInteractionState, UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotionPrimitiveKind {
    Fade,
    Slide,
    Scale,
    Shimmer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReducedMotionPolicy {
    Respect,
    ForceReduced,
    Ignore,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotionSpec {
    pub primitive: MotionPrimitiveKind,
    pub duration_ms: u16,
    pub distance_px: u16,
    pub policy: ReducedMotionPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotionPrimitive {
    label: String,
    state_id: UiStateId,
    spec: MotionSpec,
    reduced_motion: bool,
    phase: u16,
}

impl MotionPrimitive {
    #[must_use]
    pub fn new(label: impl Into<String>, spec: MotionSpec) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::MotionPrimitive),
            spec,
            reduced_motion: false,
            phase: 0,
        }
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn effective_duration_ms(&self) -> u16 {
        match (self.spec.policy, self.reduced_motion) {
            (ReducedMotionPolicy::ForceReduced, _) | (ReducedMotionPolicy::Respect, true) => 0,
            _ => self.spec.duration_ms,
        }
    }
}

impl ComponentAction for MotionPrimitive {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = state(self);
        if action.target() != &self.state_id {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        match action {
            UiAction::SetReducedMotion { reduced_motion, .. } => {
                self.reduced_motion = *reduced_motion
            }
            UiAction::AnimationTick { phase, .. } => self.phase = *phase,
            _ => return UiActionResult::ignored(self.state_id.clone(), before),
        }
        UiActionResult::handled(self.state_id.clone(), action, before, state(self))
    }
}

impl From<MotionPrimitive> for UiNode {
    fn from(value: MotionPrimitive) -> Self {
        let state = state(&value);
        UiNode::from_state(UiNodeKind::MotionPrimitive, value.label, value.state_id)
            .interaction(state)
            .style_class(format!("{:?}", value.spec.primitive))
    }
}

fn state(value: &MotionPrimitive) -> UiInteractionState {
    UiInteractionState {
        animation_phase: value.phase,
        reduced_motion: value.effective_duration_ms() == 0,
        ..UiInteractionState::default()
    }
}
