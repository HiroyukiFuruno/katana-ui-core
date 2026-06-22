use super::*;
use crate::test_assert::KucTestExpect;

const RGB_MASK: u32 = 0x00ff_ffff;

#[test]
fn text_node_respects_explicit_height_for_document_layout() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(160, 96, palette.background);
    let root = UiNode::new(UiNodeKind::Column, "")
        .child(UiNode::new(UiNodeKind::Text, "First").height(UiDimension::Px(48)))
        .child(UiNode::new(UiNodeKind::Text, "Second"));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 160,
            height: 96,
            scroll_y: 0.0,
        },
    );

    let second_row = first_row_containing_color_after(&canvas, palette.text, 40)
        .kuc_expect("second line should render after the explicit first height");
    assert!(
        second_row >= 48,
        "node height must advance layout: {second_row}"
    );
}

#[test]
fn multiline_text_node_draws_all_document_lines() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(260, 120, palette.background);
    let root = UiNode::new(UiNodeKind::Text, "First\nSecond")
        .text(UiTextProps {
            role: "body".to_string(),
            ..UiTextProps::default()
        })
        .height(UiDimension::Px(92));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 260,
            height: 120,
            scroll_y: 0.0,
        },
    );

    let second_row = first_row_containing_color_after(&canvas, palette.text, 46)
        .kuc_expect("second text line should be rasterized");
    assert!(
        second_row < 92,
        "second line should stay inside explicit document height: {second_row}"
    );
}

#[test]
fn text_node_explicit_height_clips_glyph_pixels_below_node_bounds() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(320, 80, palette.background);
    let root = UiNode::new(UiNodeKind::Text, "[^2]: Second footnote content.")
        .text(UiTextProps {
            role: "body".to_string(),
            ..UiTextProps::default()
        })
        .height(UiDimension::Px(23));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 320,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert!(
        foreground_pixels_in_rows(&canvas, palette, 0, 23) > 8,
        "text should still render inside the explicit node bounds"
    );
    assert_eq!(
        0,
        foreground_pixels_in_rows(&canvas, palette, 23, 80),
        "text glyph pixels must not bleed below explicit node height"
    );
}

#[test]
fn html_link_text_explicit_height_keeps_underline_clip_guard() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(320, 80, palette.background);
    let root = UiNode::new(UiNodeKind::Text, "Right aligned link")
        .text(UiTextProps {
            role: "html-right".to_string(),
            spans: vec![UiTextSpan {
                text: "Right aligned link".to_string(),
                style: UiTextSpanStyle::default(),
                link_target: "https://example.com/kdv".to_string(),
            }],
            ..UiTextProps::default()
        })
        .height(UiDimension::Px(23));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 320,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert!(
        foreground_pixels_in_rows(&canvas, palette, 0, 23) > 8,
        "link text should still render inside the explicit node bounds"
    );
    assert!(
        max_link_pixels_in_row(&canvas, palette, 18, 23) > 80,
        "HTML link underline must fit inside the explicit-height text node"
    );
}

#[test]
fn row_node_explicit_height_clips_text_child_pixels_below_node_bounds() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(320, 80, palette.background);
    let root = UiNode::new(UiNodeKind::Row, "")
        .child(
            UiNode::new(UiNodeKind::Text, "2.")
                .text(UiTextProps {
                    role: "list-marker".to_string(),
                    ..UiTextProps::default()
                })
                .width(UiDimension::Px(36)),
        )
        .child(
            UiNode::new(UiNodeKind::Text, "Second footnote content.").text(UiTextProps {
                role: "footnote".to_string(),
                ..UiTextProps::default()
            }),
        )
        .height(UiDimension::Px(23));

    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 320,
            height: 80,
            scroll_y: 0.0,
        },
    );

    assert!(
        foreground_pixels_in_rows(&canvas, palette, 0, 23) > 8,
        "row text should still render inside the explicit node bounds"
    );
    assert_eq!(
        0,
        foreground_pixels_in_rows(&canvas, palette, 23, 80),
        "row child glyph pixels must not bleed below explicit row height"
    );
}

fn max_link_pixels_in_row(
    canvas: &Canvas,
    palette: UiTreeCanvasPalette,
    start_y: usize,
    end_y: usize,
) -> usize {
    let link = palette.link & RGB_MASK;
    let width = canvas.width();
    (start_y..end_y.min(canvas.height()))
        .map(|y| {
            (0..canvas.width())
                .filter(|x| {
                    canvas.pixels()[y.saturating_mul(width).saturating_add(*x)] & RGB_MASK == link
                })
                .count()
        })
        .max()
        .unwrap_or(0)
}
