use crate::test_assert::KucTestExpect;
use crate::visual::canvas::Canvas;
use crate::visual::ui_tree_canvas::UiTreeCanvasRenderer;
use crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette;
use crate::visual::ui_tree_canvas_tests::pixel_at;
use crate::visual::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::atom::ImageSurface;
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiVisualRole};
use katana_ui_core::theme::ThemeSnapshot;

const HOVER_CANVAS_WIDTH: usize = 12;
const HOVER_CANVAS_HEIGHT: usize = 6;
const HOVER_CANVAS_WIDTH_PX: u16 = 12;
const HOVER_CANVAS_HEIGHT_PX: u16 = 6;
const IMAGE_WIDTH: u32 = 2;
const IMAGE_HEIGHT: u32 = 2;
const HOVER_ALPHA: u8 = 96;
const SVG_BACKGROUND_BYTE: u8 = 0x1e;
const OPAQUE_BYTE: u8 = 0xff;
const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const COLOR_BYTE_MASK: u32 = 0xff;
const ALPHA_MAX: u32 = 255;

#[test]
fn hover_surface_tints_precomposited_image_background_without_hard_svg_seam() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(HOVER_CANVAS_WIDTH, HOVER_CANVAS_HEIGHT, palette.background);
    let image: UiNode = ImageSurface::from_rgba(
        "diagram",
        "fingerprint",
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        [
            SVG_BACKGROUND_BYTE,
            SVG_BACKGROUND_BYTE,
            SVG_BACKGROUND_BYTE,
            OPAQUE_BYTE,
            SVG_BACKGROUND_BYTE,
            SVG_BACKGROUND_BYTE,
            SVG_BACKGROUND_BYTE,
            OPAQUE_BYTE,
            SVG_BACKGROUND_BYTE,
            SVG_BACKGROUND_BYTE,
            SVG_BACKGROUND_BYTE,
            OPAQUE_BYTE,
            OPAQUE_BYTE,
            OPAQUE_BYTE,
            OPAQUE_BYTE,
            OPAQUE_BYTE,
        ]
        .to_vec(),
    )
    .kuc_expect("valid image surface")
    .into();
    let root = UiNode::new(UiNodeKind::Stack, "")
        .width(UiDimension::px(HOVER_CANVAS_WIDTH_PX))
        .height(UiDimension::px(HOVER_CANVAS_HEIGHT_PX))
        .visual_role(UiVisualRole::HoverSurface)
        .child(image);

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: HOVER_CANVAS_WIDTH,
            height: HOVER_CANVAS_HEIGHT,
            scroll_y: 0.0,
        },
    );

    let expected_hovered_background =
        blend_for_test(palette.background, palette.hover_background, HOVER_ALPHA);
    assert_eq!(Some(expected_hovered_background), pixel_at(&canvas, 0, 0));
    assert_eq!(
        Some(expected_hovered_background),
        pixel_at(&canvas, 1, 0),
        "diagram SVG background pixels precomposited to viewer background must receive the same hover tint as the frame"
    );
    assert_ne!(
        Some(expected_hovered_background),
        pixel_at(&canvas, 1, 1),
        "foreground pixels should remain visible while the hover surface is applied"
    );
}

fn blend_for_test(destination: u32, source: u32, alpha: u8) -> u32 {
    let alpha = u32::from(alpha);
    let inverse = ALPHA_MAX - alpha;
    let red = blend_channel_for_test(destination, source, alpha, inverse, RED_SHIFT);
    let green = blend_channel_for_test(destination, source, alpha, inverse, GREEN_SHIFT);
    let blue = blend_channel_for_test(destination, source, alpha, inverse, 0);
    (red << RED_SHIFT) | (green << GREEN_SHIFT) | blue
}

fn blend_channel_for_test(
    destination: u32,
    source: u32,
    alpha: u32,
    inverse: u32,
    shift: u32,
) -> u32 {
    let destination_channel = (destination >> shift) & COLOR_BYTE_MASK;
    let source_channel = (source >> shift) & COLOR_BYTE_MASK;
    (source_channel * alpha + destination_channel * inverse) / ALPHA_MAX
}
