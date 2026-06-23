use super::PanelMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollapsiblePanelAction {
    ToggleExpand,
    SetMode(PanelMode),
    Resize(u16),
    ResetWidth,
    HoverTrigger,
    LeaveTrigger,
    Pin,
    Unpin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollapsiblePanelEvent {
    ModeChanged {
        from: PanelMode,
        to: PanelMode,
    },
    WidthChanged {
        width: u16,
        persist_id: Option<String>,
    },
    PinChanged {
        pinned: bool,
    },
    HoverTemporaryExpanded {
        from: PanelMode,
        to: PanelMode,
    },
    HoverTemporaryClosed {
        restored: PanelMode,
    },
    FloatingShown,
    FloatingHidden,
}
