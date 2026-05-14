use crate::composite::command_palette::types::CommandPaletteItem;
use std::cmp::Ordering;

pub(super) fn sort_by_score<P: Clone + 'static>(items: &mut [CommandPaletteItem<P>]) {
    items.sort_by(|a, b| match b.score.cmp(&a.score) {
        Ordering::Equal => a.label.cmp(&b.label),
        other => other,
    });
}

pub(super) fn move_next(len: usize, current: usize) -> usize {
    if len == 0 { 0 } else { (current + 1) % len }
}

pub(super) fn move_previous(len: usize, current: usize) -> usize {
    if len == 0 {
        0
    } else if current == 0 {
        len - 1
    } else {
        current - 1
    }
}
