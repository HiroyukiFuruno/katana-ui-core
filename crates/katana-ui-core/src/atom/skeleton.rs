use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};
use crate::render_model::{
    UiBorder, UiDimension, UiInteractionState, UiLoadingProps, UiNode, UiNodeKind, UiSize,
    UiSkeletonAnimation, UiSkeletonProps, UiSkeletonShape, UiStateId, UiTone,
};
use serde::{Deserialize, Serialize};

const DEFAULT_TEXT_LINE_HEIGHT_PX: u16 = 12;
const DEFAULT_LINE_GAP_PX: u16 = 8;
const DEFAULT_CIRCLE_SIZE_PX: u16 = 40;
const DEFAULT_RECT_WIDTH_PX: u16 = 160;
const DEFAULT_RECT_HEIGHT_PX: u16 = 80;
const DEFAULT_LINE_WIDTH_PERCENT: u16 = 100;
const DEFAULT_TEXT_LINE_RATIO: f32 = 1.0;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SkeletonShape {
    Rect,
    Circle,
    Line { thickness: f32 },
    Text { lines: usize, last_line_ratio: f32 },
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Skeleton {
    label: String,
    state_id: UiStateId,
    shape: SkeletonShape,
    size: SkeletonSize,
    visual_size: UiSize,
    animation: SkeletonAnimation,
    radius_px: u16,
    tone: UiTone,
    accessibility_label: String,
    aspect_ratio: Option<SkeletonAspectRatio>,
    reduced_motion: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkeletonAspectRatio {
    pub width: u16,
    pub height: u16,
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
            radius_px: 4,
            tone: UiTone::Neutral,
            accessibility_label: String::new(),
            aspect_ratio: None,
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
    pub fn radius_px(mut self, radius_px: u16) -> Self {
        self.radius_px = radius_px;
        self
    }

    #[must_use]
    pub fn tone(mut self, tone: UiTone) -> Self {
        self.tone = tone;
        self
    }

    #[must_use]
    pub fn accessibility_label(mut self, label: impl Into<String>) -> Self {
        self.accessibility_label = label.into();
        self
    }

    #[must_use]
    pub fn aspect_ratio(mut self, width: u16, height: u16) -> Self {
        self.aspect_ratio = Some(SkeletonAspectRatio { width, height });
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
        let node = UiNode::from_state(
            UiNodeKind::Skeleton,
            value.label.clone(),
            value.state_id.clone(),
        )
        .size(value.visual_size)
        .tone(value.tone)
        .border(UiBorder::solid(0, value.radius_px, "skeleton.border"))
        .loading_indicator(UiLoadingProps {
            reduced_motion: value.reduced_motion,
            ..UiLoadingProps::default()
        })
        .skeleton(skeleton_props(&value, effective_animation))
        .interaction(state(&value))
        .style_class(shape_class(value.shape))
        .style_class(format!("{effective_animation:?}"));
        let node = apply_accessibility(node, &value.accessibility_label);
        let node = apply_default_shape_size(node, value.shape);
        let node = apply_size(node, value.size.clone());
        append_shape_children(node, &value, effective_animation)
    }
}

fn state(value: &Skeleton) -> UiInteractionState {
    let raw = format!("{:?}", value.animation);
    let effective = format!("{:?}", value.effective_animation());
    UiInteractionState {
        reduced_motion: value.reduced_motion,
        value: if value.reduced_motion && value.animation != value.effective_animation() {
            format!("downgraded:{raw}->{effective}")
        } else {
            effective
        },
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

fn apply_accessibility(node: UiNode, label: &str) -> UiNode {
    if label.is_empty() {
        node
    } else {
        node.accessibility_label(label)
    }
}

fn apply_default_shape_size(node: UiNode, shape: SkeletonShape) -> UiNode {
    match shape {
        SkeletonShape::Rect => node
            .width(UiDimension::px(DEFAULT_RECT_WIDTH_PX))
            .height(UiDimension::px(DEFAULT_RECT_HEIGHT_PX)),
        SkeletonShape::Circle => node
            .width(UiDimension::px(DEFAULT_CIRCLE_SIZE_PX))
            .height(UiDimension::px(DEFAULT_CIRCLE_SIZE_PX)),
        SkeletonShape::Line { thickness } => node
            .width(UiDimension::percent(DEFAULT_LINE_WIDTH_PERCENT))
            .height(UiDimension::px(thickness_px(thickness))),
        SkeletonShape::Text { lines, .. } => node.height(UiDimension::px(text_height_px(lines))),
    }
}

fn append_shape_children(
    node: UiNode,
    value: &Skeleton,
    effective_animation: SkeletonAnimation,
) -> UiNode {
    match value.shape {
        SkeletonShape::Text {
            lines,
            last_line_ratio,
        } => (0..lines).fold(node, |parent, index| {
            parent.child(text_line_node(
                value,
                index,
                lines,
                last_line_ratio,
                effective_animation,
            ))
        }),
        _ => node,
    }
}

fn text_line_node(
    value: &Skeleton,
    index: usize,
    lines: usize,
    last_line_ratio: f32,
    effective_animation: SkeletonAnimation,
) -> UiNode {
    let width = if index + 1 == lines {
        UiDimension::percent(ratio_percent(last_line_ratio))
    } else {
        UiDimension::percent(DEFAULT_LINE_WIDTH_PERCENT)
    };
    UiNode::new(
        UiNodeKind::Skeleton,
        format!("{} line {}", value.label, index + 1),
    )
    .width(width)
    .height(UiDimension::px(DEFAULT_TEXT_LINE_HEIGHT_PX))
    .tone(value.tone)
    .border(UiBorder::solid(0, value.radius_px, "skeleton.border"))
    .skeleton(UiSkeletonProps {
        shape: UiSkeletonShape::Line,
        animation: animation_prop(effective_animation),
        radius_px: value.radius_px,
        line_thickness_px: DEFAULT_TEXT_LINE_HEIGHT_PX,
        ..UiSkeletonProps::default()
    })
    .style_class("Line")
    .style_class(format!("{effective_animation:?}"))
}

fn skeleton_props(value: &Skeleton, effective_animation: SkeletonAnimation) -> UiSkeletonProps {
    let (shape, line_count, last_line_percent, line_thickness_px) = match value.shape {
        SkeletonShape::Rect => (UiSkeletonShape::Rect, 1, 100, DEFAULT_TEXT_LINE_HEIGHT_PX),
        SkeletonShape::Circle => (UiSkeletonShape::Circle, 1, 100, DEFAULT_TEXT_LINE_HEIGHT_PX),
        SkeletonShape::Line { thickness } => {
            (UiSkeletonShape::Line, 1, 100, thickness_px(thickness))
        }
        SkeletonShape::Text {
            lines,
            last_line_ratio,
        } => (
            UiSkeletonShape::Text,
            lines,
            ratio_percent_u8(last_line_ratio),
            DEFAULT_TEXT_LINE_HEIGHT_PX,
        ),
    };
    let ratio = value.aspect_ratio.unwrap_or(SkeletonAspectRatio {
        width: 0,
        height: 0,
    });
    UiSkeletonProps {
        shape,
        animation: animation_prop(effective_animation),
        radius_px: value.radius_px,
        line_count,
        last_line_percent,
        line_thickness_px,
        aspect_ratio_width: ratio.width,
        aspect_ratio_height: ratio.height,
    }
}

fn animation_prop(value: SkeletonAnimation) -> UiSkeletonAnimation {
    match value {
        SkeletonAnimation::None => UiSkeletonAnimation::None,
        SkeletonAnimation::Pulse => UiSkeletonAnimation::Pulse,
        SkeletonAnimation::Wave => UiSkeletonAnimation::Wave,
        SkeletonAnimation::Shimmer => UiSkeletonAnimation::Shimmer,
    }
}

fn shape_class(value: SkeletonShape) -> String {
    match value {
        SkeletonShape::Rect => "Rect".to_string(),
        SkeletonShape::Circle => "Circle".to_string(),
        SkeletonShape::Line { .. } => "Line".to_string(),
        SkeletonShape::Text { .. } => "Text".to_string(),
    }
}

fn ratio_percent(value: f32) -> u16 {
    (value * 100.0).round() as u16
}

fn ratio_percent_u8(value: f32) -> u8 {
    u8::try_from(ratio_percent(value)).unwrap_or(u8::MAX)
}

fn thickness_px(value: f32) -> u16 {
    value.round() as u16
}

fn text_height_px(lines: usize) -> u16 {
    let line_count = u16::try_from(lines).unwrap_or(u16::MAX);
    line_count
        .saturating_mul(DEFAULT_TEXT_LINE_HEIGHT_PX)
        .saturating_add(
            line_count
                .saturating_sub(1)
                .saturating_mul(DEFAULT_LINE_GAP_PX),
        )
}

impl Default for SkeletonShape {
    fn default() -> Self {
        Self::Text {
            lines: 1,
            last_line_ratio: DEFAULT_TEXT_LINE_RATIO,
        }
    }
}
