use super::actions::{WorkspaceTabBarAction, WorkspaceTabBarIntent};
use super::bar::WorkspaceTabBar;
use super::events::WorkspaceTabBarEvent;

impl WorkspaceTabBar {
    pub fn apply_action(&mut self, action: WorkspaceTabBarAction) -> Vec<WorkspaceTabBarEvent> {
        let events = match action {
            WorkspaceTabBarAction::AddTab { tab, activate } => self.add_tab(tab, activate),
            WorkspaceTabBarAction::SelectTab { tab_id } => self.select_tab(tab_id),
            WorkspaceTabBarAction::CloseTab { tab_id } => self.close_tab(tab_id),
            WorkspaceTabBarAction::CloseOthers { tab_id } => self.close_other_tabs(tab_id),
            WorkspaceTabBarAction::CloseToRight { tab_id } => self.close_tabs_to_right(tab_id),
            WorkspaceTabBarAction::CloseToLeft { tab_id } => self.close_tabs_to_left(tab_id),
            WorkspaceTabBarAction::CloseAll => self.close_all_tabs(),
            WorkspaceTabBarAction::RestoreClosedTab => self.restore_closed_tab(),
            WorkspaceTabBarAction::PinTab { tab_id } => self.set_pinned(tab_id, true),
            WorkspaceTabBarAction::UnpinTab { tab_id } => self.set_pinned(tab_id, false),
            WorkspaceTabBarAction::MoveTab {
                tab_id,
                to_visual_index,
            } => self.move_tab(tab_id, to_visual_index),
            WorkspaceTabBarAction::MoveToGroup { tab_id, target } => {
                self.move_to_group(tab_id, target)
            }
            WorkspaceTabBarAction::MoveGroup { group_id, to_index } => {
                self.move_group(group_id, to_index)
            }
            WorkspaceTabBarAction::RenameGroup { group_id, label } => {
                self.rename_group(group_id, label)
            }
            WorkspaceTabBarAction::SetGroupColor { group_id, color } => {
                self.set_group_color(group_id, color)
            }
            WorkspaceTabBarAction::Ungroup { group_id } => self.ungroup(group_id),
            WorkspaceTabBarAction::CloseGroup { group_id } => self.close_group(group_id),
            WorkspaceTabBarAction::StartDrag { tab_id } => self.start_drag(tab_id),
            WorkspaceTabBarAction::EndDrag { committed } => self.end_drag(committed),
            WorkspaceTabBarAction::CancelDrag => self.end_drag(false),
            WorkspaceTabBarAction::HoverCollapsedGroupForDrop {
                group_id,
                elapsed_ms,
            } => self.hover_collapsed_group_for_drop(group_id, elapsed_ms),
            WorkspaceTabBarAction::ToggleGroupCollapse { group_id } => {
                self.toggle_group_collapse(group_id)
            }
            WorkspaceTabBarAction::OpenOverflow { hidden_tab_ids } => {
                self.open_overflow(hidden_tab_ids)
            }
            WorkspaceTabBarAction::ConfirmClose { tab_id } => self.confirm_close(tab_id),
        };
        self.event_log.extend(events.clone());
        events
    }

    pub fn apply_intent(&mut self, intent: WorkspaceTabBarIntent) -> Vec<WorkspaceTabBarEvent> {
        let events = match intent {
            WorkspaceTabBarIntent::RequestTabClose { tab_id } => self.request_tab_close(tab_id),
        };
        self.event_log.extend(events.clone());
        events
    }
}
