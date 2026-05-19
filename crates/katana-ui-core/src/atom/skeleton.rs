use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{
    UiDimension, UiInteractionState, UiLoadingProps, UiNode, UiNodeKind, UiSize, UiStateId,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkeletonShape {
    Text,
    Circle,
    Rectangle,
    Rounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkeletonAnimation {
    None,
    Pulse,
    Wave,
    Shimmer,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkeletonSize {
    Auto,
    Fill,
    Fixed {
        width: UiDimension,
        height: UiDimension,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Skeleton {
    label: String,
    state_id: UiStateId,
    shape: SkeletonShape,
    size: SkeletonSize,
    visual_size: UiSize,
    animation: SkeletonAnimation,
    reduced_motion: bool,
}

impl Skeleton {
    #[must_use]
    pub fn new(label: impl Into<String>, shape: SkeletonShape) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::Skeleton),
            shape,
            size: SkeletonSize::Auto,
            visual_size: UiSize::Medium,
            animation: SkeletonAnimation::Pulse,
            reduced_motion: false,
        }
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn size(mut self, size: SkeletonSize) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn animation(mut self, animation: SkeletonAnimation) -> Self {
        self.animation = animation;
        self
    }

    #[must_use]
    pub fn reduced_motion(mut self, reduced_motion: bool) -> Self {
        self.reduced_motion = reduced_motion;
        self
    }

    #[must_use]
    pub fn effective_animation(&self) -> SkeletonAnimation {
        if self.reduced_motion {
            SkeletonAnimation::None
        } else {
            self.animation
        }
    }
}

impl ComponentAction for Skeleton {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = state(self);
        if action.target() != &self.state_id {
            return UiActionResult::ignored(self.state_id.clone(), before);
        }
        match action {
            UiAction::SetReducedMotion { reduced_motion, .. } => {
                self.reduced_motion = *reduced_motion;
            }
            _ => return UiActionResult::ignored(self.state_id.clone(), before),
        }
        UiActionResult::handled(self.state_id.clone(), action, before, state(self))
    }
}

impl From<Skeleton> for UiNode {
    fn from(value: Skeleton) -> Self {
        let effective_animation = value.effective_animation();
        let node = UiNode::from_state(UiNodeKind::Skeleton, value.label, value.state_id)
            .size(value.visual_size)
            .loading_indicator(UiLoadingProps {
                reduced_motion: value.reduced_motion,
                ..UiLoadingProps::default()
            })
            .style_class(format!("{:?}", value.shape))
            .style_class(format!("{effective_animation:?}"));
        apply_size(node, value.size)
    }
}

fn state(value: &Skeleton) -> UiInteractionState {
    UiInteractionState {
        reduced_motion: value.reduced_motion,
        value: format!("{:?}", value.effective_animation()),
        ..UiInteractionState::default()
    }
}

fn apply_size(node: UiNode, size: SkeletonSize) -> UiNode {
    match size {
        SkeletonSize::Auto => node,
        SkeletonSize::Fill => node.width(UiDimension::Fill).height(UiDimension::Fill),
        SkeletonSize::Fixed { width, height } => node.width(width).height(height),
    }
}
