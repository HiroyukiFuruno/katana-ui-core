use super::super::wrap_state::WrappedTextLine;
use super::is_paragraph_separator;
use super::{ALERT_BODY_MAX_CHARS, ALERT_TITLE_MAX_CHARS};
use katana_ui_core::render_model::UiTextSpan;

#[cfg(test)]
pub(super) fn alert_plain_lines(label: &str) -> Vec<String> {
    alert_plain_lines_with_metadata(label)
        .into_iter()
        .map(|line| line.content)
        .collect()
}

pub(super) fn fixed_char_chunks(text: &str, max_chars: usize) -> Vec<String> {
    let characters = text.chars().collect::<Vec<_>>();
    let mut chunks = Vec::new();
    for chunk in characters.chunks(max_chars) {
        chunks.push(chunk.iter().collect());
    }
    if chunks.is_empty() {
        chunks.push(String::new());
    }
    chunks
}

#[cfg(test)]
pub(super) fn span_no_wrap_lines(spans: &[UiTextSpan]) -> Vec<Vec<UiTextSpan>> {
    span_no_wrap_lines_with_metadata(spans)
        .into_iter()
        .map(|line| line.content)
        .collect()
}

pub(super) fn span_no_wrap_lines_with_metadata(
    spans: &[UiTextSpan],
) -> Vec<WrappedTextLine<Vec<UiTextSpan>>> {
    let mut lines = vec![WrappedTextLine {
        content: Vec::new(),
        separator_before: String::new(),
        separator_is_explicit: false,
    }];
    for span in spans {
        push_no_wrap_span_with_metadata(&mut lines, span);
    }
    lines
}

fn push_no_wrap_span_with_metadata(
    lines: &mut Vec<WrappedTextLine<Vec<UiTextSpan>>>,
    span: &UiTextSpan,
) {
    let mut start = 0;
    for (index, character) in span.text.char_indices() {
        if !is_paragraph_separator(character) || is_crlf_continuation(&span.text, index, character)
        {
            continue;
        }
        let separator_end = separator_end(&span.text, index, character);
        push_no_wrap_part(lines, span, &span.text[start..index]);
        push_separator_line(lines, &span.text[index..separator_end]);
        start = separator_end;
    }
    push_no_wrap_part(lines, span, &span.text[start..]);
}

fn push_separator_line(lines: &mut Vec<WrappedTextLine<Vec<UiTextSpan>>>, separator: &str) {
    if separator == "\n"
        && let Some(last) = lines.last_mut()
        && last.content.is_empty()
        && last.separator_is_explicit
        && last.separator_before == "\r"
    {
        last.separator_before.push('\n');
        return;
    }
    lines.push(WrappedTextLine {
        content: Vec::new(),
        separator_before: separator.to_owned(),
        separator_is_explicit: true,
    });
}

fn push_no_wrap_part(
    lines: &mut [WrappedTextLine<Vec<UiTextSpan>>],
    span: &UiTextSpan,
    part: &str,
) {
    if part.is_empty() {
        return;
    }
    let mut segment = span.clone();
    segment.text = part.to_string();
    assert!(!lines.is_empty(), "text wrapping always has a line");
    let last_index = lines.len() - 1;
    lines[last_index].content.push(segment);
}

pub(super) fn paragraph_lines(label: &str) -> Vec<WrappedTextLine<String>> {
    let mut lines = Vec::new();
    let mut start = 0;
    let mut separator_before = String::new();
    for (index, character) in label.char_indices() {
        if !is_paragraph_separator(character) || is_crlf_continuation(label, index, character) {
            continue;
        }
        let separator_end = separator_end(label, index, character);
        lines.push(WrappedTextLine {
            content: label[start..index].to_owned(),
            separator_before,
            separator_is_explicit: !lines.is_empty(),
        });
        separator_before = label[index..separator_end].to_owned();
        start = separator_end;
    }
    lines.push(WrappedTextLine {
        content: label[start..].to_owned(),
        separator_before,
        separator_is_explicit: !lines.is_empty(),
    });
    lines
}

fn is_crlf_continuation(text: &str, index: usize, character: char) -> bool {
    character == '\n' && text[..index].ends_with('\r')
}

fn separator_end(text: &str, index: usize, character: char) -> usize {
    if character == '\r' && text[index..].starts_with("\r\n") {
        index + "\r\n".len()
    } else {
        index + character.len_utf8()
    }
}

pub(super) fn plain_lines_with_separator_metadata(
    label: &str,
    wrap: impl Fn(&str) -> Vec<WrappedTextLine<String>>,
) -> Vec<WrappedTextLine<String>> {
    let source_lines = paragraph_lines(label);
    let mut wrapped_lines = Vec::new();
    let mut trailing_source_whitespace = String::new();
    for (source_index, source_line) in source_lines.iter().enumerate() {
        let mut source_wrapped_lines = wrap(&source_line.content);
        if let Some(first) = source_wrapped_lines.first_mut() {
            first.separator_before = format!(
                "{trailing_source_whitespace}{}",
                source_line.separator_before
            );
            first.separator_is_explicit = source_line.separator_is_explicit;
        }
        trailing_source_whitespace.clear();
        if source_index + 1 < source_lines.len() {
            trailing_source_whitespace.push_str(missing_trailing_source_whitespace(
                &source_line.content,
                &source_wrapped_lines,
            ));
        }
        wrapped_lines.extend(source_wrapped_lines);
    }
    wrapped_lines
}

fn missing_trailing_source_whitespace<'a>(
    source_line: &'a str,
    wrapped_lines: &[WrappedTextLine<String>],
) -> &'a str {
    let trimmed = source_line.trim_end_matches(char::is_whitespace);
    let source_trailing_whitespace = &source_line[trimmed.len()..];
    let retained_trailing_whitespace = wrapped_lines
        .iter()
        .rev()
        .flat_map(|line| line.content.chars().rev())
        .take_while(|character| character.is_whitespace())
        .collect::<String>();
    let retained_trailing_whitespace = retained_trailing_whitespace
        .chars()
        .rev()
        .collect::<String>();

    for (offset, _) in source_trailing_whitespace.char_indices() {
        let candidate = &source_trailing_whitespace[offset..];
        if retained_trailing_whitespace.ends_with(candidate) {
            return &source_trailing_whitespace[..offset];
        }
    }
    source_trailing_whitespace
}

pub(super) fn alert_plain_lines_with_metadata(label: &str) -> Vec<WrappedTextLine<String>> {
    let source_lines = paragraph_lines(label);
    let title_max_chars = if source_lines.len() == 1 {
        ALERT_TITLE_MAX_CHARS
    } else {
        ALERT_BODY_MAX_CHARS
    };
    source_lines
        .into_iter()
        .enumerate()
        .flat_map(|(line_index, source_line)| {
            let max_chars = if line_index == 0 {
                title_max_chars
            } else {
                ALERT_BODY_MAX_CHARS
            };
            fixed_char_chunks(&source_line.content, max_chars)
                .into_iter()
                .enumerate()
                .map(move |(chunk_index, content)| WrappedTextLine {
                    content,
                    separator_before: if chunk_index == 0 {
                        source_line.separator_before.clone()
                    } else {
                        "\n".to_owned()
                    },
                    separator_is_explicit: chunk_index == 0 && source_line.separator_is_explicit,
                })
        })
        .collect()
}
