use super::support::{group_for_path, group_path_for_target};
use super::tab_strip_projection_lease::TabStripGroupPopupPresentation;
use super::{
    TabStripOverlayState, TabStripProjection, TabStripProjectionLease, TabStripRetainedError,
    TabStripRetainedState, TabStripRootItem, TabStripRouteTable, TabStripTabTarget,
};

fn tab_target_for_path(projection: &TabStripProjection, path: &str) -> Option<TabStripTabTarget> {
    let mut tab_occurrence = 0;
    let mut group_occurrence = 0;
    for item in &projection.root_items {
        match item {
            TabStripRootItem::Tab(index) => {
                if path == format!("root-tab-{tab_occurrence}") {
                    return projection.tabs.get(*index).map(|tab| tab.target.clone());
                }
                tab_occurrence += 1;
            }
            TabStripRootItem::Group(index) => {
                if let Some(group) = projection.groups.get(*index)
                    && let Some(target) =
                        tab_target_for_group(group, &format!("root-group-{group_occurrence}"), path)
                {
                    return Some(target);
                }
                group_occurrence += 1;
            }
        }
    }
    if !projection.root_items.is_empty() {
        return None;
    }
    projection
        .tabs
        .iter()
        .enumerate()
        .find_map(|(index, tab)| (path == format!("root-tab-{index}")).then(|| tab.target.clone()))
        .or_else(|| {
            projection
                .groups
                .iter()
                .enumerate()
                .find_map(|(index, group)| {
                    tab_target_for_group(group, &format!("root-group-{index}"), path)
                })
        })
}

fn tab_target_for_group(
    group: &super::TabStripGroupDescriptor,
    prefix: &str,
    path: &str,
) -> Option<TabStripTabTarget> {
    group
        .tabs
        .iter()
        .enumerate()
        .find_map(|(index, tab)| {
            (path == format!("{prefix}-tab-{index}")).then(|| tab.target.clone())
        })
        .or_else(|| {
            group.groups.iter().enumerate().find_map(|(index, child)| {
                tab_target_for_group(child, &format!("{prefix}-group-{index}"), path)
            })
        })
}

fn tab_path_for_target(
    projection: &TabStripProjection,
    target: &TabStripTabTarget,
) -> Option<String> {
    let mut tab_occurrence = 0;
    let mut group_occurrence = 0;
    for item in &projection.root_items {
        match item {
            TabStripRootItem::Tab(index) => {
                if projection
                    .tabs
                    .get(*index)
                    .is_some_and(|tab| &tab.target == target)
                {
                    return Some(format!("root-tab-{tab_occurrence}"));
                }
                tab_occurrence += 1;
            }
            TabStripRootItem::Group(index) => {
                if let Some(group) = projection.groups.get(*index)
                    && let Some(path) =
                        tab_path_for_group(group, target, &format!("root-group-{group_occurrence}"))
                {
                    return Some(path);
                }
                group_occurrence += 1;
            }
        }
    }
    if !projection.root_items.is_empty() {
        return None;
    }
    projection
        .tabs
        .iter()
        .enumerate()
        .find_map(|(index, tab)| (&tab.target == target).then(|| format!("root-tab-{index}")))
        .or_else(|| {
            projection
                .groups
                .iter()
                .enumerate()
                .find_map(|(index, group)| {
                    tab_path_for_group(group, target, &format!("root-group-{index}"))
                })
        })
}

fn tab_path_for_group(
    group: &super::TabStripGroupDescriptor,
    target: &TabStripTabTarget,
    prefix: &str,
) -> Option<String> {
    group
        .tabs
        .iter()
        .enumerate()
        .find_map(|(index, tab)| (&tab.target == target).then(|| format!("{prefix}-tab-{index}")))
        .or_else(|| {
            group.groups.iter().enumerate().find_map(|(index, child)| {
                tab_path_for_group(child, target, &format!("{prefix}-group-{index}"))
            })
        })
}

fn tab_draggable_for_target(
    projection: &TabStripProjection,
    target: &TabStripTabTarget,
) -> Option<bool> {
    for item in &projection.root_items {
        match item {
            TabStripRootItem::Tab(index) => {
                if let Some(tab) = projection.tabs.get(*index)
                    && &tab.target == target
                {
                    return Some(tab.capabilities.draggable);
                }
            }
            TabStripRootItem::Group(index) => {
                if let Some(group) = projection.groups.get(*index)
                    && let Some(draggable) = tab_draggable_in_group(group, target)
                {
                    return Some(draggable);
                }
            }
        }
    }
    if !projection.root_items.is_empty() {
        return None;
    }
    projection
        .tabs
        .iter()
        .find(|tab| &tab.target == target)
        .map(|tab| tab.capabilities.draggable)
        .or_else(|| {
            projection
                .groups
                .iter()
                .find_map(|group| tab_draggable_in_group(group, target))
        })
}

