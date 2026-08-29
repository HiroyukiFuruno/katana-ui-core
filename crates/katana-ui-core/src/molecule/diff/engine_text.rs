use super::types::{CodeDiffLine, CodeDiffLineKind, CodeDiffWhitespace, CollapsedBlock};

const MIN_COLLAPSIBLE_CONTEXT_LINES: usize = 4;

pub(super) fn changed_character_range(text: &str, paired_text: &str) -> (usize, usize) {
    let text_chars: Vec<char> = text.chars().collect();
    let paired_chars: Vec<char> = paired_text.chars().collect();
    let mut start = 0;
    while start < text_chars.len()
        && start < paired_chars.len()
        && text_chars[start] == paired_chars[start]
    {
        start += 1;
    }
    (start, changed_range_end(start, &text_chars, &paired_chars))
}

fn changed_range_end(start: usize, text_chars: &[char], paired_chars: &[char]) -> usize {
    let mut end = text_chars.len();
    let mut paired_end = paired_chars.len();
    while start < end && start < paired_end && text_chars[end - 1] == paired_chars[paired_end - 1] {
        end -= 1;
        paired_end -= 1;
    }
    end
}

pub(super) fn render_text(text: &str, whitespace: Option<&CodeDiffWhitespace>) -> String {
    if text.is_empty() {
        return "↵".to_string();
    }
    let Some(whitespace) = whitespace else {
        return text.to_string();
    };
    if !whitespace.visible {
        return text.to_string();
    }
    text.replace(' ', &whitespace.space_symbol)
        .replace('\t', &whitespace.tab_symbol)
}

pub(super) fn collapsed_blocks(lines: &[CodeDiffLine]) -> Vec<CollapsedBlock> {
    let mut blocks = Vec::new();
    let mut start_index = None;
    for (line_index, line) in lines.iter().enumerate() {
        if line.kind == CodeDiffLineKind::Context {
            if start_index.is_none() {
                start_index = Some(line_index);
            }
            continue;
        }
        push_block(&mut blocks, start_index.take(), line_index);
    }
    push_block(&mut blocks, start_index, lines.len());
    blocks
}

fn push_block(blocks: &mut Vec<CollapsedBlock>, start_index: Option<usize>, end_index: usize) {
    let Some(start_line) = start_index else {
        return;
    };
    let line_count = end_index - start_line;
    if line_count < MIN_COLLAPSIBLE_CONTEXT_LINES {
        return;
    }
    blocks.push(CollapsedBlock {
        start_line,
        line_count,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_text_and_hidden_whitespace_keep_original_text() {
        assert_eq!((4, 4), changed_character_range("same", "same"));
        let whitespace = CodeDiffWhitespace {
            visible: false,
            space_symbol: "·".to_string(),
            tab_symbol: "→".to_string(),
        };
        assert_eq!("a b", render_text("a b", Some(&whitespace)));
    }
}
