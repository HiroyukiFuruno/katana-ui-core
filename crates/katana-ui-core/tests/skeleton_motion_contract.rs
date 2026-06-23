use katana_ui_core::atom::{Skeleton, SkeletonAnimation, SkeletonShape, SkeletonSize};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{
    MotionContext, MotionDisableContext, MotionPrimitiveKind, MotionResolver, MotionSpec,
    ReducedMotionPolicy, UiAction,
};
use katana_ui_core::molecule::{MotionPrimitive, SkeletonCluster, SkeletonClusterPreset};
use katana_ui_core::render_model::{UiDimension, UiNodeKind, UiSkeletonAnimation, UiTree};
use katana_ui_core::theme::ThemeSnapshot;

const SLIDE_DURATION_MS: u16 = 180;
const SLIDE_DISTANCE_PX: u16 = 12;

#[test]
fn skeleton_respects_shape_size_animation_and_reduced_motion() {
    let skeleton = Skeleton::new("Avatar", SkeletonShape::Circle)
        .size(SkeletonSize::Fixed {
            width: UiDimension::px(48),
            height: UiDimension::px(48),
        })
        .animation(SkeletonAnimation::Shimmer)
        .reduced_motion(true);

    assert_eq!(SkeletonAnimation::None, skeleton.effective_animation());
    let tree = UiTree::new(skeleton);
    assert_eq!(UiNodeKind::Skeleton, tree.root().kind());
    assert_eq!(UiDimension::px(48), tree.root().props().common.width);
    assert_eq!(UiDimension::px(48), tree.root().props().common.height);
    assert!(tree.root().props().loading_indicator.reduced_motion);
}

#[test]
fn skeleton_cluster_owns_single_live_region_and_preset() {
    let cluster = SkeletonCluster::new("Loading messages")
        .preset(SkeletonClusterPreset::Message)
        .item(Skeleton::new("avatar", SkeletonShape::Circle))
        .item(Skeleton::new(
            "line",
            SkeletonShape::Text {
                lines: 1,
                last_line_ratio: 1.0,
            },
        ));
    assert_eq!("loading", cluster.live_region_label());

    let tree = UiTree::new(cluster);
    assert_eq!(UiNodeKind::SkeletonCluster, tree.root().kind());
    assert_eq!("Loading messages", tree.root().props().accessibility_label);
    assert_eq!(2, tree.root().children().len());
}

#[test]
fn skeleton_cluster_public_options_control_live_region_and_reduced_motion() {
    let tree = UiTree::new(
        SkeletonCluster::new("image card")
            .preset(SkeletonClusterPreset::ImageCard)
            .live_region("Loading custom image card")
            .reduced_motion(true),
    );

    let root = tree.root();
    assert_eq!(
        "Loading custom image card",
        root.props().accessibility_label
    );
    assert!(root.props().loading_indicator.reduced_motion);
    assert!(
        root.children()
            .iter()
            .all(|it| it.props().loading_indicator.reduced_motion)
    );
    assert!(
        root.children()
            .iter()
            .all(|it| it.props().skeleton.animation == UiSkeletonAnimation::None)
    );
}

#[test]
fn skeleton_size_variants_map_to_common_dimensions() {
    let auto = UiTree::new(Skeleton::new("auto", text_shape()).size(SkeletonSize::Auto));
    let fill = UiTree::new(Skeleton::new("fill", text_shape()).size(SkeletonSize::Fill));
    let fixed = UiTree::new(
        Skeleton::new("fixed", text_shape()).size(SkeletonSize::Fixed {
            width: UiDimension::px(80),
            height: UiDimension::px(12),
        }),
    );

    assert_eq!(UiDimension::Auto, auto.root().props().common.width);
    assert_eq!(UiDimension::Fill, fill.root().props().common.width);
    assert_eq!(UiDimension::Fill, fill.root().props().common.height);
    assert_eq!(UiDimension::px(80), fixed.root().props().common.width);
    assert_eq!(UiDimension::px(12), fixed.root().props().common.height);
}

fn text_shape() -> SkeletonShape {
    SkeletonShape::Text {
        lines: 1,
        last_line_ratio: 1.0,
    }
}

#[test]
fn compute_motion_respects_reduced_policy_and_context_disable() {
    let spec = MotionSpec::new(
        MotionPrimitiveKind::Slide,
        SLIDE_DURATION_MS,
        SLIDE_DISTANCE_PX,
        ReducedMotionPolicy::Respect,
    )
    .disabled_in(MotionDisableContext::Storybook);

    let disabled = MotionResolver::compute(
        &spec,
        MotionContext {
            reduced_motion: false,
            surface: MotionDisableContext::Storybook,
        },
    );
    assert!(disabled.instant);
    assert_eq!(0, disabled.duration_ms);

    let active = MotionResolver::compute(
        &spec,
        MotionContext {
            reduced_motion: false,
            surface: MotionDisableContext::Test,
        },
    );
    assert!(!active.instant);
    assert_eq!(200, active.duration_ms);
}

#[test]
fn motion_policy_can_force_or_ignore_reduced_motion() {
    let forced = MotionSpec::new(
        MotionPrimitiveKind::Fade,
        SLIDE_DURATION_MS,
        SLIDE_DISTANCE_PX,
        ReducedMotionPolicy::ForceReduced,
    );
    let ignored = MotionSpec::new(
        MotionPrimitiveKind::Fade,
        SLIDE_DURATION_MS,
        SLIDE_DISTANCE_PX,
        ReducedMotionPolicy::Ignore,
    );
    let reduced_context = MotionContext {
        reduced_motion: true,
        surface: MotionDisableContext::Test,
    };

    assert!(MotionResolver::compute(&forced, reduced_context).instant);
    assert!(!MotionResolver::compute(&ignored, reduced_context).instant);
}

#[test]
fn motion_primitive_routes_reduced_motion_action() {
    let spec = MotionSpec::new(
        MotionPrimitiveKind::Scale,
        SLIDE_DURATION_MS,
        SLIDE_DISTANCE_PX,
        ReducedMotionPolicy::Respect,
    );
    let mut motion = MotionPrimitive::new("Panel motion", spec);
    let action = UiAction::reduced_motion(motion.state_id().clone(), true);

    assert!(motion.apply_action(&action).handled);
    assert_eq!(0, motion.effective_duration_ms());
}

#[test]
fn theme_snapshot_exposes_motion_tokens() {
    let dark = ThemeSnapshot::dark();

    assert!(dark.motion.iter().any(|token| token.name == "fast"));
    assert!(dark.motion.iter().any(|token| token.name == "default"));
}
