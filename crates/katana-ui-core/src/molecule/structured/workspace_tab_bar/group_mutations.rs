use super::actions::WorkspaceTabGroupTarget;
use super::bar::WorkspaceTabBar;
use super::events::WorkspaceTabBarEvent;
use super::identifiers::{WorkspaceTabGroupId, WorkspaceTabId};

impl WorkspaceTabBar {
    pub(super) fn set_pinned(
        &mut self,
        tab_id: WorkspaceTabId,
        pinned: bool,
    ) -> Vec<WorkspaceTabBarEvent> {
        let Some(tab) = self.options.tabs.iter_mut().find(|it| it.id == tab_id) else {
            return Vec::new();
        };
        if tab.pinned == pinned && !(pinned && tab.group_id.is_some()) {
            return Vec::new();
        }
        let pin_changed = tab.pinned != pinned;
        let removed_group_id = if pinned { tab.group_id.take() } else { None };
        tab.pinned = pinned;
        self.normalize_tabs();
        let mut events = Vec::new();
        if pin_changed {
            events.push(WorkspaceTabBarEvent::TabPinChanged {
                tab_id: tab_id.clone(),
                pinned,
            });
        }
        if removed_group_id.is_some() {
            events.push(WorkspaceTabBarEvent::TabGroupChanged {
                tab_id,
                group_id: None,
            });
        }
        events
    }

    pub(super) fn move_to_group(
        &mut self,
        tab_id: WorkspaceTabId,
        target: WorkspaceTabGroupTarget,
    ) -> Vec<WorkspaceTabBarEvent> {
        let Some(tab_index) = self.options.tabs.iter().position(|tab| tab.id == tab_id) else {
            return Vec::new();
        };
        let tab = &self.options.tabs[tab_index];
        if tab.pinned || !tab.groupable {
            return Vec::new();
        }
        let (group_id, created_group_id) = self.resolve_group_target(target);
        let tab = &mut self.options.tabs[tab_index];
        if tab.group_id == group_id {
            return created_group_events(created_group_id);
        }
        tab.group_id = group_id.clone();
        let mut events = created_group_events(created_group_id);
        events.push(WorkspaceTabBarEvent::TabGroupChanged { tab_id, group_id });
        events
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

    pub(super) fn move_group(
        &mut self,
        group_id: WorkspaceTabGroupId,
        to_index: usize,
    ) -> Vec<WorkspaceTabBarEvent> {
        let Some(from) = self
            .options
            .groups
            .iter()
            .position(|group| group.id == group_id)
        else {
            return Vec::new();
        };
        let to = to_index.min(self.options.groups.len().saturating_sub(1));
        if from == to {
            return Vec::new();
        }
        let group = self.options.groups.remove(from);
        self.options.groups.insert(to, group);
        vec![WorkspaceTabBarEvent::GroupReordered { group_id, from, to }]
    }

    pub(super) fn rename_group(
        &mut self,
        group_id: WorkspaceTabGroupId,
        label: String,
    ) -> Vec<WorkspaceTabBarEvent> {
        let Some(group) = self.options.groups.iter_mut().find(|it| it.id == group_id) else {
            return Vec::new();
        };
        if group.label == label {
            return Vec::new();
        }
        group.label = label.clone();
        vec![WorkspaceTabBarEvent::GroupRenamed { group_id, label }]
    }

    pub(super) fn set_group_color(
        &mut self,
        group_id: WorkspaceTabGroupId,
        color: String,
    ) -> Vec<WorkspaceTabBarEvent> {
        let Some(group) = self.options.groups.iter_mut().find(|it| it.id == group_id) else {
            return Vec::new();
        };
        if group.color == color {
            return Vec::new();
        }
        group.color = color.clone();
        vec![WorkspaceTabBarEvent::GroupColorChanged { group_id, color }]
    }

    pub(super) fn ungroup(&mut self, group_id: WorkspaceTabGroupId) -> Vec<WorkspaceTabBarEvent> {
        if self.options.groups.iter().all(|group| group.id != group_id) {
            return Vec::new();
        }
        let mut events = Vec::new();
        for tab in self
            .options
            .tabs
            .iter_mut()
            .filter(|tab| tab.group_id.as_ref() == Some(&group_id))
        {
            tab.group_id = None;
            events.push(WorkspaceTabBarEvent::TabGroupChanged {
                tab_id: tab.id.clone(),
                group_id: None,
            });
        }
        self.remove_group(&group_id);
        self.normalize_tabs();
        events.push(WorkspaceTabBarEvent::GroupRemoved { group_id });
        events
    }

    pub(super) fn close_group(
        &mut self,
        group_id: WorkspaceTabGroupId,
    ) -> Vec<WorkspaceTabBarEvent> {
        if self.options.groups.iter().all(|group| group.id != group_id) {
            return Vec::new();
        }
        let tab_ids = self.group_tab_ids(&group_id);
        let mut events = self.close_tab_ids(tab_ids, None);
        if self.group_tab_ids(&group_id).is_empty() {
            self.remove_group(&group_id);
            self.normalize_tabs();
            events.push(WorkspaceTabBarEvent::GroupRemoved { group_id });
        }
        events
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

    fn resolve_group_target(
        &mut self,
        target: WorkspaceTabGroupTarget,
    ) -> (Option<WorkspaceTabGroupId>, Option<WorkspaceTabGroupId>) {
        match target {
            WorkspaceTabGroupTarget::Existing(group_id) => (Some(group_id), None),
            WorkspaceTabGroupTarget::Ungrouped => (None, None),
            WorkspaceTabGroupTarget::NewGroup(group) => {
                let group_id = group.id.clone();
                self.options.groups.push(group);
                (Some(group_id.clone()), Some(group_id))
            }
        }
    }

    fn group_tab_ids(&self, group_id: &WorkspaceTabGroupId) -> Vec<WorkspaceTabId> {
        self.options
            .tabs
            .iter()
            .filter(|tab| tab.group_id.as_ref() == Some(group_id))
            .map(|tab| tab.id.clone())
            .collect()
    }

    fn remove_group(&mut self, group_id: &WorkspaceTabGroupId) {
        self.options.groups.retain(|group| &group.id != group_id);
    }
}

fn created_group_events(group_id: Option<WorkspaceTabGroupId>) -> Vec<WorkspaceTabBarEvent> {
    group_id
        .into_iter()
        .map(|group_id| WorkspaceTabBarEvent::GroupCreated { group_id })
        .collect()
}
