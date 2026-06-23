use katana_ui_core::accessibility::{ReducedMotionPreference, ReducedMotionQuery};
use katana_ui_core::interaction::{
    MotionContext, MotionDisableContext, MotionDistanceToken, MotionDurationToken,
    MotionEasingToken, MotionPrimitive, MotionResolver, MotionSpec, ReducedMotionPolicy,
    ScaleOrigin, ShimmerDirection, ShimmerSpeed, SlideDirection,
};
use katana_ui_core::molecule::{MotionDefaults, MotionTarget};
use katana_ui_core::theme::ThemeSnapshot;

#[test]
fn motion_spec_resolves_duration_easing_and_distance_from_tokens() {
    let theme = ThemeSnapshot::dark();
    let spec = MotionSpec::slide(
        MotionDurationToken::Default,
        MotionEasingToken::Emphasized,
        MotionDistanceToken::Default,
        SlideDirection::Up,
    );

    let snapshot = MotionResolver::compute_with_theme(
        &spec,
        MotionContext::for_test(false),
        &theme.motion_tokens(),
    );

    assert_eq!(200, snapshot.duration_ms);
    assert_eq!(8, snapshot.distance_px);
    assert_eq!("emphasized", snapshot.easing.as_str());
    assert_eq!(
        MotionPrimitive::Slide {
            distance: MotionDistanceToken::Default,
            direction: SlideDirection::Up,
        },
        snapshot.primitive
    );
}

#[test]
fn motion_primitives_keep_typed_parameters() {
    let fade = MotionSpec::fade(
        MotionDurationToken::Fast,
        MotionEasingToken::Standard,
        0.0,
        1.0,
    );
    let scale = MotionSpec::scale(
        MotionDurationToken::Default,
        MotionEasingToken::Decelerate,
        0.96,
        1.0,
        ScaleOrigin::Center,
    );
    let shimmer = MotionSpec::shimmer(
        MotionDurationToken::Slow,
        MotionEasingToken::Linear,
        ShimmerSpeed::Default,
        ShimmerDirection::LeftToRight,
    );

    assert!(matches!(
        fade.primitive,
        MotionPrimitive::Fade { from: 0.0, to: 1.0 }
    ));
    assert!(matches!(
        scale.primitive,
        MotionPrimitive::Scale {
            origin: ScaleOrigin::Center,
            ..
        }
    ));
    assert!(matches!(
        shimmer.primitive,
        MotionPrimitive::Shimmer {
            speed: ShimmerSpeed::Default,
            direction: ShimmerDirection::LeftToRight
        }
    ));
}

#[test]
fn reduced_motion_query_downgrades_respect_policy_and_ignore_logs_override() {
    let theme = ThemeSnapshot::light();
    let query = ReducedMotionQuery::new(ReducedMotionPreference::Reduce);
    let respect = MotionSpec::fade(
        MotionDurationToken::Default,
        MotionEasingToken::Standard,
        0.0,
        1.0,
    );
    let ignore = respect.clone().policy(ReducedMotionPolicy::Ignore);

    let reduced = MotionResolver::compute_with_theme(
        &respect,
        MotionContext::from_reduced_motion_query(query, MotionDisableContext::Test),
        &theme.motion_tokens(),
    );
    let ignored = MotionResolver::compute_with_theme(
        &ignore,
        MotionContext::from_reduced_motion_query(query, MotionDisableContext::Test),
        &theme.motion_tokens(),
    );

    assert!(reduced.instant);
    assert_eq!(0, reduced.duration_ms);
    assert!(!ignored.instant);
    assert_eq!("override=Ignore", ignored.diagnostics.as_str());
}

#[test]
fn overlay_inside_overlay_only_downgrades_inner_motion() {
    let theme = ThemeSnapshot::dark();
    let modal = MotionDefaults::for_target(MotionTarget::Modal);
    let popover = MotionDefaults::for_target(MotionTarget::Popover)
        .disabled_in(MotionDisableContext::OverlayInsideOverlay);

    let outer = MotionResolver::compute_with_theme(
        &modal,
        MotionContext::new(false, MotionDisableContext::Test),
        &theme.motion_tokens(),
    );
    let inner = MotionResolver::compute_with_theme(
        &popover,
        MotionContext::new(false, MotionDisableContext::OverlayInsideOverlay),
        &theme.motion_tokens(),
    );

    assert!(!outer.instant);
    assert!(inner.instant);
    assert_eq!("context=OverlayInsideOverlay", inner.diagnostics.as_str());
}

#[test]
fn all_required_molecules_have_documented_default_motion() {
    let theme = ThemeSnapshot::dark();

    for target in MotionTarget::required_molecules() {
        let spec = MotionDefaults::for_target(*target);
        let snapshot = MotionResolver::compute_with_theme(
            &spec,
            MotionContext::for_test(false),
            &theme.motion_tokens(),
        );

        assert!(
            snapshot.duration_ms > 0 || *target == MotionTarget::Skeleton,
            "{target:?} must resolve a default motion duration"
        );
    }
}
