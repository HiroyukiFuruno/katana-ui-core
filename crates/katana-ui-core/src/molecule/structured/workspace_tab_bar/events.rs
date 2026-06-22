use super::identifiers::{WorkspaceTabGroupId, WorkspaceTabId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceTabBarEvent {
    TabAdded {
        tab_id: WorkspaceTabId,
    },
    TabSelected {
        tab_id: WorkspaceTabId,
    },
    TabCloseRequested {
        tab_id: WorkspaceTabId,
    },
    TabClosed {
        tab_id: WorkspaceTabId,
    },
    TabRestored {
        tab_id: WorkspaceTabId,
    },
    TabReordered {
        tab_id: WorkspaceTabId,
        from: usize,
        to: usize,
    },
    TabPinChanged {
        tab_id: WorkspaceTabId,
        pinned: bool,
    },
    TabGroupChanged {
        tab_id: WorkspaceTabId,
        group_id: Option<WorkspaceTabGroupId>,
    },
    GroupCreated {
        group_id: WorkspaceTabGroupId,
    },
    GroupReordered {
        group_id: WorkspaceTabGroupId,
        from: usize,
        to: usize,
    },
    GroupRenamed {
        group_id: WorkspaceTabGroupId,
        label: String,
    },
    GroupColorChanged {
        group_id: WorkspaceTabGroupId,
        color: String,
    },
    GroupRemoved {
        group_id: WorkspaceTabGroupId,
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
            Self::TabAdded { .. } => "closeable_tab_added",
            Self::TabSelected { .. } => "closeable_tab_selected",
            Self::TabCloseRequested { .. } => "closeable_tab_close_requested",
            Self::TabClosed { .. } => "closeable_tab_closed",
            Self::TabRestored { .. } => "closeable_tab_restored",
            Self::TabReordered { .. } => "closeable_tab_reordered",
            Self::TabPinChanged { .. } => "closeable_tab_pin_changed",
            Self::TabGroupChanged { .. } => "closeable_tab_group_changed",
            Self::GroupCreated { .. } => "closeable_tab_group_created",
            Self::GroupReordered { .. } => "closeable_tab_group_reordered",
            Self::GroupRenamed { .. } => "closeable_tab_group_renamed",
            Self::GroupColorChanged { .. } => "closeable_tab_group_color_changed",
            Self::GroupRemoved { .. } => "closeable_tab_group_removed",
            Self::DragStarted { .. } => "closeable_tab_drag_started",
            Self::DragEnded { .. } => "closeable_tab_drag_ended",
            Self::GroupCollapseChanged { .. } => "closeable_tab_group_collapse_changed",
            Self::OverflowOpened { .. } => "closeable_tab_overflow_opened",
        }
    }
}
