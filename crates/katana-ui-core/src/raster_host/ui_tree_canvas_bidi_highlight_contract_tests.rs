use crate::raster_host::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use katana_ui_core::atom::Text;
use katana_ui_core::render_model::{UiNode, UiRect, UiTextSpan, UiTextSpanStyle};
use katana_ui_core::text_raster::{PlatformTextFaceSelection, PlatformTextRasterConfig};
use katana_ui_core::text_selection::UiTextSelectionRange;
use katana_ui_core::theme::ThemeSnapshot;

const AREA_HEIGHT: usize = 80;
/* WHY: RTL のラスタ境界は固定レイアウト幅内の visual position を保つため、
 * 選択・hit・装飾を同じ座標系で観測できる描画面を使う。 */
const BIDI_RASTER_WIDTH: usize = 8_192;
const LINK_TARGET: &str = "https://example.test/arabic";

#[test]
fn adjacent_styled_bidi_spans_share_a_renderer_shaping_request() {
    let theme = ThemeSnapshot::dark();
    let renderer = UiTreeCanvasRenderer::new(theme.clone()).with_code_text_raster_config(
        PlatformTextRasterConfig::default(),
        PlatformTextFaceSelection::CandidateChain,
    );
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: BIDI_RASTER_WIDTH,
        height: AREA_HEIGHT,
        scroll_y: 0.0,
    };
    let root = adjacent_bidi_root(true);
    let mut canvas = Canvas::new(BIDI_RASTER_WIDTH, AREA_HEIGHT, 0);
    let mut without_highlight = Canvas::new(BIDI_RASTER_WIDTH, AREA_HEIGHT, 0);

    let link_hit = renderer
        .host_action_hit_rects(&root, area)
        .into_iter()
        .find(|hit| hit.action.payload == LINK_TARGET)
        .expect("the individual linked span keeps its hit geometry");
    renderer.render(&mut canvas, &root, area);
    renderer.render(&mut without_highlight, &adjacent_bidi_root(false), area);

    let body_run = canvas
        .text_runs()
        .iter()
        .find(|run| run.text() == "\u{0633}\u{0644}\u{0627}")
        .expect("three adjacent body spans in one bidi visual run share one shaping request");
    let non_linked_rect = body_run
        .model()
        .highlight_rects(UiTextSelectionRange::new(1, 2))
        .into_iter()
        .next()
        .expect("the underlined source grapheme must retain geometry");
    let linked_rect = body_run
        .model()
        .highlight_rects(UiTextSelectionRange::new(2, 3))
        .into_iter()
        .next()
        .expect("the linked source grapheme must retain geometry");
    assert_eq!(
        (linked_rect.x as usize, linked_rect.width as usize),
        (link_hit.rect.x, link_hit.rect.width),
        "the link hit must use exactly the linked fragment geometry",
    );
    assert!(
        !link_hit.contains_point(
            non_linked_rect.x as f32 + non_linked_rect.width as f32 / 2.0,
            link_hit.rect.y as f32 + 0.5,
        ),
        "the link hit must not extend into the adjacent underlined fragment",
    );

    assert!(
        rect_differs(&canvas, &without_highlight, non_linked_rect),
        "the underlined fragment background must cover its own raster-derived geometry",
    );
    assert!(
        !rect_differs(&canvas, &without_highlight, linked_rect),
        "the background must not leak into the linked fragment",
    );
}

fn adjacent_bidi_root(highlight: bool) -> UiNode {
    Text::new("\u{0633}\u{0644}\u{0627}x")
        .text_role("heading-3")
        .text_spans(vec![
            UiTextSpan::plain("\u{0633}"),
            UiTextSpan {
                text: "\u{0644}".to_owned(),
                style: UiTextSpanStyle {
                    underline: true,
                    highlight,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            },
            UiTextSpan {
                text: "\u{0627}".to_owned(),
                style: UiTextSpanStyle::default(),
                link_target: LINK_TARGET.to_owned(),
            },
            UiTextSpan {
                text: "x".to_owned(),
                style: UiTextSpanStyle {
                    inline_code: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            },
        ])
        .into()
}

fn rect_differs(canvas: &Canvas, baseline: &Canvas, rect: UiRect) -> bool {
    let left = rect.x.max(0) as usize;
    let top = rect.y.max(0) as usize;
    let right = (rect.x.saturating_add(rect.width as i32).max(0) as usize).min(canvas.width());
    let bottom = (rect.y.saturating_add(rect.height as i32).max(0) as usize).min(canvas.height());
    (top..bottom).any(|y| {
        (left..right).any(|x| {
            let index = y * canvas.width() + x;
            canvas.pixels()[index] != baseline.pixels()[index]
        })
    })
}
