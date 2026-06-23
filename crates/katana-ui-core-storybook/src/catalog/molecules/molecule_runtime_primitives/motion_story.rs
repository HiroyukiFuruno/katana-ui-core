use super::{
    ComponentAction, MOTION_PHASE, MotionContext, MotionDefaults, MotionDisableContext,
    MotionDistanceToken, MotionDurationToken, MotionEasingToken, MotionResolver, MotionSpec,
    MotionTarget, ReducedMotionPolicy, ScaleOrigin, ShimmerDirection, ShimmerSpeed, SlideDirection,
    StoryCatalog, StoryExample, ThemeSnapshot, UiAction, UiCallbackLog, UiStateId, atom, layout,
    molecule,
};

const MOTION_SCALE_REDUCED_START: f32 = 0.96;

pub(super) fn motion_story() -> StoryExample {
    let spec = MotionSpec::slide(
        MotionDurationToken::Default,
        MotionEasingToken::Emphasized,
        MotionDistanceToken::Default,
        SlideDirection::Up,
    );
    let mut motion = molecule::MotionPrimitive::new("Motion primitive", spec);
    let target = motion.state_id().clone();
    let reduced = motion.apply_action(&UiAction::reduced_motion(target.clone(), true));
    let tick = motion.apply_action(&UiAction::animation_tick(target.clone(), MOTION_PHASE));
    let tokens = ThemeSnapshot::dark().motion_tokens();
    let force = MotionResolver::compute_with_theme(
        &MotionSpec::fade(
            MotionDurationToken::Default,
            MotionEasingToken::Standard,
            0.0,
            1.0,
        )
        .policy(ReducedMotionPolicy::ForceReduced),
        MotionContext::for_test(false),
        &tokens,
    );
    let ignore = MotionResolver::compute_with_theme(
        &MotionSpec::scale(
            MotionDurationToken::Default,
            MotionEasingToken::Emphasized,
            MOTION_SCALE_REDUCED_START,
            1.0,
            ScaleOrigin::Center,
        )
        .policy(ReducedMotionPolicy::Ignore),
        MotionContext::for_test(true),
        &tokens,
    );
    let override_snapshot = MotionResolver::compute_with_theme(
        &MotionDefaults::for_target(MotionTarget::Popover),
        MotionContext::new(false, MotionDisableContext::Test),
        &tokens,
    );
    let logs = vec![
        UiCallbackLog::new(
            UiStateId::new("state:MotionPrimitive:storybook"),
            "motion_reduce",
            "instant=false",
            format!("events={:?}", reduced.callback_log),
        ),
        UiCallbackLog::new(
            UiStateId::new("state:MotionPrimitive:storybook"),
            "motion_tick",
            "phase=0",
            format!("events={:?}", tick.callback_log),
        ),
        UiCallbackLog::new(
            target.clone(),
            "motion_force",
            "policy=Respect",
            format!("instant={} duration={}", force.instant, force.duration_ms),
        ),
        UiCallbackLog::new(
            target.clone(),
            "motion_ignore",
            "prefers_reduced_motion=true",
            format!("diagnostics={}", ignore.diagnostics),
        ),
        UiCallbackLog::new(
            target,
            "motion_override",
            "target=Modal default",
            format!("target=Popover duration={}", override_snapshot.duration_ms),
        ),
    ];
    StoryCatalog::interactive_story(
        "motion",
        layout::Column::new()
            .child(motion)
            .child(atom::Text::new("primitive: Fade Slide Scale Shimmer"))
            .child(atom::Text::new(
                "tokens: duration=Default easing=Emphasized distance=Default",
            ))
            .child(atom::Text::new(
                "state: instant=false duration=200 distance=8",
            ))
            .child(atom::Text::new(
                "event: reduced_motion_query override=Ignore context=Storybook",
            ))
            .child(atom::Text::new(
                "action: motion_reduce motion_tick motion_force motion_ignore motion_override",
            ))
            .child(atom::Text::new(
                "quality: token_resolution reduced_static override_isolated",
            ))
            .child(atom::Text::new(format!(
                "typed: {:?} {:?} {:?}",
                MotionSpec::fade(
                    MotionDurationToken::Fast,
                    MotionEasingToken::Standard,
                    0.0,
                    1.0,
                )
                .primitive,
                MotionSpec::scale(
                    MotionDurationToken::Default,
                    MotionEasingToken::Decelerate,
                    0.96,
                    1.0,
                    ScaleOrigin::Center,
                )
                .primitive,
                MotionSpec::shimmer(
                    MotionDurationToken::Slow,
                    MotionEasingToken::Linear,
                    ShimmerSpeed::Default,
                    ShimmerDirection::LeftToRight,
                )
                .primitive
            ))),
        logs,
    )
}
