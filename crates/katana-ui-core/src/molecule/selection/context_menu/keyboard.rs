use super::types::{ContextMenuItem, ContextMenuItemKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextMenuKeyboardInput {
    ArrowDown,
    ArrowUp,
    Home,
    End,
    Enter,
    Space,
    Escape,
    ArrowRight,
    ArrowLeft,
    TypeAhead(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextMenuKeyboardIntent {
    MoveTo(usize),
    Activate,
    Close,
    OpenSubmenu,
    CloseSubmenu,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextMenuTypeAheadBuffer {
    prefix: String,
    last_input_ms: u64,
    timeout_ms: u64,
}

impl ContextMenuTypeAheadBuffer {
    #[must_use]
    pub fn new(timeout_ms: u64) -> Self {
        Self {
            prefix: String::new(),
            last_input_ms: 0,
            timeout_ms,
        }
    }

    pub fn push(&mut self, input: &str, now_ms: u64) -> String {
        if now_ms.saturating_sub(self.last_input_ms) > self.timeout_ms {
            self.prefix.clear();
        }
        self.prefix.push_str(input);
        self.last_input_ms = now_ms;
        self.prefix.clone()
    }
}

pub struct ContextMenuKeyboardNavigator;

impl ContextMenuKeyboardNavigator {
    #[must_use]
    pub fn move_highlight(
        items: &[ContextMenuItem],
        current: Option<usize>,
        input: &ContextMenuKeyboardInput,
    ) -> Option<usize> {
        match Self::intent(items, current, input) {
            ContextMenuKeyboardIntent::MoveTo(index) => return Some(index),
            ContextMenuKeyboardIntent::Activate
            | ContextMenuKeyboardIntent::Close
            | ContextMenuKeyboardIntent::OpenSubmenu
            | ContextMenuKeyboardIntent::CloseSubmenu
            | ContextMenuKeyboardIntent::None => {}
        }
        None
    }

    #[must_use]
    pub fn intent(
        items: &[ContextMenuItem],
        current: Option<usize>,
        input: &ContextMenuKeyboardInput,
    ) -> ContextMenuKeyboardIntent {
        let enabled = enabled_item_indices(items);
        if enabled.is_empty() {
            return ContextMenuKeyboardIntent::None;
        }
        match input {
            ContextMenuKeyboardInput::ArrowDown => {
                move_to(step(&enabled, current, Direction::Forward))
            }
            ContextMenuKeyboardInput::ArrowUp => move_to(step(&enabled, current, Direction::Back)),
            ContextMenuKeyboardInput::Home => move_to(enabled.first().copied()),
            ContextMenuKeyboardInput::End => move_to(enabled.last().copied()),
            ContextMenuKeyboardInput::Enter | ContextMenuKeyboardInput::Space => {
                ContextMenuKeyboardIntent::Activate
            }
            ContextMenuKeyboardInput::Escape => ContextMenuKeyboardIntent::Close,
            ContextMenuKeyboardInput::ArrowRight => ContextMenuKeyboardIntent::OpenSubmenu,
            ContextMenuKeyboardInput::ArrowLeft => ContextMenuKeyboardIntent::CloseSubmenu,
            ContextMenuKeyboardInput::TypeAhead(prefix) => {
                move_to(typeahead(items, &enabled, prefix))
            }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Forward,
    Back,
}

fn step(enabled: &[usize], current: Option<usize>, direction: Direction) -> Option<usize> {
    let Some(current) = current else {
        return match direction {
            Direction::Forward => enabled.first().copied(),
            Direction::Back => enabled.last().copied(),
        };
    };
    let position = enabled
        .iter()
        .position(|index| *index == current)
        .unwrap_or(0);
    let delta = match direction {
        Direction::Forward => 1,
        Direction::Back => -1,
    };
    let next = (position as isize + delta).rem_euclid(enabled.len() as isize);
    enabled.get(next as usize).copied()
}

fn move_to(index: Option<usize>) -> ContextMenuKeyboardIntent {
    index.map_or(
        ContextMenuKeyboardIntent::None,
        ContextMenuKeyboardIntent::MoveTo,
    )
}

fn typeahead(items: &[ContextMenuItem], enabled: &[usize], prefix: &str) -> Option<usize> {
    let normalized = prefix.to_lowercase();
    enabled.iter().copied().find(|index| {
        items
            .get(*index)
            .is_some_and(|item| item.label.to_lowercase().starts_with(&normalized))
    })
}
