use super::bar::WorkspaceTabBar;
use super::events::WorkspaceTabBarEvent;
use super::identifiers::WorkspaceTabId;
use super::options::WorkspaceTab;
use super::ordering::ordered_tabs;

impl WorkspaceTabBar {
    pub(super) fn close_other_tabs(&mut self, tab_id: WorkspaceTabId) -> Vec<WorkspaceTabBarEvent> {
        if self.options.tabs.iter().all(|tab| tab.id != tab_id) {
            return Vec::new();
        }
        let tab_ids = self
            .visual_tab_ids()
            .into_iter()
            .filter(|candidate| candidate != &tab_id)
            .collect();
        self.close_tab_ids(tab_ids, Some(tab_id))
    }

    pub(super) fn close_tabs_to_right(
        &mut self,
        tab_id: WorkspaceTabId,
    ) -> Vec<WorkspaceTabBarEvent> {
        let visual_tab_ids = self.visual_tab_ids();
        let Some(index) = visual_tab_ids
            .iter()
            .position(|candidate| candidate == &tab_id)
        else {
            return Vec::new();
        };
        self.close_tab_ids(
            visual_tab_ids.into_iter().skip(index + 1).collect(),
            Some(tab_id),
        )
    }

    pub(super) fn close_tabs_to_left(
        &mut self,
        tab_id: WorkspaceTabId,
    ) -> Vec<WorkspaceTabBarEvent> {
        let visual_tab_ids = self.visual_tab_ids();
        let Some(index) = visual_tab_ids
            .iter()
            .position(|candidate| candidate == &tab_id)
        else {
            return Vec::new();
        };
        self.close_tab_ids(
            visual_tab_ids.into_iter().take(index).collect(),
            Some(tab_id),
        )
    }

    pub(super) fn close_all_tabs(&mut self) -> Vec<WorkspaceTabBarEvent> {
        self.close_tab_ids(self.visual_tab_ids(), None)
    }

    pub(super) fn close_tab_ids(
        &mut self,
        tab_ids: Vec<WorkspaceTabId>,
        fallback_active: Option<WorkspaceTabId>,
    ) -> Vec<WorkspaceTabBarEvent> {
        let mut events = Vec::new();
        let mut removable_tab_ids = Vec::new();
        for tab_id in tab_ids {
            let Some(tab) = self.options.tabs.iter().find(|tab| tab.id == tab_id) else {
                continue;
            };
            if tab.pinned || !tab.closeable {
                continue;
            }
            if tab.dirty {
                self.state.pending_close_confirm = Some(tab_id.clone());
                events.push(WorkspaceTabBarEvent::TabCloseRequested { tab_id });
                continue;
            }
            removable_tab_ids.push(tab_id);
        }
        if removable_tab_ids.is_empty() {
            return events;
        }
        self.remove_clean_tabs(&removable_tab_ids, fallback_active.as_ref());
        events.extend(
            removable_tab_ids
                .into_iter()
                .map(|tab_id| WorkspaceTabBarEvent::TabClosed { tab_id }),
        );
        events
    }

    fn remove_clean_tabs(
        &mut self,
        tab_ids: &[WorkspaceTabId],
        fallback_active: Option<&WorkspaceTabId>,
    ) {
        let removed_tabs: Vec<WorkspaceTab> = self
            .options
            .tabs
            .iter()
            .filter(|tab| tab_ids.iter().any(|candidate| candidate == &tab.id))
            .cloned()
            .collect();
        self.options
            .tabs
            .retain(|tab| !tab_ids.iter().any(|candidate| candidate == &tab.id));
        for tab in removed_tabs {
            self.state.record_closed_tab(tab);
        }
        let active_removed = self
            .state
            .active_tab_id
            .as_ref()
            .is_some_and(|active| tab_ids.iter().any(|candidate| candidate == active));
        if active_removed || self.state.active_tab_id.is_none() {
            self.state.active_tab_id = self.next_active_tab(fallback_active);
        }
        self.state.sync_child_states(&self.options.tabs);
    }

    fn next_active_tab(&self, fallback_active: Option<&WorkspaceTabId>) -> Option<WorkspaceTabId> {
        if let Some(tab_id) = fallback_active
            && self.options.tabs.iter().any(|tab| &tab.id == tab_id)
        {
            return Some(tab_id.clone());
        }
        self.options.tabs.first().map(|tab| tab.id.clone())
    }

    fn visual_tab_ids(&self) -> Vec<WorkspaceTabId> {
        ordered_tabs(&self.options.tabs, &self.options.groups)
            .into_iter()
            .map(|tab| tab.id.clone())
            .collect()
    }
}
