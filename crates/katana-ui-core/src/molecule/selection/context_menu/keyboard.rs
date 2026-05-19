use super::types::{ContextMenuItem, ContextMenuItemKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextMenuKeyboardInput {
    ArrowDown,
    ArrowUp,
    Home,
    End,
    TypeAhead(String),
}

pub struct ContextMenuKeyboardNavigator;

impl ContextMenuKeyboardNavigator {
    #[must_use]
    pub fn move_highlight(
        items: &[ContextMenuItem],
        current: Option<usize>,
        input: &ContextMenuKeyboardInput,
    ) -> Option<usize> {
        let enabled = enabled_item_indices(items);
        if enabled.is_empty() {
            return None;
        }
        match input {
            ContextMenuKeyboardInput::ArrowDown => step(&enabled, current, 1),
            ContextMenuKeyboardInput::ArrowUp => step(&enabled, current, -1),
            ContextMenuKeyboardInput::Home => enabled.first().copied(),
            ContextMenuKeyboardInput::End => enabled.last().copied(),
            ContextMenuKeyboardInput::TypeAhead(prefix) => typeahead(items, &enabled, prefix),
        }
    }
}

fn enabled_item_indices(items: &[ContextMenuItem]) -> Vec<usize> {
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| selectable(item).then_some(index))
        .collect()
}

fn selectable(item: &ContextMenuItem) -> bool {
    !item.disabled
        && !matches!(
            item.kind,
            ContextMenuItemKind::Divider | ContextMenuItemKind::Section
        )
}

fn step(enabled: &[usize], current: Option<usize>, direction: isize) -> Option<usize> {
    let position = current
        .and_then(|value| enabled.iter().position(|index| *index == value))
        .unwrap_or(0);
    let next = (position as isize + direction).rem_euclid(enabled.len() as isize);
    enabled.get(next as usize).copied()
}

fn typeahead(items: &[ContextMenuItem], enabled: &[usize], prefix: &str) -> Option<usize> {
    let normalized = prefix.to_lowercase();
    enabled.iter().copied().find(|index| {
        items
            .get(*index)
            .is_some_and(|item| item.label.to_lowercase().starts_with(&normalized))
    })
}
