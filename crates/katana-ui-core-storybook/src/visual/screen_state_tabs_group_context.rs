use super::screen_state_tabs_core::core_event_name;
use super::screen_state_tabs_types::{
    TabsContextMenuCommand, TabsContextMenuState, TabsScreenState, TabsScreenUpdate, tabs_update,
};
use katana_ui_core::render_model::UiNode;
use katana_ui_core::widget::molecules::{
    CloseableTabContextMenu, CloseableTabGroupContextCommand, CloseableTabGroupId,
    ContextMenuAnchor,
};

const RENAMED_GROUP_LABEL: &str = "Reference";
const RECOLORED_GROUP_COLOR: &str = "#5aa65a";

impl TabsScreenState {
    pub(in crate::visual) fn open_context_menu_for_group(
        &mut self,
        group_id: &str,
        x: usize,
        y: usize,
    ) -> TabsScreenUpdate {
        let Some(group) = self.groups.iter().find(|it| it.id == group_id).cloned() else {
            self.context_menu = None;
            return group_context_update(
                "group_context_menu",
                "closeable_tab_group_context_menu_missing",
                "none",
                "tabs.group=missing",
            );
        };
        let commands = TabsContextMenuCommand::for_group(&group);
        let anchor = ContextMenuAnchor::Pointer {
            x: x as i32,
            y: y as i32,
        };
        let mut items = Vec::new();
        for command in &commands {
            items.push(command.to_context_menu_item(&[]));
        }
        let context_node = UiNode::from(CloseableTabContextMenu::menu(
            "Storybook group menu",
            anchor,
            items,
        ));
        self.context_menu = Some(TabsContextMenuState {
            tab_id: String::new(),
            group_id: Some(group.id),
            x,
            y,
            commands,
            items: context_node.props().context_menu.items.clone(),
        });
        group_context_update(
            "group_context_menu",
            "closeable_tab_group_context_menu_opened",
            "open",
            "tabs.context=group-menu",
        )
    }

    pub(in crate::visual) fn apply_group_context_command(
        &mut self,
        group_id: &str,
        command: TabsContextMenuCommand,
    ) -> TabsScreenUpdate {
        match &command {
            TabsContextMenuCommand::GroupCollapse | TabsContextMenuCommand::GroupExpand => {
                self.toggle_group_from_context(group_id, command)
            }
            TabsContextMenuCommand::GroupRename => self.rename_group_from_context(group_id),
            TabsContextMenuCommand::GroupSetColor => self.set_group_color_from_context(group_id),
            TabsContextMenuCommand::GroupMove => self.move_group_from_context(group_id),
            TabsContextMenuCommand::GroupUngroup => self.ungroup_from_context(group_id),
            TabsContextMenuCommand::GroupClose => self.close_group_from_context(group_id),
            _ => group_context_update(
                "group_context_command",
                "closeable_tab_group_context_command_missing",
                "none",
                "tabs.command=invalid",
            ),
        }
    }

    fn toggle_group_from_context(
        &mut self,
        group_id: &str,
        command: TabsContextMenuCommand,
    ) -> TabsScreenUpdate {
        let Some(group) = self.groups.iter().find(|it| it.id == group_id).cloned() else {
            return group_context_update(
                "group_context_toggle",
                "closeable_tab_group_context_command_missing",
                "none",
                "tabs.group=missing",
            );
        };
        let command_id = command.id();
        let Some(group_command) = CloseableTabGroupContextCommand::from_id(command_id.as_str())
        else {
            return group_context_update(
                "group_context_toggle",
                "closeable_tab_group_context_command_missing",
                "none",
                "tabs.command=missing",
            );
        };
        let Some(action) = group_command.to_group_action(&group.to_core_group()) else {
            return group_context_update(
                "group_context_toggle",
                "closeable_tab_group_context_noop",
                "none",
                "tabs.context=noop",
            );
        };
        let events = self.apply_core_tab_action(action);
        group_context_update(
            "group_context_toggle",
            core_event_name(&events, "closeable_tab_group_context_command_missing"),
            "toggle",
            "tabs.context=applied",
        )
    }

