use super::context_commands::{WorkspaceGroupContextCommand, WorkspaceTabContextCommand};
use super::options::{WorkspaceTab, WorkspaceTabGroup};
use crate::molecule::selection::{
    ContextMenu, ContextMenuAnchor, ContextMenuItem, ContextMenuItemKind,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTabContextMenu;

impl WorkspaceTabContextMenu {
    #[must_use]
    pub fn menu(
        label: impl Into<String>,
        anchor: ContextMenuAnchor,
        items: Vec<ContextMenuItem>,
    ) -> ContextMenu {
        ContextMenu::new(label).anchor(anchor).items(items)
    }

    #[must_use]
    pub fn tab_menu(
        label: impl Into<String>,
        tab: &WorkspaceTab,
        groups: &[WorkspaceTabGroup],
        anchor: ContextMenuAnchor,
    ) -> ContextMenu {
        let commands = Self::tab_commands(tab, groups);
        Self::menu(label, anchor, Self::tab_command_items(&commands, groups))
    }

    #[must_use]
    pub fn group_menu(
        label: impl Into<String>,
        group: &WorkspaceTabGroup,
        anchor: ContextMenuAnchor,
    ) -> ContextMenu {
        let mut items = Vec::new();
        for command in Self::group_commands(group) {
            items.push(command.to_context_menu_item());
        }
        Self::menu(label, anchor, items)
    }

    #[must_use]
    pub fn tab_commands(
        tab: &WorkspaceTab,
        groups: &[WorkspaceTabGroup],
    ) -> Vec<WorkspaceTabContextCommand> {
        Self::tab_commands_with_restore(tab, groups, false)
    }

    #[must_use]
    pub fn tab_commands_with_restore(
        tab: &WorkspaceTab,
        groups: &[WorkspaceTabGroup],
        restore_available: bool,
    ) -> Vec<WorkspaceTabContextCommand> {
        let mut commands = vec![
            WorkspaceTabContextCommand::Close,
            WorkspaceTabContextCommand::CloseOthers,
            WorkspaceTabContextCommand::CloseAll,
            WorkspaceTabContextCommand::CloseToRight,
            WorkspaceTabContextCommand::CloseToLeft,
            pin_command(tab),
        ];
        if !tab.pinned && tab.groupable {
            commands.push(WorkspaceTabContextCommand::MoveToNewGroup);
            if !groups.is_empty() {
                commands.push(WorkspaceTabContextCommand::MoveToGroup);
            }
        }
        if restore_available {
            commands.push(WorkspaceTabContextCommand::RestoreClosed);
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
            WorkspaceGroupContextCommand::SetColor,
            collapse_command,
            WorkspaceGroupContextCommand::Move,
            WorkspaceGroupContextCommand::Ungroup,
            WorkspaceGroupContextCommand::Close,
        ]
    }

    #[must_use]
    pub fn tab_command_item(
        command: WorkspaceTabContextCommand,
        groups: &[WorkspaceTabGroup],
    ) -> ContextMenuItem {
        if command == WorkspaceTabContextCommand::MoveToGroup {
            return Self::group_submenu_item(groups);
        }
        command.to_context_menu_item()
    }

    #[must_use]
    pub fn tab_command_items(
        commands: &[WorkspaceTabContextCommand],
        groups: &[WorkspaceTabGroup],
    ) -> Vec<ContextMenuItem> {
        let mut items = Vec::new();
        let mut index = 0;
        while index < commands.len() {
            let command = commands[index];
            if command == WorkspaceTabContextCommand::MoveToNewGroup {
                if groups.is_empty() {
                    items.push(command.to_context_menu_item());
                    index += 1;
                    continue;
                }
                items.push(Self::group_submenu_item(groups));
                index += 1;
                if commands.get(index) == Some(&WorkspaceTabContextCommand::MoveToGroup) {
                    index += 1;
                }
                continue;
            }
            items.push(Self::tab_command_item(command, groups));
            index += 1;
        }
        items
    }

    fn group_submenu_item(groups: &[WorkspaceTabGroup]) -> ContextMenuItem {
        let command = WorkspaceTabContextCommand::MoveToGroup;
        let mut item =
            ContextMenuItem::new(command.id(), command.label(), ContextMenuItemKind::Submenu);
        item = item.child(WorkspaceTabContextCommand::MoveToNewGroup.to_context_menu_item());
        for group in groups {
            item = item.child(ContextMenuItem::action(
                WorkspaceTabContextCommand::move_to_group_item_id(&group.id),
                group.label.as_str(),
            ));
        }
        item
    }
}

const fn pin_command(tab: &WorkspaceTab) -> WorkspaceTabContextCommand {
    if tab.pinned {
        WorkspaceTabContextCommand::Unpin
    } else {
        WorkspaceTabContextCommand::Pin
    }
}
