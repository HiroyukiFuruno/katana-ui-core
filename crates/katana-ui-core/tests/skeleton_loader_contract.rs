use katana_ui_core::atom::{Skeleton, SkeletonAnimation, SkeletonShape, SkeletonSize};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::molecule::{SkeletonCluster, SkeletonClusterPreset};
use katana_ui_core::render_model::{
    UiDimension, UiNode, UiNodeKind, UiSkeletonAnimation, UiSkeletonShape, UiTone, UiTree,
};

#[test]
fn text_shape_renders_stable_lines_with_short_last_line() -> Result<(), String> {
    let tree = UiTree::new(
        Skeleton::new(
            "paragraph",
            SkeletonShape::Text {
                lines: 3,
                last_line_ratio: 0.6,
            },
        )
        .size(SkeletonSize::Fixed {
            width: UiDimension::px(240),
            height: UiDimension::px(72),
        }),
    );

    let root = tree.root();
    assert_eq!(UiNodeKind::Skeleton, root.kind());
    assert_eq!(UiSkeletonShape::Text, root.props().skeleton.shape);
    let children = root.children();
    if children.len() != 3 {
        return Err("text skeleton must render three lines".to_string());
    }
    assert_line(&children[0], UiDimension::percent(100));
    assert_line(&children[1], UiDimension::percent(100));
    assert_line(&children[2], UiDimension::percent(60));
    Ok(())
}

#[test]
fn shape_animation_and_tone_are_render_props_not_text_markers() {
    let cases = [
        (
            SkeletonShape::Rect,
            SkeletonAnimation::None,
            UiSkeletonShape::Rect,
            UiSkeletonAnimation::None,
        ),
        (
            SkeletonShape::Circle,
            SkeletonAnimation::Pulse,
            UiSkeletonShape::Circle,
            UiSkeletonAnimation::Pulse,
        ),
        (
            SkeletonShape::Line { thickness: 6.0 },
            SkeletonAnimation::Shimmer,
            UiSkeletonShape::Line,
            UiSkeletonAnimation::Shimmer,
        ),
        (
            SkeletonShape::Text {
                lines: 2,
                last_line_ratio: 0.75,
            },
            SkeletonAnimation::Wave,
            UiSkeletonShape::Text,
            UiSkeletonAnimation::Wave,
        ),
    ];

    for (shape, animation, expected_shape, expected_animation) in cases {
        let node = UiTree::new(
            Skeleton::new("loading", shape)
                .animation(animation)
                .tone(UiTone::Accent)
                .radius_px(8),
        );

        assert_eq!(expected_shape, node.root().props().skeleton.shape);
        assert_eq!(expected_animation, node.root().props().skeleton.animation);
        assert_eq!(UiTone::Accent, node.root().props().tone);
        assert_eq!(8, node.root().props().skeleton.radius_px);
    }
}

#[test]
fn reduced_motion_records_animation_downgrade_in_render_props() {
    let tree = UiTree::new(
        Skeleton::new("loading", SkeletonShape::Rect)
            .animation(SkeletonAnimation::Shimmer)
            .reduced_motion(true),
    );

    assert_eq!(
        UiSkeletonAnimation::None,
        tree.root().props().skeleton.animation
    );
    assert_eq!(
        "downgraded:Shimmer->None",
        tree.root().props().interaction.value
    );
}

#[test]
fn cluster_presets_emit_stable_children_and_single_live_region() {
    let expectations = [
        (SkeletonClusterPreset::Card, 2, "Card"),
        (SkeletonClusterPreset::ListRow, 2, "ListRow"),
        (SkeletonClusterPreset::Message, 3, "Message"),
        (SkeletonClusterPreset::Paragraph, 1, "Paragraph"),
        (SkeletonClusterPreset::ImageCard, 3, "ImageCard"),
        (SkeletonClusterPreset::CodeBlock, 5, "CodeBlock"),
    ];

    for (preset, child_count, style_class) in expectations {
        let cluster = SkeletonCluster::new("messages").preset(preset);
        assert!(!cluster.state_id().as_str().is_empty());
        let tree = UiTree::new(cluster);
        let root = tree.root();

        assert_eq!(UiNodeKind::SkeletonCluster, root.kind());
        assert_eq!("Loading messages", root.props().accessibility_label);
        assert_eq!(child_count, root.children().len(), "{preset:?}");
        assert!(
            root.props()
                .style_classes
                .contains(&style_class.to_string())
        );
        assert!(
            root.children()
                .iter()
                .all(|it| it.props().accessibility_label.is_empty())
        );
    }
}

#[test]
fn skeleton_identity_accessibility_aspect_default_and_actions_are_typed() {
    assert!(matches!(
        SkeletonShape::default(),
        SkeletonShape::Text {
            lines: 1,
            last_line_ratio: 1.0,
        }
    ));

    let mut skeleton = Skeleton::new("media", SkeletonShape::Rect)
        .accessibility_label("Loading preview")
        .aspect_ratio(16, 9);
    let state_id = skeleton.state_id().clone();
    let other = Skeleton::new("other", SkeletonShape::Circle);

    assert!(
        !skeleton
            .apply_action(&UiAction::reduced_motion(other.state_id().clone(), true))
            .handled
    );
    assert!(
        !skeleton
            .apply_action(&UiAction::focus(state_id.clone()))
            .handled
    );
    assert!(
        skeleton
            .apply_action(&UiAction::reduced_motion(state_id, true))
            .handled
    );

    let node = UiNode::from(skeleton);
    assert_eq!("Loading preview", node.props().accessibility_label);
    assert_eq!(16, node.props().skeleton.aspect_ratio_width);
    assert_eq!(9, node.props().skeleton.aspect_ratio_height);
    assert_eq!(UiSkeletonAnimation::None, node.props().skeleton.animation);
}

#[test]
fn code_block_preset_uses_varying_line_widths() {
    let tree = UiTree::new(SkeletonCluster::new("code").preset(SkeletonClusterPreset::CodeBlock));
    let widths: Vec<UiDimension> = tree
        .root()
        .children()
        .iter()
        .map(|it| it.props().common.width.clone())
        .collect();

    assert_eq!(
        vec![
            UiDimension::percent(100),
            UiDimension::percent(92),
            UiDimension::percent(76),
            UiDimension::percent(88),
            UiDimension::percent(64),
        ],
        widths
    );
}

fn assert_line(node: &UiNode, width: UiDimension) {
    assert_eq!(UiNodeKind::Skeleton, node.kind());
    assert_eq!(UiSkeletonShape::Line, node.props().skeleton.shape);
    assert_eq!(width, node.props().common.width);
}
