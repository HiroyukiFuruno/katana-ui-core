use super::types::{ContextMenuAnchor, ContextMenuPlacement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextMenuCloseReason {
    Escape,
    OutsideClick,
    Selected,
    FocusReturn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextMenuEvent {
    Opened {
        anchor: ContextMenuAnchor,
        placement_used: ContextMenuPlacement,
    },
    Closed {
        reason: ContextMenuCloseReason,
    },
    ItemHighlighted {
        path: Vec<usize>,
    },
    ItemSelected {
        path: Vec<usize>,
        command: String,
    },
    ItemActivationBlocked {
        path: Vec<usize>,
    },
    SubmenuOpened {
        path: Vec<usize>,
    },
    SubmenuClosed {
        path: Vec<usize>,
    },
    TypeAheadMatched {
        prefix: String,
        path: Vec<usize>,
    },
}

impl ContextMenuEvent {
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Opened { .. } => "context_menu_opened",
            Self::Closed { .. } => "context_menu_closed",
            Self::ItemHighlighted { .. } => "context_menu_item_highlighted",
            Self::ItemSelected { .. } => "context_menu_item_selected",
            Self::ItemActivationBlocked { .. } => "context_menu_item_activation_blocked",
            Self::SubmenuOpened { .. } => "context_menu_submenu_opened",
            Self::SubmenuClosed { .. } => "context_menu_submenu_closed",
            Self::TypeAheadMatched { .. } => "context_menu_typeahead_matched",
        }
    }
}
