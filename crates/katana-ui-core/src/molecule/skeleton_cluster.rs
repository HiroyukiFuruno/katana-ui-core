use crate::atom::Skeleton;
use crate::render_model::{UiNode, UiNodeKind, UiStateId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkeletonClusterPreset {
    List,
    Message,
    Card,
    Paragraph,
    CodeBlock,
    ImageCard,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkeletonCluster {
    label: String,
    state_id: UiStateId,
    preset: SkeletonClusterPreset,
    items: Vec<Skeleton>,
    live_region_label: String,
}

impl SkeletonCluster {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state_id: UiStateId::next_for(UiNodeKind::SkeletonCluster),
            preset: SkeletonClusterPreset::List,
            items: Vec::new(),
            live_region_label: String::from("loading"),
        }
    }

    #[must_use]
    pub fn state_id(&self) -> &UiStateId {
        &self.state_id
    }

    #[must_use]
    pub fn preset(mut self, preset: SkeletonClusterPreset) -> Self {
        self.preset = preset;
        self
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
                .accessibility_label(value.live_region_label)
                .style_class(format!("{:?}", value.preset)),
            |node, item| node.child(item),
        )
    }
}
