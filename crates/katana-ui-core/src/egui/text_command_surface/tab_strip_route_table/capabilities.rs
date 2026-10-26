use super::super::tab_strip_projection_lease::{
    TabStripGroupCapabilities, TabStripGroupTarget, TabStripTabCapabilities, TabStripTabTarget,
};
use super::{TabStripGroupDescriptor, TabStripProjection, TabStripRoute};

pub(super) struct RouteCapabilities;

impl RouteCapabilities {
    pub(super) fn allows(projection: &TabStripProjection, route: &TabStripRoute) -> bool {
        match route {
            TabStripRoute::Previous => projection.capabilities.previous_available,
            TabStripRoute::Next => projection.capabilities.next_available,
            TabStripRoute::OpenOverflow => projection.capabilities.overflow_available,
            TabStripRoute::SelectTab(target) => {
                Self::tab(projection, target).is_some_and(|caps| caps.selectable)
            }
            TabStripRoute::RequestClose(target) => {
                Self::tab(projection, target).is_some_and(|caps| caps.closeable)
            }
            TabStripRoute::CreateGroup(target) | TabStripRoute::RemoveFromGroup(target) => {
                Self::tab(projection, target).is_some_and(|caps| caps.groupable)
            }
            TabStripRoute::MoveTabToGroup { tab, .. } => {
                Self::tab(projection, tab).is_some_and(|caps| caps.groupable)
            }
            TabStripRoute::SetGroupCollapsed { group, .. } => {
                Self::group(projection, group).is_some_and(|caps| caps.collapsible)
            }
            TabStripRoute::RenameGroup(group) => {
                Self::group(projection, group).is_some_and(|caps| caps.renamable)
            }
            TabStripRoute::RecolorGroup { group, .. } => {
                Self::group(projection, group).is_some_and(|caps| caps.recolorable)
            }
            TabStripRoute::Ungroup(group) => {
                Self::group(projection, group).is_some_and(|caps| caps.ungroupable)
            }
            TabStripRoute::CloseGroup(group) => {
                Self::group(projection, group).is_some_and(|caps| caps.closeable)
            }
            TabStripRoute::RestoreClosed => projection.capabilities.restore_available,
            TabStripRoute::CloseOthers(_)
            | TabStripRoute::CloseAll
            | TabStripRoute::CloseToLeft(_)
            | TabStripRoute::CloseToRight(_)
            | TabStripRoute::Unpin(_)
            | TabStripRoute::Pin(_) => true,
        }
    }

    fn tab<'a>(
        projection: &'a TabStripProjection,
        target: &TabStripTabTarget,
    ) -> Option<&'a TabStripTabCapabilities> {
        projection
            .tabs
            .iter()
            .find(|tab| tab.target == *target)
            .map(|tab| &tab.capabilities)
            .or_else(|| {
                projection
                    .groups
                    .iter()
                    .find_map(|group| Self::group_tab(group, target))
            })
    }

    fn group_tab<'a>(
        group: &'a TabStripGroupDescriptor,
        target: &TabStripTabTarget,
    ) -> Option<&'a TabStripTabCapabilities> {
        group
            .tabs
            .iter()
            .find(|tab| tab.target == *target)
            .map(|tab| &tab.capabilities)
            .or_else(|| {
                group
                    .groups
                    .iter()
                    .find_map(|group| Self::group_tab(group, target))
            })
    }

    fn group<'a>(
        projection: &'a TabStripProjection,
        target: &TabStripGroupTarget,
    ) -> Option<&'a TabStripGroupCapabilities> {
        projection
            .groups
            .iter()
            .find_map(|group| Self::nested_group(group, target))
    }

    fn nested_group<'a>(
        group: &'a TabStripGroupDescriptor,
        target: &TabStripGroupTarget,
    ) -> Option<&'a TabStripGroupCapabilities> {
        if group.target == *target {
            Some(&group.capabilities)
        } else {
            group
                .groups
                .iter()
                .find_map(|group| Self::nested_group(group, target))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::TabStripRouteTable;
    use super::*;
    use crate::egui::text_command_surface::{
        TabStripCorrelation, TabStripGroupPopupPresentation, TabStripMenuEntry,
        TabStripMenuOperation, TabStripText,
    };
    use crate::render_model::UiRect;

    fn projection(enabled: bool) -> TabStripProjection {
        TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"capability")).group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"group"),
                TabStripText::new("Group"),
            )
            .capabilities(
                TabStripGroupCapabilities::new()
                    .renamable(enabled)
                    .recolorable(enabled)
                    .ungroupable(enabled)
                    .closeable(enabled),
            )
            .popup(
                TabStripGroupPopupPresentation::new()
                    .rename_placeholder(TabStripText::new("Rename"))
                    .entry(TabStripMenuEntry::action(
                        TabStripText::new("Ungroup"),
                        TabStripText::new("Ungroup"),
                        TabStripMenuOperation::Ungroup,
                    ))
                    .entry(TabStripMenuEntry::action(
                        TabStripText::new("Close"),
                        TabStripText::new("Close"),
                        TabStripMenuOperation::CloseGroup,
                    )),
            ),
        )
    }

    #[test]
    fn unavailable_group_capabilities_remain_accessible_but_cannot_propose_effects() {
        let mut disabled = TabStripRouteTable::from_projection(&projection(false));
        for (index, path) in [
            "root-group-0-popup-rename",
            "root-group-0-popup-0",
            "root-group-0-popup-1",
        ]
        .iter()
        .enumerate()
        {
            let response = egui::Id::new(("disabled-capability", index));
            disabled.register_response(
                path,
                response,
                UiRect::new(0, 0, 40, 20),
                "Unavailable",
                false,
            );
            assert!(
                disabled
                    .route_for_response(response)
                    .expect("accessible disabled control")
                    .2
            );
            assert!(disabled.proposal_for(path).is_none());
            assert!(disabled.proposal_for_response(response).is_none());
        }
        let enabled = TabStripRouteTable::from_projection(&projection(true));
        assert!(
            enabled
                .rename_proposal_for("root-group-0-popup-rename", TabStripText::new("Updated"))
                .is_some()
        );
        for path in ["root-group-0-popup-0", "root-group-0-popup-1"] {
            assert!(enabled.proposal_for(path).is_some());
        }
    }
}
