use crate::test_assert::KucTestExpect;
use crate::visual::canvas::Canvas;
use crate::visual::ui_tree_canvas::UiTreeCanvasRenderer;
use crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette;
use crate::visual::ui_tree_canvas_tests::pixel_at;
use crate::visual::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::atom::ImageSurface;
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind};
use katana_ui_core::theme::ThemeSnapshot;

#[test]
fn explicit_height_container_clips_image_surface_children() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(240, 260, palette.background);
    let image: UiNode = ImageSurface::from_rgba(
        "tall",
        "fingerprint",
        100,
        600,
        [0, 0, 0, 255].repeat(100 * 600),
    )
    .kuc_expect("valid image surface")
    .into();
    let root = UiNode::new(UiNodeKind::Stack, "")
        .height(UiDimension::Px(180))
        .child(image);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 240,
            height: 260,
            scroll_y: 0.0,
        },
    );

    assert_ne!(Some(palette.background), pixel_at(&canvas, 10, 179));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 10, 180));
}
