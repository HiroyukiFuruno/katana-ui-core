use crate::atom::{Skeleton, SkeletonAnimation, SkeletonShape, SkeletonSize};
use crate::render_model::{UiDimension, UiNode, UiNodeKind, UiStateId, UiTone};
use serde::{Deserialize, Serialize};

const CARD_MEDIA_WIDTH_PX: u16 = 280;
const CARD_MEDIA_HEIGHT_PX: u16 = 140;
const CARD_SUMMARY_LINE_COUNT: usize = 2;
const CARD_SUMMARY_LAST_LINE_RATIO: f32 = 0.72;
const LIST_ROW_AVATAR_SIZE_PX: u16 = 40;
const LIST_ROW_LINE_COUNT: usize = 2;
const LIST_ROW_LAST_LINE_RATIO: f32 = 0.64;
const MESSAGE_AVATAR_SIZE_PX: u16 = 36;
const MESSAGE_BODY_LINE_COUNT: usize = 2;
const MESSAGE_BODY_LAST_LINE_RATIO: f32 = 0.78;
const MESSAGE_META_WIDTH_PERCENT: u16 = 42;
const PARAGRAPH_LINE_COUNT: usize = 5;
const PARAGRAPH_LAST_LINE_RATIO: f32 = 0.65;
const IMAGE_CARD_IMAGE_WIDTH_PX: u16 = 320;
const IMAGE_CARD_IMAGE_HEIGHT_PX: u16 = 180;
const IMAGE_CARD_TITLE_LINE_COUNT: usize = 2;
const IMAGE_CARD_TITLE_LAST_LINE_RATIO: f32 = 0.68;
const IMAGE_CARD_META_WIDTH_PERCENT: u16 = 52;
const CODE_BLOCK_LINE_WIDTH_PERCENTAGES: [u16; PARAGRAPH_LINE_COUNT] = [100, 92, 76, 88, 64];
const LINE_THICKNESS_PX: f32 = 12.0;
const LINE_HEIGHT_PX: u16 = 12;

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
        SkeletonClusterPreset::Card => vec![
            rect("media", CARD_MEDIA_WIDTH_PX, CARD_MEDIA_HEIGHT_PX),
            text(
                "summary",
                CARD_SUMMARY_LINE_COUNT,
                CARD_SUMMARY_LAST_LINE_RATIO,
            ),
        ],
        SkeletonClusterPreset::ListRow => vec![
            circle("avatar", LIST_ROW_AVATAR_SIZE_PX),
            text("row", LIST_ROW_LINE_COUNT, LIST_ROW_LAST_LINE_RATIO),
        ],
        SkeletonClusterPreset::Message => vec![
            circle("avatar", MESSAGE_AVATAR_SIZE_PX),
            text(
                "body",
                MESSAGE_BODY_LINE_COUNT,
                MESSAGE_BODY_LAST_LINE_RATIO,
            ),
            line("meta", MESSAGE_META_WIDTH_PERCENT),
        ],
        SkeletonClusterPreset::Paragraph => vec![text(
            "paragraph",
            PARAGRAPH_LINE_COUNT,
            PARAGRAPH_LAST_LINE_RATIO,
        )],
        SkeletonClusterPreset::ImageCard => vec![
            rect(
                "image",
                IMAGE_CARD_IMAGE_WIDTH_PX,
                IMAGE_CARD_IMAGE_HEIGHT_PX,
            ),
            text(
                "title",
                IMAGE_CARD_TITLE_LINE_COUNT,
                IMAGE_CARD_TITLE_LAST_LINE_RATIO,
            ),
            line("meta", IMAGE_CARD_META_WIDTH_PERCENT),
        ],
        SkeletonClusterPreset::CodeBlock => CODE_BLOCK_LINE_WIDTH_PERCENTAGES
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
    Skeleton::new(
        label,
        SkeletonShape::Line {
            thickness: LINE_THICKNESS_PX,
        },
    )
    .size(SkeletonSize::Fixed {
        width: UiDimension::percent(width_percent),
        height: UiDimension::px(LINE_HEIGHT_PX),
    })
}

fn code_line(index: usize, width_percent: u16) -> Skeleton {
    let display_index = index + 1;
    line(&format!("code line {display_index}"), width_percent).animation(SkeletonAnimation::Wave)
}
