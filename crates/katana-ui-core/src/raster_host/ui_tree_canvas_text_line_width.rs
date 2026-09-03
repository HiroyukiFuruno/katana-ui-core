use crate::raster_host::text::{RichTextStyle, TextRenderer};
use crate::raster_host::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use katana_ui_core::render_model::{UiNode, UiTextSpan};

const PRESERVED_WHITESPACE_WIDTH_FACTOR: f32 = 0.58;
const COLLAPSED_WHITESPACE_WIDTH_FACTOR: f32 = 0.30;

#[derive(Clone, Copy)]
pub(in crate::raster_host) struct SpanTextRenderers<'a> {
    text: &'a TextRenderer,
    code: &'a TextRenderer,
}

impl<'a> SpanTextRenderers<'a> {
    pub(in crate::raster_host) const fn new(
        text: &'a TextRenderer,
        code: &'a TextRenderer,
    ) -> Self {
        Self { text, code }
    }

    pub(in crate::raster_host) fn for_span(self, span: &UiTextSpan) -> &'a TextRenderer {
        if span.style.inline_code || span.style.monospace {
            return self.code;
        }
        self.text
    }
}

pub(in crate::raster_host) fn span_line_width(
    renderers: SpanTextRenderers<'_>,
    line: &[UiTextSpan],
    metrics: UiTreeTextMetrics,
    preserve_whitespace: bool,
) -> usize {
    line.iter()
        .map(|span| span_part_width(renderers, span, metrics, preserve_whitespace))
        .sum::<usize>()
        .max(1)
}

pub(in crate::raster_host) fn span_part_width(
    renderers: SpanTextRenderers<'_>,
    span: &UiTextSpan,
    metrics: UiTreeTextMetrics,
    preserve_whitespace: bool,
) -> usize {
    span_text_part_width(
        renderers,
        span,
        span.text.as_str(),
        metrics,
        preserve_whitespace,
    )
}

pub(in crate::raster_host) fn span_visible_part_bounds(
    renderers: SpanTextRenderers<'_>,
    span: &UiTextSpan,
    metrics: UiTreeTextMetrics,
    preserve_whitespace: bool,
) -> (usize, usize) {
    let full_width = span_part_width(renderers, span, metrics, preserve_whitespace);
    if span.text.chars().all(is_inline_whitespace) {
        return (0, full_width);
    }

    let leading_width =
        inline_whitespace_prefix_width(span.text.as_str(), metrics, preserve_whitespace);
    let trailing_width =
        inline_whitespace_suffix_width(span.text.as_str(), metrics, preserve_whitespace);
    let visible_width = full_width
        .saturating_sub(leading_width)
        .saturating_sub(trailing_width)
        .max(1);
    (leading_width, visible_width)
}

fn span_text_part_width(
    renderers: SpanTextRenderers<'_>,
    span: &UiTextSpan,
    text: &str,
    metrics: UiTreeTextMetrics,
    preserve_whitespace: bool,
) -> usize {
    let mut width = 0usize;
    let mut segment = String::new();
    for character in text.chars() {
        if character.is_whitespace() && character != '\n' {
            width = width.saturating_add(measured_text_width(renderers, span, &segment, metrics));
            segment.clear();
            width = width.saturating_add(whitespace_width(metrics, preserve_whitespace));
            continue;
        }
        segment.push(character);
    }
    width
        .saturating_add(measured_text_width(renderers, span, &segment, metrics))
        .max(1)
}

fn measured_text_width(
    renderers: SpanTextRenderers<'_>,
    span: &UiTextSpan,
    text: &str,
    metrics: UiTreeTextMetrics,
) -> usize {
    if text.is_empty() {
        return 0;
    }
    let renderer = renderers.for_span(span);
    renderer
        .measure_width_rich(text, span_rich_text_style(span, metrics, 0))
        .max(1)
}

pub(in crate::raster_host) fn span_rich_text_style(
    span: &UiTextSpan,
    metrics: UiTreeTextMetrics,
    color: u32,
) -> RichTextStyle {
    RichTextStyle::new(metrics.font_size, color)
        .bold(span.style.bold)
        .italic(span.style.italic)
        .emoji(span.style.emoji)
        .raster_vertical_scale(metrics.raster_vertical_scale)
}

pub(in crate::raster_host) fn whitespace_width(
    metrics: UiTreeTextMetrics,
    preserve_whitespace: bool,
) -> usize {
    let factor = if preserve_whitespace {
        PRESERVED_WHITESPACE_WIDTH_FACTOR
    } else {
        COLLAPSED_WHITESPACE_WIDTH_FACTOR
    };
    (metrics.font_size * factor).ceil() as usize
}

pub(in crate::raster_host) fn preserves_whitespace(node: &UiNode) -> bool {
    node.props().text.role == "code"
}

fn inline_whitespace_prefix_width(
    text: &str,
    metrics: UiTreeTextMetrics,
    preserve_whitespace: bool,
) -> usize {
    text.chars()
        .take_while(|character| is_inline_whitespace(*character))
        .map(|_| whitespace_width(metrics, preserve_whitespace))
        .sum()
}

fn inline_whitespace_suffix_width(
    text: &str,
    metrics: UiTreeTextMetrics,
    preserve_whitespace: bool,
) -> usize {
    text.chars()
        .rev()
        .take_while(|character| is_inline_whitespace(*character))
        .map(|_| whitespace_width(metrics, preserve_whitespace))
        .sum()
}

fn is_inline_whitespace(character: char) -> bool {
    character.is_whitespace() && character != '\n'
}
