use super::*;
use crate::test_assert::KucTestExpect;

#[test]
fn media_frame_image_surface_is_centered_inside_media_box() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(16, 42, palette.background);
    let image: UiNode =
        ImageSurface::from_rgba("media", "fingerprint", 4, 2, [0, 255, 0, 255].repeat(4 * 2))
            .kuc_expect("valid image surface")
            .into();
    let root = image
        .height(UiDimension::Px(40))
        .visual_role(UiVisualRole::MediaFrame);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 16,
            height: 42,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.background), pixel_at(&canvas, 0, 0));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 6, 18));
    assert_eq!(Some(0x00ff00), pixel_at(&canvas, 6, 19));
    assert_eq!(Some(0x00ff00), pixel_at(&canvas, 9, 20));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 10, 20));
}

#[test]
fn media_frame_paints_theme_background_behind_centered_image() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(16, 42, 0x202833);
    let image: UiNode =
        ImageSurface::from_rgba("media", "fingerprint", 4, 2, [0, 255, 0, 255].repeat(4 * 2))
            .kuc_expect("valid image surface")
            .into();
    let root = image
        .height(UiDimension::Px(40))
        .visual_role(UiVisualRole::MediaFrame);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 16,
            height: 42,
            scroll_y: 0.0,
        },
    );

    assert_eq!(
        Some(palette.background),
        pixel_at(&canvas, 0, 0),
        "media frame background must match the viewer background even when the parent canvas differs"
    );
    assert_eq!(Some(palette.background), pixel_at(&canvas, 6, 18));
    assert_eq!(Some(0x00ff00), pixel_at(&canvas, 6, 19));
}

#[test]
fn media_frame_stack_paints_full_row_background_behind_exact_image_body() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(16, 8, 0x202833);
    let image: UiNode =
        ImageSurface::from_rgba("media", "fingerprint", 4, 2, [0, 255, 0, 255].repeat(4 * 2))
            .kuc_expect("valid image surface")
            .into();
    let image = image
        .visual_role(UiVisualRole::MediaFrame)
        .position(UiPosition::Absolute);
    let root: UiNode = Stack::new().child(image).into();
    let root = root
        .width(UiDimension::Px(16))
        .height(UiDimension::Px(8))
        .visual_role(UiVisualRole::MediaFrame);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 16,
            height: 8,
            scroll_y: 0.0,
        },
    );

    assert_eq!(
        Some(palette.background),
        pixel_at(&canvas, 0, 0),
        "full row media frame background must be owned by the Stack wrapper"
    );
    assert_eq!(Some(0x00ff00), pixel_at(&canvas, 6, 0));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 10, 0));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 15, 7));
}

#[test]
fn export_media_frame_image_surface_keeps_kdv_surface_top_margin() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(16, 42, palette.background);
    let image: UiNode =
        ImageSurface::from_rgba("media", "fingerprint", 4, 2, [0, 255, 0, 255].repeat(4 * 2))
            .kuc_expect("valid image surface")
            .into();
    let root = image
        .height(UiDimension::Px(40))
        .visual_role(UiVisualRole::ExportMediaFrame);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 16,
            height: 42,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.background), pixel_at(&canvas, 6, 17));
    assert_eq!(Some(0x00ff00), pixel_at(&canvas, 6, 18));
    assert_eq!(Some(0x00ff00), pixel_at(&canvas, 9, 19));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 10, 19));
}

#[test]
fn media_frame_scales_large_image_by_container_width_not_control_padding() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(16, 42, palette.background);
    let image: UiNode = ImageSurface::from_rgba(
        "media",
        "fingerprint",
        40,
        20,
        [0, 255, 0, 255].repeat(40 * 20),
    )
    .kuc_expect("valid image surface")
    .into();
    let root = image
        .height(UiDimension::Px(40))
        .visual_role(UiVisualRole::MediaFrame);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 16,
            height: 42,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.background), pixel_at(&canvas, 0, 15));
    assert_eq!(Some(0x00ff00), pixel_at(&canvas, 0, 16));
    assert_eq!(Some(0x00ff00), pixel_at(&canvas, 15, 23));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 0, 24));
}

#[test]
fn media_frame_accepts_fractional_display_size_from_svg_viewbox() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(360, 560, palette.background);
    let image: UiNode = ImageSurface::from_rgba(
        "media",
        "fingerprint",
        650,
        1049,
        [0, 255, 0, 255].repeat(650 * 1049),
    )
    .kuc_expect("valid image surface")
    .content_scale(200)
    .display_size_exact(324.9855, 524.3)
    .into();
    let root = image.visual_role(UiVisualRole::MediaFrame);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 360,
            height: 560,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(0x00ff00), pixel_at(&canvas, 17, 0));
    assert_eq!(Some(0x00ff00), pixel_at(&canvas, 341, 523));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 342, 523));
}
