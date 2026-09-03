use super::*;
use crate::test_assert::KucTestExpect;

#[test]
fn absolute_controls_follow_explicit_button_dimensions_for_overlay_position() {
    let frame = ImageSurface::from_rgba("surface", "surface", 1, 1, vec![0x20, 0x24, 0x28, 0xff])
        .kuc_expect("image surface should be valid");
    let root = UiNode::from(
        Stack::new()
            .child(
                UiNode::from(frame)
                    .visual_role(UiVisualRole::MediaFrame)
                    .height(UiDimension::Px(180)),
            )
            .child(
                UiNode::from(
                    Row::new()
                        .child(
                            UiNode::from(Button::new("primary"))
                                .width(UiDimension::Px(44))
                                .height(UiDimension::Px(44))
                                .host_action(UiHostActionSpec::command(
                                    "surface.overlay.primary",
                                    "primary",
                                )),
                        )
                        .child(
                            UiNode::from(Button::new("secondary"))
                                .width(UiDimension::Px(44))
                                .height(UiDimension::Px(44))
                                .host_action(UiHostActionSpec::command(
                                    "surface.overlay.secondary",
                                    "secondary",
                                )),
                        ),
                )
                .position(UiPosition::Absolute)
                .margin(bottom_right_overlay_margin())
                .z_index(UiZIndex::value(2)),
            )
            .child(
                UiNode::from(
                    Row::new()
                        .child(
                            UiNode::from(Button::new("tertiary"))
                                .width(UiDimension::Px(44))
                                .height(UiDimension::Px(44))
                                .host_action(UiHostActionSpec::command(
                                    "surface.overlay.tertiary",
                                    "tertiary",
                                )),
                        )
                        .child(
                            UiNode::from(Button::new("quaternary"))
                                .width(UiDimension::Px(44))
                                .height(UiDimension::Px(44))
                                .host_action(UiHostActionSpec::command(
                                    "surface.overlay.quaternary",
                                    "quaternary",
                                )),
                        ),
                )
                .position(UiPosition::Absolute)
                .margin(top_right_overlay_margin())
                .z_index(UiZIndex::value(2)),
            ),
    )
    .height(UiDimension::Px(180));

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 200,
            height: 220,
            scroll_y: 0.0,
        },
    );

    let bottom_first = hits
        .iter()
        .find(|it| it.action.action_id == "surface.overlay.primary")
        .kuc_expect("bottom overlay controls should be collected");
    let top_first = hits
        .iter()
        .find(|it| it.action.action_id == "surface.overlay.tertiary")
        .kuc_expect("top overlay controls should be collected");

    assert_eq!(104, bottom_first.rect.x);
    assert_eq!(104, top_first.rect.x);
    assert_eq!(128, bottom_first.rect.y);
    assert_eq!(8, top_first.rect.y);
}

#[test]
fn absolute_overlay_icon_button_keeps_transparent_base_without_chrome() {
    const MEDIA_BACKGROUND: u32 = 0x202428;
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let frame = ImageSurface::from_rgba("surface", "surface", 1, 1, vec![0x20, 0x24, 0x28, 0xff])
        .kuc_expect("image surface should be valid");
    let root = UiNode::from(
        Stack::new()
            .child(
                UiNode::from(frame)
                    .visual_role(UiVisualRole::MediaFrame)
                    .height(UiDimension::Px(180)),
            )
            .child(
                UiNode::from(
                    Row::new().child(
                        UiNode::from(Button::new("F"))
                            .variant(UiVariant::Icon)
                            .width(UiDimension::Px(44))
                            .height(UiDimension::Px(44))
                            .host_action(UiHostActionSpec::command(
                                "surface.overlay.primary",
                                "primary",
                            )),
                    ),
                )
                .position(UiPosition::Absolute)
                .margin(top_right_overlay_margin())
                .z_index(UiZIndex::value(2)),
            ),
    )
    .height(UiDimension::Px(180));
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 200,
        height: 220,
        scroll_y: 0.0,
    };
    let hits = UiTreeHostActionHitCollector::collect(&root, area);
    let hit = hits
        .iter()
        .find(|it| it.action.action_id == "surface.overlay.primary")
        .kuc_expect("overlay control should be collected");

    let mut canvas = Canvas::new(200, 220, MEDIA_BACKGROUND);
    UiTreeCanvasRenderer::new(theme).render(&mut canvas, &root, area);

    let top_left = canvas.pixels()[hit.rect.y * canvas.width() + hit.rect.x];
    let interior = canvas.pixels()[(hit.rect.y + 2) * canvas.width() + hit.rect.x + 2];
    assert_eq!(
        palette.background, top_left,
        "absolute overlay icon button border must inherit media frame background until hover"
    );
    assert_eq!(
        palette.background, interior,
        "absolute overlay button fill must stay transparent"
    );
}
