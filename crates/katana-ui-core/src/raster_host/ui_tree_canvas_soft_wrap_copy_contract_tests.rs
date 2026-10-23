use super::preserves_whitespace;
use crate::raster_host::text::TextRenderer;
use crate::raster_host::ui_tree_canvas_text_line_width::{
    SpanTextRenderers, span_part_width, whitespace_width,
};
use crate::raster_host::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use crate::raster_host::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use katana_ui_core::atom::Text;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::render_model::{UiNode, UiTextSpan, UiTextSpanStyle, UiTextWrapMode};
use katana_ui_core::text_raster::{PlatformTextFaceSelection, PlatformTextRasterConfig};
use katana_ui_core::theme::ThemeSnapshot;

const COPY_AREA_HEIGHT: usize = 1_200;
const CODE_TEXT: &str = "a = b; a = b; a = b; a = b; a = b; a = b;";

#[test]
fn soft_wrapped_code_copy_does_not_add_visual_line_separators() {
    let renderer = code_renderer();
    let root = wrapped_code_node();

    for width in [24, 48, 120] {
        assert_soft_wrapped_copy(&renderer, &root, width);
    }
}

#[test]
fn mixed_prose_and_inline_code_preserves_code_spaces_without_changing_prose_mode() {
    let renderer = code_renderer();
    let root = mixed_prose_and_inline_code_node();
    assert!(
        !preserves_whitespace(&root),
        "inline code must not switch the whole prose node to preserved whitespace"
    );

    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let text_renderer = TextRenderer::load(&facade, "body");
    let code_text_renderer = TextRenderer::load(&facade, "code");
    let renderers = SpanTextRenderers::new(&text_renderer, &code_text_renderer);
    let metrics = UiTreeTextMetrics::for_node(&root);
    let prose_spaces = UiTextSpan::plain("  ");
    assert_eq!(
        2 * whitespace_width(metrics, false),
        span_part_width(
            renderers,
            &prose_spaces,
            metrics,
            preserves_whitespace(&root)
        ),
        "ordinary prose whitespace keeps collapsed document spacing"
    );

    for width in [40, 64] {
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width,
            height: COPY_AREA_HEIGHT,
            scroll_y: 0.0,
        };
        let mut canvas = Canvas::new(width, area.height, 0);
        renderer.render(&mut canvas, &root, area);

        assert!(
            canvas.text_runs().len() > 1,
            "mixed text must wrap at {width}"
        );
        assert_eq!(
            Some("lead   a  b  tail".to_owned()),
            canvas.copy_text_in_selection(Some((0, 0)), Some((width, area.height))),
            "copy must preserve inline-code source spaces without adding visual-wrap newlines at {width}"
        );
    }
}

fn code_renderer() -> UiTreeCanvasRenderer {
    UiTreeCanvasRenderer::new(ThemeSnapshot::dark()).with_code_text_raster_config(
        PlatformTextRasterConfig::default(),
        PlatformTextFaceSelection::CandidateChain,
    )
}

fn wrapped_code_node() -> UiNode {
    Text::new(CODE_TEXT)
        .text_role("markdown-paragraph")
        .wrap(UiTextWrapMode::Wrap)
        .text_spans(vec![UiTextSpan {
            text: CODE_TEXT.to_owned(),
            style: UiTextSpanStyle {
                monospace: true,
                inline_code: true,
                ..UiTextSpanStyle::default()
            },
            link_target: "https://example.test/wrapped-code".to_owned(),
        }])
        .into()
}

fn mixed_prose_and_inline_code_node() -> UiNode {
    Text::new("lead   a  b  tail")
        .text_role("markdown-paragraph")
        .wrap(UiTextWrapMode::Wrap)
        .text_spans(vec![
            UiTextSpan::plain("lead "),
            UiTextSpan {
                text: "  a  b  ".to_owned(),
                style: UiTextSpanStyle {
                    monospace: true,
                    inline_code: true,
                    ..UiTextSpanStyle::default()
                },
                link_target: String::new(),
            },
            UiTextSpan::plain("tail"),
        ])
        .into()
}

fn assert_soft_wrapped_copy(renderer: &UiTreeCanvasRenderer, root: &UiNode, width: usize) {
    let area = UiTreeRenderArea {
        x: 0,
        y: 0,
        width,
        height: COPY_AREA_HEIGHT,
        scroll_y: 0.0,
    };
    let mut canvas = Canvas::new(width, area.height, 0);
    renderer.render(&mut canvas, root, area);
    assert!(
        canvas.text_runs().len() > 1,
        "code must wrap at width {width}"
    );
    assert_eq!(
        Some(CODE_TEXT.to_owned()),
        canvas.copy_text_in_selection(Some((0, 0)), Some((width, area.height))),
        "visual wrapping must not add source newlines at width {width}"
    );
}
