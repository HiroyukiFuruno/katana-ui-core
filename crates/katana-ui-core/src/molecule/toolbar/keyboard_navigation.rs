use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolbarKeyboardInput {
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Home,
    End,
    Enter,
    Space,
    Escape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarKeyboardResult {
    focused_index: Option<usize>,
    activated_index: Option<usize>,
}

impl ToolbarKeyboardResult {
    #[must_use]
    pub const fn new(focused_index: Option<usize>, activated_index: Option<usize>) -> Self {
        Self {
            focused_index,
            activated_index,
        }
    }

    #[must_use]
    pub const fn focused_index(&self) -> Option<usize> {
        self.focused_index
    }

    #[must_use]
    pub const fn activated_index(&self) -> Option<usize> {
        self.activated_index
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolbarKeyboardNavigator;

impl ToolbarKeyboardNavigator {
    #[must_use]
    pub fn apply(
        focused_index: Option<usize>,
        action_count: usize,
        input: ToolbarKeyboardInput,
    ) -> ToolbarKeyboardResult {
        if action_count == 0 {
            return ToolbarKeyboardResult::new(None, None);
        }
        let focused = focused_index.unwrap_or(0).min(action_count - 1);
        match input {
            ToolbarKeyboardInput::ArrowLeft | ToolbarKeyboardInput::ArrowUp => {
                ToolbarKeyboardResult::new(Some(focused.saturating_sub(1)), None)
            }
            ToolbarKeyboardInput::ArrowRight | ToolbarKeyboardInput::ArrowDown => {
                ToolbarKeyboardResult::new(
                    Some(focused.saturating_add(1).min(action_count - 1)),
                    None,
                )
            }
            ToolbarKeyboardInput::Home => ToolbarKeyboardResult::new(Some(0), None),
            ToolbarKeyboardInput::End => ToolbarKeyboardResult::new(Some(action_count - 1), None),
            ToolbarKeyboardInput::Enter | ToolbarKeyboardInput::Space => {
                ToolbarKeyboardResult::new(Some(focused), Some(focused))
            }
            ToolbarKeyboardInput::Escape => ToolbarKeyboardResult::new(Some(focused), None),
        }
    }
}
