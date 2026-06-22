use super::actions::{WorkspaceTabBarAction, WorkspaceTabGroupTarget};
use super::bar::WorkspaceTabBar;
use super::events::WorkspaceTabBarEvent;
use super::identifiers::{WorkspaceTabGroupId, WorkspaceTabId};
use super::options::{WorkspaceTab, WorkspaceTabGroup};
use crate::component::ComponentAction;
use crate::interaction::{UiAction, UiActionResult};

impl ComponentAction for WorkspaceTabBar {
    fn apply_action(&mut self, action: &UiAction) -> UiActionResult {
        let before = self.state.interaction(self.options.tabs.len());
        if action.target() != &self.state.state_id {
            return UiActionResult::ignored(self.state.state_id.clone(), before);
        }
        let events = self.apply_ui_action(action);
        if events.is_empty() {
            return UiActionResult::ignored(self.state.state_id.clone(), before);
        }
        UiActionResult::handled(
            self.state.state_id.clone(),
            action,
            before,
            self.state.interaction(self.options.tabs.len()),
        )
    }
}

impl WorkspaceTabBar {
    fn apply_ui_action(&mut self, action: &UiAction) -> Vec<WorkspaceTabBarEvent> {
        match action {
            UiAction::SetSelectedIndex { selected_index, .. } => {
                self.select_tab_by_visual_index(*selected_index)
            }
            UiAction::TabSelect { tab_id, .. } => {
                self.apply_action(WorkspaceTabBarAction::SelectTab {
                    tab_id: WorkspaceTabId::new(tab_id),
                })
            }
            UiAction::TabAdd {
                tab_id,
                label,
                activate,
                ..
            } => self.apply_action(WorkspaceTabBarAction::AddTab {
                tab: WorkspaceTab::new(tab_id.as_str(), label.as_str()),
                activate: *activate,
            }),
            UiAction::TabClose { tab_id, .. } => {
                self.apply_action(WorkspaceTabBarAction::CloseTab {
                    tab_id: WorkspaceTabId::new(tab_id),
                })
            }
            UiAction::TabCloseOthers { tab_id, .. } => {
                self.apply_action(WorkspaceTabBarAction::CloseOthers {
                    tab_id: WorkspaceTabId::new(tab_id),
                })
            }
            UiAction::TabCloseToRight { tab_id, .. } => {
                self.apply_action(WorkspaceTabBarAction::CloseToRight {
                    tab_id: WorkspaceTabId::new(tab_id),
                })
            }
            UiAction::TabCloseToLeft { tab_id, .. } => {
                self.apply_action(WorkspaceTabBarAction::CloseToLeft {
                    tab_id: WorkspaceTabId::new(tab_id),
                })
            }
            UiAction::TabCloseAll { .. } => self.apply_action(WorkspaceTabBarAction::CloseAll),
            UiAction::TabRestoreClosed { .. } => {
                self.apply_action(WorkspaceTabBarAction::RestoreClosedTab)
            }
            UiAction::TabPin { tab_id, pinned, .. } => self.apply_pin(tab_id, *pinned),
            UiAction::TabMove {
                tab_id,
                to_visual_index,
                ..
            } => self.apply_action(WorkspaceTabBarAction::MoveTab {
                tab_id: WorkspaceTabId::new(tab_id),
                to_visual_index: *to_visual_index,
            }),
            UiAction::TabMoveToGroup {
                tab_id, group_id, ..
            } => self.apply_action(WorkspaceTabBarAction::MoveToGroup {
                tab_id: WorkspaceTabId::new(tab_id),
                target: group_target(group_id),
            }),
            UiAction::TabMoveToNewGroup {
                tab_id,
                group_id,
                group_label,
                ..
            } => self.apply_action(WorkspaceTabBarAction::MoveToGroup {
                tab_id: WorkspaceTabId::new(tab_id),
                target: WorkspaceTabGroupTarget::NewGroup(WorkspaceTabGroup::new(
                    group_id.as_str(),
                    group_label.as_str(),
                )),
            }),
            UiAction::TabMoveGroup {
                group_id, to_index, ..
            } => self.apply_action(WorkspaceTabBarAction::MoveGroup {
                group_id: WorkspaceTabGroupId::new(group_id),
                to_index: *to_index,
            }),
            UiAction::TabRenameGroup {
                group_id, label, ..
            } => self.apply_action(WorkspaceTabBarAction::RenameGroup {
                group_id: WorkspaceTabGroupId::new(group_id),
                label: label.clone(),
            }),
            UiAction::TabSetGroupColor {
                group_id, color, ..
            } => self.apply_action(WorkspaceTabBarAction::SetGroupColor {
                group_id: WorkspaceTabGroupId::new(group_id),
                color: color.clone(),
            }),
            UiAction::TabUngroup { group_id, .. } => {
                self.apply_action(WorkspaceTabBarAction::Ungroup {
                    group_id: WorkspaceTabGroupId::new(group_id),
                })
            }
            UiAction::TabCloseGroup { group_id, .. } => {
                self.apply_action(WorkspaceTabBarAction::CloseGroup {
                    group_id: WorkspaceTabGroupId::new(group_id),
                })
            }
            UiAction::TabToggleGroupCollapse { group_id, .. } => {
                self.apply_action(WorkspaceTabBarAction::ToggleGroupCollapse {
                    group_id: WorkspaceTabGroupId::new(group_id),
                })
            }
            _ => Vec::new(),
        }
    }

    fn select_tab_by_visual_index(&mut self, index: usize) -> Vec<WorkspaceTabBarEvent> {
        let Some(tab_id) = self.visual_tabs().get(index).map(|tab| tab.id.clone()) else {
            return Vec::new();
        };
        self.apply_action(WorkspaceTabBarAction::SelectTab { tab_id })
    }

    fn apply_pin(&mut self, tab_id: &str, pinned: bool) -> Vec<WorkspaceTabBarEvent> {
        let tab_id = WorkspaceTabId::new(tab_id);
        let action = if pinned {
            WorkspaceTabBarAction::PinTab { tab_id }
        } else {
            WorkspaceTabBarAction::UnpinTab { tab_id }
        };
        self.apply_action(action)
    }
}

fn group_target(group_id: &Option<String>) -> WorkspaceTabGroupTarget {
    group_id
        .as_ref()
        .map_or(WorkspaceTabGroupTarget::Ungrouped, |value| {
            WorkspaceTabGroupTarget::Existing(WorkspaceTabGroupId::new(value))
        })
}
