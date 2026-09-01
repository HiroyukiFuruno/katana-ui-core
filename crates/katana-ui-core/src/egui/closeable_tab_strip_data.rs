use crate::molecule::structured::{
    CloseableTab, CloseableTabGroup, CloseableTabGroupId, CloseableTabStrip,
};

pub(super) struct TabStripItems {
    pub(super) tabs: Vec<CloseableTab>,
    pub(super) groups: Vec<CloseableTabGroup>,
    pub(super) pinned_tabs: Vec<CloseableTab>,
    pub(super) root_groups: Vec<CloseableTabGroup>,
    pub(super) unknown_group_tabs: Vec<CloseableTab>,
    pub(super) ungrouped_tabs: Vec<CloseableTab>,
}

impl TabStripItems {
    pub(super) fn from_strip(strip: &CloseableTabStrip) -> Self {
        let options = strip.options();
        let tabs = options.tabs.clone();
        let groups = options.groups.clone();
        let pinned_tabs = options
            .tabs
            .iter()
            .filter(|tab| tab.pinned)
            .cloned()
            .collect();
        let root_groups = options
            .groups
            .iter()
            .filter(|group| group.parent_group_id.is_none())
            .cloned()
            .collect();
        let unknown_group_tabs = options
            .tabs
            .iter()
            .filter(|tab| {
                !tab.pinned
                    && tab.group_id.as_ref().is_some_and(|group_id| {
                        options.groups.iter().all(|group| group.id != *group_id)
                    })
            })
            .cloned()
            .collect();
        let ungrouped_tabs = options
            .tabs
            .iter()
            .filter(|tab| !tab.pinned && tab.group_id.is_none())
            .cloned()
            .collect();

        Self {
            tabs,
            groups,
            pinned_tabs,
            root_groups,
            unknown_group_tabs,
            ungrouped_tabs,
        }
    }

    pub(super) fn tabs_for_group<'a>(
        &'a self,
        group_id: &CloseableTabGroupId,
    ) -> impl Iterator<Item = &'a CloseableTab> {
        self.all_tabs()
            .filter(move |tab| tab.group_id.as_ref() == Some(group_id))
    }

    pub(super) fn groups_for_parent<'a>(
        &'a self,
        parent_group_id: &CloseableTabGroupId,
    ) -> impl Iterator<Item = &'a CloseableTabGroup> {
        self.all_groups()
            .filter(move |group| group.parent_group_id.as_ref() == Some(parent_group_id))
    }

    pub(super) fn all_tabs(&self) -> impl Iterator<Item = &CloseableTab> {
        self.tabs.iter()
    }

    pub(super) fn all_groups(&self) -> impl Iterator<Item = &CloseableTabGroup> {
        self.groups.iter()
    }
}