fn tab_draggable_in_group(
    group: &super::TabStripGroupDescriptor,
    target: &TabStripTabTarget,
) -> Option<bool> {
    if group.capabilities.collapsed {
        return None;
    }
    group
        .tabs
        .iter()
        .find(|tab| &tab.target == target)
        .map(|tab| tab.capabilities.draggable)
        .or_else(|| {
            group
                .groups
                .iter()
                .find_map(|child| tab_draggable_in_group(child, target))
        })
}

fn group_rename_presentation_for_target<'a>(
    projection: &'a TabStripProjection,
    target: &super::TabStripGroupTarget,
) -> Option<&'a TabStripGroupPopupPresentation> {
    group_path_for_target(projection, target)
        .and_then(|path| group_for_path(projection, &path))
        .filter(|group| group.capabilities.renamable)
        .and_then(|group| group.popup.as_ref())
        .filter(|popup| popup.rename_placeholder.is_some())
}

impl TabStripRetainedState {
    pub(crate) fn synchronize_lease(
        &mut self,
        lease: TabStripProjectionLease,
    ) -> Result<bool, TabStripRetainedError> {
        let (projection, port) = lease.into_parts();
        let overlay_tab_target = match &self.overlay {
            TabStripOverlayState::TabMenu { path, .. } => {
                tab_target_for_path(&self.projection, path)
            }
            _ => None,
        };
        let overlay_group_target = match &self.overlay {
            TabStripOverlayState::GroupPopup { path, .. } => {
                group_for_path(&self.projection, path).map(|group| group.target.clone())
            }
            _ => None,
        };
        let overlay_has_rename_editor = matches!(
            &self.overlay,
            TabStripOverlayState::GroupPopup {
                rename: Some(_),
                ..
            }
        );
        let invalid_tab_drag = self.drag.as_ref().is_some_and(|drag| {
            !tab_draggable_for_target(&projection, &drag.source).unwrap_or(false)
        });
        if invalid_tab_drag {
            self.suspend_interaction()?;
        }
        self.inline_rename_consumed.retain(|target| {
            group_rename_presentation_for_target(&projection, target)
                .is_some_and(|popup| popup.request_inline_rename)
        });
        if let Some(drag) = &self.group_drag {
            let valid = group_path_for_target(&projection, &drag.source)
                .and_then(|path| group_for_path(&projection, &path))
                .is_some_and(|group| group.capabilities.draggable);
            if !valid {
                self.group_drag = None;
                self.group_drag_release_pending = false;
            }
        }
        self.bind_drag_origin();
        self.routes = TabStripRouteTable::from_projection(&projection);
        self.active_reveal_pending = projection.scroll_presentation.request_active_reveal;
        self.projection = projection;
        self.port = port;
        if let Some(target) = overlay_tab_target {
            if let Some(path) = tab_path_for_target(&self.projection, &target) {
                if let TabStripOverlayState::TabMenu { path: current, .. } = &mut self.overlay {
                    *current = path;
                }
            } else {
                self.overlay = TabStripOverlayState::Closed;
            }
        } else if let Some(target) = overlay_group_target {
            if let Some(path) = group_path_for_target(&self.projection, &target)
                && (!overlay_has_rename_editor
                    || group_rename_presentation_for_target(&self.projection, &target).is_some())
            {
                if let TabStripOverlayState::GroupPopup { path: current, .. } = &mut self.overlay {
                    *current = path;
                }
            } else {
                self.overlay = TabStripOverlayState::Closed;
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egui::text_command_surface::{
        TabStripCorrelation, TabStripGroupCapabilities, TabStripGroupDescriptor,
        TabStripGroupTarget, TabStripTabCapabilities, TabStripTabDescriptor, TabStripText,
    };

    fn projection() -> (
        TabStripProjection,
        TabStripTabTarget,
        TabStripTabTarget,
        TabStripTabTarget,
        TabStripTabTarget,
    ) {
        let root_tab = TabStripTabTarget::from_opaque_bytes(b"root-tab");
        let direct_tab = TabStripTabTarget::from_opaque_bytes(b"direct-tab");
        let nested_tab = TabStripTabTarget::from_opaque_bytes(b"nested-tab");
        let grandchild_tab = TabStripTabTarget::from_opaque_bytes(b"grandchild-tab");
        let projection =
            TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"helper-tests"))
                .group(
                    TabStripGroupDescriptor::new(
                        TabStripGroupTarget::from_opaque_bytes(b"parent"),
                        TabStripText::new("parent"),
                    )
                    .tab(
                        TabStripTabDescriptor::new(direct_tab.clone(), TabStripText::new("direct"))
                            .capabilities(TabStripTabCapabilities::new().draggable(true)),
                    )
                    .group(
                        TabStripGroupDescriptor::new(
                            TabStripGroupTarget::from_opaque_bytes(b"child"),
                            TabStripText::new("child"),
                        )
                        .tab(TabStripTabDescriptor::new(
                            grandchild_tab.clone(),
                            TabStripText::new("grandchild"),
                        )),
                    ),
                )
                .tab(TabStripTabDescriptor::new(
                    root_tab.clone(),
                    TabStripText::new("root"),
                ));
        (projection, root_tab, direct_tab, nested_tab, grandchild_tab)
    }

    #[test]
    fn tab_overlay_path_helpers_follow_mixed_nested_and_fallback_layouts() {
        let (projection, root_tab, direct_tab, _nested_tab, grandchild_tab) = projection();
        assert_eq!(
            tab_target_for_path(&projection, "root-tab-0"),
            Some(root_tab.clone())
        );
        assert_eq!(
            tab_target_for_path(&projection, "root-group-0-tab-0"),
            Some(direct_tab.clone())
        );
        assert_eq!(
            tab_target_for_path(&projection, "root-group-0-group-0-tab-0"),
            Some(grandchild_tab.clone())
        );
        assert_eq!(
            tab_path_for_target(&projection, &root_tab),
            Some("root-tab-0".to_owned())
        );
        assert_eq!(
            tab_path_for_target(&projection, &grandchild_tab),
            Some("root-group-0-group-0-tab-0".to_owned())
        );
        assert_eq!(
            tab_draggable_for_target(&projection, &root_tab),
            Some(false)
        );
        assert_eq!(
            tab_draggable_for_target(&projection, &direct_tab),
            Some(true)
        );
        assert_eq!(
            tab_draggable_for_target(&projection, &grandchild_tab),
            Some(false)
        );
        let missing = TabStripTabTarget::from_opaque_bytes(b"missing");
        assert!(tab_path_for_target(&projection, &missing).is_none());
        assert!(tab_draggable_for_target(&projection, &missing).is_none());
        assert!(tab_target_for_path(&projection, "root-group-9-tab-0").is_none());

        let mut fallback = projection;
        fallback.root_items.clear();
        assert_eq!(
            tab_path_for_target(&fallback, &root_tab),
            Some("root-tab-0".to_owned())
        );
        assert_eq!(tab_draggable_for_target(&fallback, &root_tab), Some(false));
        assert_eq!(
            tab_draggable_for_target(&fallback, &grandchild_tab),
            Some(false)
        );
        assert_eq!(
            tab_target_for_path(&fallback, "root-tab-0"),
            Some(root_tab.clone())
        );
        assert_eq!(
            tab_target_for_path(&fallback, "root-group-0-tab-0"),
            Some(direct_tab.clone())
        );
        assert_eq!(
            tab_path_for_target(&fallback, &direct_tab),
            Some("root-group-0-tab-0".to_owned())
        );
        assert_eq!(tab_draggable_for_target(&fallback, &direct_tab), Some(true));
        assert_eq!(
            tab_path_for_target(&fallback, &grandchild_tab),
            Some("root-group-0-group-0-tab-0".to_owned())
        );
        assert!(tab_target_for_path(&fallback, "root-group-9-tab-0").is_none());
        assert!(tab_path_for_target(&fallback, &missing).is_none());
        assert!(tab_draggable_for_target(&fallback, &missing).is_none());

        fallback.root_items = vec![TabStripRootItem::Tab(usize::MAX)];
        assert!(tab_target_for_path(&fallback, "root-tab-0").is_none());
        assert!(tab_path_for_target(&fallback, &root_tab).is_none());
        assert!(tab_draggable_for_target(&fallback, &root_tab).is_none());
        fallback.root_items = vec![TabStripRootItem::Group(usize::MAX)];
        assert!(tab_draggable_for_target(&fallback, &direct_tab).is_none());

        let collapsed =
            TabStripProjection::new(2, TabStripCorrelation::from_opaque_bytes(b"collapsed")).group(
                TabStripGroupDescriptor::new(
                    TabStripGroupTarget::from_opaque_bytes(b"collapsed-group"),
                    TabStripText::new("collapsed"),
                )
                .capabilities(TabStripGroupCapabilities::new().collapsed(true))
                .tab(
                    TabStripTabDescriptor::new(direct_tab.clone(), TabStripText::new("hidden"))
                        .capabilities(TabStripTabCapabilities::new().draggable(true)),
                ),
            );
        assert!(tab_draggable_for_target(&collapsed, &direct_tab).is_none());
    }
}
