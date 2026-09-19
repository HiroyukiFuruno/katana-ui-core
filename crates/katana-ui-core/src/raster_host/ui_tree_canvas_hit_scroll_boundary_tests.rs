use super::*;

#[test]
fn fixed_height_incremental_scroll_container_stops_child_hit_collection_at_its_bottom() {
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 120,
            viewport_height: 40,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .height(UiDimension::px(10))
                .child(
                    UiNode::from(Button::new("visible"))
                        .height(UiDimension::px(10))
                        .host_action(UiHostActionSpec::command("visible", "visible")),
                )
                .child(
                    UiNode::from(Button::new("clipped"))
                        .height(UiDimension::px(10))
                        .host_action(UiHostActionSpec::command("clipped", "clipped")),
                ),
        );
    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 120,
            height: 40,
            scroll_y: 0.0,
        },
    );
    assert_eq!(vec!["visible"], action_ids(&hits));
}

#[test]
fn automatic_scroll_container_stops_hits_at_the_viewport_without_content_height() {
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 120,
            viewport_height: 40,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(
                    UiNode::from(Button::new("visible"))
                        .height(UiDimension::px(40))
                        .host_action(UiHostActionSpec::command("visible", "visible")),
                )
                .child(
                    UiNode::from(Button::new("outside viewport"))
                        .height(UiDimension::px(40))
                        .host_action(UiHostActionSpec::command("outside", "outside")),
                ),
        );
    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 120,
            height: 40,
            scroll_y: 0.0,
        },
    );
    assert_eq!(vec!["visible"], action_ids(&hits));
    assert_eq!(40, hits[0].rect.height);
}

#[test]
fn viewport_scroll_hit_collection_does_not_visit_the_offscreen_content_tail() {
    let mut column = UiNode::new(UiNodeKind::Column, "");
    for index in 0..1_000 {
        column = column.child(
            UiNode::from(Button::new(format!("button-{index}")))
                .height(UiDimension::px(40))
                .host_action(UiHostActionSpec::command(
                    format!("button-{index}"),
                    "button",
                )),
        );
    }
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 120,
            viewport_height: 40,
            content_height: 40_000,
            ..UiScrollAreaProps::default()
        })
        .child(column);

    let (hits, visited_node_count) = UiTreeHostActionHitCollector::collect_with_visit_count(
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 120,
            height: 40,
            scroll_y: 0.0,
        },
    );

    assert_eq!(vec!["button-0"], action_ids(&hits));
    assert!(
        visited_node_count < 10,
        "viewport collection must stop before building the 1,000-child tail; visited={visited_node_count}"
    );
}
