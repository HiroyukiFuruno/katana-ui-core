use super::bar::WorkspaceTabBar;
use super::events::WorkspaceTabBarEvent;
use super::identifiers::WorkspaceTabId;

impl WorkspaceTabBar {
    pub(super) fn open_overflow(
        &mut self,
        hidden_tab_ids: Vec<WorkspaceTabId>,
    ) -> Vec<WorkspaceTabBarEvent> {
        self.state.overflow_visible = true;
        vec![WorkspaceTabBarEvent::OverflowOpened { hidden_tab_ids }]
    }
}
