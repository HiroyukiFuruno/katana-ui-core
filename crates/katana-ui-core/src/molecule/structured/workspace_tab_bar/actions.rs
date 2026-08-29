use super::identifiers::{WorkspaceTabGroupId, WorkspaceTabId};
use super::options::{WorkspaceTab, WorkspaceTabGroup};
use serde::{Deserialize, Serialize};

pub const CLOSEABLE_TAB_DRAG_TAG: &str = "katana-ui-core/closeable-tab";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceTabBarAction {
    AddTab {
        tab: WorkspaceTab,
        activate: bool,
    },
    SelectTab {
        tab_id: WorkspaceTabId,
    },
    CloseTab {
        tab_id: WorkspaceTabId,
    },
    CloseOthers {
        tab_id: WorkspaceTabId,
    },
    CloseToRight {
        tab_id: WorkspaceTabId,
    },
    CloseToLeft {
        tab_id: WorkspaceTabId,
    },
    CloseAll,
    RestoreClosedTab,
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
    MoveGroup {
        group_id: WorkspaceTabGroupId,
        to_index: usize,
    },
    RenameGroup {
        group_id: WorkspaceTabGroupId,
        label: String,
    },
    SetGroupColor {
        group_id: WorkspaceTabGroupId,
        color: String,
    },
    Ungroup {
        group_id: WorkspaceTabGroupId,
    },
    CloseGroup {
        group_id: WorkspaceTabGroupId,
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
pub enum WorkspaceTabBarIntent {
    RequestTabClose { tab_id: WorkspaceTabId },
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
        let grouped_count = tabs
            .iter()
            .filter(|tab| !tab.pinned && tab.group_id.is_some())
            .count();
        let grouped_without_dragged = grouped_count
            .saturating_sub(usize::from(!dragged.pinned && dragged.group_id.is_some()));
        let pinned_count = tabs.iter().filter(|tab| tab.pinned).count();
        let pinned_without_dragged = pinned_count.saturating_sub(usize::from(dragged.pinned));
        let grouped_start = pinned_without_dragged;
        let ungrouped_start = pinned_without_dragged + grouped_without_dragged;

        if dragged.pinned {
            return to_visual_index <= pinned_without_dragged;
        }
        if dragged.group_id.is_some() {
            return to_visual_index >= grouped_start && to_visual_index <= ungrouped_start;
        }
        to_visual_index >= ungrouped_start
    }
}
