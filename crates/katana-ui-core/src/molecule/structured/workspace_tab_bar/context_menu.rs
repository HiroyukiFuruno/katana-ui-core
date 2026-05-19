use super::options::{WorkspaceTab, WorkspaceTabGroup};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceTabContextCommand {
    Close,
    CloseOthers,
    CloseToRight,
    CloseAll,
    Pin,
    Unpin,
    MoveToNewGroup,
    MoveToGroup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceGroupContextCommand {
    Rename,
    Collapse,
    Expand,
    Move,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabContextMenu;

impl WorkspaceTabContextMenu {
    #[must_use]
    pub fn tab_commands(
        tab: &WorkspaceTab,
        groups: &[WorkspaceTabGroup],
    ) -> Vec<WorkspaceTabContextCommand> {
        let mut commands = vec![
            WorkspaceTabContextCommand::Close,
            WorkspaceTabContextCommand::CloseOthers,
            WorkspaceTabContextCommand::CloseToRight,
            WorkspaceTabContextCommand::CloseAll,
            pin_command(tab),
            WorkspaceTabContextCommand::MoveToNewGroup,
        ];
        if !groups.is_empty() {
            commands.push(WorkspaceTabContextCommand::MoveToGroup);
        }
        commands
    }

    #[must_use]
    pub fn group_commands(group: &WorkspaceTabGroup) -> Vec<WorkspaceGroupContextCommand> {
        let collapse_command = if group.collapsed {
            WorkspaceGroupContextCommand::Expand
        } else {
            WorkspaceGroupContextCommand::Collapse
        };
        vec![
            WorkspaceGroupContextCommand::Rename,
            collapse_command,
            WorkspaceGroupContextCommand::Move,
        ]
    }
}

fn pin_command(tab: &WorkspaceTab) -> WorkspaceTabContextCommand {
    if tab.pinned {
        WorkspaceTabContextCommand::Unpin
    } else {
        WorkspaceTabContextCommand::Pin
    }
}
