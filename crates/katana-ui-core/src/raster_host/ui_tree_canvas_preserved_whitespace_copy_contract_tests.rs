use crate::raster_host::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use katana_ui_core::atom::Text;
use katana_ui_core::render_model::{UiNode, UiTextSpan, UiTextSpanStyle, UiTextWrapMode};
use katana_ui_core::theme::ThemeSnapshot;

const COPY_TEST_WIDTHS: [usize; 3] = [72, 96, 120];
const COPY_TEST_HEIGHT: usize = 1_200;

#[test]
fn code_trailing_spaces_are_not_duplicated_after_narrow_soft_wraps() {
    let expected = "word  \nbeta";
    let root: UiNode = Text::new(expected)
        .text_role("code")
        .wrap(UiTextWrapMode::Wrap)
        .into();
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());

    for width in [1, 8] {
        let area = UiTreeRenderArea {
            x: 0,
            y: 0,
            width,
            height: 1_200,
            scroll_y: 0.0,
        };
        let mut canvas = Canvas::new(width, area.height, 0);
        renderer.render(&mut canvas, &root, area);

        assert_eq!(
            Some(expected.to_owned()),
            canvas.copy_text_in_selection(Some((0, 0)), Some((width, area.height))),
            "preserved trailing spaces must not be appended twice at width {width}"
        );
    }
}

#[test]
fn rich_soft_wrap_preserves_spaces_from_prior_spans_before_explicit_separators() {
    let word = "pneumonoultramicroscopicsilicovolcanoconiosis";
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    for separator in ["\n", "\r", "\r\n", "\u{2028}", "\u{2029}"] {
        let expected = format!("{word} {separator}beta");
        let root = rich_wrapped_node(
            &expected,
            vec![
                bold_span(word),
                UiTextSpan::plain(" "),
                UiTextSpan::plain(separator),
                UiTextSpan::plain("beta"),
            ],
        );
        assert_copy_contract(&renderer, &root, &expected);
    }

    let expected = format!("{word} \r\nbeta");
    let root = rich_wrapped_node(
        &expected,
        vec![
            bold_span(word),
            UiTextSpan::plain(" "),
            UiTextSpan::plain("\r"),
            UiTextSpan::plain("\n"),
            UiTextSpan::plain("beta"),
        ],
    );
    assert_copy_contract(&renderer, &root, &expected);
}

fn rich_wrapped_node(label: &str, spans: Vec<UiTextSpan>) -> UiNode {
    Text::new(label)
        .text_role("markdown-paragraph")
        .wrap(UiTextWrapMode::Wrap)
        .text_spans(spans)
        .into()
}

fn bold_span(text: &str) -> UiTextSpan {
    UiTextSpan {
        text: text.to_owned(),
        style: UiTextSpanStyle {
            bold: true,
            ..UiTextSpanStyle::default()
        },
        link_target: String::new(),
    }
}

fn assert_copy_contract(renderer: &UiTreeCanvasRenderer, root: &UiNode, expected: &str) {
    for width in COPY_TEST_WIDTHS {
        for _ in 0..2 {
            let area = UiTreeRenderArea {
                x: 0,
                y: 0,
                width,
                height: COPY_TEST_HEIGHT,
                scroll_y: 0.0,
            };
            let mut canvas = Canvas::new(width, area.height, 0);
            renderer.render(&mut canvas, root, area);

            assert_eq!(
                Some(expected.to_owned()),
                canvas.copy_text_in_selection(Some((0, 0)), Some((width, area.height))),
                "source separators after an oversized rich word must survive at width {width}"
            );
        }
    }
}
