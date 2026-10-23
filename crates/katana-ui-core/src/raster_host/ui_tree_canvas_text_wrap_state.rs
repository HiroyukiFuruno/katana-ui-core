use super::super::super::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use crate::raster_host::ui_tree_canvas_text_line_width::{SpanTextRenderers, span_part_width};
use katana_ui_core::render_model::UiTextSpan;

#[path = "ui_tree_canvas_text_wrap_segments.rs"]
mod segments;
use segments::{segment_with_text, span_segments_with_source_metadata};

#[cfg(test)]
pub(super) fn span_segments(span: &UiTextSpan) -> Vec<UiTextSpan> {
    segments::span_segments(span)
}

pub(super) struct WrappedTextLine<T> {
    pub(super) content: T,
    pub(super) separator_before: String,
    pub(super) separator_is_explicit: bool,
}

pub(super) struct SpanWrapState {
    max_width: usize,
    preserve_whitespace: bool,
    lines: Vec<WrappedTextLine<Vec<UiTextSpan>>>,
    current_line: Vec<UiTextSpan>,
    current_separator_before: String,
    current_separator_is_explicit: bool,
    current_width: usize,
}

impl SpanWrapState {
    pub(super) fn new(max_width: usize, preserve_whitespace: bool) -> Self {
        Self {
            max_width,
            preserve_whitespace,
            lines: Vec::new(),
            current_line: Vec::new(),
            current_separator_before: String::new(),
            current_separator_is_explicit: false,
            current_width: 0,
        }
    }

    #[cfg(test)]
    pub(super) fn push(
        &mut self,
        renderers: SpanTextRenderers<'_>,
        segment: UiTextSpan,
        metrics: UiTreeTextMetrics,
    ) {
        self.push_with_source_adjacency(renderers, segment, metrics, true);
    }

    pub(super) fn push_spans(
        &mut self,
        renderers: SpanTextRenderers<'_>,
        spans: &[UiTextSpan],
        metrics: UiTreeTextMetrics,
    ) {
        let mut previous_source_span_ends_with_carriage_return = false;
        for span in spans {
            for segment in span_segments_with_source_metadata(span) {
                let source_adjacent_to_carriage_return = segment.lf_follows_carriage_return
                    || (segment.lf_is_at_source_span_start
                        && previous_source_span_ends_with_carriage_return);
                self.push_with_source_adjacency(
                    renderers,
                    segment.span,
                    metrics,
                    source_adjacent_to_carriage_return,
                );
            }
            if !span.text.is_empty() {
                previous_source_span_ends_with_carriage_return = span.text.ends_with('\r');
            }
        }
    }

    fn push_with_source_adjacency(
        &mut self,
        renderers: SpanTextRenderers<'_>,
        mut segment: UiTextSpan,
        metrics: UiTreeTextMetrics,
        source_adjacent_to_carriage_return: bool,
    ) {
        if matches!(
            segment.text.as_str(),
            "\n" | "\r" | "\r\n" | "\u{2028}" | "\u{2029}"
        ) {
            self.start_new_line_with_source_adjacency(
                segment.text.as_str(),
                true,
                source_adjacent_to_carriage_return,
            );
            return;
        }
        let preserve_segment_whitespace =
            self.preserve_whitespace || segment.style.inline_code || segment.style.monospace;
        if !preserve_segment_whitespace && self.current_width == 0 {
            segment.text = segment.text.trim_start().to_string();
        }
        if segment.text.is_empty() {
            return;
        }
        let width = span_part_width(renderers, &segment, metrics, preserve_segment_whitespace);
        if self.current_width > 0 && self.current_width.saturating_add(width) > self.max_width {
            self.start_new_line("\n", false);
        }
        if !preserve_segment_whitespace && self.current_width == 0 {
            segment.text = segment.text.trim_start().to_string();
        }
        if !preserve_segment_whitespace && segment.text.trim().is_empty() && self.current_width == 0
        {
            return;
        }
        let width = span_part_width(renderers, &segment, metrics, preserve_segment_whitespace);
        if width > self.max_width {
            self.push_oversized_segment(renderers, segment, metrics);
            return;
        }
        self.current_width = self.current_width.saturating_add(width);
        self.current_line.push(segment);
    }

