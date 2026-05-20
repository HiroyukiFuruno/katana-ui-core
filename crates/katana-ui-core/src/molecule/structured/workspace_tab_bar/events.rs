use super::identifiers::{WorkspaceTabGroupId, WorkspaceTabId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceTabBarEvent {
    TabSelected {
        tab_id: WorkspaceTabId,
    },
    TabCloseRequested {
        tab_id: WorkspaceTabId,
    },
    TabClosed {
        tab_id: WorkspaceTabId,
    },
    TabReordered {
        tab_id: WorkspaceTabId,
        from: usize,
        to: usize,
    },
    DragStarted {
        tab_id: WorkspaceTabId,
    },
    DragEnded {
        tab_id: WorkspaceTabId,
        committed: bool,
    },
    GroupCollapseChanged {
        group_id: WorkspaceTabGroupId,
        collapsed: bool,
    },
    OverflowOpened {
        hidden_tab_ids: Vec<WorkspaceTabId>,
    },
}

impl WorkspaceTabBarEvent {
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::TabSelected { .. } => "closeable_tab_selected",
            Self::TabCloseRequested { .. } => "closeable_tab_close_requested",
            Self::TabClosed { .. } => "closeable_tab_closed",
            Self::TabReordered { .. } => "closeable_tab_reordered",
            Self::DragStarted { .. } => "closeable_tab_drag_started",
            Self::DragEnded { .. } => "closeable_tab_drag_ended",
            Self::GroupCollapseChanged { .. } => "closeable_tab_group_collapse_changed",
            Self::OverflowOpened { .. } => "closeable_tab_overflow_opened",
        }
    }
}
