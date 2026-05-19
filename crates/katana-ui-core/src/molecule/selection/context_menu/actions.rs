use super::events::ContextMenuCloseReason;
use super::placement::{ContextMenuSize, ContextMenuViewport};
use super::types::ContextMenuAnchor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextMenuAction {
    Open {
        anchor: ContextMenuAnchor,
    },
    OpenWithLayout {
        anchor: ContextMenuAnchor,
        menu_size: ContextMenuSize,
        viewport: ContextMenuViewport,
    },
    Close {
        reason: ContextMenuCloseReason,
    },
    Highlight {
        path: Vec<usize>,
    },
    Activate {
        path: Vec<usize>,
    },
    OpenSubmenu {
        path: Vec<usize>,
    },
    CloseSubmenu {
        path: Vec<usize>,
    },
    TypeAhead {
        prefix: String,
    },
}
