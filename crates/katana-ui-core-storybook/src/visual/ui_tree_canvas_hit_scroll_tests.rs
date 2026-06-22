use super::*;
use crate::test_assert::KucTestExpect;

#[test]
fn scroll_area_button_hit_rect_uses_rendered_child_origin_without_container_indent() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(220, 80, palette.background);
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 220,
            viewport_height: 80,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::from(Button::new("zoom"))
                .width(UiDimension::px(44))
                .height(UiDimension::px(44))
                .host_action(UiHostActionSpec::command("surface.action.primary", "open")),
        );
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 220,
        height: 80,
        scroll_y: 0.0,
    };

    let hits = UiTreeHostActionHitCollector::collect(&root, area);
    UiTreeCanvasRenderer::new(theme).render(&mut canvas, &root, area);

    let (min_x, min_y, width, height) =
        bounds_for_color(&canvas, palette.selection).kuc_expect("expected a painted button area");
    assert_eq!(min_x, hits[0].rect.x);
    assert_eq!(min_y, hits[0].rect.y);
    assert_eq!(width, hits[0].rect.width);
    assert_eq!(height, hits[0].rect.height);
}

#[test]
fn scroll_area_button_hit_rects_follow_rendered_scroll_offset() {
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 120,
            viewport_height: 20,
            offset_y: 20,
            content_height: 40,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(
                    UiNode::from(Button::new("first"))
                        .width(UiDimension::px(80))
                        .height(UiDimension::px(20))
                        .host_action(UiHostActionSpec::command("first", "first")),
                )
                .child(
                    UiNode::from(Button::new("second"))
                        .width(UiDimension::px(80))
                        .height(UiDimension::px(20))
                        .host_action(UiHostActionSpec::command("second", "second")),
                ),
        );

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 4,
            y: 8,
            width: 120,
            height: 20,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len());
    assert_eq!("second", hits[0].action.action_id);
    assert_eq!(
        UiTreeHitRect {
            x: 4,
            y: 8,
            width: 80,
            height: 20,
        },
        hits[0].rect
    );
}

#[test]
fn root_scroll_area_hit_rects_and_render_keep_viewport_origin_when_scrolled() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 120,
            viewport_height: 60,
            content_height: 80,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(UiNode::from(Text::new("hidden")).height(UiDimension::px(20)))
                .child(
                    UiNode::from(Button::new("second"))
                        .width(UiDimension::px(60))
                        .height(UiDimension::px(20))
                        .host_action(UiHostActionSpec::command("second", "second")),
                ),
        );
    let area = UiTreeRenderArea {
        x: 10,
        y: 40,
        width: 120,
        height: 60,
        scroll_y: 20.0,
    };
    let renderer = UiTreeCanvasRenderer::new(theme);

    let hits = renderer.host_action_hit_rects(&root, area);
    let hit = hits
        .first()
        .kuc_expect("scrolled button hit must be collected");
    assert_eq!("second", hit.action.action_id);
    assert_eq!(
        UiTreeHitRect {
            x: 10,
            y: 40,
            width: 60,
            height: 20,
        },
        hit.rect
    );

    let mut canvas = Canvas::new(180, 120, palette.background);
    renderer.render(&mut canvas, &root, area);

    assert_eq!(
        palette.selection,
        canvas.pixels()[hit.rect.y * canvas.width() + hit.rect.x]
    );
    assert_eq!(
        palette.background,
        canvas.pixels()[hit.rect.y.saturating_sub(1) * canvas.width() + hit.rect.x]
    );
}

#[test]
fn scroll_area_document_hit_rects_include_offscreen_children_without_viewport_clip() {
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 120,
            viewport_height: 20,
            content_height: 60,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(
                    UiNode::from(Button::new("first"))
                        .width(UiDimension::px(80))
                        .height(UiDimension::px(20))
                        .host_action(UiHostActionSpec::command("first", "first")),
                )
                .child(
                    UiNode::from(Button::new("second"))
                        .width(UiDimension::px(80))
                        .height(UiDimension::px(20))
                        .host_action(UiHostActionSpec::command("second", "second")),
                )
                .child(
                    UiNode::from(Button::new("third"))
                        .width(UiDimension::px(80))
                        .height(UiDimension::px(20))
                        .host_action(UiHostActionSpec::command("third", "third")),
                ),
        );
    let area = UiTreeRenderArea {
        x: 4,
        y: 8,
        width: 120,
        height: 20,
        scroll_y: 0.0,
    };

    let clipped = renderer.host_action_hit_rects(&root, area);
    let document = renderer.document_host_action_hit_rects(&root, area);

    assert_eq!(vec!["first"], action_ids(&clipped));
    assert_eq!(vec!["first", "second", "third"], action_ids(&document));
    assert_eq!(8, document[0].rect.y);
    assert_eq!(28, document[1].rect.y);
    assert_eq!(48, document[2].rect.y);
}

#[test]
fn container_padding_offsets_hit_rect_like_rendered_child_origin() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(220, 100, palette.background);
    let root = UiNode::new(UiNodeKind::Column, "")
        .common(UiCommonProps::default().padding(UiEdgeInsets {
            top: UiDimension::px(12),
            right: UiDimension::px(8),
            bottom: UiDimension::px(4),
            left: UiDimension::px(16),
        }))
        .child(
            UiNode::from(Button::new("zoom"))
                .width(UiDimension::px(44))
                .height(UiDimension::px(44))
                .host_action(UiHostActionSpec::command("surface.action.primary", "open")),
        );
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 220,
        height: 100,
        scroll_y: 0.0,
    };

    let hits = UiTreeHostActionHitCollector::collect(&root, area);
    UiTreeCanvasRenderer::new(theme).render(&mut canvas, &root, area);

    let (min_x, min_y, width, height) =
        bounds_for_color(&canvas, palette.selection).kuc_expect("expected a painted button area");
    assert_eq!(min_x, hits[0].rect.x);
    assert_eq!(min_y, hits[0].rect.y);
    assert_eq!(width, hits[0].rect.width);
    assert_eq!(height, hits[0].rect.height);
}

#[test]
fn explicit_button_width_is_used_for_row_slot_layout() {
    let root = UiNode::from(
        Row::new()
            .child(
                UiNode::from(Button::new("left"))
                    .width(UiDimension::px(36))
                    .height(UiDimension::px(20))
                    .host_action(UiHostActionSpec::command("left", "left")),
            )
            .child(
                UiNode::from(Button::new("right"))
                    .width(UiDimension::px(44))
                    .height(UiDimension::px(20))
                    .host_action(UiHostActionSpec::command("right", "right")),
            ),
    );
    assert_eq!(
        UiDimension::Px(36),
        root.children()[0].props().common.width,
        "first button width should be explicitly set"
    );
    assert_eq!(
        UiDimension::Px(44),
        root.children()[1].props().common.width,
        "second button width should be explicitly set"
    );

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 240,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert_eq!(2, hits.len());
    assert_eq!(0, hits[0].rect.x);
    assert_eq!(36, hits[1].rect.x);
    assert_eq!(44, hits[1].rect.width);
}
