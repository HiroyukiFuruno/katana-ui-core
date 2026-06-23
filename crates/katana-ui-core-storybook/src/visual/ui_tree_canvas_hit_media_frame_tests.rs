use super::*;
use crate::test_assert::KucTestExpect;
use crate::visual::ui_tree_canvas_text_metrics::UiTreeDocumentTypography;

#[test]
fn media_frame_image_surface_emits_rendered_image_hit_for_semantic_viewer_target() {
    let frame = ImageSurface::from_rgba("surface", "surface", 4, 2, [0, 255, 0, 255].repeat(8))
        .kuc_expect("image surface should be valid");
    let root = UiNode::from(frame)
        .visual_role(UiVisualRole::MediaFrame)
        .height(UiDimension::Px(40));
    let common = root
        .props()
        .common
        .clone()
        .semantic_node_id("viewer-diagram-node");
    let root = root.common(common);

    let hits = UiTreeHostActionHitCollector::collect_node_hits_with_renderers(
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 16,
            height: 42,
            scroll_y: 0.0,
        },
        &TextRenderer::load(&UiCoreFacade::default(), "body"),
        &TextRenderer::load(&UiCoreFacade::default(), "body"),
        &TextRenderer::load(&UiCoreFacade::default(), "code"),
        UiTreeDocumentTypography::default(),
    );

    let hit = hits
        .iter()
        .find(|hit| {
            hit.semantic_node_id
                .as_ref()
                .is_some_and(|id| id.as_str() == "viewer-diagram-node")
        })
        .kuc_expect("media frame semantic node hit");
    assert_eq!(6, hit.rect.x);
    assert_eq!(19, hit.rect.y);
    assert_eq!(4, hit.rect.width);
    assert_eq!(2, hit.rect.height);
}

#[test]
fn media_frame_centers_image_and_places_overlay_controls_on_frame_right_edge() {
    let frame = ImageSurface::from_rgba(
        "surface",
        "surface",
        400,
        80,
        [0, 255, 0, 255].repeat(400 * 80),
    )
    .kuc_expect("image surface should be valid");
    let image = UiNode::from(frame)
        .visual_role(UiVisualRole::MediaFrame)
        .height(UiDimension::Px(120));
    let root = UiNode::from(
        Stack::new().child(image).child(
            UiNode::from(Button::new("Zoom"))
                .width(UiDimension::px(20))
                .height(UiDimension::px(20))
                .host_action(UiHostActionSpec::surface_control("ui.surface.zoom", "Zoom"))
                .position(UiPosition::Absolute)
                .margin(top_right_overlay_margin())
                .z_index(UiZIndex::value(2)),
        ),
    )
    .height(UiDimension::Px(120));
    let common = root
        .props()
        .common
        .clone()
        .semantic_node_id("viewer-diagram-node");
    let root = root.common(common);

    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 1000,
        height: 160,
        scroll_y: 0.0,
    };
    let hits = UiTreeHostActionHitCollector::collect(&root, area);
    let node_hits = UiTreeHostActionHitCollector::collect_node_hits_with_renderers(
        &root,
        area,
        &TextRenderer::load(&UiCoreFacade::default(), "body"),
        &TextRenderer::load(&UiCoreFacade::default(), "body"),
        &TextRenderer::load(&UiCoreFacade::default(), "code"),
        UiTreeDocumentTypography::default(),
    );

    let image_hit = node_hits
        .iter()
        .find(|hit| {
            hit.semantic_node_id
                .as_ref()
                .is_some_and(|id| id.as_str() == "viewer-diagram-node")
        })
        .kuc_expect("media frame semantic node hit");
    assert_eq!(
        300, image_hit.rect.x,
        "diagram image must be centered in the full media frame width"
    );
    assert_eq!(
        972, hits[0].rect.x,
        "diagram overlay controls must attach to the full media frame right edge, not the image edge"
    );
    assert!(
        hits[0].rect.x > image_hit.rect.x + image_hit.rect.width,
        "control should live at the frame right edge outside a narrow centered image"
    );
}

#[test]
fn absolute_overlay_controls_do_not_expand_parent_semantic_node_hit() {
    let frame = ImageSurface::from_rgba("surface", "surface", 4, 2, [0, 255, 0, 255].repeat(8))
        .kuc_expect("image surface should be valid");
    let image = UiNode::from(frame)
        .visual_role(UiVisualRole::MediaFrame)
        .height(UiDimension::Px(40));
    let root = UiNode::from(
        Stack::new().child(image).child(
            UiNode::from(Button::new("Zoom"))
                .width(UiDimension::px(4))
                .height(UiDimension::px(4))
                .host_action(UiHostActionSpec::surface_control("ui.surface.zoom", "Zoom"))
                .position(UiPosition::Absolute)
                .margin(bottom_right_overlay_margin())
                .z_index(UiZIndex::value(2)),
        ),
    )
    .height(UiDimension::Px(40));
    let common = root
        .props()
        .common
        .clone()
        .semantic_node_id("viewer-diagram-node");
    let root = root.common(common);

    let hits = UiTreeHostActionHitCollector::collect_node_hits_with_renderers(
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 16,
            height: 42,
            scroll_y: 0.0,
        },
        &TextRenderer::load(&UiCoreFacade::default(), "body"),
        &TextRenderer::load(&UiCoreFacade::default(), "body"),
        &TextRenderer::load(&UiCoreFacade::default(), "code"),
        UiTreeDocumentTypography::default(),
    );

    let semantic_hits = hits
        .iter()
        .filter(|hit| {
            hit.semantic_node_id
                .as_ref()
                .is_some_and(|id| id.as_str() == "viewer-diagram-node")
        })
        .collect::<Vec<_>>();

    assert_eq!(1, semantic_hits.len());
    assert_eq!(6, semantic_hits[0].rect.x);
    assert_eq!(4, semantic_hits[0].rect.width);
}