    pub(super) fn finish(self) -> Vec<Vec<UiTextSpan>> {
        self.finish_with_metadata()
            .into_iter()
            .map(|line| line.content)
            .collect()
    }

    pub(super) fn finish_with_metadata(mut self) -> Vec<WrappedTextLine<Vec<UiTextSpan>>> {
        if !self.current_line.is_empty()
            || self.lines.is_empty()
            || self.current_separator_is_explicit
        {
            self.push_current_line();
        }
        self.lines
    }

    fn start_new_line(&mut self, next_separator_before: &str, separator_is_explicit: bool) {
        self.start_new_line_with_source_adjacency(
            next_separator_before,
            separator_is_explicit,
            true,
        );
    }

    fn start_new_line_with_source_adjacency(
        &mut self,
        next_separator_before: &str,
        separator_is_explicit: bool,
        source_adjacent_to_carriage_return: bool,
    ) {
        if source_adjacent_to_carriage_return
            && self.current_line.is_empty()
            && self.current_separator_is_explicit
            && self.current_separator_before == "\r"
            && next_separator_before == "\n"
        {
            self.current_separator_before.push('\n');
            return;
        }
        if self.current_line.is_empty() {
            if !separator_is_explicit {
                return;
            }
            if self.current_separator_is_explicit || self.lines.is_empty() {
                self.push_current_line();
            }
        } else {
            self.push_current_line();
        }
        self.current_separator_before = next_separator_before.to_owned();
        self.current_separator_is_explicit = separator_is_explicit;
    }

    fn push_current_line(&mut self) {
        self.lines.push(WrappedTextLine {
            content: std::mem::take(&mut self.current_line),
            separator_before: std::mem::take(&mut self.current_separator_before),
            separator_is_explicit: std::mem::replace(
                &mut self.current_separator_is_explicit,
                false,
            ),
        });
        self.current_width = 0;
    }

    fn push_oversized_segment(
        &mut self,
        renderers: SpanTextRenderers<'_>,
        segment: UiTextSpan,
        metrics: UiTreeTextMetrics,
    ) {
        debug_assert_eq!(
            0, self.current_width,
            "oversized segments must start on a fresh line"
        );
        let preserve_segment_whitespace =
            self.preserve_whitespace || segment.style.inline_code || segment.style.monospace;
        let mut chunk = String::new();
        for character in segment.text.chars() {
            let mut candidate = chunk.clone();
            candidate.push(character);
            let candidate_segment = segment_with_text(&segment, candidate.as_str());
            let candidate_width = span_part_width(
                renderers,
                &candidate_segment,
                metrics,
                preserve_segment_whitespace,
            );
            if candidate_width > self.max_width && !chunk.is_empty() {
                self.push_current_chunk(&segment, &mut chunk, renderers, metrics);
            }
            chunk.push(character);
        }
        self.push_current_chunk(&segment, &mut chunk, renderers, metrics);
    }

