use super::*;
use crate::test_assert::KucTestExpect;
use katana_ui_core::render_model::{UiTextSpan, UiTextWrapMode};

const LINK_TEXT: &str = "documentationdocumentationdocumentation";
const LINK_TARGET: &str = "https://example.test/documentation";
const FIRST_LABEL: &str = "Foo";
const SECOND_LABEL: &str = "Bar";
const JAPANESE_LABEL: &str = "日本";
const SECOND_LINK_TARGET: &str = "https://example.test/japanese";
const AREA_X: usize = 7;
const AREA_Y: usize = 11;
const AREA_WIDTH: usize = 120;
const LINE_HEIGHT: usize = 34;
const LINK_GRAPHEME_COUNT: usize = 39;
const COMPLETE_AREA_HEIGHT: usize = LINE_HEIGHT * LINK_GRAPHEME_COUNT;

#[test]
fn root_text_link_hit_uses_the_same_scrolled_line_origin_as_glyph_drawing() {
    let theme = ThemeSnapshot::dark();
    let palette = UiTreeCanvasPalette::from_theme(&theme);
    let root = root_link();
    let area = UiTreeRenderArea {
        x: AREA_X,
        y: AREA_Y,
        width: AREA_WIDTH,
        height: LINE_HEIGHT,
        scroll_y: LINE_HEIGHT as f32,
    };
    let renderer = UiTreeCanvasRenderer::new(theme);
    let hits = renderer.host_action_hit_rects(&root, area);
    let viewport_hits = renderer.viewport_host_action_hit_rects(&root, area);
    let document_hits = renderer.document_host_action_hit_rects(&root, area);

    assert_eq!(
        1,
        hits.len(),
        "only the second line is visible after scroll"
    );
    assert_eq!(0, hits[0].rect.y, "visible_line_y rebases document row 34");
    assert_eq!(LINE_HEIGHT, hits[0].rect.height);
    assert_eq!(1, viewport_hits.len());
    assert_eq!(
        AREA_Y, viewport_hits[0].rect.y,
        "viewport adds its y origin once"
    );
    assert!(
        document_hits.iter().any(|hit| hit.rect.y == LINE_HEIGHT),
        "document hits retain the visible line's source row: {document_hits:?}"
    );

    let mut canvas = Canvas::new(AREA_WIDTH + AREA_X, LINE_HEIGHT, palette.background);
    renderer.render(&mut canvas, &root, area);
    let glyph_top = canvas
        .pixels()
        .chunks(canvas.width())
        .position(|row| row.iter().any(|pixel| *pixel != palette.background))
        .kuc_expect("the visible wrapped line must draw glyph pixels");
    assert!(
        glyph_top >= hits[0].rect.y && glyph_top < (hits[0].rect.y + hits[0].rect.height),
        "glyph row {glyph_top} must lie inside the link hit={:?}",
        hits[0].rect
    );

    let complete_area = UiTreeRenderArea {
        y: 0,
        height: COMPLETE_AREA_HEIGHT,
        scroll_y: 0.0,
        ..area
    };
    let all_visible_hits = renderer.host_action_hit_rects(&root, complete_area);
    let complete_document_hits = renderer.document_host_action_hit_rects(&root, complete_area);
    assert_eq!(all_visible_hits, complete_document_hits);
    assert!(
        all_visible_hits.len() >= 2,
        "wrapped link hits={all_visible_hits:?}"
    );
    assert!(
        all_visible_hits
            .iter()
            .all(|hit| hit.action.label == LINK_TEXT),
        "split source span must retain its original action: {all_visible_hits:?}"
    );

    let mut complete_canvas = Canvas::new(
        AREA_WIDTH + AREA_X,
        COMPLETE_AREA_HEIGHT,
        palette.background,
    );
    renderer.render(&mut complete_canvas, &root, complete_area);
    for hit in &complete_document_hits {
        assert!(
            hit_has_glyph(&complete_canvas, palette.background, hit),
            "document hit must cover its rendered glyph row: {hit:?}"
        );
    }
}

