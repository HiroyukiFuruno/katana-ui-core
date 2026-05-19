pub(super) fn clamp_to_char_boundary(value: &str, offset: usize) -> usize {
    if offset >= value.len() {
        return value.len();
    }
    let mut boundary = offset;
    while boundary > 0 && !value.is_char_boundary(boundary) {
        boundary -= 1;
    }
    boundary
}

pub(super) fn previous_grapheme_start(value: &str, caret: usize) -> usize {
    let caret = clamp_to_char_boundary(value, caret);
    grapheme_ranges(value)
        .into_iter()
        .find(|(_, end)| *end >= caret)
        .map_or(0, |(start, _)| start)
}

pub(super) fn next_grapheme_end(value: &str, caret: usize) -> usize {
    let caret = clamp_to_char_boundary(value, caret);
    grapheme_ranges(value)
        .into_iter()
        .find(|(_, end)| *end > caret)
        .map_or(value.len(), |(_, end)| end)
}

pub(super) fn delete_previous_grapheme(value: &mut String, caret: usize) -> usize {
    let caret = clamp_to_char_boundary(value, caret);
    if caret == 0 {
        return 0;
    }
    let start = previous_grapheme_start(value, caret);
    value.replace_range(start..caret, "");
    start
}

pub(super) fn count_graphemes(value: &str) -> usize {
    grapheme_ranges(value).len()
}

fn grapheme_ranges(value: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut start = None;
    let mut previous = None;

    for (index, character) in value.char_indices() {
        match (start, previous) {
            (None, _) => start = Some(index),
            (Some(current_start), Some(previous_character))
                if !joins_previous(previous_character, character) =>
            {
                ranges.push((current_start, index));
                start = Some(index);
            }
            _ => {}
        }
        previous = Some(character);
    }

    if let Some(current_start) = start {
        ranges.push((current_start, value.len()));
    }
    ranges
}

fn joins_previous(previous: char, current: char) -> bool {
    previous == '\u{200d}'
        || current == '\u{200d}'
        || is_variation_selector(current)
        || is_combining_mark(current)
        || is_emoji_modifier(current)
}

fn is_variation_selector(character: char) -> bool {
    matches!(character as u32, 0xfe00..=0xfe0f)
}

fn is_combining_mark(character: char) -> bool {
    matches!(character as u32, 0x0300..=0x036f | 0x1ab0..=0x1aff | 0x1dc0..=0x1dff)
}

fn is_emoji_modifier(character: char) -> bool {
    matches!(character as u32, 0x1f3fb..=0x1f3ff)
}
