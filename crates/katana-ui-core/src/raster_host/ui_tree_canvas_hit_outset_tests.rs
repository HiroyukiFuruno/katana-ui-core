use super::super::UiTreeSurfaceHost;
use super::*;

#[test]
fn viewport_interaction_hit_outset_clips_to_viewport_and_preserves_document_origin() {
    let root = UiNode::from(Button::new("embedded"))
        .height(UiDimension::px(20))
        .common(UiCommonProps::default().hit_outset_left(16))
        .host_action(UiHostActionSpec::command("embedded", "embedded"));
    let area = UiTreeRenderArea {
        x: 20,
        y: 0,
        width: 120,
        height: 20,
        scroll_y: 0.0,
    };
    let host = UiTreeSurfaceHost::new(ThemeSnapshot::dark());

    let (actions, nodes) = host.viewport_interaction_hits(&root, area);
    let document_actions = host.document_host_action_hits(&root, area);
    let document_nodes = host.document_node_hits(&root, area);

    assert_eq!(20, actions[0].rect.x);
    assert_eq!(20, nodes[0].rect.x);
    assert_eq!(4, document_actions[0].rect.x);
    assert_eq!(4, document_nodes[0].rect.x);
}

#[test]
fn viewport_interaction_hit_outset_uses_root_left_boundary_inside_nested_card() {
    let root = UiNode::new(UiNodeKind::Card, "").child(
        UiNode::new(UiNodeKind::Column, "").child(
            UiNode::from(Button::new("embedded"))
                .height(UiDimension::px(20))
                .common(UiCommonProps::default().hit_outset_left(16))
                .host_action(UiHostActionSpec::command("embedded", "embedded")),
        ),
    );
    let area = UiTreeRenderArea {
        x: 20,
        y: 0,
        width: 120,
        height: 20,
        scroll_y: 0.0,
    };
    let host = UiTreeSurfaceHost::new(ThemeSnapshot::dark());

    let (actions, nodes) = host.viewport_interaction_hits(&root, area);

    assert_eq!(20, actions[0].rect.x);
    assert_eq!(20, nodes[0].rect.x);
}
