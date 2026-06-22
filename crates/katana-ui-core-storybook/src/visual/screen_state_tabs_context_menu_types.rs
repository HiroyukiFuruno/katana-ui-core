use super::screen_state_tabs_types::{TabsContextMenuCommand, TabsScreenGroup, TabsScreenTab};
use katana_ui_core::widget::molecules::{
    CloseableTabContextCommand, CloseableTabContextMenu, CloseableTabGroup,
    CloseableTabGroupContextCommand, CloseableTabGroupId, ContextMenuItem, ContextMenuItemKind,
};

impl TabsContextMenuCommand {
    pub(in crate::visual) fn for_tab(
        tab: &TabsScreenTab,
        groups: &[TabsScreenGroup],
        restore_available: bool,
    ) -> Vec<Self> {
        let core_tab = tab.to_core_tab();
        let core_groups: Vec<CloseableTabGroup> =
            groups.iter().map(TabsScreenGroup::to_core_group).collect();
        let core_commands = CloseableTabContextMenu::tab_commands_with_restore(
            &core_tab,
            &core_groups,
            restore_available,
        );
        let mut commands = Vec::new();
        for command in core_commands {
            commands.push(Self::from_core_command(command));
        }
        commands
    }

    pub(in crate::visual) fn to_context_menu_item(
        &self,
        groups: &[TabsScreenGroup],
    ) -> ContextMenuItem {
        if matches!(self, Self::MoveToGroup) {
            return Self::group_submenu_item(groups);
        }
        ContextMenuItem::action(self.id(), self.label())
    }

    pub(in crate::visual) fn to_context_menu_items(
        commands: &[Self],
        groups: &[TabsScreenGroup],
    ) -> Vec<ContextMenuItem> {
        let mut items = Vec::new();
        let mut index = 0;
        while index < commands.len() {
            let command = &commands[index];
            if matches!(command, Self::NewGroup) {
                if groups.is_empty() {
                    items.push(command.to_context_menu_item(groups));
                    index += 1;
                    continue;
                }
                items.push(Self::group_submenu_item(groups));
                index += 1;
                if commands
                    .get(index)
                    .is_some_and(|it| matches!(it, Self::MoveToGroup))
                {
                    index += 1;
                }
                continue;
            }
            items.push(command.to_context_menu_item(groups));
            index += 1;
        }
        items
    }

    fn group_submenu_item(groups: &[TabsScreenGroup]) -> ContextMenuItem {
        let mut item = ContextMenuItem::new(
            Self::MoveToGroup.id(),
            Self::MoveToGroup.label(),
            ContextMenuItemKind::Submenu,
        );
        item = item.child(ContextMenuItem::action(
            Self::NewGroup.id(),
            Self::NewGroup.label(),
        ));
        for group in groups {
            item = item.child(ContextMenuItem::action(
                CloseableTabContextCommand::move_to_group_item_id(&CloseableTabGroupId::new(
                    group.id.as_str(),
                )),
                group.title.as_str(),
            ));
        }
        item
    }

    pub(in crate::visual) fn for_group(group: &TabsScreenGroup) -> Vec<Self> {
        let core_group = group.to_core_group();
        let core_commands = CloseableTabContextMenu::group_commands(&core_group);
        let mut commands = Vec::new();
        for command in core_commands {
            commands.push(Self::from_core_group_command(command));
        }
        commands
    }

    pub(in crate::visual) fn from_item_id(id: &str, group_menu: bool) -> Option<Self> {
        if group_menu {
            return CloseableTabGroupContextCommand::from_id(id).map(Self::from_core_group_command);
        }
        if let Some(group_id) = CloseableTabContextCommand::move_to_group_id_from_item_id(id) {
            return Some(Self::MoveToExistingGroup(group_id.as_str().to_string()));
        }
        CloseableTabContextCommand::from_id(id).map(Self::from_core_command)
    }

    const fn from_core_command(command: CloseableTabContextCommand) -> Self {
        match command {
            CloseableTabContextCommand::Close => Self::Close,
            CloseableTabContextCommand::CloseOthers => Self::CloseOthers,
            CloseableTabContextCommand::CloseAll => Self::CloseAll,
            CloseableTabContextCommand::CloseToRight => Self::CloseToRight,
            CloseableTabContextCommand::CloseToLeft => Self::CloseToLeft,
            CloseableTabContextCommand::RestoreClosed => Self::RestoreClosed,
            CloseableTabContextCommand::Pin => Self::Pin,
            CloseableTabContextCommand::Unpin => Self::Unpin,
            CloseableTabContextCommand::MoveToNewGroup => Self::NewGroup,
            CloseableTabContextCommand::MoveToGroup => Self::MoveToGroup,
        }
    }

    const fn from_core_group_command(command: CloseableTabGroupContextCommand) -> Self {
        match command {
            CloseableTabGroupContextCommand::Rename => Self::GroupRename,
            CloseableTabGroupContextCommand::SetColor => Self::GroupSetColor,
            CloseableTabGroupContextCommand::Collapse => Self::GroupCollapse,
            CloseableTabGroupContextCommand::Expand => Self::GroupExpand,
            CloseableTabGroupContextCommand::Move => Self::GroupMove,
            CloseableTabGroupContextCommand::Ungroup => Self::GroupUngroup,
            CloseableTabGroupContextCommand::Close => Self::GroupClose,
        }
    }

    pub(in crate::visual) fn id(&self) -> String {
        match self {
            Self::Close => "close".to_string(),
            Self::CloseOthers => "close-others".to_string(),
            Self::CloseAll => "close-all".to_string(),
            Self::CloseToRight => "close-to-right".to_string(),
            Self::CloseToLeft => "close-to-left".to_string(),
            Self::RestoreClosed => "restore-closed".to_string(),
            Self::Pin => "pin".to_string(),
            Self::Unpin => "unpin".to_string(),
            Self::NewGroup => "move-to-new-group".to_string(),
            Self::MoveToGroup => "move-to-group".to_string(),
            Self::MoveToExistingGroup(group_id) => {
                CloseableTabContextCommand::move_to_group_item_id(&CloseableTabGroupId::new(
                    group_id.as_str(),
                ))
            }
            Self::GroupRename => "rename".to_string(),
            Self::GroupSetColor => "set-color".to_string(),
            Self::GroupCollapse => "collapse".to_string(),
            Self::GroupExpand => "expand".to_string(),
            Self::GroupMove => "move".to_string(),
            Self::GroupUngroup => "ungroup".to_string(),
            Self::GroupClose => "close-group".to_string(),
        }
    }

    pub(in crate::visual) fn label(&self) -> &'static str {
        match self {
            Self::Close => "閉じる",
            Self::CloseOthers => "他のタブを閉じる",
            Self::CloseAll => "すべて閉じる",
            Self::CloseToRight => "右側のタブを閉じる",
            Self::CloseToLeft => "左側のタブを閉じる",
            Self::RestoreClosed => "閉じたタブを復元",
            Self::Pin => "ピン留め",
            Self::Unpin => "ピン留めを解除",
            Self::NewGroup => "新しいグループを作成",
            Self::MoveToGroup | Self::MoveToExistingGroup(_) => "グループに追加",
            Self::GroupRename => "グループ名を変更",
            Self::GroupSetColor => "グループ色を変更",
            Self::GroupCollapse => "グループを折りたたむ",
            Self::GroupExpand => "グループを展開",
            Self::GroupMove => "グループを移動",
            Self::GroupUngroup => "グループ解除",
            Self::GroupClose => "グループを閉じる",
        }
    }
}
