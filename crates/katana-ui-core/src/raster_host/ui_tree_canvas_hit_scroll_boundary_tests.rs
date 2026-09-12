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
