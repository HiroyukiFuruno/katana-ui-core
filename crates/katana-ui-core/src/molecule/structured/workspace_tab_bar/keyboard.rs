use super::actions::WorkspaceTabBarAction;
use super::identifiers::WorkspaceTabId;
use serde::{Deserialize, Serialize};

const LAST_VISIBLE_DIGIT: u8 = 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceTabKey {
    Tab,
    W,
    Digit(u8),
    Escape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabKeyboardShortcut {
    pub key: WorkspaceTabKey,
    pub command_or_control: bool,
    pub shift: bool,
}

impl WorkspaceTabKeyboardShortcut {
    #[must_use]
    pub const fn new(key: WorkspaceTabKey, command_or_control: bool, shift: bool) -> Self {
        Self {
            key,
            command_or_control,
            shift,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceTabKeyboardInput {
    NextTab,
    PreviousTab,
    CloseActiveTab,
    SelectVisible(usize),
    SelectLastVisible,
    CancelDrag,
}

impl WorkspaceTabKeyboardInput {
    #[must_use]
    pub fn from_shortcut(shortcut: WorkspaceTabKeyboardShortcut) -> Option<Self> {
        if shortcut.key == WorkspaceTabKey::Escape {
            return Some(Self::CancelDrag);
        }
        if !shortcut.command_or_control {
            return None;
        }
        match shortcut.key {
            WorkspaceTabKey::Tab if shortcut.shift => Some(Self::PreviousTab),
            WorkspaceTabKey::Tab => Some(Self::NextTab),
            WorkspaceTabKey::W => Some(Self::CloseActiveTab),
            WorkspaceTabKey::Digit(0) => Some(Self::SelectLastVisible),
            WorkspaceTabKey::Digit(value @ 1..=LAST_VISIBLE_DIGIT) => {
                Some(Self::SelectVisible(usize::from(value)))
            }
            WorkspaceTabKey::Digit(_) | WorkspaceTabKey::Escape => None,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabKeyboardController;

impl WorkspaceTabKeyboardController {
    #[must_use]
    pub fn action_for_input(
        input: &WorkspaceTabKeyboardInput,
        active_tab_id: Option<&WorkspaceTabId>,
        visible_tab_ids: &[WorkspaceTabId],
    ) -> Option<WorkspaceTabBarAction> {
        match input {
            WorkspaceTabKeyboardInput::NextTab => {
                select_relative(active_tab_id, visible_tab_ids, 1)
            }
            WorkspaceTabKeyboardInput::PreviousTab => {
                select_relative(active_tab_id, visible_tab_ids, -1)
            }
            WorkspaceTabKeyboardInput::CloseActiveTab => active_tab_id
                .cloned()
                .map(|tab_id| WorkspaceTabBarAction::CloseTab { tab_id }),
            WorkspaceTabKeyboardInput::SelectVisible(index) => {
                select_number(*index, visible_tab_ids)
            }
            WorkspaceTabKeyboardInput::SelectLastVisible => visible_tab_ids
                .last()
                .cloned()
                .map(|tab_id| WorkspaceTabBarAction::SelectTab { tab_id }),
            WorkspaceTabKeyboardInput::CancelDrag => None,
        }
    }
}

fn select_number(
    one_based_index: usize,
    visible_tab_ids: &[WorkspaceTabId],
) -> Option<WorkspaceTabBarAction> {
    visible_tab_ids
        .get(one_based_index.saturating_sub(1))
        .cloned()
        .map(|tab_id| WorkspaceTabBarAction::SelectTab { tab_id })
}

fn select_relative(
    active_tab_id: Option<&WorkspaceTabId>,
    visible_tab_ids: &[WorkspaceTabId],
    step: isize,
) -> Option<WorkspaceTabBarAction> {
    let active_id = active_tab_id?;
    let active_index = visible_tab_ids
        .iter()
        .position(|tab_id| tab_id == active_id)?;
    let next_index = wrapped_index(active_index, visible_tab_ids.len(), step);
    visible_tab_ids
        .get(next_index)
        .cloned()
        .map(|tab_id| WorkspaceTabBarAction::SelectTab { tab_id })
}

fn wrapped_index(index: usize, len: usize, step: isize) -> usize {
    if step.is_negative() && index == 0 {
        return len - 1;
    }
    if step.is_positive() && index + 1 == len {
        return 0;
    }
    index.wrapping_add_signed(step)
}
