use super::identifiers::{WorkspaceTabGroupId, WorkspaceTabId};
use super::options::{WorkspaceTab, WorkspaceTabGroup};
use serde::{Deserialize, Serialize};

pub const CLOSEABLE_TAB_DRAG_TAG: &str = "katana-ui-core/closeable-tab";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceTabBarAction {
    SelectTab {
        tab_id: WorkspaceTabId,
    },
    CloseTab {
        tab_id: WorkspaceTabId,
    },
    PinTab {
        tab_id: WorkspaceTabId,
    },
    UnpinTab {
        tab_id: WorkspaceTabId,
    },
    MoveTab {
        tab_id: WorkspaceTabId,
        to_visual_index: usize,
    },
    MoveToGroup {
        tab_id: WorkspaceTabId,
        target: WorkspaceTabGroupTarget,
    },
    StartDrag {
        tab_id: WorkspaceTabId,
    },
    EndDrag {
        committed: bool,
    },
    CancelDrag,
    HoverCollapsedGroupForDrop {
        group_id: WorkspaceTabGroupId,
        elapsed_ms: u16,
    },
    ToggleGroupCollapse {
        group_id: WorkspaceTabGroupId,
    },
    OpenOverflow {
        hidden_tab_ids: Vec<WorkspaceTabId>,
    },
    ConfirmClose {
        tab_id: WorkspaceTabId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceTabGroupTarget {
    Existing(WorkspaceTabGroupId),
    Ungrouped,
    NewGroup(WorkspaceTabGroup),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceTabDropPosition {
    Before,
    After,
    InsideGroup,
    NewGroup,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabDropRules;

impl WorkspaceTabDropRules {
    #[must_use]
    pub fn can_accept(
        tabs: &[WorkspaceTab],
        dragged_tab_id: &WorkspaceTabId,
        to_visual_index: usize,
    ) -> bool {
        let Some(dragged) = tabs.iter().find(|tab| &tab.id == dragged_tab_id) else {
            return false;
        };
        let pinned_count = tabs.iter().filter(|tab| tab.pinned).count();
        let pinned_without_dragged = pinned_count.saturating_sub(usize::from(dragged.pinned));

        if dragged.pinned {
            return to_visual_index <= pinned_without_dragged;
        }
        to_visual_index >= pinned_without_dragged
    }
}
