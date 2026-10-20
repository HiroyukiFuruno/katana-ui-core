use super::*;
use crate::test_assert::KucTestExpect;

#[test]
fn host_action_hits_clip_left_outset_to_the_viewport() {
    let root = UiNode::from(Button::new("embedded"))
        .height(UiDimension::px(20))
        .hit_outset_left(16)
        .host_action(UiHostActionSpec::command("embedded", "embedded"));
    let area = UiTreeRenderArea {
        x: 20,
        y: 0,
        width: 120,
        height: 20,
        scroll_y: 0.0,
    };
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());

    let hit = renderer
        .host_action_hit_rects(&root, area)
        .into_iter()
        .next()
        .kuc_expect("viewport action hit");

    assert_eq!(20, hit.rect.x);
    assert!(
        renderer
            .host_action_hit_at(&root, area, 19.0, 10.0)
            .is_none()
    );
    assert_eq!(
        Some(hit),
        renderer.host_action_hit_at(&root, area, 20.0, 10.0)
    );
}
