use super::super::UiTreeSurfaceHost;
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
fn interaction_target_at_does_not_visit_the_offscreen_content_tail() {
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

    let target = UiTreeSurfaceHost::new(ThemeSnapshot::dark())
        .interaction_target_at(
            &root,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 120,
                height: 40,
                scroll_y: 0.0,
            },
            60.0,
            20.0,
        )
        .expect("visible button interaction target");
    assert_eq!(
        "button-0",
        target.action.expect("visible button action").action_id
    );
    let visited_node_count = UiTreeHostActionHitCollector::viewport_interaction_visit_count();
    assert!(
        (2..10).contains(&visited_node_count),
        "interaction collection must stop before building the 1,000-child tail; visited={visited_node_count}"
    );
}

#[test]
fn nested_viewport_scroll_hit_collection_reports_the_inner_visited_nodes() {
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
    let inner = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 120,
            viewport_height: 40,
            content_height: 40_000,
            ..UiScrollAreaProps::default()
        })
        .child(column);
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 120,
            viewport_height: 40,
            content_height: 40,
            ..UiScrollAreaProps::default()
        })
        .child(inner);

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
        (3..10).contains(&visited_node_count),
        "nested viewport traversal must propagate its counted nodes without visiting the tail; visited={visited_node_count}"
    );
}

#[test]
fn outer_scroll_does_not_shift_a_visible_nested_scroll_area() {
    let inner = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 120,
            viewport_height: 20,
            content_height: 40,
            ..UiScrollAreaProps::default()
        })
        .child(
            UiNode::new(UiNodeKind::Column, "")
                .child(
                    UiNode::from(Button::new("first"))
                        .height(UiDimension::px(20))
                        .host_action(UiHostActionSpec::command("first", "first")),
                )
                .child(
                    UiNode::from(Button::new("second"))
                        .height(UiDimension::px(20))
                        .host_action(UiHostActionSpec::command("second", "second")),
                ),
        );
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
                .child(UiNode::new(UiNodeKind::Text, "spacer").height(UiDimension::px(20)))
                .child(inner),
        );

    let target = UiTreeSurfaceHost::new(ThemeSnapshot::dark())
        .interaction_target_at(
            &root,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: 120,
                height: 20,
                scroll_y: 0.0,
            },
            60.0,
            10.0,
        )
        .expect("visible nested button interaction target");

    assert_eq!("first", target.action.expect("nested action").action_id);
}
