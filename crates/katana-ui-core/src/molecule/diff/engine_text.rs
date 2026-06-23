use super::types::{CodeDiffLine, CodeDiffLineKind, CodeDiffWhitespace, CollapsedBlock};

const MIN_COLLAPSIBLE_CONTEXT_LINES: usize = 4;

pub(super) fn changed_character_range(text: &str, paired_text: &str) -> Option<(usize, usize)> {
    if text == paired_text {
        return None;
    }
    let text_chars: Vec<char> = text.chars().collect();
    let paired_chars: Vec<char> = paired_text.chars().collect();
    let mut start = 0;
    while start < text_chars.len()
        && start < paired_chars.len()
        && text_chars[start] == paired_chars[start]
    {
        start += 1;
    }
    Some((start, changed_range_end(start, &text_chars, &paired_chars)))
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
