use super::motion_primitive::{
    legacy_primitive, standard_easing, token_from_distance, token_from_duration,
};
use super::{
    MotionPrimitive, MotionPrimitiveKind, ScaleOrigin, ShimmerDirection, ShimmerSpeed,
    SlideDirection,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotionDurationToken {
    Instant,
    Fast,
    Default,
    Slow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotionEasingToken {
    Linear,
    Standard,
    Emphasized,
    Decelerate,
    Accelerate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotionDistanceToken {
    Compact,
    Default,
    Spacious,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReducedMotionPolicy {
    Respect,
    ForceReduced,
    Ignore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotionDisableContext {
    Storybook,
    Test,
    StaticExport,
    OverlayInsideOverlay,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotionSpec {
    pub primitive: MotionPrimitive,
    pub duration: MotionDurationToken,
    pub easing: MotionEasingToken,
    pub distance: MotionDistanceToken,
    pub policy: ReducedMotionPolicy,
    pub disable_in: Vec<MotionDisableContext>,
}

impl MotionSpec {
    #[must_use]
    pub fn new(
        primitive: MotionPrimitiveKind,
        duration_ms: u16,
        distance_px: u16,
        policy: ReducedMotionPolicy,
    ) -> Self {
        Self {
            primitive: legacy_primitive(primitive),
            duration: token_from_duration(duration_ms),
            easing: standard_easing(),
            distance: token_from_distance(distance_px),
            policy,
            disable_in: Vec::new(),
        }
    }

    #[must_use]
    pub const fn fade(
        duration: MotionDurationToken,
        easing: MotionEasingToken,
        from: f32,
        to: f32,
    ) -> Self {
        Self::from_parts(
            MotionPrimitive::Fade { from, to },
            duration,
            easing,
            MotionDistanceToken::Compact,
        )
    }

    #[must_use]
    pub const fn slide(
        duration: MotionDurationToken,
        easing: MotionEasingToken,
        distance: MotionDistanceToken,
        direction: SlideDirection,
    ) -> Self {
        Self::from_parts(
            MotionPrimitive::Slide {
                distance,
                direction,
            },
            duration,
            easing,
            distance,
        )
    }

    #[must_use]
    pub const fn scale(
        duration: MotionDurationToken,
        easing: MotionEasingToken,
        from: f32,
        to: f32,
        origin: ScaleOrigin,
    ) -> Self {
        Self::from_parts(
            MotionPrimitive::Scale { from, to, origin },
            duration,
            easing,
            MotionDistanceToken::Compact,
        )
    }

    #[must_use]
    pub const fn shimmer(
        duration: MotionDurationToken,
        easing: MotionEasingToken,
        speed: ShimmerSpeed,
        direction: ShimmerDirection,
    ) -> Self {
        Self::from_parts(
            MotionPrimitive::Shimmer { speed, direction },
            duration,
            easing,
            MotionDistanceToken::Spacious,
        )
    }

    #[must_use]
    pub fn disabled_in(mut self, context: MotionDisableContext) -> Self {
        self.disable_in.push(context);
        self
    }

    #[must_use]
    pub fn policy(mut self, policy: ReducedMotionPolicy) -> Self {
        self.policy = policy;
        self
    }

    const fn from_parts(
        primitive: MotionPrimitive,
        duration: MotionDurationToken,
        easing: MotionEasingToken,
        distance: MotionDistanceToken,
    ) -> Self {
        Self {
            primitive,
            duration,
            easing,
            distance,
            policy: ReducedMotionPolicy::Respect,
            disable_in: Vec::new(),
        }
    }
}
