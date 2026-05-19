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
            Self::TabSelected { .. } => "workspace_tab_selected",
            Self::TabCloseRequested { .. } => "workspace_tab_close_requested",
            Self::TabClosed { .. } => "workspace_tab_closed",
            Self::TabReordered { .. } => "workspace_tab_reordered",
            Self::GroupCollapseChanged { .. } => "workspace_tab_group_collapse_changed",
            Self::OverflowOpened { .. } => "workspace_tab_overflow_opened",
        }
    }
}
