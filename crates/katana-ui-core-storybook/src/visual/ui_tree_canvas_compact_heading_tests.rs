use crate::test_assert::KucTestExpect;
use crate::visual::canvas::Canvas;
use crate::visual::ui_tree_canvas::UiTreeCanvasRenderer;
use crate::visual::ui_tree_canvas_palette::UiTreeCanvasPalette;
use crate::visual::ui_tree_canvas_tests::foreground_pixels_in_rows;
use crate::visual::ui_tree_canvas_types::UiTreeRenderArea;
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiTextProps, UiTextSpan};
use katana_ui_core::theme::ThemeSnapshot;

const RGB_MASK: u32 = 0x00ff_ffff;
const DOCUMENT_BODY_FONT_SIZE: f32 = 14.0;
const DOCUMENT_BODY_FONT_WEIGHT: u16 = 400;
const COMPACT_HEADING_ROLE: &str = "heading-3";
const COMPACT_HEADING_WIDTH: usize = 360;
const COMPACT_HEADING_HEIGHT: usize = 60;
const COMPACT_HEADING_NODE_HEIGHT: u16 = 30;
const COMPACT_STACK_WIDTH: usize = 260;
const COMPACT_STACK_HEIGHT: usize = 80;
const BODY_ROW_HEIGHT: u16 = 23;
const DESCENDER_SCAN_START_Y: usize = 26;
const DESCENDER_SCAN_END_Y: usize = 30;
const MIN_DESCENDER_PIXEL_DELTA: usize = 4;
const SPAN_WIDTH_TOLERANCE: usize = 2;

#[test]
fn compact_heading_descender_survives_inside_explicit_row_height() {
    let mut theme = ThemeSnapshot::dark();
    theme.fonts.push(katana_ui_core::theme::FontToken {
        name: "document-body".to_string(),
        family: katana_ui_core::theme::FontFamily::Proportional,
        size: DOCUMENT_BODY_FONT_SIZE,
        weight: DOCUMENT_BODY_FONT_WEIGHT,
    });
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let heading = compact_heading_canvas(&theme, "H6 Heading");
    let no_descender = compact_heading_canvas(&theme, "H6 Headina");

    let heading_bottom = foreground_pixels_in_rows(
        &heading,
        palette,
        DESCENDER_SCAN_START_Y,
        DESCENDER_SCAN_END_Y,
    );
    let no_descender_bottom = foreground_pixels_in_rows(
        &no_descender,
        palette,
        DESCENDER_SCAN_START_Y,
        DESCENDER_SCAN_END_Y,
    );

    assert!(
        heading_bottom > no_descender_bottom + MIN_DESCENDER_PIXEL_DELTA,
        "compact heading descender must stay visible inside the 30px row: heading={heading_bottom} no_descender={no_descender_bottom}"
    );
}

#[test]
fn compact_heading_spans_inherit_heading_role_ink_width() {
    let mut theme = ThemeSnapshot::dark();
    theme.fonts.push(katana_ui_core::theme::FontToken {
        name: "document-body".to_string(),
        family: katana_ui_core::theme::FontFamily::Proportional,
        size: DOCUMENT_BODY_FONT_SIZE,
        weight: DOCUMENT_BODY_FONT_WEIGHT,
    });
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let plain = compact_heading_only_canvas(&theme, "KatanA Rendering", Vec::new());
    let spanned = compact_heading_only_canvas(
        &theme,
        "",
        vec![UiTextSpan::plain("KatanA "), UiTextSpan::plain("Rendering")],
    );

    let plain_width = foreground_width(&plain, palette)
        .kuc_expect("plain heading should render foreground pixels");
    let spanned_width = foreground_width(&spanned, palette)
        .kuc_expect("spanned heading should render foreground pixels");

    assert!(
        spanned_width + SPAN_WIDTH_TOLERANCE >= plain_width,
        "heading spans must inherit heading role metrics: plain={plain_width} spanned={spanned_width}"
    );
}

fn foreground_width(canvas: &Canvas, palette: UiTreeCanvasPalette) -> Option<usize> {
    foreground_bounds(canvas, palette).map(|bounds| bounds.0)
}

fn foreground_bounds(canvas: &Canvas, palette: UiTreeCanvasPalette) -> Option<(usize, usize)> {
    let mut min_x = usize::MAX;
    let mut max_x = 0usize;
    let mut min_y = usize::MAX;
    let mut max_y = 0usize;
    let width = canvas.width();
    for y in 0..canvas.height() {
        for x in 0..width {
            let pixel = canvas.pixels()[y.saturating_mul(width).saturating_add(x)] & RGB_MASK;
            if pixel != (palette.background & RGB_MASK) {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }
    if min_x == usize::MAX {
        return None;
    }
    Some((
        max_x.saturating_sub(min_x).saturating_add(1),
        max_y.saturating_sub(min_y).saturating_add(1),
    ))
}

fn compact_heading_only_canvas(
    theme: &ThemeSnapshot,
    label: &str,
    spans: Vec<UiTextSpan>,
) -> Canvas {
    compact_heading_role_canvas(
        theme,
        label,
        spans,
        COMPACT_HEADING_ROLE,
        COMPACT_HEADING_WIDTH,
        COMPACT_HEADING_HEIGHT,
        COMPACT_HEADING_NODE_HEIGHT,
    )
}

fn compact_heading_role_canvas(
    theme: &ThemeSnapshot,
    label: &str,
    spans: Vec<UiTextSpan>,
    role: &str,
    width: usize,
    height: usize,
    node_height: u16,
) -> Canvas {
    let palette = UiTreeCanvasPalette::from_theme(theme);
    let mut canvas = Canvas::new(width, height, palette.background);
    let root = UiNode::new(UiNodeKind::Text, label)
        .text(UiTextProps {
            role: role.to_string(),
            spans,
            ..UiTextProps::default()
        })
        .height(UiDimension::Px(node_height));

    UiTreeCanvasRenderer::new(theme.clone()).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width,
            height,
            scroll_y: 0.0,
        },
    );
    canvas
}

fn compact_heading_canvas(theme: &ThemeSnapshot, label: &str) -> Canvas {
    let palette = UiTreeCanvasPalette::from_theme(theme);
    let mut canvas = Canvas::new(
        COMPACT_STACK_WIDTH,
        COMPACT_STACK_HEIGHT,
        palette.background,
    );
    let root = UiNode::new(UiNodeKind::Column, "")
        .child(
            UiNode::new(UiNodeKind::Text, label)
                .text(UiTextProps {
                    role: COMPACT_HEADING_ROLE.to_string(),
                    ..UiTextProps::default()
                })
                .height(UiDimension::Px(COMPACT_HEADING_NODE_HEIGHT)),
        )
        .child(
            UiNode::new(UiNodeKind::Text, "Next")
                .text(UiTextProps {
                    role: "body".to_string(),
                    ..UiTextProps::default()
                })
                .height(UiDimension::Px(BODY_ROW_HEIGHT)),
        );

    UiTreeCanvasRenderer::new(theme.clone()).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: COMPACT_STACK_WIDTH,
            height: COMPACT_STACK_HEIGHT,
            scroll_y: 0.0,
        },
    );
    canvas
}
