use crate::interaction::{
    MotionDistanceToken, MotionDurationToken, MotionEasingToken, MotionSpec, ScaleOrigin,
    ShimmerDirection, ShimmerSpeed, SlideDirection,
};

const CONTEXT_MENU_SCALE_FROM: f32 = 0.95;
const MODAL_SCALE_FROM: f32 = 0.96;
const DRAG_PREVIEW_SCALE_FROM: f32 = 1.0;
const FADE_OPACITY_FROM: f32 = 0.0;
const FADE_OPACITY_TO: f32 = 1.0;
const SCALE_TO: f32 = 1.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionTarget {
    Popover,
    HoverCard,
    ContextMenu,
    Modal,
    NotificationToast,
    ToastStackManager,
    Banner,
    Accordion,
    DragPreview,
    Skeleton,
    SkeletonCluster,
}

impl MotionTarget {
    #[must_use]
    pub const fn required_molecules() -> &'static [Self] {
        &[
            Self::Popover,
            Self::HoverCard,
            Self::ContextMenu,
            Self::Modal,
            Self::NotificationToast,
            Self::ToastStackManager,
            Self::Banner,
            Self::Accordion,
            Self::DragPreview,
            Self::Skeleton,
            Self::SkeletonCluster,
        ]
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MotionDefaults;

impl MotionDefaults {
    #[must_use]
    pub fn for_target(target: MotionTarget) -> MotionSpec {
        match target {
            MotionTarget::Popover | MotionTarget::HoverCard => slide_up(),
            MotionTarget::ContextMenu => scale(MotionDurationToken::Fast, CONTEXT_MENU_SCALE_FROM),
            MotionTarget::Modal => scale(MotionDurationToken::Default, MODAL_SCALE_FROM),
            MotionTarget::NotificationToast | MotionTarget::ToastStackManager => slide_down(),
            MotionTarget::Banner => slide_down(),
            MotionTarget::Accordion => fade(MotionDurationToken::Default),
            MotionTarget::DragPreview => scale(MotionDurationToken::Fast, DRAG_PREVIEW_SCALE_FROM),
            MotionTarget::Skeleton | MotionTarget::SkeletonCluster => MotionSpec::shimmer(
                MotionDurationToken::Slow,
                MotionEasingToken::Linear,
                ShimmerSpeed::Default,
                ShimmerDirection::LeftToRight,
            ),
        }
    }
}

fn fade(duration: MotionDurationToken) -> MotionSpec {
    MotionSpec::fade(
        duration,
        MotionEasingToken::Standard,
        FADE_OPACITY_FROM,
        FADE_OPACITY_TO,
    )
}

fn slide_up() -> MotionSpec {
    MotionSpec::slide(
        MotionDurationToken::Default,
        MotionEasingToken::Emphasized,
        MotionDistanceToken::Default,
        SlideDirection::Up,
    )
}

fn slide_down() -> MotionSpec {
    MotionSpec::slide(
        MotionDurationToken::Default,
        MotionEasingToken::Decelerate,
        MotionDistanceToken::Default,
        SlideDirection::Down,
    )
}

fn scale(duration: MotionDurationToken, from: f32) -> MotionSpec {
    MotionSpec::scale(
        duration,
        MotionEasingToken::Emphasized,
        from,
        SCALE_TO,
        ScaleOrigin::Center,
    )
}
