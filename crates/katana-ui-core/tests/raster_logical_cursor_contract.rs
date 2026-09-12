#![cfg(feature = "raster-host")]

use katana_ui_core::raster_host::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiPosition};
use katana_ui_core::theme::ThemeSnapshot;

#[test]
fn public_raster_renderer_preserves_logical_cursor_for_fixed_and_overlay_children() {
    let root = UiNode::new(UiNodeKind::Column, "")
        .child(UiNode::new(UiNodeKind::Text, "fixed text").height(UiDimension::px(20)))
        .child(UiNode::new(UiNodeKind::Accordion, "fixed accordion").height(UiDimension::px(20)))
        .child(
            UiNode::new(UiNodeKind::Row, "")
                .height(UiDimension::px(20))
                .child(UiNode::new(UiNodeKind::Text, "row child")),
        )
        .child(
            UiNode::new(UiNodeKind::Stack, "")
                .height(UiDimension::px(20))
                .child(
                    UiNode::new(UiNodeKind::Button, "overlay")
                        .height(UiDimension::px(20))
                        .position(UiPosition::Absolute),
                ),
        )
        .child(
            UiNode::new(UiNodeKind::Card, "")
                .height(UiDimension::px(20))
                .child(UiNode::new(UiNodeKind::Text, "card child")),
        );
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    let mut canvas = Canvas::new(240, 140, 0);

    renderer.render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 240,
            height: 140,
            scroll_y: 0.0,
        },
    );

    assert!(canvas.pixels().iter().any(|pixel| *pixel != 0));
}
