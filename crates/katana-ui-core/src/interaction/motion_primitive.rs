use super::{MotionDistanceToken, MotionDurationToken, MotionEasingToken};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotionPrimitiveKind {
    Fade,
    Slide,
    Scale,
    Shimmer,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MotionPrimitive {
    Fade {
        from: f32,
        to: f32,
    },
    Slide {
        distance: MotionDistanceToken,
        direction: SlideDirection,
    },
    Scale {
        from: f32,
        to: f32,
        origin: ScaleOrigin,
    },
    Shimmer {
        speed: ShimmerSpeed,
        direction: ShimmerDirection,
    },
}

impl MotionPrimitive {
    #[must_use]
    pub const fn kind(self) -> MotionPrimitiveKind {
        match self {
            Self::Fade { .. } => MotionPrimitiveKind::Fade,
            Self::Slide { .. } => MotionPrimitiveKind::Slide,
            Self::Scale { .. } => MotionPrimitiveKind::Scale,
            Self::Shimmer { .. } => MotionPrimitiveKind::Shimmer,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlideDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScaleOrigin {
    Center,
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShimmerSpeed {
    Slow,
    Default,
    Fast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShimmerDirection {
    LeftToRight,
    RightToLeft,
}

pub(crate) fn legacy_primitive(kind: MotionPrimitiveKind) -> MotionPrimitive {
    match kind {
        MotionPrimitiveKind::Fade => MotionPrimitive::Fade { from: 0.0, to: 1.0 },
        MotionPrimitiveKind::Slide => MotionPrimitive::Slide {
            distance: MotionDistanceToken::Default,
            direction: SlideDirection::Up,
        },
        MotionPrimitiveKind::Scale => MotionPrimitive::Scale {
            from: 0.96,
            to: 1.0,
            origin: ScaleOrigin::Center,
        },
        MotionPrimitiveKind::Shimmer => MotionPrimitive::Shimmer {
            speed: ShimmerSpeed::Default,
            direction: ShimmerDirection::LeftToRight,
        },
    }
}

pub(crate) fn token_from_duration(duration_ms: u16) -> MotionDurationToken {
    match duration_ms {
        0 => MotionDurationToken::Instant,
        1..=160 => MotionDurationToken::Fast,
        161..=260 => MotionDurationToken::Default,
        _ => MotionDurationToken::Slow,
    }
}

pub(crate) fn token_from_distance(distance_px: u16) -> MotionDistanceToken {
    match distance_px {
        0..=6 => MotionDistanceToken::Compact,
        7..=12 => MotionDistanceToken::Default,
        _ => MotionDistanceToken::Spacious,
    }
}

pub(crate) fn standard_easing() -> MotionEasingToken {
    MotionEasingToken::Standard
}
