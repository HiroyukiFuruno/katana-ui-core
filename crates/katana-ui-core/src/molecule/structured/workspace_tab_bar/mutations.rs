use super::actions::WorkspaceTabDropRules;
use super::bar::WorkspaceTabBar;
use super::events::WorkspaceTabBarEvent;
use super::identifiers::WorkspaceTabId;
use super::options::WorkspaceTab;
use super::ordering::ordered_tabs;

impl WorkspaceTabBar {
    pub(super) fn request_tab_close(
        &mut self,
        tab_id: WorkspaceTabId,
    ) -> Vec<WorkspaceTabBarEvent> {
        let Some(tab) = self.find_tab(&tab_id) else {
            return Vec::new();
        };
        if tab.pinned || !tab.closeable {
            return Vec::new();
        }
        vec![WorkspaceTabBarEvent::TabCloseRequested { tab_id }]
    }

    pub(super) fn add_tab(
        &mut self,
        tab: WorkspaceTab,
        activate: bool,
    ) -> Vec<WorkspaceTabBarEvent> {
        if self.find_tab(&tab.id).is_some() {
            return Vec::new();
        }
        let tab_id = tab.id.clone();
        self.options.tabs.push(tab);
        self.normalize_tabs();
        if activate {
            self.state.active_tab_id = Some(tab_id.clone());
        }
        vec![WorkspaceTabBarEvent::TabAdded { tab_id }]
    }

    pub(super) fn select_tab(&mut self, tab_id: WorkspaceTabId) -> Vec<WorkspaceTabBarEvent> {
        if self.find_tab(&tab_id).is_none() {
            return Vec::new();
        }
        self.state.active_tab_id = Some(tab_id.clone());
        vec![WorkspaceTabBarEvent::TabSelected { tab_id }]
    }

    pub(super) fn close_tab(&mut self, tab_id: WorkspaceTabId) -> Vec<WorkspaceTabBarEvent> {
        let Some(tab) = self.find_tab(&tab_id) else {
            return Vec::new();
        };
        if tab.pinned || !tab.closeable {
            return Vec::new();
        }
        if tab.dirty {
            self.state.pending_close_confirm = Some(tab_id.clone());
            return vec![WorkspaceTabBarEvent::TabCloseRequested { tab_id }];
        }
        self.remove_tab(&tab_id);
        vec![WorkspaceTabBarEvent::TabClosed { tab_id }]
    }

    pub(super) fn confirm_close(&mut self, tab_id: WorkspaceTabId) -> Vec<WorkspaceTabBarEvent> {
        if self.state.pending_close_confirm.as_ref() != Some(&tab_id) {
            return Vec::new();
        }
        self.state.pending_close_confirm = None;
        self.remove_tab(&tab_id);
        vec![WorkspaceTabBarEvent::TabClosed { tab_id }]
    }

    pub(super) fn restore_closed_tab(&mut self) -> Vec<WorkspaceTabBarEvent> {
        while let Some(closed) = self.state.recently_closed_tabs.pop() {
            if self.find_tab(&closed.tab.id).is_some() {
                continue;
            }
            let tab_id = closed.tab.id.clone();
            self.options.tabs.push(closed.tab);
            self.normalize_tabs();
            self.state.active_tab_id = Some(tab_id.clone());
            return vec![WorkspaceTabBarEvent::TabRestored { tab_id }];
        }
        Vec::new()
    }

    pub(super) fn move_tab(
        &mut self,
        tab_id: WorkspaceTabId,
        to_visual_index: usize,
    ) -> Vec<WorkspaceTabBarEvent> {
        if !WorkspaceTabDropRules::can_accept(&self.options.tabs, &tab_id, to_visual_index) {
            return Vec::new();
        }
        let mut visual_tabs: Vec<WorkspaceTab> =
            ordered_tabs(&self.options.tabs, &self.options.groups)
                .into_iter()
                .cloned()
                .collect();
        let Some(from) = visual_tabs.iter().position(|tab| tab.id == tab_id) else {
            return Vec::new();
        };
        let tab = visual_tabs.remove(from);
        let to = to_visual_index.min(visual_tabs.len());
        visual_tabs.insert(to, tab);
        self.options.tabs = visual_tabs;
        self.state.sync_child_states(&self.options.tabs);
        vec![WorkspaceTabBarEvent::TabReordered { tab_id, from, to }]
    }

    pub(super) fn start_drag(&mut self, tab_id: WorkspaceTabId) -> Vec<WorkspaceTabBarEvent> {
        if self.find_tab(&tab_id).is_none() {
            return Vec::new();
        }
        self.state.drag_in_progress = true;
        self.state.dragged_tab_id = Some(tab_id.clone());
        vec![WorkspaceTabBarEvent::DragStarted { tab_id }]
    }

    pub(super) fn end_drag(&mut self, committed: bool) -> Vec<WorkspaceTabBarEvent> {
        let Some(tab_id) = self.state.dragged_tab_id.take() else {
            return Vec::new();
        };
        self.state.drag_in_progress = false;
        vec![WorkspaceTabBarEvent::DragEnded { tab_id, committed }]
    }
}

impl WorkspaceTabBar {
    fn remove_tab(&mut self, tab_id: &WorkspaceTabId) {
        if let Some(tab) = self.find_tab(tab_id).cloned() {
            self.state.record_closed_tab(tab);
        }
        self.options.tabs.retain(|tab| &tab.id != tab_id);
        if self.state.active_tab_id.as_ref() == Some(tab_id) {
            self.state.active_tab_id = self.options.tabs.first().map(|tab| tab.id.clone());
        }
        self.state.sync_child_states(&self.options.tabs);
    }

    pub(super) fn normalize_tabs(&mut self) {
        self.options.tabs = ordered_tabs(&self.options.tabs, &self.options.groups)
            .into_iter()
            .cloned()
            .collect();
        self.state.sync_child_states(&self.options.tabs);
    }

    fn find_tab(&self, tab_id: &WorkspaceTabId) -> Option<&WorkspaceTab> {
        self.options.tabs.iter().find(|tab| &tab.id == tab_id)
    }
}
