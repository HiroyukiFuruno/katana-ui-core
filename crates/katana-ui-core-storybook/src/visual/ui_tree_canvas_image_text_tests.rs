use super::*;
use crate::test_assert::KucTestExpect;
#[test]
fn image_surface_respects_explicit_viewer_node_height() {
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
    let root = image.height(UiDimension::Px(180));

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

#[test]
fn image_surface_content_scale_keeps_logical_extent() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(80, 40, palette.background);
    let image: UiNode = ImageSurface::from_rgba(
        "retina",
        "fingerprint",
        80,
        40,
        [255, 0, 0, 255].repeat(80 * 40),
    )
    .kuc_expect("valid image surface")
    .content_scale(200)
    .into();

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &image,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 80,
            height: 40,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(0xff0000), pixel_at(&canvas, 39, 19));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 40, 20));
}

#[test]
fn image_surface_display_size_overrides_content_scale_logical_extent() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(80, 40, palette.background);
    let image: UiNode = ImageSurface::from_rgba(
        "retina-display",
        "fingerprint",
        80,
        40,
        [255, 0, 0, 255].repeat(80 * 40),
    )
    .kuc_expect("valid image surface")
    .content_scale(200)
    .display_size(50, 25)
    .into();

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &image,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 80,
            height: 40,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(0xff0000), pixel_at(&canvas, 49, 24));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 50, 25));
}

#[test]
fn media_frame_keeps_logical_display_size_when_raster_scale_exceeds_layout_scale() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new_scaled_with_raster_scale(80, 40, 1.0, 2.0, palette.background);
    let image: UiNode = ImageSurface::from_rgba(
        "retina-display",
        "fingerprint",
        40,
        20,
        [255, 0, 0, 255].repeat(40 * 20),
    )
    .kuc_expect("valid image surface")
    .display_size(20, 10)
    .into();
    let image = image
        .visual_role(UiVisualRole::MediaFrame)
        .height(UiDimension::px(10));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &image,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 80,
            height: 40,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.background), pixel_at(&canvas, 29, 0));
    assert_eq!(Some(0xff0000), pixel_at(&canvas, 30, 0));
    assert_eq!(Some(0xff0000), pixel_at(&canvas, 49, 9));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 50, 10));
}

#[test]
fn non_media_image_surface_keeps_logical_display_size_for_reference_capture() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new_scaled_with_raster_scale(80, 40, 1.0, 2.0, palette.background)
        .with_reference_capture_image_surface_extents();
    let image: UiNode = ImageSurface::from_rgba(
        "retina-display",
        "fingerprint",
        40,
        20,
        [255, 0, 0, 255].repeat(40 * 20),
    )
    .kuc_expect("valid image surface")
    .content_scale(200)
    .into();

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &image,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 80,
            height: 40,
            scroll_y: 0.0,
        },
    );

    let left = first_content_x(&canvas).kuc_expect("image should draw");
    let right = rightmost_content_x(&canvas, palette.background).kuc_expect("image should draw");
    let top =
        first_row_for_non_background(&canvas, palette.background).kuc_expect("image should draw");
    let bottom = canvas
        .pixels()
        .iter()
        .rposition(|pixel| *pixel != palette.background)
        .map(|index| index / canvas.width())
        .kuc_expect("image should draw");

    assert_eq!(20, right - left + 1);
    assert_eq!(10, bottom - top + 1);
}

#[test]
fn media_frame_can_preserve_raster_extent_for_reference_capture() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new_scaled_with_raster_scale(80, 40, 1.0, 2.0, palette.background)
        .with_reference_capture_image_surface_extents();
    let image: UiNode = ImageSurface::from_rgba(
        "retina-display",
        "fingerprint",
        40,
        20,
        [255, 0, 0, 255].repeat(40 * 20),
    )
    .kuc_expect("valid image surface")
    .display_size(20, 10)
    .into();
    let image = image
        .visual_role(UiVisualRole::MediaFrame)
        .height(UiDimension::px(10));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &image,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 80,
            height: 40,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.background), pixel_at(&canvas, 19, 0));
    assert_eq!(Some(0xff0000), pixel_at(&canvas, 20, 0));
    assert_eq!(Some(0xff0000), pixel_at(&canvas, 59, 19));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 60, 20));
}

#[test]
fn image_surface_selection_text_reaches_copy_payload_without_visible_text_overlay() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(120, 80, palette.background);
    let image: UiNode = ImageSurface::from_rgba(
        "diagram",
        "fingerprint",
        20,
        10,
        [0, 0, 0, 255].repeat(20 * 10),
    )
    .kuc_expect("valid image surface")
    .selection_text("Artifact Needle")
    .into();

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &image,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 120,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert_eq!(
        Some("Artifact Needle".to_string()),
        canvas.copy_text_in_selection(Some((0, 0)), Some((120, 80)))
    );
    assert!(
        !canvas.pixels().contains(&palette.text),
        "selection text must not draw duplicate visible glyph pixels"
    );
}

#[test]
fn image_surface_blends_rgba_alpha_instead_of_painting_transparent_pixels_black() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(4, 2, palette.background);
    let image: UiNode = ImageSurface::from_rgba(
        "transparent",
        "fingerprint",
        2,
        1,
        vec![255, 0, 0, 0, 0, 255, 0, 255],
    )
    .kuc_expect("valid image surface")
    .into();

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &image,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 4,
            height: 2,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.background), pixel_at(&canvas, 0, 0));
    assert_eq!(Some(0x00ff00), pixel_at(&canvas, 1, 0));
}
