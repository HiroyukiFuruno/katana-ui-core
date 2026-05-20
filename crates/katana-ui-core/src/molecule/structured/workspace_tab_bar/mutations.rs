use super::actions::{WorkspaceTabDropRules, WorkspaceTabGroupTarget};
use super::bar::WorkspaceTabBar;
use super::events::WorkspaceTabBarEvent;
use super::identifiers::{WorkspaceTabGroupId, WorkspaceTabId};
use super::options::WorkspaceTab;
use super::ordering::ordered_tabs;

impl WorkspaceTabBar {
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
        if !tab.closeable {
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

    pub(super) fn set_pinned(
        &mut self,
        tab_id: WorkspaceTabId,
        pinned: bool,
    ) -> Vec<WorkspaceTabBarEvent> {
        if let Some(tab) = self.find_tab_mut(&tab_id) {
            tab.pinned = pinned;
            self.normalize_tabs();
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
        let mut visual_tabs: Vec<WorkspaceTab> = ordered_tabs(&self.options.tabs)
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

    pub(super) fn move_to_group(
        &mut self,
        tab_id: WorkspaceTabId,
        target: WorkspaceTabGroupTarget,
    ) -> Vec<WorkspaceTabBarEvent> {
        let group_id = self.resolve_group_target(target);
        if let Some(tab) = self.find_tab_mut(&tab_id) {
            tab.group_id = group_id;
        }
        Vec::new()
    }

    pub(super) fn toggle_group_collapse(
        &mut self,
        group_id: WorkspaceTabGroupId,
    ) -> Vec<WorkspaceTabBarEvent> {
        let Some(group) = self.options.groups.iter_mut().find(|it| it.id == group_id) else {
            return Vec::new();
        };
        group.collapsed = !group.collapsed;
        vec![WorkspaceTabBarEvent::GroupCollapseChanged {
            group_id,
            collapsed: group.collapsed,
        }]
    }

    pub(super) fn hover_collapsed_group_for_drop(
        &mut self,
        group_id: WorkspaceTabGroupId,
        elapsed_ms: u16,
    ) -> Vec<WorkspaceTabBarEvent> {
        if elapsed_ms < self.options.collapsed_group_auto_expand_ms {
            return Vec::new();
        }
        let Some(group) = self.options.groups.iter_mut().find(|it| it.id == group_id) else {
            return Vec::new();
        };
        if !group.collapsed {
            return Vec::new();
        }
        group.collapsed = false;
        vec![WorkspaceTabBarEvent::GroupCollapseChanged {
            group_id,
            collapsed: false,
        }]
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
    pub(super) fn open_overflow(
        &mut self,
        hidden_tab_ids: Vec<WorkspaceTabId>,
    ) -> Vec<WorkspaceTabBarEvent> {
        self.state.overflow_visible = true;
        vec![WorkspaceTabBarEvent::OverflowOpened { hidden_tab_ids }]
    }

    fn resolve_group_target(
        &mut self,
        target: WorkspaceTabGroupTarget,
    ) -> Option<WorkspaceTabGroupId> {
        match target {
            WorkspaceTabGroupTarget::Existing(group_id) => Some(group_id),
            WorkspaceTabGroupTarget::Ungrouped => None,
            WorkspaceTabGroupTarget::NewGroup(group) => {
                let group_id = group.id.clone();
                self.options.groups.push(group);
                Some(group_id)
            }
        }
    }

    fn remove_tab(&mut self, tab_id: &WorkspaceTabId) {
        self.options.tabs.retain(|tab| &tab.id != tab_id);
        if self.state.active_tab_id.as_ref() == Some(tab_id) {
            self.state.active_tab_id = self.options.tabs.first().map(|tab| tab.id.clone());
        }
        self.state.sync_child_states(&self.options.tabs);
    }

    pub(super) fn normalize_tabs(&mut self) {
        self.options.tabs = ordered_tabs(&self.options.tabs)
            .into_iter()
            .cloned()
            .collect();
        self.state.sync_child_states(&self.options.tabs);
    }

    fn find_tab(&self, tab_id: &WorkspaceTabId) -> Option<&WorkspaceTab> {
        self.options.tabs.iter().find(|tab| &tab.id == tab_id)
    }

    fn find_tab_mut(&mut self, tab_id: &WorkspaceTabId) -> Option<&mut WorkspaceTab> {
        self.options.tabs.iter_mut().find(|tab| &tab.id == tab_id)
    }
}
