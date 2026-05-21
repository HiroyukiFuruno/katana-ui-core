use super::{Skeleton, SkeletonAnimation, SkeletonAspectRatio, SkeletonShape, SkeletonSize};
use crate::render_model::{
    UiBorder, UiDimension, UiInteractionState, UiLoadingProps, UiNode, UiNodeKind,
    UiSkeletonAnimation, UiSkeletonProps, UiSkeletonShape,
};

const DEFAULT_TEXT_LINE_HEIGHT_PX: u16 = 12;
const DEFAULT_LINE_GAP_PX: u16 = 8;
const DEFAULT_CIRCLE_SIZE_PX: u16 = 40;
const DEFAULT_RECT_WIDTH_PX: u16 = 160;
const DEFAULT_RECT_HEIGHT_PX: u16 = 80;
const DEFAULT_LINE_WIDTH_PERCENT: u16 = 100;
const FULL_PERCENT: u8 = 100;
const ZERO_ASPECT_RATIO: u16 = 0;
const PERCENT_SCALE: f32 = 100.0;

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

pub(super) fn state(value: &Skeleton) -> UiInteractionState {
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
        SkeletonShape::Rect => (
            UiSkeletonShape::Rect,
            1,
            FULL_PERCENT,
            DEFAULT_TEXT_LINE_HEIGHT_PX,
        ),
        SkeletonShape::Circle => (
            UiSkeletonShape::Circle,
            1,
            FULL_PERCENT,
            DEFAULT_TEXT_LINE_HEIGHT_PX,
        ),
        SkeletonShape::Line { thickness } => (
            UiSkeletonShape::Line,
            1,
            FULL_PERCENT,
            thickness_px(thickness),
        ),
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
        width: ZERO_ASPECT_RATIO,
        height: ZERO_ASPECT_RATIO,
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
    (value * PERCENT_SCALE).round() as u16
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
