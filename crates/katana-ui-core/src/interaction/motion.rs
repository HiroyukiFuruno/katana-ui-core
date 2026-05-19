use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotionPrimitiveKind {
    Fade,
    Slide,
    Scale,
    Shimmer,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotionSpec {
    pub primitive: MotionPrimitiveKind,
    pub duration_ms: u16,
    pub distance_px: u16,
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
            primitive,
            duration_ms,
            distance_px,
            policy,
            disable_in: Vec::new(),
        }
    }

    #[must_use]
    pub fn disabled_in(mut self, context: MotionDisableContext) -> Self {
        self.disable_in.push(context);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotionContext {
    pub reduced_motion: bool,
    pub surface: MotionDisableContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotionSnapshot {
    pub primitive: MotionPrimitiveKind,
    pub duration_ms: u16,
    pub distance_px: u16,
    pub instant: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MotionResolver;

impl MotionResolver {
    #[must_use]
    pub fn compute(spec: &MotionSpec, context: MotionContext) -> MotionSnapshot {
        let disabled_by_context = spec.disable_in.contains(&context.surface);
        let reduced = match spec.policy {
            ReducedMotionPolicy::ForceReduced => true,
            ReducedMotionPolicy::Respect => context.reduced_motion,
            ReducedMotionPolicy::Ignore => false,
        };
        if reduced || disabled_by_context {
            return MotionSnapshot {
                primitive: spec.primitive,
                duration_ms: 0,
                distance_px: 0,
                instant: true,
            };
        }
        MotionSnapshot {
            primitive: spec.primitive,
            duration_ms: spec.duration_ms,
            distance_px: spec.distance_px,
            instant: false,
        }
    }
}
