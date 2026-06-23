use super::*;
use crate::test_assert::KucTestExpect;

#[test]
fn document_blockquote_depth_draws_nested_bars_and_offsets_text() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(260, 60, palette.background);
    let root = UiNode::new(UiNodeKind::Text, "Inner quote")
        .text(UiTextProps {
            role: "blockquote".to_string(),
            ..UiTextProps::default()
        })
        .common(quote_depth_common(2))
        .height(UiDimension::Px(46));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 204,
            height: 60,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.muted_border), pixel_at(&canvas, 56, 8));
    assert_eq!(Some(palette.muted_border), pixel_at(&canvas, 88, 8));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 64, 8));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 96, 8));
    let text_x = first_row_content_x_after(&canvas, palette.text, 90)
        .kuc_expect("nested quote text should be offset after the second quote bar");
    assert!(text_x >= 120, "text x must follow quote depth: {text_x}");
}

#[test]
fn inline_code_span_background_uses_theme_inline_code_background() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(260, 60, palette.background);
    let root = UiNode::new(UiNodeKind::Text, "inline")
        .text(UiTextProps {
            role: "body".to_string(),
            spans: vec![UiTextSpan {
                text: "inline".to_string(),
                style: UiTextSpanStyle {
                    inline_code: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            }],
            ..UiTextProps::default()
        })
        .height(UiDimension::Px(46));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 204,
            height: 60,
            scroll_y: 0.0,
        },
    );

    assert!(count_pixel(&canvas, palette.inline_code_background) > 40);
    assert_ne!(Some(0x34312a), pixel_at(&canvas, 56, 0));
}

#[test]
fn document_code_block_insets_box_inside_viewer_block_height() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(260, 100, palette.background);
    let root = UiNode::new(UiNodeKind::Text, "let x = 1;")
        .text(UiTextProps {
            role: "code".to_string(),
            ..UiTextProps::default()
        })
        .common(document_code_common(20))
        .height(UiDimension::Px(84));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 204,
            height: 100,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.background), pixel_at(&canvas, 56, 13));
    assert_eq!(Some(palette.muted_border), pixel_at(&canvas, 56, 14));
    assert_eq!(Some(palette.code_background), pixel_at(&canvas, 58, 15));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 56, 70));
}

#[test]
fn document_code_block_explicit_height_does_not_clip_multiline_text() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(420, 180, palette.background);
    let root = UiNode::new(UiNodeKind::Text, "fn main() {\n    println!(\"Hello\");\n}")
        .text(UiTextProps {
            role: "code".to_string(),
            ..UiTextProps::default()
        })
        .font_role("code")
        .common(document_code_common(20))
        .height(UiDimension::Px(140));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 360,
            height: 180,
            scroll_y: 0.0,
        },
    );

    assert!(
        foreground_pixels_in_rows(&canvas, palette, 54, 82) > 8,
        "second code line must remain visible"
    );
    assert!(
        foreground_pixels_in_rows(&canvas, palette, 88, 116) > 8,
        "third code line must remain visible"
    );
}

#[test]
fn quoted_code_block_draws_quote_bar_before_indented_code_box() {
    let theme = ThemeSnapshot::light();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(260, 80, palette.background);
    let root = UiNode::new(UiNodeKind::Text, "let quoted_code = true;")
        .text(UiTextProps {
            role: "code".to_string(),
            ..UiTextProps::default()
        })
        .common(document_code_common(8).margin(quote_depth_margin(1)))
        .height(UiDimension::Px(56));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 56,
            y: 0,
            width: 204,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert_eq!(Some(palette.muted_border), pixel_at(&canvas, 56, 8));
    assert_eq!(Some(palette.muted_border), pixel_at(&canvas, 88, 8));
    assert_eq!(Some(palette.code_background), pixel_at(&canvas, 90, 8));
    assert_eq!(Some(palette.background), pixel_at(&canvas, 84, 8));
}
