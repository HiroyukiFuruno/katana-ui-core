use crate::atom::{Skeleton, SkeletonAnimation, SkeletonShape, SkeletonSize};
use crate::render_model::{UiDimension, UiNode, UiNodeKind, UiStateId, UiTone};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkeletonClusterPreset {
    ListRow,
    Message,
    Card,
    Paragraph,
    CodeBlock,
    ImageCard,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
            preset: SkeletonClusterPreset::ListRow,
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
        let live_region = cluster_live_region(&value);
        let items = if value.items.is_empty() {
            preset_items(value.preset)
        } else {
            value.items
        };
        items.into_iter().fold(
            UiNode::from_state(UiNodeKind::SkeletonCluster, value.label, value.state_id)
                .accessibility_label(live_region)
                .style_class(format!("{:?}", value.preset)),
            |node, item| node.child(UiNode::from(item).accessibility_label("")),
        )
    }
}

fn cluster_live_region(value: &SkeletonCluster) -> String {
    if value.live_region_label != "loading" {
        return value.live_region_label.clone();
    }
    if value.label.to_ascii_lowercase().starts_with("loading") {
        value.label.clone()
    } else {
        format!("Loading {}", value.label)
    }
}

fn preset_items(preset: SkeletonClusterPreset) -> Vec<Skeleton> {
    match preset {
        SkeletonClusterPreset::Card => vec![rect("media", 280, 140), text("summary", 2, 0.72)],
        SkeletonClusterPreset::ListRow => vec![circle("avatar", 40), text("row", 2, 0.64)],
        SkeletonClusterPreset::Message => vec![
            circle("avatar", 36),
            text("body", 2, 0.78),
            line("meta", 42),
        ],
        SkeletonClusterPreset::Paragraph => vec![text("paragraph", 5, 0.65)],
        SkeletonClusterPreset::ImageCard => vec![
            rect("image", 320, 180),
            text("title", 2, 0.68),
            line("meta", 52),
        ],
        SkeletonClusterPreset::CodeBlock => [100, 92, 76, 88, 64]
            .into_iter()
            .enumerate()
            .map(|(index, width_percent)| code_line(index, width_percent))
            .collect(),
    }
}

fn rect(label: &str, width: u16, height: u16) -> Skeleton {
    Skeleton::new(label, SkeletonShape::Rect)
        .size(SkeletonSize::Fixed {
            width: UiDimension::px(width),
            height: UiDimension::px(height),
        })
        .tone(UiTone::Neutral)
        .animation(SkeletonAnimation::Shimmer)
}

fn circle(label: &str, size: u16) -> Skeleton {
    Skeleton::new(label, SkeletonShape::Circle)
        .size(SkeletonSize::Fixed {
            width: UiDimension::px(size),
            height: UiDimension::px(size),
        })
        .tone(UiTone::Neutral)
        .animation(SkeletonAnimation::Pulse)
}

fn text(label: &str, lines: usize, last_line_ratio: f32) -> Skeleton {
    Skeleton::new(
        label,
        SkeletonShape::Text {
            lines,
            last_line_ratio,
        },
    )
    .animation(SkeletonAnimation::Shimmer)
}

fn line(label: &str, width_percent: u16) -> Skeleton {
    Skeleton::new(label, SkeletonShape::Line { thickness: 12.0 }).size(SkeletonSize::Fixed {
        width: UiDimension::percent(width_percent),
        height: UiDimension::px(12),
    })
}

fn code_line(index: usize, width_percent: u16) -> Skeleton {
    line(&format!("code line {}", index + 1), width_percent).animation(SkeletonAnimation::Wave)
}
