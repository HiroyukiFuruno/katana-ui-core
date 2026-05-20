#[path = "motion_primitive.rs"]
mod motion_primitive;
#[path = "motion_resolver.rs"]
mod motion_resolver;
#[path = "motion_tokens.rs"]
mod motion_tokens;

pub use motion_primitive::{
    MotionPrimitive, MotionPrimitiveKind, ScaleOrigin, ShimmerDirection, ShimmerSpeed,
    SlideDirection,
};
pub use motion_resolver::{MotionContext, MotionResolver, MotionSnapshot};
pub use motion_tokens::{
    MotionDisableContext, MotionDistanceToken, MotionDurationToken, MotionEasingToken, MotionSpec,
    ReducedMotionPolicy,
};
