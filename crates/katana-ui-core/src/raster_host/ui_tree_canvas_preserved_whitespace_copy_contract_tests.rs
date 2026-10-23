use crate::raster_host::text::TextRenderer;
use crate::raster_host::ui_tree_canvas_text_line_width::{SpanTextRenderers, span_part_width};
use crate::raster_host::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use crate::raster_host::{Canvas, UiTreeCanvasRenderer, UiTreeRenderArea};
use katana_ui_core::atom::Text;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::render_model::{UiNode, UiTextSpan, UiTextSpanStyle, UiTextWrapMode};
use katana_ui_core::theme::ThemeSnapshot;

const COPY_TEST_WIDTHS: [usize; 3] = [72, 96, 120];
const COPY_TEST_HEIGHT: usize = 1_200;
const MIN_WRAP_WIDTH: usize = 120;
const NORMAL_WORD_SEARCH_LIMIT: usize = 256;
const OVERSIZED_WORD_WIDTH: usize = 72;
const SPACE_SPAN_COUNTS: [usize; 3] = [1, 2, 3];
const SOURCE_SEPARATORS: [&str; 5] = ["\n", "\r", "\r\n", "\u{2028}", "\u{2029}"];

#[test]
fn code_trailing_spaces_are_not_duplicated_after_narrow_soft_wraps() {
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());

    for expected in ["word  \nbeta", "word  "] {
        let root: UiNode = Text::new(expected)
            .text_role("code")
            .wrap(UiTextWrapMode::Wrap)
            .into();
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
                "preserved trailing spaces must stay exact at width {width}"
            );
        }
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

#[test]
fn plain_and_rich_copy_keep_split_whitespace_after_wrapped_words() {
    let renderer = UiTreeCanvasRenderer::new(ThemeSnapshot::dark());
    let normal_plain_word = normal_word_that_fills_minimum_wrap_width(false);
    let normal_rich_word = normal_word_that_fills_minimum_wrap_width(true);

    for (plain_word, rich_word, width) in [
        (
            normal_plain_word.as_str(),
            normal_rich_word.as_str(),
            MIN_WRAP_WIDTH,
        ),
        (
            "pneumonoultramicroscopicsilicovolcanoconiosis",
            "pneumonoultramicroscopicsilicovolcanoconiosis",
            OVERSIZED_WORD_WIDTH,
        ),
    ] {
        for space_span_count in SPACE_SPAN_COUNTS {
            for separator in std::iter::once("").chain(SOURCE_SEPARATORS) {
                let spaces = " ".repeat(space_span_count);
                let plain_expected = format!("{plain_word}{spaces}{separator}beta");
                let plain_root: UiNode = Text::new(&plain_expected)
                    .text_role("markdown-paragraph")
                    .wrap(UiTextWrapMode::Wrap)
                    .into();
                let rich_expected = format!("{rich_word}{spaces}{separator}beta");
                let mut spans = vec![UiTextSpan {
                    text: rich_word.to_owned(),
                    style: UiTextSpanStyle {
                        bold: true,
                        ..UiTextSpanStyle::default()
                    },
                    link_target: String::new(),
                }];
                spans.extend((0..space_span_count).map(|_| UiTextSpan {
                    text: " ".to_owned(),
                    style: UiTextSpanStyle {
                        italic: true,
                        ..UiTextSpanStyle::default()
                    },
                    link_target: String::new(),
                }));
                append_separator_spans(&mut spans, separator);
                spans.push(UiTextSpan::plain("beta"));

                let rich_root = rich_wrapped_node(&rich_expected, spans);
                if separator.is_empty() && space_span_count == 1 {
                    assert_canvas_copy(&renderer, &plain_root, &plain_expected, width);
                }
                assert_canvas_copy(&renderer, &rich_root, &rich_expected, width);
            }
        }
    }

    for separator in SOURCE_SEPARATORS {
        let expected = format!("{} {separator}beta", normal_plain_word);
        let root: UiNode = Text::new(&expected)
            .text_role("markdown-paragraph")
            .wrap(UiTextWrapMode::Wrap)
            .into();
        assert_canvas_copy(&renderer, &root, &expected, MIN_WRAP_WIDTH);
    }
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

fn normal_word_that_fills_minimum_wrap_width(bold: bool) -> String {
    let root: UiNode = Text::new("")
        .text_role("markdown-paragraph")
        .wrap(UiTextWrapMode::Wrap)
        .into();
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let body = TextRenderer::load(&facade, "body");
    let code = TextRenderer::load(&facade, "code");
    let renderers = SpanTextRenderers::new(&body, &code);
    let metrics = UiTreeTextMetrics::for_node(&root);
    let space_width = span_part_width(renderers, &UiTextSpan::plain(" "), metrics, false);
    let mut selected = String::new();
    let mut selected_width = 0;
    for character in ['W', 'M', 'w', 'm', 'i', 'l', '1', 'A'] {
        for count in 1..=NORMAL_WORD_SEARCH_LIMIT {
            let candidate = character.to_string().repeat(count);
            let mut span = UiTextSpan::plain(candidate.as_str());
            span.style.bold = bold;
            let width = span_part_width(renderers, &span, metrics, false);
            if width < MIN_WRAP_WIDTH
                && width + space_width > MIN_WRAP_WIDTH
                && width > selected_width
            {
                selected = candidate;
                selected_width = width;
            }
        }
    }
    assert!(
        !selected.is_empty()
            && selected_width < MIN_WRAP_WIDTH
            && selected_width + space_width > MIN_WRAP_WIDTH,
        "select a normal word whose following whitespace overflows the minimum layout width"
    );
    selected
}

fn append_separator_spans(spans: &mut Vec<UiTextSpan>, separator: &str) {
    if separator == "\r\n" {
        spans.push(UiTextSpan::plain("\r"));
        spans.push(UiTextSpan::plain("\n"));
    } else if !separator.is_empty() {
        spans.push(UiTextSpan::plain(separator));
    }
}

fn assert_canvas_copy(
    renderer: &UiTreeCanvasRenderer,
    root: &UiNode,
    expected: &str,
    width: usize,
) {
    let text_kind = if root.props().text.spans.is_empty() {
        "plain"
    } else {
        "rich"
    };
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
        assert!(
            canvas
                .text_runs()
                .first()
                .zip(canvas.text_runs().last())
                .is_some_and(|(first, last)| first.y() < last.y()),
            "the source must cross a rendered line boundary: {expected:?} at width {width}"
        );

        assert_eq!(
            Some(expected.to_owned()),
            canvas.copy_text_in_selection(Some((0, 0)), Some((width, area.height))),
            "split source whitespace must survive for {text_kind} text at width {width} from {expected:?}"
        );
    }
}
