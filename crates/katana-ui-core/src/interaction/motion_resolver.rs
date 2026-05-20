use super::{MotionDisableContext, MotionPrimitive, MotionSpec, ReducedMotionPolicy};
use crate::accessibility::ReducedMotionQuery;
use crate::theme::MotionTokenSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotionContext {
    pub reduced_motion: bool,
    pub surface: MotionDisableContext,
}

impl MotionContext {
    #[must_use]
    pub const fn new(reduced_motion: bool, surface: MotionDisableContext) -> Self {
        Self {
            reduced_motion,
            surface,
        }
    }

    #[must_use]
    pub const fn for_test(reduced_motion: bool) -> Self {
        Self::new(reduced_motion, MotionDisableContext::Test)
    }

    #[must_use]
    pub const fn from_reduced_motion_query(
        query: ReducedMotionQuery,
        surface: MotionDisableContext,
    ) -> Self {
        Self::new(query.prefers_reduced_motion(), surface)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotionSnapshot {
    pub primitive: MotionPrimitive,
    pub duration_ms: u16,
    pub distance_px: u16,
    pub easing: String,
    pub instant: bool,
    pub diagnostics: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MotionResolver;

impl MotionResolver {
    #[must_use]
    pub fn compute(spec: &MotionSpec, context: MotionContext) -> MotionSnapshot {
        let tokens = MotionTokenSet::default();
        Self::compute_with_theme(spec, context, &tokens)
    }

    #[must_use]
    pub fn compute_with_theme(
        spec: &MotionSpec,
        context: MotionContext,
        tokens: &MotionTokenSet,
    ) -> MotionSnapshot {
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
                easing: tokens.easing(spec.easing).to_string(),
                instant: true,
                diagnostics: downgrade_reason(spec.policy, disabled_by_context, context),
            };
        }
        MotionSnapshot {
            primitive: spec.primitive,
            duration_ms: tokens.duration(spec.duration),
            distance_px: tokens.distance(spec.distance),
            easing: tokens.easing(spec.easing).to_string(),
            instant: false,
            diagnostics: active_reason(spec.policy, context),
        }
    }
}

fn downgrade_reason(
    policy: ReducedMotionPolicy,
    disabled_by_context: bool,
    context: MotionContext,
) -> String {
    if disabled_by_context {
        return format!("context={:?}", context.surface);
    }
    match policy {
        ReducedMotionPolicy::ForceReduced => "policy=ForceReduced".to_string(),
        ReducedMotionPolicy::Respect => "prefers_reduced_motion=true".to_string(),
        ReducedMotionPolicy::Ignore => String::new(),
    }
}

fn active_reason(policy: ReducedMotionPolicy, context: MotionContext) -> String {
    if policy == ReducedMotionPolicy::Ignore && context.reduced_motion {
        return "override=Ignore".to_string();
    }
    String::new()
}
