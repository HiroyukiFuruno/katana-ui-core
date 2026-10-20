use katana_ui_core::render_model::{UiCommonProps, UiNode, UiNodeKind};

#[test]
fn hit_outset_round_trips_and_legacy_json_defaults_to_zero()
-> Result<(), Box<dyn std::error::Error>> {
    for outset in [0, 16, u16::MAX] {
        let node = UiNode::new(UiNodeKind::Text, "body").hit_outset_left(outset);
        let restored: UiNode = serde_json::from_str(&serde_json::to_string(&node)?)?;
        assert_eq!(outset, restored.hit_outset_left_px());
    }
    let mut legacy = serde_json::to_value(UiNode::new(UiNodeKind::Text, "body"))?;
    legacy
        .as_object_mut()
        .ok_or("object")?
        .remove("hit_outset_left");
    let restored: UiNode = serde_json::from_value(legacy)?;
    assert_eq!(0, restored.hit_outset_left_px());
    Ok(())
}

#[cfg(feature = "raster-host")]
mod raster {
    use super::*;
    use katana_ui_core::raster_host::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
    use katana_ui_core::render_model::{UiHostActionSpec, UiNode, UiNodeKind, UiScrollAreaProps};
    use katana_ui_core::theme::ThemeSnapshot;

    fn card(outset: u16) -> UiNode {
        UiNode::new(UiNodeKind::Card, "")
            .common(UiCommonProps::default().child_indent(16))
            .child(
                UiNode::new(UiNodeKind::Text, "body text ".repeat(100))
                    .stable_node_id("body")
                    .hit_outset_left(outset)
                    .host_action(UiHostActionSpec::command("body.open", "open")),
            )
            .child(UiNode::new(UiNodeKind::Text, "tail").stable_node_id("tail"))
    }

    fn area() -> UiTreeRenderArea {
        UiTreeRenderArea {
            x: 20,
            y: 0,
            width: 560,
            height: 300,
            scroll_y: 0.0,
        }
    }

    #[test]
    fn same_body_id_keeps_full_viewport_hit_without_changing_draw_or_layout()
    -> Result<(), Box<dyn std::error::Error>> {
        let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
        let base = card(0);
        let expanded = card(16);
        let mut before = Canvas::new(600, 300, 0);
        let mut after = Canvas::new(600, 300, 0);
        renderer.render(&mut before, &base, area());
        renderer.render(&mut after, &expanded, area());
        assert_eq!(before.pixels(), after.pixels());
        for (viewport, before_hits, after_hits) in [
            (
                false,
                renderer.document_node_hit_rects(&base, area()),
                renderer.document_node_hit_rects(&expanded, area()),
            ),
            (
                true,
                renderer.viewport_node_hit_rects(&base, area()),
                renderer.viewport_node_hit_rects(&expanded, area()),
            ),
        ] {
            let before_hit = before_hits
                .iter()
                .find(|hit| hit.node_id.as_str() == "body")
                .ok_or("base body")?;
            let after_hit = after_hits
                .iter()
                .find(|hit| hit.node_id.as_str() == "body")
                .ok_or("body")?;
            assert_eq!(36, before_hit.rect.x);
            assert_eq!(20, after_hit.rect.x);
            assert_eq!(if viewport { 560 } else { 576 }, after_hit.rect.width);
            assert_eq!(before_hit.rect.y, after_hit.rect.y);
            assert_eq!(before_hit.rect.height, after_hit.rect.height);
            assert!(after_hit.contains_point(20.0, after_hit.rect.y as f32));
            let tail_before = before_hits
                .iter()
                .find(|hit| hit.node_id.as_str() == "tail");
            let tail_after = after_hits.iter().find(|hit| hit.node_id.as_str() == "tail");
            assert_eq!(tail_before, tail_after);
        }
        let actions = renderer.viewport_host_action_hit_rects(&expanded, area());
        let action = actions
            .iter()
            .find(|hit| hit.action.action_id == "body.open")
            .ok_or("body action")?;
        assert_eq!((20, 560), (action.rect.x, action.rect.width));
        Ok(())
    }

    #[test]
    fn hit_outset_saturates_at_document_origin() -> Result<(), Box<dyn std::error::Error>> {
        let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
        let hits = renderer.document_node_hit_rects(&card(u16::MAX), area());
        let hit = hits
            .iter()
            .find(|hit| hit.node_id.as_str() == "body")
            .ok_or("body")?;
        assert_eq!((0, 596), (hit.rect.x, hit.rect.width));
        Ok(())
    }

    #[test]
    fn scroll_measurement_and_clip_preserve_visual_layout_and_expanded_hit()
    -> Result<(), Box<dyn std::error::Error>> {
        let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
        for offset_y in [0, 20, 60] {
            let scroll = |outset| {
                UiNode::new(UiNodeKind::ScrollArea, "")
                    .scroll_area(UiScrollAreaProps {
                        viewport_width: 560,
                        viewport_height: 100,
                        content_height: 400,
                        offset_y,
                        ..UiScrollAreaProps::default()
                    })
                    .child(card(outset))
            };
            let mut before = Canvas::new(600, 300, 0);
            let mut after = Canvas::new(600, 300, 0);
            renderer.render(&mut before, &scroll(0), area());
            renderer.render(&mut after, &scroll(16), area());
            assert_eq!(before.pixels(), after.pixels());
            let hits = renderer.viewport_node_hit_rects(&scroll(16), area());
            let hit = hits
                .iter()
                .find(|hit| hit.node_id.as_str() == "body")
                .ok_or("scroll body")?;
            assert_eq!((20, 560), (hit.rect.x, hit.rect.width));
            assert!(hit.rect.y + hit.rect.height <= 100);
        }
        Ok(())
    }
}
