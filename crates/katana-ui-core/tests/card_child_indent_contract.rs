use katana_ui_core::render_model::{UiCommonProps, UiNode, UiNodeKind};

#[test]
fn card_child_indent_round_trips_through_public_render_model()
-> Result<(), Box<dyn std::error::Error>> {
    for indent in [0, 24, u16::MAX] {
        let node =
            UiNode::new(UiNodeKind::Card, "").common(UiCommonProps::default().child_indent(indent));
        let json = serde_json::to_string(&node)?;
        let restored: UiNode = serde_json::from_str(&json)?;
        assert_eq!(Some(indent), restored.props().common.child_indent);
    }
    Ok(())
}

#[test]
fn legacy_common_json_preserves_default_card_indent() -> Result<(), Box<dyn std::error::Error>> {
    let mut json = serde_json::to_value(UiCommonProps::default())?;
    json.as_object_mut()
        .ok_or("props object")?
        .remove("child_indent");
    let props: UiCommonProps = serde_json::from_value(json)?;
    assert_eq!(None, props.child_indent);
    Ok(())
}

#[cfg(feature = "raster-host")]
#[test]
fn card_body_retains_requested_width_and_hit_coordinates() -> Result<(), Box<dyn std::error::Error>>
{
    use katana_ui_core::raster_host::{UiTreeCanvasRenderer, UiTreeRenderArea};
    use katana_ui_core::theme::ThemeSnapshot;
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    let area = UiTreeRenderArea {
        x: 20,
        y: 0,
        width: 560,
        height: 200,
        scroll_y: 0.0,
    };
    for (indent, expected_x, expected_width) in
        [(None, 36, 560), (Some(0), 20, 560), (Some(24), 44, 560)]
    {
        let common = UiCommonProps {
            child_indent: indent,
            ..UiCommonProps::default()
        };
        let child = UiNode::new(UiNodeKind::Text, "body");
        let child_id = child.id().clone();
        let root = UiNode::new(UiNodeKind::Card, "")
            .common(common)
            .child(child);
        let hits = renderer.document_node_hit_rects(&root, area);
        let hit = hits
            .iter()
            .find(|hit| hit.node_id == child_id)
            .ok_or("body hit")?;
        assert_eq!(expected_x, hit.rect.x);
        assert_eq!(expected_width, hit.rect.width);
        assert!(hit.contains_point(expected_x as f32, hit.rect.y as f32));
        let viewport_hits = renderer.viewport_node_hit_rects(&root, area);
        let viewport_hit = viewport_hits
            .iter()
            .find(|hit| hit.node_id == child_id)
            .ok_or("viewport body hit")?;
        assert_eq!(expected_x, viewport_hit.rect.x);
        assert_eq!(580 - expected_x, viewport_hit.rect.width);
    }
    Ok(())
}

#[cfg(feature = "raster-host")]
#[test]
fn card_indent_keeps_padding_additive_and_viewport_hits_aligned()
-> Result<(), Box<dyn std::error::Error>> {
    use katana_ui_core::raster_host::{UiTreeCanvasRenderer, UiTreeRenderArea};
    use katana_ui_core::render_model::{UiDimension, UiEdgeInsets};
    use katana_ui_core::theme::ThemeSnapshot;
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    let area = UiTreeRenderArea {
        x: 20,
        y: 0,
        width: 560,
        height: 200,
        scroll_y: 0.0,
    };
    for indent in [0, 24] {
        let child = UiNode::new(UiNodeKind::Text, "body");
        let child_id = child.id().clone();
        let root = UiNode::new(UiNodeKind::Card, "")
            .common(
                UiCommonProps::default()
                    .child_indent(indent)
                    .padding(UiEdgeInsets {
                        left: UiDimension::Px(7),
                        right: UiDimension::Px(9),
                        ..UiEdgeInsets::default()
                    }),
            )
            .child(child);
        for (viewport, hits) in [
            (false, renderer.document_node_hit_rects(&root, area)),
            (true, renderer.viewport_node_hit_rects(&root, area)),
        ] {
            let hit = hits
                .iter()
                .find(|hit| hit.node_id == child_id)
                .ok_or("body hit")?;
            assert_eq!(27 + usize::from(indent), hit.rect.x);
            let expected_width = if viewport {
                544.min(553 - usize::from(indent))
            } else {
                544
            };
            assert_eq!(expected_width, hit.rect.width);
        }
    }
    Ok(())
}
