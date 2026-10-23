use super::super::super::is_paragraph_separator;
use katana_ui_core::render_model::UiTextSpan;

pub(super) struct SourceSpanSegment {
    pub(super) span: UiTextSpan,
    pub(super) lf_is_at_source_span_start: bool,
    pub(super) lf_follows_carriage_return: bool,
    pub(super) whitespace_before_separator: String,
}

#[cfg(test)]
pub(super) fn span_segments(span: &UiTextSpan) -> Vec<UiTextSpan> {
    span_segments_with_source_metadata(span)
        .into_iter()
        .map(|segment| segment.span)
        .collect()
}

pub(super) fn span_segments_with_source_metadata(span: &UiTextSpan) -> Vec<SourceSpanSegment> {
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut pending_space = String::new();
    let mut characters = span.text.chars().peekable();
    let mut previous_source_character_was_carriage_return = false;
    let mut source_has_prior_character = false;
    while let Some(character) = characters.next() {
        if is_paragraph_separator(character) {
            push_current(&mut segments, span, &mut current);
            let whitespace_before_separator = std::mem::take(&mut pending_space);
            let lf_is_at_source_span_start = character == '\n' && !source_has_prior_character;
            let mut separator = character.to_string();
            if character == '\r'
                && matches!(characters.peek(), Some('\n'))
                && let Some(next) = characters.next()
            {
                separator.push(next);
                push_segment(
                    &mut segments,
                    span,
                    separator,
                    false,
                    false,
                    whitespace_before_separator,
                );
                previous_source_character_was_carriage_return = false;
            } else {
                let lf_follows_carriage_return =
                    character == '\n' && previous_source_character_was_carriage_return;
                push_segment(
                    &mut segments,
                    span,
                    separator,
                    lf_is_at_source_span_start,
                    lf_follows_carriage_return,
                    whitespace_before_separator,
                );
                previous_source_character_was_carriage_return = character == '\r';
            }
            source_has_prior_character = true;
        } else if character.is_whitespace() {
            push_current(&mut segments, span, &mut current);
            pending_space.push(character);
            previous_source_character_was_carriage_return = false;
            source_has_prior_character = true;
        } else {
            if current.is_empty() {
                current.push_str(&pending_space);
                pending_space.clear();
            }
            current.push(character);
            previous_source_character_was_carriage_return = false;
            source_has_prior_character = true;
        }
    }
    push_current(&mut segments, span, &mut current);
    if !pending_space.is_empty() {
        push_segment(
            &mut segments,
            span,
            pending_space,
            false,
            false,
            String::new(),
        );
    }
    segments
}

fn push_current(segments: &mut Vec<SourceSpanSegment>, span: &UiTextSpan, current: &mut String) {
    if !current.is_empty() {
        push_segment(
            segments,
            span,
            std::mem::take(current),
            false,
            false,
            String::new(),
        );
    }
}

fn push_segment(
    segments: &mut Vec<SourceSpanSegment>,
    span: &UiTextSpan,
    text: String,
    lf_is_at_source_span_start: bool,
    lf_follows_carriage_return: bool,
    whitespace_before_separator: String,
) {
    segments.push(SourceSpanSegment {
        span: segment_with_text(span, text.as_str()),
        lf_is_at_source_span_start,
        lf_follows_carriage_return,
        whitespace_before_separator,
    });
}

pub(super) fn segment_with_text(span: &UiTextSpan, text: &str) -> UiTextSpan {
    let mut segment = span.clone();
    segment.text = text.to_string();
    segment
}
