use super::*;
use crate::test_assert::KucTestExpect;

#[test]
fn absolute_overlay_collects_bottom_right_control_without_style_class() {
    let root = UiNode::from(
        Stack::new()
            .child(
                UiNode::from(Button::new("media"))
                    .height(UiDimension::px(80))
                    .width(UiDimension::px(160)),
            )
            .child(
                UiNode::from(
                    Row::new().child(
                        UiNode::from(Button::new("Zoom"))
                            .width(UiDimension::px(96))
                            .height(UiDimension::px(20))
                            .host_action(UiHostActionSpec::surface_control(
                                "surface.overlay.primary",
                                "Open",
                            )),
                    ),
                )
                .position(UiPosition::Absolute)
                .margin(bottom_right_overlay_margin())
                .z_index(UiZIndex::value(2)),
            ),
    )
    .height(UiDimension::px(180));

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 500,
            height: 300,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len());
    assert_eq!("surface.overlay.primary", hits[0].action.action_id);
    assert_eq!(396, hits[0].rect.x);
    assert_eq!(152, hits[0].rect.y);
}

#[test]
fn explicit_stack_action_uses_stack_rect_without_stealing_child_button() {
    let root = UiNode::from(
        Stack::new()
            .child(
                UiNode::from(Button::new("content"))
                    .width(UiDimension::px(180))
                    .height(UiDimension::px(90)),
            )
            .child(
                UiNode::from(Button::new("Close"))
                    .width(UiDimension::px(32))
                    .height(UiDimension::px(32))
                    .host_action(UiHostActionSpec::surface_control(
                        "surface.overlay.close",
                        "Close",
                    ))
                    .position(UiPosition::Absolute)
                    .margin(top_right_overlay_margin())
                    .z_index(UiZIndex::value(2)),
            ),
    )
    .width(UiDimension::px(640))
    .height(UiDimension::px(360))
    .host_action(UiHostActionSpec::surface_control(
        "surface.overlay.backdrop",
        "Close",
    ));

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 640,
            height: 360,
            scroll_y: 0.0,
        },
    );

    assert_eq!(
        vec!["surface.overlay.backdrop", "surface.overlay.close"],
        action_ids(&hits)
    );
    let backdrop = hits
        .iter()
        .find(|hit| hit.action.action_id == "surface.overlay.backdrop")
        .kuc_expect("backdrop hit");
    assert_eq!(0, backdrop.rect.x);
    assert_eq!(0, backdrop.rect.y);
    assert_eq!(640, backdrop.rect.width);
    assert_eq!(360, backdrop.rect.height);
    let close = hits
        .iter()
        .find(|hit| hit.action.action_id == "surface.overlay.close")
        .kuc_expect("close hit");
    assert!(close.rect.area() < backdrop.rect.area());
}

#[test]
fn explicit_button_dimensions_are_used_for_both_hit_and_render_rects() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(220, 120, palette.background);
    let root = UiNode::from(
        Row::new().child(
            UiNode::from(Button::new("zoom"))
                .width(UiDimension::px(44))
                .height(UiDimension::px(44))
                .host_action(UiHostActionSpec::command("surface.action.primary", "open")),
        ),
    );

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 4,
            y: 8,
            width: 320,
            height: 200,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len());
    assert_eq!(44, hits[0].rect.width);
    assert_eq!(44, hits[0].rect.height);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 4,
            y: 8,
            width: 320,
            height: 120,
            scroll_y: 0.0,
        },
    );

    let (min_x, min_y, width, height) =
        bounds_for_color(&canvas, palette.selection).kuc_expect("expected a painted button area");
    assert_eq!(hits[0].rect.x, min_x);
    assert_eq!(hits[0].rect.y, min_y);
    assert_eq!(hits[0].rect.width, width);
    assert_eq!(hits[0].rect.height, height);
}