#[test]
fn scrolled_link_hit_keeps_the_action_for_its_second_same_url_source_span() {
    let root = UiNode::from(
        Text::new("Foo\nBar")
            .text_role("paragraph")
            .wrap(UiTextWrapMode::Wrap)
            .text_spans(vec![
                link_span(FIRST_LABEL),
                UiTextSpan::plain("\n"),
                link_span(SECOND_LABEL),
            ]),
    );
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: AREA_WIDTH,
        height: LINE_HEIGHT,
        scroll_y: LINE_HEIGHT as f32,
    };
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());

    let visible_hits = renderer.host_action_hit_rects(&root, area);
    let document_hits = renderer.document_host_action_hit_rects(&root, area);

    assert_eq!(vec![SECOND_LABEL], action_labels(&visible_hits));
    assert_eq!(
        vec![FIRST_LABEL, SECOND_LABEL],
        action_labels(&document_hits)
    );
    assert_eq!(LINE_HEIGHT, document_hits[1].rect.y);
}

#[test]
fn normalized_leading_whitespace_does_not_shift_the_next_same_url_link_action() {
    let first_source_label = " Foo";
    let root = UiNode::from(
        Text::new(" Foo\nBar")
            .text_role("paragraph")
            .wrap(UiTextWrapMode::Wrap)
            .text_spans(vec![
                link_span(first_source_label),
                UiTextSpan::plain("\n"),
                link_span(SECOND_LABEL),
            ]),
    );
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: AREA_WIDTH,
        height: LINE_HEIGHT,
        scroll_y: LINE_HEIGHT as f32,
    };
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());

    let visible_hits = renderer.host_action_hit_rects(&root, area);
    let document_hits = renderer.document_host_action_hit_rects(&root, area);

    assert_eq!(vec![SECOND_LABEL], action_labels(&visible_hits));
    assert_eq!(
        vec![first_source_label, SECOND_LABEL],
        action_labels(&document_hits)
    );
}

#[test]
fn code_no_wrap_line_break_keeps_the_following_link_source_action() {
    assert_eq!(
        JAPANESE_LABEL,
        following_code_link_label(UiTextWrapMode::NoWrap)
    );
}

#[test]
fn code_wrapped_line_break_keeps_the_following_link_source_action() {
    assert_eq!(
        JAPANESE_LABEL,
        following_code_link_label(UiTextWrapMode::Wrap)
    );
}

fn root_link() -> UiNode {
    UiNode::from(
        Text::new(LINK_TEXT)
            .text_role("paragraph")
            .wrap(UiTextWrapMode::Wrap)
            .text_spans(vec![UiTextSpan {
                text: LINK_TEXT.to_owned(),
                style: Default::default(),
                link_target: LINK_TARGET.to_owned(),
            }]),
    )
}

fn link_span(label: &str) -> UiTextSpan {
    UiTextSpan {
        text: label.to_owned(),
        style: Default::default(),
        link_target: LINK_TARGET.to_owned(),
    }
}

fn action_labels(hits: &[UiTreeHostActionHit]) -> Vec<&str> {
    hits.iter().map(|hit| hit.action.label.as_str()).collect()
}

fn following_code_link_label(wrap: UiTextWrapMode) -> String {
    let root = UiNode::from(
        Text::new("Foo \nBar日本")
            .text_role("code")
            .wrap(wrap)
            .text_spans(vec![
                link_span("Foo \nBar"),
                UiTextSpan {
                    text: JAPANESE_LABEL.to_owned(),
                    style: Default::default(),
                    link_target: SECOND_LINK_TARGET.to_owned(),
                },
            ]),
    );
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    renderer
        .document_host_action_hit_rects(
            &root,
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: AREA_WIDTH,
                height: COMPLETE_AREA_HEIGHT,
                scroll_y: 0.0,
            },
        )
        .into_iter()
        .find(|hit| hit.action.payload == SECOND_LINK_TARGET)
        .kuc_expect("following Japanese link hit")
        .action
        .label
}

fn hit_has_glyph(canvas: &Canvas, background: u32, hit: &UiTreeHostActionHit) -> bool {
    let top = hit.rect.y;
    let bottom = hit.rect.y + hit.rect.height;
    let left = hit.rect.x;
    let right = hit.rect.x + hit.rect.width;
    (top..bottom)
        .any(|y| (left..right).any(|x| canvas.pixels()[y * canvas.width() + x] != background))
}
