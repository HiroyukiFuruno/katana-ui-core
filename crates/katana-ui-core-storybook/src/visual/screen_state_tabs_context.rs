use super::screen_state_tabs_core::core_event_name;
use super::screen_state_tabs_types::{
    TabsContextMenuCommand, TabsContextMenuState, TabsScreenState, TabsScreenUpdate, tabs_update,
};
use katana_ui_core::render_model::UiNode;
use katana_ui_core::widget::molecules::{
    CloseableTabContextMenu, CloseableTabGroup, CloseableTabGroupId, CloseableTabGroupTarget,
    CloseableTabId, CloseableTabStripAction, ContextMenuAnchor,
};

const CONTEXT_GROUP_ID: &str = "context-group";
const CONTEXT_GROUP_TITLE: &str = "New group";

impl TabsScreenState {
    pub(in crate::visual) fn open_context_menu_for_tab(
        &mut self,
        tab_id: &str,
        x: usize,
        y: usize,
    ) -> TabsScreenUpdate {
        let Some(tab) = self.tabs.iter().find(|tab| tab.id == tab_id).cloned() else {
            self.context_menu = None;
            return context_update(
                "tab_context_menu",
                "closeable_tab_context_menu_missing",
                "none",
                "tabs.tab=missing",
            );
        };
        self.active_tab_id = tab.id.clone();
        let commands = TabsContextMenuCommand::for_tab(
            &tab,
            &self.groups,
            !self.recently_closed_tabs.is_empty(),
        );
        let items = TabsContextMenuCommand::to_context_menu_items(&commands, &self.groups);
        let anchor = ContextMenuAnchor::Pointer {
            x: x as i32,
            y: y as i32,
        };
        let context_node = UiNode::from(CloseableTabContextMenu::menu(
            "Storybook tab menu",
            anchor,
            items,
        ));
        self.context_menu = Some(TabsContextMenuState {
            tab_id: tab.id.clone(),
            group_id: None,
            x,
            y,
            commands,
            items: context_node.props().context_menu.items.clone(),
        });
        context_update(
            "tab_context_menu",
            "closeable_tab_context_menu_opened",
            "open",
            "tabs.context=tab-menu",
        )
    }

    pub(in crate::visual) fn apply_context_command(
        &mut self,
        command: TabsContextMenuCommand,
    ) -> TabsScreenUpdate {
        let Some(menu) = self.context_menu.clone() else {
            return context_update(
                "tab_context_command",
                "closeable_tab_context_command_missing",
                "none",
                "tabs.context=missing",
            );
        };
        self.context_menu = None;
        if let Some(group_id) = menu.group_id.as_ref() {
            return self.apply_group_context_command(group_id, command);
        }
        match command {
            TabsContextMenuCommand::Close => self.close_menu_tab(menu.tab_id.as_str()),
            TabsContextMenuCommand::CloseOthers => self.close_other_menu_tabs(menu.tab_id.as_str()),
            TabsContextMenuCommand::CloseAll => self.close_all_menu_tabs(),
            TabsContextMenuCommand::CloseToRight => {
                self.close_menu_tabs_to_right(menu.tab_id.as_str())
            }
            TabsContextMenuCommand::CloseToLeft => {
                self.close_menu_tabs_to_left(menu.tab_id.as_str())
            }
            TabsContextMenuCommand::RestoreClosed => self.restore_closed_tab_from_context(),
            TabsContextMenuCommand::Pin => self.set_menu_tab_pinned(menu.tab_id.as_str(), true),
            TabsContextMenuCommand::Unpin => self.set_menu_tab_pinned(menu.tab_id.as_str(), false),
            TabsContextMenuCommand::NewGroup => {
                self.create_group_for_menu_tab(menu.tab_id.as_str())
            }
            TabsContextMenuCommand::MoveToExistingGroup(group_id) => {
                self.move_menu_tab_to_existing_group(menu.tab_id.as_str(), group_id.as_str())
            }
            _ => context_update(
                "tab_context_command",
                "closeable_tab_context_command_missing",
                "none",
                "tabs.command=invalid",
            ),
        }
    }

