use super::*;
use crate::raster_host::ui_tree_canvas_text_metrics::UiTreeDocumentTypography;
use crate::raster_host::{
    UiTreeDocumentTypography as UiTreeDocumentTypographyOverrides, UiTreeTextRoleBaselineTypography,
};
use crate::test_assert::KucTestExpect;
use katana_ui_core::render_model::{UiCommonProps, UiDimension, UiTextWrapMode};

#[test]
fn paragraph_link_hit_uses_the_wrapped_second_line() {
    let root = UiNode::from(
        Text::new("A very long docs")
            .text_role("paragraph")
            .wrap(UiTextWrapMode::Wrap)
            .text_spans(vec![
                UiTextSpan::plain("A very long "),
                UiTextSpan {
                    text: "docs".to_string(),
                    style: Default::default(),
                    link_target: "https://example.test/docs".to_string(),
                },
            ]),
    );

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 7,
            y: 11,
            width: 120,
            height: 120,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len());
    assert_eq!("https://example.test/docs", hits[0].action.payload);
    assert_eq!(45, hits[0].rect.y);
    assert!(hits[0].rect.x < 120, "second-line hit={:?}", hits[0].rect);
    assert!(hits[0].rect.height <= 34);
    let rendered_line_center = (hits[0].rect.x + hits[0].rect.width / 2, 45 + 17);
    let legacy_first_line_center = (rendered_line_center.0, 11 + 17);
    assert!(
        rendered_line_center.1 >= hits[0].rect.y
            && rendered_line_center.1 < hits[0].rect.y + hits[0].rect.height,
        "the link's drawn second-line center must be clickable: hit={:?}",
        hits[0].rect
    );
    assert!(
        legacy_first_line_center.1 < hits[0].rect.y,
        "the obsolete single-line location must not remain clickable: hit={:?}",
        hits[0].rect
    );
}

#[test]
fn wrapped_link_segments_keep_their_shared_navigation_action() {
    let target = "https://example.test/documentation";
    let root = UiNode::from(
        Text::new("documentationdocumentationdocumentation")
            .text_role("paragraph")
            .wrap(UiTextWrapMode::Wrap)
            .text_spans(vec![UiTextSpan {
                text: "documentationdocumentationdocumentation".to_string(),
                style: Default::default(),
                link_target: target.to_string(),
            }]),
    );

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 7,
            y: 11,
            width: 120,
            height: 160,
            scroll_y: 0.0,
        },
    );

    assert!(hits.len() >= 2, "wrapped link hits={hits:?}");
    assert!(hits.iter().all(|hit| hit.action.payload == target));
    assert!(hits.windows(2).all(|pair| pair[0].rect.y < pair[1].rect.y));
}

#[test]
fn wrapped_link_hit_uses_scroll_area_clip_for_the_visible_drawn_line() {
    let root = UiNode::new(UiNodeKind::ScrollArea, "")
        .scroll_area(UiScrollAreaProps {
            viewport_width: 120,
            viewport_height: 34,
            offset_y: 34,
            content_height: 102,
            ..UiScrollAreaProps::default()
        })
        .child(UiNode::from(
            Text::new("documentationdocumentationdocumentation")
                .text_role("paragraph")
                .wrap(UiTextWrapMode::Wrap)
                .text_spans(vec![UiTextSpan {
                    text: "documentationdocumentationdocumentation".to_string(),
                    style: Default::default(),
                    link_target: "https://example.test/documentation".to_string(),
                }]),
        ));

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 7,
            y: 11,
            width: 120,
            height: 34,
            scroll_y: 0.0,
        },
    );

    assert_eq!(1, hits.len(), "only the second drawn line is visible");
    assert_eq!(
        11, hits[0].rect.y,
        "scroll clipping maps document y=34 to viewport y=11"
    );
    assert_eq!(34, hits[0].rect.height);

    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let mut canvas = Canvas::new(140, 56, palette.background);
    UiTreeCanvasRenderer::new(theme).render(
        &mut canvas,
        &root,
        UiTreeRenderArea {
            x: 7,
            y: 11,
            width: 120,
            height: 34,
            scroll_y: 0.0,
        },
    );
    let glyph_top = canvas
        .pixels()
        .chunks(canvas.width())
        .position(|row| row.iter().any(|pixel| *pixel != palette.background))
        .kuc_expect("the clipped second line must draw glyph pixels");
    assert!(glyph_top >= hits[0].rect.y && glyph_top < hits[0].rect.y + hits[0].rect.height);
}

#[test]
fn wrapped_link_hit_drops_lower_line_outside_explicit_height() {
    let root = UiNode::from(
        Text::new("A very long docs")
            .text_role("paragraph")
            .wrap(UiTextWrapMode::Wrap)
            .common(UiCommonProps::default().height(UiDimension::Px(1)))
            .text_spans(vec![
                UiTextSpan::plain("A very long "),
                UiTextSpan {
                    text: "docs".to_string(),
                    style: Default::default(),
                    link_target: "https://example.test/docs".to_string(),
                },
            ]),
    );

    let hits = UiTreeHostActionHitCollector::collect(
        &root,
        UiTreeRenderArea {
            x: 7,
            y: 11,
            width: 120,
            height: 120,
            scroll_y: 0.0,
        },
    );

    assert!(
        hits.is_empty(),
        "the second line lies outside explicit height=1"
    );
}

#[test]
fn wrapped_link_hit_floors_fractional_body_cursor_like_draw_origin() {
    let document_typography = UiTreeDocumentTypographyOverrides::new()
        .with_body_baseline(UiTreeTextRoleBaselineTypography::new(16.0, 31.5, 18.5));
    let typography = UiTreeDocumentTypography::from_theme_with_document_typography(
        &ThemeSnapshot::light(),
        document_typography,
    );
    let root = UiNode::new(UiNodeKind::Column, "")
        .child(UiNode::from(
            Text::new("first paragraph").text_role("paragraph"),
        ))
        .child(UiNode::from(
            Text::new("docs")
                .text_role("paragraph")
                .text_spans(vec![UiTextSpan {
                    text: "docs".to_string(),
                    style: Default::default(),
                    link_target: "https://example.test/docs".to_string(),
                }]),
        ));
    let facade = UiCoreFacade::default();
    let text = TextRenderer::load(&facade, facade.default_font_role());
    let export_text = TextRenderer::load(&facade, facade.default_font_role());
    let code_text = TextRenderer::load(&facade, "code");

    let hits = UiTreeHostActionHitCollector::collect_with_renderers(
        &root,
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: 320,
            height: 120,
            scroll_y: 0.0,
        },
        &text,
        &export_text,
        &code_text,
        typography,
    );

    assert_eq!(1, hits.len());
    assert_eq!(31, hits[0].rect.y, "floor(31.5) is the draw origin");
    assert_eq!(32, hits[0].rect.height, "ceil(31.5) is the line hit height");
}
