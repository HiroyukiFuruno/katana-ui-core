use super::actions::{WorkspaceTabBarAction, WorkspaceTabGroupTarget};
use super::identifiers::{WorkspaceTabGroupId, WorkspaceTabId};
use super::options::WorkspaceTabGroup;
use crate::molecule::selection::ContextMenuItem;
use serde::{Deserialize, Serialize};

const MOVE_TO_GROUP_ITEM_PREFIX: &str = "move-to-group:";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceTabContextCommand {
    Close,
    CloseOthers,
    CloseAll,
    CloseToRight,
    CloseToLeft,
    RestoreClosed,
    Pin,
    Unpin,
    MoveToNewGroup,
    MoveToGroup,
}

impl WorkspaceTabContextCommand {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Close => "close",
            Self::CloseOthers => "close-others",
            Self::CloseAll => "close-all",
            Self::CloseToRight => "close-to-right",
            Self::CloseToLeft => "close-to-left",
            Self::RestoreClosed => "restore-closed",
            Self::Pin => "pin",
            Self::Unpin => "unpin",
            Self::MoveToNewGroup => "move-to-new-group",
            Self::MoveToGroup => "move-to-group",
        }
    }

    #[must_use]
    pub fn from_id(value: &str) -> Option<Self> {
        match value {
            "close" => Some(Self::Close),
            "close-others" => Some(Self::CloseOthers),
            "close-all" => Some(Self::CloseAll),
            "close-to-right" => Some(Self::CloseToRight),
            "close-to-left" => Some(Self::CloseToLeft),
            "restore-closed" => Some(Self::RestoreClosed),
            "pin" => Some(Self::Pin),
            "unpin" => Some(Self::Unpin),
            "move-to-new-group" => Some(Self::MoveToNewGroup),
            "move-to-group" => Some(Self::MoveToGroup),
            _ => None,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Close => "Close",
            Self::CloseOthers => "Close Others",
            Self::CloseAll => "Close All",
            Self::CloseToRight => "Close Tabs to the Right",
            Self::CloseToLeft => "Close Tabs to the Left",
            Self::RestoreClosed => "Restore Closed Tab",
            Self::Pin => "Pin",
            Self::Unpin => "Unpin",
            Self::MoveToNewGroup => "Move to New Group",
            Self::MoveToGroup => "Add to Group",
        }
    }

    #[must_use]
    pub fn to_context_menu_item(self) -> ContextMenuItem {
        ContextMenuItem::action(self.id(), self.label())
    }

    #[must_use]
    pub fn move_to_group_item_id(group_id: &WorkspaceTabGroupId) -> String {
        format!("{MOVE_TO_GROUP_ITEM_PREFIX}{}", group_id.as_str())
    }

    #[must_use]
    pub fn move_to_group_id_from_item_id(value: &str) -> Option<WorkspaceTabGroupId> {
        value
            .strip_prefix(MOVE_TO_GROUP_ITEM_PREFIX)
            .map(WorkspaceTabGroupId::new)
    }

    #[must_use]
    pub fn to_tab_action(self, tab_id: WorkspaceTabId) -> Option<WorkspaceTabBarAction> {
        match self {
            Self::Close => Some(WorkspaceTabBarAction::CloseTab { tab_id }),
            Self::CloseOthers => Some(WorkspaceTabBarAction::CloseOthers { tab_id }),
            Self::CloseAll => Some(WorkspaceTabBarAction::CloseAll),
            Self::CloseToRight => Some(WorkspaceTabBarAction::CloseToRight { tab_id }),
            Self::CloseToLeft => Some(WorkspaceTabBarAction::CloseToLeft { tab_id }),
            Self::RestoreClosed => Some(WorkspaceTabBarAction::RestoreClosedTab),
            Self::Pin => Some(WorkspaceTabBarAction::PinTab { tab_id }),
            Self::Unpin => Some(WorkspaceTabBarAction::UnpinTab { tab_id }),
            Self::MoveToNewGroup | Self::MoveToGroup => None,
        }
    }

    #[must_use]
    pub fn move_to_existing_group_action(
        tab_id: WorkspaceTabId,
        group_id: WorkspaceTabGroupId,
    ) -> WorkspaceTabBarAction {
        WorkspaceTabBarAction::MoveToGroup {
            tab_id,
            target: WorkspaceTabGroupTarget::Existing(group_id),
        }
    }

    #[must_use]
    pub fn move_to_new_group_action(
        tab_id: WorkspaceTabId,
        group: WorkspaceTabGroup,
    ) -> WorkspaceTabBarAction {
        WorkspaceTabBarAction::MoveToGroup {
            tab_id,
            target: WorkspaceTabGroupTarget::NewGroup(group),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceGroupContextCommand {
    Rename,
    SetColor,
    Collapse,
    Expand,
    Move,
    Ungroup,
    Close,
}

impl WorkspaceGroupContextCommand {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Rename => "rename",
            Self::SetColor => "set-color",
            Self::Collapse => "collapse",
            Self::Expand => "expand",
            Self::Move => "move",
            Self::Ungroup => "ungroup",
            Self::Close => "close-group",
        }
    }

    #[must_use]
    pub fn from_id(value: &str) -> Option<Self> {
        match value {
            "rename" => Some(Self::Rename),
            "set-color" => Some(Self::SetColor),
            "collapse" => Some(Self::Collapse),
            "expand" => Some(Self::Expand),
            "move" => Some(Self::Move),
            "ungroup" => Some(Self::Ungroup),
            "close-group" => Some(Self::Close),
            _ => None,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Rename => "Rename Group",
            Self::SetColor => "Set Group Color",
            Self::Collapse => "Collapse Group",
            Self::Expand => "Expand Group",
            Self::Move => "Move Group",
            Self::Ungroup => "Ungroup",
            Self::Close => "Close Group",
        }
    }

    #[must_use]
    pub fn to_context_menu_item(self) -> ContextMenuItem {
        ContextMenuItem::action(self.id(), self.label())
    }

    #[must_use]
    pub fn to_group_action(self, group: &WorkspaceTabGroup) -> Option<WorkspaceTabBarAction> {
        match self {
            Self::Collapse if !group.collapsed => {
                Some(WorkspaceTabBarAction::ToggleGroupCollapse {
                    group_id: group.id.clone(),
                })
            }
            Self::Expand if group.collapsed => Some(WorkspaceTabBarAction::ToggleGroupCollapse {
                group_id: group.id.clone(),
            }),
            Self::Ungroup => Some(WorkspaceTabBarAction::Ungroup {
                group_id: group.id.clone(),
            }),
            Self::Close => Some(WorkspaceTabBarAction::CloseGroup {
                group_id: group.id.clone(),
            }),
            _ => None,
        }
    }

    #[must_use]
    pub fn move_group_action(
        group_id: WorkspaceTabGroupId,
        to_index: usize,
    ) -> WorkspaceTabBarAction {
        WorkspaceTabBarAction::MoveGroup { group_id, to_index }
    }

    #[must_use]
    pub fn rename_group_action(
        group_id: WorkspaceTabGroupId,
        label: impl Into<String>,
    ) -> WorkspaceTabBarAction {
        WorkspaceTabBarAction::RenameGroup {
            group_id,
            label: label.into(),
        }
    }

    #[must_use]
    pub fn set_group_color_action(
        group_id: WorkspaceTabGroupId,
        color: impl Into<String>,
    ) -> WorkspaceTabBarAction {
        WorkspaceTabBarAction::SetGroupColor {
            group_id,
            color: color.into(),
        }
    }
}
