use super::layout_model::{TextSurfaceGraphemeBox, TextSurfaceLineBox};
use crate::render_model::UiRect;
use std::collections::BTreeMap;

pub(super) fn line_starts(text: &str) -> Vec<usize> {
    let mut starts = vec![0];
    let mut characters = text.char_indices().peekable();
    while let Some((index, character)) = characters.next() {
        if character == '\r' && characters.peek().is_some_and(|(_, next)| *next == '\n') {
            if let Some((next, character)) = characters.next() {
                starts.push(next + character.len_utf8());
            }
        } else if matches!(character, '\n' | '\r' | '\u{2028}' | '\u{2029}') {
            starts.push(index + character.len_utf8());
        }
    }
    starts
}

pub(super) fn line_boxes(
    text: &str,
    graphemes: &[TextSurfaceGraphemeBox],
    content_bounds: UiRect,
) -> Vec<TextSurfaceLineBox> {
    let starts = line_starts(text);
    let mut rows = BTreeMap::<(usize, i32), UiRect>::new();
    for grapheme in graphemes {
        let logical_row = starts
            .partition_point(|start| *start <= grapheme.byte_start)
            .saturating_sub(1);
        rows.entry((logical_row, grapheme.bounds.y))
            .and_modify(|bounds| {
                *bounds = super::layout_model::TextSurfaceLayout::union(*bounds, grapheme.bounds);
            })
            .or_insert(grapheme.bounds);
    }
    let line_height = rows
        .values()
        .map(|bounds| bounds.height)
        .max()
        .unwrap_or(content_bounds.height.max(1));
    for logical_row in 0..starts.len() {
        let key = (logical_row, i32::MIN);
        if rows.range(key..=(logical_row, i32::MAX)).next().is_some() {
            continue;
        }
        let y = rows
            .range(..key)
            .next_back()
            .map_or(content_bounds.y, |(_, bounds)| {
                bounds.y.saturating_add_unsigned(bounds.height)
            });
        rows.insert(
            (logical_row, y),
            UiRect::new(content_bounds.x, y, 1, line_height),
        );
    }
    rows.into_iter()
        .map(|((logical_row, _), bounds)| TextSurfaceLineBox {
            logical_row,
            bounds,
        })
        .collect()
}
