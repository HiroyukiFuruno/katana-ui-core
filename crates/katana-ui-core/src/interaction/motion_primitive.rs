use super::{MotionDistanceToken, MotionDurationToken, MotionEasingToken};
use serde::{Deserialize, Serialize};

const FADE_START_OPACITY: f32 = 0.0;
const FADE_END_OPACITY: f32 = 1.0;
const SCALE_START_RATIO: f32 = 0.96;
const SCALE_END_RATIO: f32 = 1.0;
const INSTANT_DURATION_MS: u16 = 0;
const FAST_DURATION_MIN_MS: u16 = 1;
const FAST_DURATION_MAX_MS: u16 = 160;
const DEFAULT_DURATION_MIN_MS: u16 = 161;
const DEFAULT_DURATION_MAX_MS: u16 = 260;
const COMPACT_DISTANCE_MAX_PX: u16 = 6;
const DEFAULT_DISTANCE_MIN_PX: u16 = 7;
const DEFAULT_DISTANCE_MAX_PX: u16 = 12;

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MotionPrimitiveResolver;

impl MotionPrimitiveResolver {
    pub(crate) fn legacy_primitive(kind: MotionPrimitiveKind) -> MotionPrimitive {
        match kind {
            MotionPrimitiveKind::Fade => MotionPrimitive::Fade {
                from: FADE_START_OPACITY,
                to: FADE_END_OPACITY,
            },
            MotionPrimitiveKind::Slide => MotionPrimitive::Slide {
                distance: MotionDistanceToken::Default,
                direction: SlideDirection::Up,
            },
            MotionPrimitiveKind::Scale => MotionPrimitive::Scale {
                from: SCALE_START_RATIO,
                to: SCALE_END_RATIO,
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
            INSTANT_DURATION_MS => MotionDurationToken::Instant,
            FAST_DURATION_MIN_MS..=FAST_DURATION_MAX_MS => MotionDurationToken::Fast,
            DEFAULT_DURATION_MIN_MS..=DEFAULT_DURATION_MAX_MS => MotionDurationToken::Default,
            _ => MotionDurationToken::Slow,
        }
    }

    pub(crate) fn token_from_distance(distance_px: u16) -> MotionDistanceToken {
        match distance_px {
            0..=COMPACT_DISTANCE_MAX_PX => MotionDistanceToken::Compact,
            DEFAULT_DISTANCE_MIN_PX..=DEFAULT_DISTANCE_MAX_PX => MotionDistanceToken::Default,
            _ => MotionDistanceToken::Spacious,
        }
    }

    pub(crate) const fn standard_easing() -> MotionEasingToken {
        MotionEasingToken::Standard
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_kinds_and_legacy_numeric_boundaries_are_total() {
        for (primitive, kind) in [
            (
                MotionPrimitive::Fade { from: 0.0, to: 1.0 },
                MotionPrimitiveKind::Fade,
            ),
            (
                MotionPrimitive::Slide {
                    distance: MotionDistanceToken::Compact,
                    direction: SlideDirection::Down,
                },
                MotionPrimitiveKind::Slide,
            ),
            (
                MotionPrimitive::Scale {
                    from: 0.9,
                    to: 1.0,
                    origin: ScaleOrigin::Top,
                },
                MotionPrimitiveKind::Scale,
            ),
            (
                MotionPrimitive::Shimmer {
                    speed: ShimmerSpeed::Fast,
                    direction: ShimmerDirection::RightToLeft,
                },
                MotionPrimitiveKind::Shimmer,
            ),
        ] {
            assert_eq!(kind, primitive.kind());
            assert_eq!(kind, MotionPrimitiveResolver::legacy_primitive(kind).kind());
        }

        for (duration, token) in [
            (0, MotionDurationToken::Instant),
            (1, MotionDurationToken::Fast),
            (160, MotionDurationToken::Fast),
            (161, MotionDurationToken::Default),
            (260, MotionDurationToken::Default),
            (261, MotionDurationToken::Slow),
        ] {
            assert_eq!(
                token,
                MotionPrimitiveResolver::token_from_duration(duration)
            );
        }
        for (distance, token) in [
            (0, MotionDistanceToken::Compact),
            (6, MotionDistanceToken::Compact),
            (7, MotionDistanceToken::Default),
            (12, MotionDistanceToken::Default),
            (13, MotionDistanceToken::Spacious),
        ] {
            assert_eq!(
                token,
                MotionPrimitiveResolver::token_from_distance(distance)
            );
        }
        assert_eq!(
            MotionEasingToken::Standard,
            MotionPrimitiveResolver::standard_easing()
        );
    }
}