    fn move_group_from_context(&mut self, group_id: &str) -> TabsScreenUpdate {
        let Some(from) = self.groups.iter().position(|group| group.id == group_id) else {
            return group_context_update(
                "group_context_move",
                "closeable_tab_group_context_command_missing",
                "none",
                "tabs.group=missing",
            );
        };
        if self.groups.len() < 2 {
            return group_context_update(
                "group_context_move",
                "closeable_tab_group_context_noop",
                "none",
                "tabs.context=noop",
            );
        }
        let to_index = if from + 1 < self.groups.len() {
            from + 1
        } else {
            0
        };
        let action = CloseableTabGroupContextCommand::move_group_action(
            CloseableTabGroupId::new(group_id),
            to_index,
        );
        let events = self.apply_core_tab_action(action);
        group_context_update(
            "group_context_move",
            core_event_name(&events, "closeable_tab_group_context_command_missing"),
            group_move_value(to_index),
            group_move_state(to_index),
        )
    }

    fn rename_group_from_context(&mut self, group_id: &str) -> TabsScreenUpdate {
        let action = CloseableTabGroupContextCommand::rename_group_action(
            CloseableTabGroupId::new(group_id),
            RENAMED_GROUP_LABEL,
        );
        let events = self.apply_core_tab_action(action);
        group_context_update(
            "group_context_rename",
            core_event_name(&events, "closeable_tab_group_context_command_missing"),
            RENAMED_GROUP_LABEL,
            "tabs.context=applied group=renamed",
        )
    }

    fn set_group_color_from_context(&mut self, group_id: &str) -> TabsScreenUpdate {
        let action = CloseableTabGroupContextCommand::set_group_color_action(
            CloseableTabGroupId::new(group_id),
            RECOLORED_GROUP_COLOR,
        );
        let events = self.apply_core_tab_action(action);
        group_context_update(
            "group_context_color",
            core_event_name(&events, "closeable_tab_group_context_command_missing"),
            "color=accent",
            "tabs.context=applied group=color",
        )
    }

    fn ungroup_from_context(&mut self, group_id: &str) -> TabsScreenUpdate {
        let Some(group) = self.groups.iter().find(|it| it.id == group_id).cloned() else {
            return missing_group_update("group_context_ungroup");
        };
        let Some(action) =
            CloseableTabGroupContextCommand::Ungroup.to_group_action(&group.to_core_group())
        else {
            return noop_group_update("group_context_ungroup");
        };
        let events = self.apply_core_tab_action(action);
        group_context_update(
            "group_context_ungroup",
            core_event_name(&events, "closeable_tab_group_context_command_missing"),
            "ungroup",
            "tabs.context=applied group=ungrouped",
        )
    }

    fn close_group_from_context(&mut self, group_id: &str) -> TabsScreenUpdate {
        let Some(group) = self.groups.iter().find(|it| it.id == group_id).cloned() else {
            return missing_group_update("group_context_close_group");
        };
        let Some(action) =
            CloseableTabGroupContextCommand::Close.to_group_action(&group.to_core_group())
        else {
            return noop_group_update("group_context_close_group");
        };
        let events = self.apply_core_tab_action_confirming_dirty(action);
        group_context_update(
            "group_context_close_group",
            core_event_name(&events, "closeable_tab_group_context_command_missing"),
            "close-group",
            "tabs.context=applied group=closed",
        )
    }
}

const fn group_move_value(to_index: usize) -> &'static str {
    match to_index {
        0 => "target_index=0",
        1 => "target_index=1",
        _ => "target_index=overflow",
    }
}

const fn group_move_state(to_index: usize) -> &'static str {
    match to_index {
        0 => "tabs.context=applied target_index=0",
        1 => "tabs.context=applied target_index=1",
        _ => "tabs.context=applied target_index=overflow",
    }
}

pub(in crate::visual) fn group_context_update(
    action: &'static str,
    event: &'static str,
    value: &'static str,
    state: &'static str,
) -> TabsScreenUpdate {
    tabs_update(action, event, "tabs.group_context_menu", value, state)
}

fn missing_group_update(action: &'static str) -> TabsScreenUpdate {
    group_context_update(
        action,
        "closeable_tab_group_context_command_missing",
        "none",
        "tabs.group=missing",
    )
}

fn noop_group_update(action: &'static str) -> TabsScreenUpdate {
    group_context_update(
        action,
        "closeable_tab_group_context_noop",
        "none",
        "tabs.context=noop",
    )
}