    fn push_current_chunk(
        &mut self,
        segment: &UiTextSpan,
        chunk: &mut String,
        renderers: SpanTextRenderers<'_>,
        metrics: UiTreeTextMetrics,
    ) {
        if chunk.is_empty() {
            return;
        }
        let next = segment_with_text(segment, chunk.as_str());
        let preserve_segment_whitespace =
            self.preserve_whitespace || segment.style.inline_code || segment.style.monospace;
        let width = span_part_width(renderers, &next, metrics, preserve_segment_whitespace);
        self.current_width = width;
        self.current_line.push(next);
        self.start_new_line("\n", false);
        chunk.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raster_host::text::TextRenderer;
    use katana_ui_core::atom::Text;
    use katana_ui_core::facade::UiCoreFacade;
    use katana_ui_core::render_model::UiNode;

    fn text_context() -> (TextRenderer, TextRenderer, UiTreeTextMetrics) {
        let facade = UiCoreFacade::default();
        let text = TextRenderer::load(&facade, "body");
        let code = TextRenderer::load(&facade, "code");
        let node: UiNode = Text::new("sample").into();
        (text, code, UiTreeTextMetrics::for_node(&node))
    }

    #[test]
    fn span_segments_preserve_word_spacing_newlines_and_trailing_space() {
        let span = UiTextSpan::plain(" alpha  beta\n gamma ");
        let segments = span_segments(&span);
        let text = segments
            .iter()
            .map(|segment| segment.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(text, vec![" alpha", "  beta", "\n", " gamma", " "]);
        assert!(span_segments(&UiTextSpan::plain("")).is_empty());
    }

    #[test]
    fn span_segments_preserve_all_paragraph_separators_at_utf8_boundaries() {
        let span = UiTextSpan::plain("a\u{2029}b\rc\r\nd");
        let segments = span_segments(&span);

        assert_eq!(
            segments
                .iter()
                .map(|segment| segment.text.as_str())
                .collect::<Vec<_>>(),
            vec!["a", "\u{2029}", "b", "\r", "c", "\r\n", "d"]
        );
    }

    #[test]
    fn wrap_state_handles_empty_newline_whitespace_and_oversized_segments() {
        let (text, code, metrics) = text_context();
        let renderers = SpanTextRenderers::new(&text, &code);
        let mut state = SpanWrapState::new(12, false);

        state.start_new_line("\n", false);
        state.push(renderers, UiTextSpan::plain("   "), metrics);
        state.push(renderers, UiTextSpan::plain("a"), metrics);
        state.push(renderers, UiTextSpan::plain("\n"), metrics);
        state.push(renderers, UiTextSpan::plain("oversized"), metrics);
        let lines = state.finish();

        assert!(lines.len() > 2);
        assert_eq!(lines[0][0].text, "a");
        assert_eq!(
            lines
                .iter()
                .flatten()
                .map(|segment| segment.text.as_str())
                .collect::<String>(),
            "aoversized"
        );

        let first = UiTextSpan::plain("a");
        let first_width = span_part_width(renderers, &first, metrics, false);
        let mut wrapped_whitespace = SpanWrapState::new(first_width, false);
        wrapped_whitespace.push(renderers, first, metrics);
        wrapped_whitespace.push(renderers, UiTextSpan::plain("   "), metrics);
        assert_eq!(
            "a",
            wrapped_whitespace
                .finish()
                .into_iter()
                .flatten()
                .map(|segment| segment.text)
                .collect::<String>()
        );
    }

    #[test]
    fn wrap_state_attaches_explicit_separators_to_the_following_logical_line() {
        let (text, code, metrics) = text_context();
        let renderers = SpanTextRenderers::new(&text, &code);
        let mut state = SpanWrapState::new(1_000, false);

        state.push(renderers, UiTextSpan::plain("a"), metrics);
        state.push(renderers, UiTextSpan::plain("\r\n"), metrics);
        state.push(renderers, UiTextSpan::plain("\u{2029}"), metrics);
        state.push(renderers, UiTextSpan::plain("b"), metrics);
        let lines = state.finish_with_metadata();

        assert_eq!(
            lines
                .iter()
                .map(|line| {
                    line.content
                        .iter()
                        .map(|segment| segment.text.as_str())
                        .collect::<String>()
                })
                .collect::<Vec<_>>(),
            vec!["a", "", "b"]
        );
        assert_eq!(
            lines
                .iter()
                .map(|line| line.separator_before.as_str())
                .collect::<Vec<_>>(),
            vec!["", "\r\n", "\u{2029}"]
        );
        assert_eq!(
            lines
                .iter()
                .map(|line| line.separator_is_explicit)
                .collect::<Vec<_>>(),
            vec![false, true, true]
        );
    }

    #[test]
    fn wrap_state_replaces_an_empty_automatic_line_with_an_explicit_separator() {
        let (text, code, metrics) = text_context();
        let renderers = SpanTextRenderers::new(&text, &code);
        let mut state = SpanWrapState::new(12, false);

        state.push(renderers, UiTextSpan::plain("oversized"), metrics);
        state.push(renderers, UiTextSpan::plain("\r\n"), metrics);
        state.push(renderers, UiTextSpan::plain("b"), metrics);
        let lines = state.finish_with_metadata();

        assert!(lines.iter().all(|line| !line.content.is_empty()));
        assert!(!lines.is_empty(), "wrapped trailing line");
        let last_index = lines.len() - 1;
        let last = &lines[last_index];
        assert_eq!(
            last.content
                .iter()
                .map(|segment| segment.text.as_str())
                .collect::<String>(),
            "b"
        );
        assert_eq!(last.separator_before, "\r\n");
        assert!(last.separator_is_explicit);
    }

    #[test]
    fn wrap_state_merges_crlf_across_adjacent_spans() {
        let (text, code, metrics) = text_context();
        let renderers = SpanTextRenderers::new(&text, &code);
        let mut state = SpanWrapState::new(1_000, false);
        let spans = [UiTextSpan::plain("a\r"), UiTextSpan::plain("\nb")];

        state.push_spans(renderers, &spans, metrics);
        let lines = state.finish_with_metadata();

        assert_eq!(
            lines
                .iter()
                .map(|line| {
                    line.content
                        .iter()
                        .map(|segment| segment.text.as_str())
                        .collect::<String>()
                })
                .collect::<Vec<_>>(),
            vec!["a", "b"]
        );
        assert_eq!(
            lines
                .iter()
                .map(|line| line.separator_before.as_str())
                .collect::<Vec<_>>(),
            vec!["", "\r\n"]
        );
    }

    #[test]
    fn wrap_state_does_not_merge_nonadjacent_cr_and_lf() {
        let (text, code, metrics) = text_context();
        let renderers = SpanTextRenderers::new(&text, &code);

        for spans in [
            vec![UiTextSpan::plain("a\r \nb")],
            vec![UiTextSpan::plain("a\r"), UiTextSpan::plain(" \nb")],
        ] {
            let mut state = SpanWrapState::new(1_000, false);
            state.push_spans(renderers, &spans, metrics);
            let lines = state.finish_with_metadata();

            assert_eq!(
                lines
                    .iter()
                    .map(|line| {
                        line.content
                            .iter()
                            .map(|segment| segment.text.as_str())
                            .collect::<String>()
                    })
                    .collect::<Vec<_>>(),
                vec!["a", "", "b"]
            );
            assert_eq!(
                lines
                    .iter()
                    .map(|line| line.separator_before.as_str())
                    .collect::<Vec<_>>(),
                vec!["", "\r", "\n"]
            );
        }
    }

    #[test]
    fn wrap_state_preserves_whitespace_and_wraps_between_segments() {
        let (text, code, metrics) = text_context();
        let renderers = SpanTextRenderers::new(&text, &code);
        let mut state = SpanWrapState::new(30, true);

        state.push(renderers, UiTextSpan::plain("  "), metrics);
        state.push(renderers, UiTextSpan::plain("abc"), metrics);
        state.push(renderers, UiTextSpan::plain("def"), metrics);
        let lines = state.finish();

        assert!(lines.len() >= 2);
        assert_eq!(lines[0][0].text, "  ");

        let mut empty_chunk = String::new();
        let mut direct = SpanWrapState::new(30, false);
        direct.push_current_chunk(
            &UiTextSpan::plain("x"),
            &mut empty_chunk,
            renderers,
            metrics,
        );
        assert_eq!(direct.finish(), vec![Vec::new()]);
    }
}
