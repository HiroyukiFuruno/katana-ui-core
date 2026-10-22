use super::super::underline_y_offset;
use super::*;
use crate::raster_host::ui_tree_canvas_text::text_lines::visual_span_positions;
use katana_ui_core::text_raster::{PlatformTextFaceSelection, PlatformTextRasterConfig};

#[test]
fn bidi_grouped_underlined_fragment_uses_its_own_visual_bounds() {
    let theme = ThemeSnapshot::dark();
    let facade = UiCoreFacade::new(theme.clone());
    let renderer = TextRenderer::load(&facade, "body");
    let code_renderer = TextRenderer::load(&facade, "code").with_raster_config(
        PlatformTextRasterConfig::default(),
        PlatformTextFaceSelection::CandidateChain,
    );
    let node: UiNode = Text::new("\u{0633}\u{0644}\u{0627}x")
        .text_role("body")
        .text_spans(vec![
            UiTextSpan::plain("\u{0633}"),
            UiTextSpan {
                text: "\u{0644}".to_owned(),
                style: UiTextSpanStyle {
                    underline: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            },
            UiTextSpan::plain("\u{0627}"),
            UiTextSpan {
                text: "x".to_owned(),
                style: UiTextSpanStyle {
                    inline_code: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            },
        ])
        .into();
    let metrics = UiTreeTextMetrics::for_node(&node);
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width: 480,
        height: 96,
        scroll_y: 0.0,
    };
    let palette =
        crate::raster_host::ui_tree_canvas_palette::UiTreeCanvasPalette::from_theme(&theme);
    let renderers = SpanTextRenderers::new(&renderer, &code_renderer);
    let underlined = visual_span_positions(renderers, &node.props().text.spans, metrics, 0)
        .expect("mixed body/code line must use visual bidi positions")
        .into_iter()
        .find(|span| span.line_span_index == 1)
        .expect("the underlined body fragment must retain a visual position");
    let mut canvas = Canvas::new(8_192, 96, TEST_BACKGROUND);

    UiTreeTextLines::draw_spans(
        &mut canvas,
        UiTreeTextLineContext {
            renderer: &renderer,
            code_renderer: &code_renderer,
            node: &node,
            area,
            palette,
            metrics,
        },
        area.x,
        20,
    );

    let underline_y = 20 + underline_y_offset(metrics, &node);
    let (min_x, max_x) = row_color_bounds(&canvas, underline_y, palette.text)
        .expect("the underlined non-link fragment must paint one decoration row");
    assert_eq!(underlined.x as usize, min_x);
    assert_eq!(underlined.x as usize + underlined.width - 1, max_x);
}
