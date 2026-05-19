use super::events::ContextMenuCloseReason;
use super::types::ContextMenuAnchor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextMenuAction {
    Open { anchor: ContextMenuAnchor },
    Close { reason: ContextMenuCloseReason },
    Highlight { path: Vec<usize> },
    Activate { path: Vec<usize> },
    OpenSubmenu { path: Vec<usize> },
    CloseSubmenu { path: Vec<usize> },
    TypeAhead { prefix: String },
}
