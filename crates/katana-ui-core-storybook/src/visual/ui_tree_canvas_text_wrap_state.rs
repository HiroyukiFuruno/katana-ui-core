use super::super::super::ui_tree_canvas_text_metrics::UiTreeTextMetrics;
use crate::visual::ui_tree_canvas_text_line_width::{SpanTextRenderers, span_part_width};
use katana_ui_core::render_model::UiTextSpan;

pub(super) struct SpanWrapState {
    max_width: usize,
    preserve_whitespace: bool,
    lines: Vec<Vec<UiTextSpan>>,
    current_line: Vec<UiTextSpan>,
    current_width: usize,
}

impl SpanWrapState {
    pub(super) fn new(max_width: usize, preserve_whitespace: bool) -> Self {
        Self {
            max_width,
            preserve_whitespace,
            lines: Vec::new(),
            current_line: Vec::new(),
            current_width: 0,
        }
    }

    pub(super) fn push(
        &mut self,
        renderers: SpanTextRenderers<'_>,
        mut segment: UiTextSpan,
        metrics: UiTreeTextMetrics,
    ) {
        if segment.text == "\n" {
            self.start_new_line();
            return;
        }
        if !self.preserve_whitespace && self.current_width == 0 {
            segment.text = segment.text.trim_start().to_string();
        }
        if segment.text.is_empty() {
            return;
        }
        let width = span_part_width(renderers, &segment, metrics, self.preserve_whitespace);
        if self.current_width > 0 && self.current_width.saturating_add(width) > self.max_width {
            self.start_new_line();
        }
        if !self.preserve_whitespace && self.current_width == 0 {
            segment.text = segment.text.trim_start().to_string();
        }
        if !self.preserve_whitespace && segment.text.trim().is_empty() && self.current_width == 0 {
            return;
        }
        let width = span_part_width(renderers, &segment, metrics, self.preserve_whitespace);
        if width > self.max_width {
            self.push_oversized_segment(renderers, segment, metrics);
            return;
        }
        self.current_width = self.current_width.saturating_add(width);
        self.current_line.push(segment);
    }

    pub(super) fn finish(mut self) -> Vec<Vec<UiTextSpan>> {
        self.start_new_line();
        if self.lines.is_empty() {
            vec![Vec::new()]
        } else {
            self.lines
        }
    }

    fn start_new_line(&mut self) {
        if self.current_line.is_empty() {
            return;
        }
        self.lines.push(std::mem::take(&mut self.current_line));
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
        let mut chunk = String::new();
        for character in segment.text.chars() {
            let mut candidate = chunk.clone();
            candidate.push(character);
            let candidate_segment = segment_with_text(&segment, candidate.as_str());
            let candidate_width = span_part_width(
                renderers,
                &candidate_segment,
                metrics,
                self.preserve_whitespace,
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
        let width = span_part_width(renderers, &next, metrics, self.preserve_whitespace);
        self.current_width = width;
        self.current_line.push(next);
        self.start_new_line();
        chunk.clear();
    }
}

pub(super) fn span_segments(span: &UiTextSpan) -> Vec<UiTextSpan> {
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut pending_space = String::new();
    for character in span.text.chars() {
        if character == '\n' {
            push_current(&mut segments, span, &mut current);
            pending_space.clear();
            push_segment(&mut segments, span, "\n".to_string());
        } else if character.is_whitespace() {
            push_current(&mut segments, span, &mut current);
            pending_space.push(character);
        } else {
            if current.is_empty() {
                current.push_str(&pending_space);
                pending_space.clear();
            }
            current.push(character);
        }
    }
    push_current(&mut segments, span, &mut current);
    if !pending_space.is_empty() {
        push_segment(&mut segments, span, pending_space);
    }
    segments
}

fn push_current(segments: &mut Vec<UiTextSpan>, span: &UiTextSpan, current: &mut String) {
    if !current.is_empty() {
        push_segment(segments, span, std::mem::take(current));
    }
}

fn push_segment(segments: &mut Vec<UiTextSpan>, span: &UiTextSpan, text: String) {
    segments.push(segment_with_text(span, text.as_str()));
}

fn segment_with_text(span: &UiTextSpan, text: &str) -> UiTextSpan {
    let mut segment = span.clone();
    segment.text = text.to_string();
    segment
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::text::TextRenderer;
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
    fn wrap_state_handles_empty_newline_whitespace_and_oversized_segments() {
        let (text, code, metrics) = text_context();
        let renderers = SpanTextRenderers::new(&text, &code);
        let mut state = SpanWrapState::new(12, false);

        state.start_new_line();
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