    fn set_menu_tab_pinned(&mut self, tab_id: &str, pinned: bool) -> TabsScreenUpdate {
        if !self.tabs.iter().any(|tab| tab.id == tab_id) {
            return context_update(
                "tab_context_pin",
                "closeable_tab_context_command_missing",
                "none",
                "tabs.tab=missing",
            );
        };
        let action = if pinned {
            CloseableTabStripAction::PinTab {
                tab_id: CloseableTabId::new(tab_id),
            }
        } else {
            CloseableTabStripAction::UnpinTab {
                tab_id: CloseableTabId::new(tab_id),
            }
        };
        let events = self.apply_core_tab_action(action);
        context_update(
            "tab_context_pin",
            core_event_name(&events, "closeable_tab_context_command_missing"),
            "pin",
            "tabs.context=applied",
        )
    }

    fn create_group_for_menu_tab(&mut self, tab_id: &str) -> TabsScreenUpdate {
        let events = self.apply_core_tab_action(CloseableTabStripAction::MoveToGroup {
            tab_id: CloseableTabId::new(tab_id),
            target: CloseableTabGroupTarget::NewGroup(CloseableTabGroup::new(
                CONTEXT_GROUP_ID,
                CONTEXT_GROUP_TITLE,
            )),
        });
        context_update(
            "tab_context_new_group",
            core_event_name(&events, "closeable_tab_context_command_missing"),
            "new-group",
            "tabs.context=applied",
        )
    }

    fn move_menu_tab_to_existing_group(
        &mut self,
        tab_id: &str,
        group_id: &str,
    ) -> TabsScreenUpdate {
        let events = self.apply_core_tab_action(CloseableTabStripAction::MoveToGroup {
            tab_id: CloseableTabId::new(tab_id),
            target: CloseableTabGroupTarget::Existing(CloseableTabGroupId::new(group_id)),
        });
        context_update(
            "tab_context_move_group",
            core_event_name(&events, "closeable_tab_context_command_missing"),
            "existing-group",
            "tabs.context=applied",
        )
    }

    fn restore_closed_tab_from_context(&mut self) -> TabsScreenUpdate {
        let events = self.apply_core_tab_action(CloseableTabStripAction::RestoreClosedTab);
        context_update(
            "tab_context_restore_closed",
            core_event_name(&events, "closeable_tab_context_command_missing"),
            "restore-closed",
            "tabs.context=applied",
        )
    }
}

pub(in crate::visual) fn context_update(
    action: &'static str,
    event: &'static str,
    value: &'static str,
    state: &'static str,
) -> TabsScreenUpdate {
    tabs_update(action, event, "tabs.context_menu", value, state)
}

#[cfg(test)]
mod tests {
    use super::{TabsContextMenuCommand, TabsScreenState};

    #[test]
    fn tab_context_reports_missing_menu_invalid_group_command_and_removed_tab() {
        let mut state = TabsScreenState::default();
        let missing = state.open_context_menu_for_tab("missing", 10, 20);
        assert_eq!("closeable_tab_context_menu_missing", missing.event);
        assert!(state.context_menu.is_none());

        let no_menu = state.apply_context_command(TabsContextMenuCommand::Close);
        assert_eq!("closeable_tab_context_command_missing", no_menu.event);

        let tab_id = state.tabs[0].id.clone();
        state.open_context_menu_for_tab(tab_id.as_str(), 10, 20);
        let invalid = state.apply_context_command(TabsContextMenuCommand::GroupRename);
        assert_eq!("tabs.command=invalid", invalid.state);

        state.open_context_menu_for_tab(tab_id.as_str(), 10, 20);
        state.tabs.clear();
        let removed = state.apply_context_command(TabsContextMenuCommand::Pin);
        assert_eq!("tabs.tab=missing", removed.state);
    }
}
