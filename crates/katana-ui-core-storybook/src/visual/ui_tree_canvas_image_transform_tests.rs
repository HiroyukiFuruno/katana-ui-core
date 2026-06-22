use super::{
    Canvas, UiTreeCanvasRenderer, UiTreeRenderArea, visual_interaction_test_support::pixel_at,
};
use crate::test_assert::KucTestExpect;
use crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette;
use katana_ui_core::atom::ImageSurface;
use katana_ui_core::render_model::{UiDimension, UiImageSurfaceTransform, UiNode, UiVisualRole};
use katana_ui_core::theme::ThemeSnapshot;

#[test]
fn image_surface_transform_zoom_clips_inside_media_frame() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(4, 2, palette.background);
    let image: UiNode = ImageSurface::from_rgba(
        "media",
        "fingerprint",
        2,
        1,
        vec![255, 0, 0, 255, 0, 0, 255, 255],
    )
    .kuc_expect("valid image surface")
    .transform(UiImageSurfaceTransform::new(200, 0, 0))
    .into();
    let root = image
        .height(UiDimension::Px(1))
        .visual_role(UiVisualRole::MediaFrame);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 4,
            height: 2,
            scroll_y: 0.0,
        },
    );

    assert_ne!(Some(0x0000ff), pixel_at(&canvas, 3, 0));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 0, 1));
}

#[test]
fn image_surface_transform_pan_changes_visible_source_region() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(4, 2, palette.background);
    let image: UiNode = ImageSurface::from_rgba(
        "media",
        "fingerprint",
        2,
        1,
        vec![255, 0, 0, 255, 0, 0, 255, 255],
    )
    .kuc_expect("valid image surface")
    .transform(UiImageSurfaceTransform::new(200, -1, 0))
    .into();
    let root = image
        .height(UiDimension::Px(1))
        .visual_role(UiVisualRole::MediaFrame);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 4,
            height: 2,
            scroll_y: 0.0,
        },
    );

    assert_ne!(Some(palette.background), pixel_at(&canvas, 0, 0));
    assert_eq!(Some(0x0000ff), pixel_at(&canvas, 3, 0));
}
