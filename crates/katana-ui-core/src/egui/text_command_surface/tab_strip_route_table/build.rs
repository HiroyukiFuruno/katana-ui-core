use super::super::tab_strip_projection_lease::{
    TabStripContextMenuPresentation, TabStripGroupDescriptor, TabStripMenuEntry,
    TabStripTabDescriptor, TabStripTabTarget,
};
use super::{TabStripRoute, TabStripRouteTable};
use std::collections::BTreeMap;

impl TabStripRouteTable {
    pub(super) fn insert_tab(
        routes: &mut BTreeMap<String, TabStripRoute>,
        tab: &TabStripTabDescriptor,
        path: String,
    ) {
        if tab.capabilities.selectable {
            routes.insert(
                format!("{path}-label"),
                TabStripRoute::SelectTab(tab.target.copy_for_route()),
            );
        }
        if (tab.capabilities.closeable || tab.capabilities.pinned) && tab.trailing_control.is_some()
        {
            let route = if tab.capabilities.pinned {
                TabStripRoute::Unpin(tab.target.copy_for_route())
            } else {
                TabStripRoute::RequestClose(tab.target.copy_for_route())
            };
            routes.insert(format!("{path}-trailing"), route);
        }
        if let Some(menu) = tab.context_menu.as_ref() {
            Self::insert_tab_menu(routes, menu, &tab.target, format!("{path}-menu"));
        }
    }

    pub(super) fn insert_group(
        routes: &mut BTreeMap<String, TabStripRoute>,
        group: &TabStripGroupDescriptor,
        path: String,
    ) {
        if group.capabilities.collapsible {
            routes.insert(
                format!("{path}-header"),
                TabStripRoute::SetGroupCollapsed {
                    group: group.target.copy_for_route(),
                    collapsed: !group.capabilities.collapsed,
                },
            );
        }
        if let Some(popup) = group.popup.as_ref() {
            Self::insert_group_popup(routes, popup, group, format!("{path}-popup"));
            if popup.rename_placeholder.is_some() {
                routes.insert(
                    format!("{path}-popup-rename"),
                    TabStripRoute::RenameGroup(group.target.copy_for_route()),
                );
            }
        }
        for (index, tab) in group.tabs.iter().enumerate() {
            Self::insert_tab(routes, tab, format!("{path}-tab-{index}"));
        }
        for (index, child) in group.groups.iter().enumerate() {
            Self::insert_group(routes, child, format!("{path}-group-{index}"));
        }
    }

    fn insert_tab_menu(
        routes: &mut BTreeMap<String, TabStripRoute>,
        menu: &TabStripContextMenuPresentation,
        tab: &TabStripTabTarget,
        path: String,
    ) {
        for (index, entry) in menu.entries.iter().enumerate() {
            Self::insert_tab_menu_entry(routes, entry, tab, format!("{path}-{index}"));
        }
    }

    fn insert_tab_menu_entry(
        routes: &mut BTreeMap<String, TabStripRoute>,
        entry: &TabStripMenuEntry,
        tab: &TabStripTabTarget,
        path: String,
    ) {
        if let Some(route) = Self::tab_menu_route(entry.operation.as_ref(), tab) {
            routes.insert(path.clone(), route);
        }
        for (index, child) in entry.children.iter().enumerate() {
            Self::insert_tab_menu_entry(routes, child, tab, format!("{path}-{index}"));
        }
    }
}
