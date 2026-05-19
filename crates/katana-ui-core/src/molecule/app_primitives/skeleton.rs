use crate::render_model::{UiLoadingProps, UiNode, UiNodeKind, UiSize, UiStateId};
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
pub struct Skeleton {
    label: String,
    state_id: UiStateId,
    shape: SkeletonShape,
    size: UiSize,
    animation: SkeletonAnimation,
    reduced_motion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkeletonCluster {
    label: String,
    state_id: UiStateId,
    items: Vec<Skeleton>,
    live_region_label: String,
}

impl Skeleton {
    #[must_use]
    pub fn new(label: impl Into<String>, shape: SkeletonShape) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::Skeleton),
            shape,
            size: UiSize::Medium,
            animation: SkeletonAnimation::Pulse,
            reduced_motion: false,
        }
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

impl From<Skeleton> for UiNode {
    fn from(value: Skeleton) -> Self {
        let effective_animation = value.effective_animation();
        UiNode::from_state(UiNodeKind::Skeleton, value.label, value.state_id)
            .size(value.size)
            .loading_indicator(UiLoadingProps {
                reduced_motion: value.reduced_motion,
                ..UiLoadingProps::default()
            })
            .style_class(format!("{:?}", value.shape))
            .style_class(format!("{effective_animation:?}"))
    }
}

impl SkeletonCluster {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::SkeletonCluster),
            items: Vec::new(),
            live_region_label: String::from("loading"),
        }
    }

    #[must_use]
    pub fn item(mut self, item: Skeleton) -> Self {
        self.items.push(item);
        self
    }

    #[must_use]
    pub fn live_region_label(&self) -> &str {
        &self.live_region_label
    }
}

impl From<SkeletonCluster> for UiNode {
    fn from(value: SkeletonCluster) -> Self {
        value.items.into_iter().fold(
            UiNode::from_state(UiNodeKind::SkeletonCluster, value.label, value.state_id)
                .accessibility_label(value.live_region_label),
            |node, item| node.child(item),
        )
    }
}
