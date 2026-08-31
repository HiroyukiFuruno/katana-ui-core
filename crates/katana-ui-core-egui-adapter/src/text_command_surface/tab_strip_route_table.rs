//! KUC-private structural routes for the retained tab-strip renderer.
//!
//! Projection descriptors carry host-issued opaque targets only while the lease
//! is consumed. Pointer and keyboard paths resolve through this table, so the
//! renderer never derives a proposal directly from a visible descriptor.

use super::tab_strip_projection_lease::{
    TabStripCorrelation, TabStripGroupDescriptor, TabStripGroupPopupPresentation,
    TabStripGroupTarget, TabStripMenuEntry, TabStripMenuOperation, TabStripProjection,
    TabStripSwatchTarget, TabStripTabTarget,
};
use super::tab_strip_proposal_port::TabStripProposalOperation;
use katana_ui_core::render_model::UiRect;
use std::collections::{BTreeMap, HashMap};

mod build;
mod proposal;

pub(super) struct TabStripRouteTable {
    correlation: TabStripCorrelation,
    routes: BTreeMap<String, TabStripRoute>,
    frame_response_paths: HashMap<egui::Id, String>,
    frame_routes: HashMap<egui::Id, TabStripFrameRoute>,
}

struct TabStripFrameRoute {
    path: String,
    bounds: UiRect,
    label: String,
    disabled: bool,
}

enum TabStripRoute {
    SelectTab(TabStripTabTarget),
    Previous,
    Next,
    OpenOverflow,
    RequestClose(TabStripTabTarget),
    CloseOthers(TabStripTabTarget),
    CloseAll,
    CloseToLeft(TabStripTabTarget),
    CloseToRight(TabStripTabTarget),
    RestoreClosed,
    Unpin(TabStripTabTarget),
    Pin(TabStripTabTarget),
    CreateGroup(TabStripTabTarget),
    MoveTabToGroup {
        tab: TabStripTabTarget,
        group: TabStripGroupTarget,
    },
    RemoveFromGroup(TabStripTabTarget),
    SetGroupCollapsed {
        group: TabStripGroupTarget,
        collapsed: bool,
    },
    RenameGroup(TabStripGroupTarget),
    RecolorGroup {
        group: TabStripGroupTarget,
        swatch: TabStripSwatchTarget,
    },
    Ungroup(TabStripGroupTarget),
    CloseGroup(TabStripGroupTarget),
}

impl TabStripRouteTable {
    pub(super) fn from_projection(projection: &TabStripProjection) -> Self {
        let mut routes = BTreeMap::new();
        if let Some(navigation) = projection.navigation.as_ref() {
            let _ = navigation;
            routes.insert("tab-strip-previous".to_owned(), TabStripRoute::Previous);
            routes.insert("tab-strip-next".to_owned(), TabStripRoute::Next);
            if projection
                .navigation
                .as_ref()
                .is_some_and(|value| value.overflow.is_some())
            {
                routes.insert("tab-strip-overflow".to_owned(), TabStripRoute::OpenOverflow);
            }
        }
        for (index, tab) in projection.tabs.iter().enumerate() {
            Self::insert_tab(&mut routes, tab, format!("root-tab-{index}"));
        }
        for (index, group) in projection.groups.iter().enumerate() {
            Self::insert_group(&mut routes, group, format!("root-group-{index}"));
        }
        Self {
            correlation: projection.correlation.copy_for_route(),
            routes,
            frame_response_paths: HashMap::new(),
            frame_routes: HashMap::new(),
        }
    }

    pub(super) fn begin_frame(&mut self) {
        self.frame_response_paths.clear();
        self.frame_routes.clear();
    }

    pub(super) fn correlation_for_proposal(&self) -> TabStripCorrelation {
        self.correlation.copy_for_route()
    }

    pub(super) fn register_response(
        &mut self,
        path: &str,
        response_id: egui::Id,
        bounds: UiRect,
        label: &str,
        disabled: bool,
    ) {
        if self.routes.contains_key(path) {
            self.frame_response_paths
                .insert(response_id, path.to_owned());
            self.frame_routes.insert(
                response_id,
                TabStripFrameRoute {
                    path: path.to_owned(),
                    bounds,
                    label: label.to_owned(),
                    disabled,
                },
            );
        }
    }

    pub(super) fn proposal_for(
        &self,
        path: &str,
    ) -> Option<(TabStripCorrelation, TabStripProposalOperation)> {
        self.routes
            .get(path)
            .map(|route| (self.correlation.copy_for_route(), route.proposal()))
    }

    pub(super) fn proposal_for_response(
        &self,
        response_id: egui::Id,
    ) -> Option<(TabStripCorrelation, TabStripProposalOperation)> {
        self.frame_routes
            .get(&response_id)
            .filter(|route| !route.disabled)
            .map(|route| route.path.as_str())
            .and_then(|path| self.proposal_for(path))
    }

    pub(super) fn rename_proposal_for(
        &self,
        path: &str,
        name: super::tab_strip_projection_lease::TabStripText,
    ) -> Option<(TabStripCorrelation, TabStripProposalOperation)> {
        match self.routes.get(path)? {
            TabStripRoute::RenameGroup(group) => Some((
                self.correlation.copy_for_route(),
                TabStripProposalOperation::RenameGroup {
                    group: group.copy_for_route(),
                    name,
                },
            )),
            _ => None,
        }
    }

    pub(super) fn route_for_response(
        &self,
        response_id: egui::Id,
    ) -> Option<(&UiRect, &str, bool)> {
        self.frame_routes.get(&response_id).map(|route| {
            let _ = &route.path;
            (&route.bounds, route.label.as_str(), route.disabled)
        })
    }

    pub(super) fn response_is_disabled(&self, response_id: egui::Id) -> bool {
        self.frame_routes
            .get(&response_id)
            .is_some_and(|route| route.disabled)
    }

    fn insert_group_popup(
        routes: &mut BTreeMap<String, TabStripRoute>,
        popup: &TabStripGroupPopupPresentation,
        group: &TabStripGroupDescriptor,
        path: String,
    ) {
        for (index, entry) in popup.entries.iter().enumerate() {
            Self::insert_group_popup_entry(routes, entry, &group.target, format!("{path}-{index}"));
        }
        for (index, swatch) in group.swatches.iter().enumerate() {
            routes.insert(
                format!("{path}-swatch-{index}"),
                TabStripRoute::RecolorGroup {
                    group: group.target.copy_for_route(),
                    swatch: swatch.target.copy_for_route(),
                },
            );
        }
    }

    fn insert_group_popup_entry(
        routes: &mut BTreeMap<String, TabStripRoute>,
        entry: &TabStripMenuEntry,
        group: &TabStripGroupTarget,
        path: String,
    ) {
        if let Some(route) = Self::group_popup_route(entry.operation.as_ref(), group) {
            routes.insert(path.clone(), route);
        }
        for (index, child) in entry.children.iter().enumerate() {
            Self::insert_group_popup_entry(routes, child, group, format!("{path}-{index}"));
        }
    }

    fn tab_menu_route(
        operation: Option<&TabStripMenuOperation>,
        tab: &TabStripTabTarget,
    ) -> Option<TabStripRoute> {
        match operation? {
            TabStripMenuOperation::RequestClose => {
                Some(TabStripRoute::RequestClose(tab.copy_for_route()))
            }
            TabStripMenuOperation::CloseOthers => {
                Some(TabStripRoute::CloseOthers(tab.copy_for_route()))
            }
            TabStripMenuOperation::CloseAll => Some(TabStripRoute::CloseAll),
            TabStripMenuOperation::CloseToLeft => {
                Some(TabStripRoute::CloseToLeft(tab.copy_for_route()))
            }
            TabStripMenuOperation::CloseToRight => {
                Some(TabStripRoute::CloseToRight(tab.copy_for_route()))
            }
            TabStripMenuOperation::RestoreClosed => Some(TabStripRoute::RestoreClosed),
            TabStripMenuOperation::SetPinned(true) => {
                Some(TabStripRoute::Pin(tab.copy_for_route()))
            }
            TabStripMenuOperation::SetPinned(false) => {
                Some(TabStripRoute::Unpin(tab.copy_for_route()))
            }
            TabStripMenuOperation::CreateGroup => {
                Some(TabStripRoute::CreateGroup(tab.copy_for_route()))
            }
            TabStripMenuOperation::MoveToGroup(group) => Some(TabStripRoute::MoveTabToGroup {
                tab: tab.copy_for_route(),
                group: group.copy_for_route(),
            }),
            TabStripMenuOperation::RemoveFromGroup => {
                Some(TabStripRoute::RemoveFromGroup(tab.copy_for_route()))
            }
            TabStripMenuOperation::Ungroup
            | TabStripMenuOperation::CloseGroup
            | TabStripMenuOperation::Recolor(_) => None,
        }
    }

    fn group_popup_route(
        operation: Option<&TabStripMenuOperation>,
        group: &TabStripGroupTarget,
    ) -> Option<TabStripRoute> {
        match operation? {
            TabStripMenuOperation::Ungroup => Some(TabStripRoute::Ungroup(group.copy_for_route())),
            TabStripMenuOperation::CloseGroup => {
                Some(TabStripRoute::CloseGroup(group.copy_for_route()))
            }
            TabStripMenuOperation::Recolor(swatch) => Some(TabStripRoute::RecolorGroup {
                group: group.copy_for_route(),
                swatch: swatch.copy_for_route(),
            }),
            TabStripMenuOperation::RequestClose
            | TabStripMenuOperation::CloseOthers
            | TabStripMenuOperation::CloseAll
            | TabStripMenuOperation::CloseToLeft
            | TabStripMenuOperation::CloseToRight
            | TabStripMenuOperation::RestoreClosed
            | TabStripMenuOperation::SetPinned(_)
            | TabStripMenuOperation::CreateGroup
            | TabStripMenuOperation::MoveToGroup(_)
            | TabStripMenuOperation::RemoveFromGroup => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::tab_strip_proposal_port::TabStripProposalOperation;
    use super::TabStripRouteTable;
    use crate::text_command_surface::TabStripControlPresentation;
    use crate::text_command_surface::{
        TabStripContextMenuPresentation, TabStripCorrelation, TabStripGroupCapabilities,
        TabStripGroupDescriptor, TabStripGroupPopupPresentation, TabStripGroupTarget,
        TabStripMenuEntry, TabStripMenuOperation, TabStripProjection, TabStripSurfaceCapabilities,
        TabStripSwatchDescriptor, TabStripSwatchTarget, TabStripTabCapabilities,
        TabStripTabDescriptor, TabStripTabTarget, TabStripText,
    };
    use katana_ui_core::render_model::UiRect;

    #[test]
    fn routes_visible_controls_without_requiring_renderer_descriptor_access() {
        let projection =
            TabStripProjection::new(9, TabStripCorrelation::from_opaque_bytes(b"correlation"))
                .capabilities(
                    TabStripSurfaceCapabilities::new()
                        .previous_available(true)
                        .next_available(true),
                )
                .tab(
                    TabStripTabDescriptor::new(
                        TabStripTabTarget::from_opaque_bytes(b"tab"),
                        TabStripText::new("label"),
                    )
                    .capabilities(TabStripTabCapabilities::new().selectable(true)),
                )
                .group(
                    TabStripGroupDescriptor::new(
                        TabStripGroupTarget::from_opaque_bytes(b"group"),
                        TabStripText::new("group"),
                    )
                    .capabilities(TabStripGroupCapabilities::new().collapsible(true)),
                );
        let routes = TabStripRouteTable::from_projection(&projection);

        assert!(matches!(
            routes.proposal_for("root-tab-0-label"),
            Some((_, TabStripProposalOperation::SelectTab(_)))
        ));
        assert!(matches!(
            routes.proposal_for("root-group-0-header"),
            Some((
                _,
                TabStripProposalOperation::SetGroupCollapsed {
                    collapsed: true,
                    ..
                }
            ))
        ));
        assert!(routes.proposal_for("tab-strip-next").is_none());
        assert!(routes.proposal_for("missing").is_none());
    }

    #[test]
    fn routes_nested_group_controls_and_tabs_recursively() {
        let nested = TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"nested-group"),
            TabStripText::new("nested"),
        )
        .capabilities(TabStripGroupCapabilities::new().collapsible(true))
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"nested-tab"),
                TabStripText::new("nested tab"),
            )
            .capabilities(TabStripTabCapabilities::new().selectable(true)),
        );
        let projection =
            TabStripProjection::new(10, TabStripCorrelation::from_opaque_bytes(b"correlation"))
                .group(
                    TabStripGroupDescriptor::new(
                        TabStripGroupTarget::from_opaque_bytes(b"parent-group"),
                        TabStripText::new("parent"),
                    )
                    .group(nested),
                );
        let routes = TabStripRouteTable::from_projection(&projection);

        assert!(matches!(
            routes.proposal_for("root-group-0-group-0-header"),
            Some((
                _,
                TabStripProposalOperation::SetGroupCollapsed {
                    collapsed: true,
                    ..
                }
            ))
        ));
        assert!(matches!(
            routes.proposal_for("root-group-0-group-0-tab-0-label"),
            Some((_, TabStripProposalOperation::SelectTab(_)))
        ));
    }

    #[test]
    fn routes_include_overflow_and_navigation_controls_when_navigation_is_present() {
        let projection =
            TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"correlation"))
                .navigation(
                    crate::text_command_surface::TabStripNavigationPresentation::new(
                        crate::text_command_surface::TabStripControlPresentation::new(
                            TabStripText::new("prev"),
                            TabStripText::new("prev a11y"),
                        ),
                        crate::text_command_surface::TabStripControlPresentation::new(
                            TabStripText::new("next"),
                            TabStripText::new("next a11y"),
                        ),
                    )
                    .overflow(
                        crate::text_command_surface::TabStripControlPresentation::new(
                            TabStripText::new("more"),
                            TabStripText::new("more a11y"),
                        ),
                    ),
                )
                .tab(
                    TabStripTabDescriptor::new(
                        TabStripTabTarget::from_opaque_bytes(b"tab"),
                        TabStripText::new("tab"),
                    )
                    .capabilities(TabStripTabCapabilities::new().selectable(true)),
                );
        let routes = TabStripRouteTable::from_projection(&projection);

        assert!(matches!(
            routes.proposal_for("tab-strip-previous"),
            Some((_, TabStripProposalOperation::SelectPrevious))
        ));
        assert!(matches!(
            routes.proposal_for("tab-strip-next"),
            Some((_, TabStripProposalOperation::SelectNext))
        ));
        assert!(matches!(
            routes.proposal_for("tab-strip-overflow"),
            Some((_, TabStripProposalOperation::OpenOverflow))
        ));
    }

    #[test]
    fn tab_trailing_routes_distinguish_closeable_and_pinned_behavior() {
        let projection =
            TabStripProjection::new(2, TabStripCorrelation::from_opaque_bytes(b"correlation"))
                .tab(
                    TabStripTabDescriptor::new(
                        TabStripTabTarget::from_opaque_bytes(b"closeable-tab"),
                        TabStripText::new("closeable"),
                    )
                    .capabilities(
                        TabStripTabCapabilities::new()
                            .selectable(true)
                            .closeable(true),
                    )
                    .trailing_control(TabStripControlPresentation::new(
                        TabStripText::new("close"),
                        TabStripText::new("close a11y"),
                    )),
                )
                .tab(
                    TabStripTabDescriptor::new(
                        TabStripTabTarget::from_opaque_bytes(b"pinned-tab"),
                        TabStripText::new("pinned"),
                    )
                    .capabilities(TabStripTabCapabilities::new().selectable(true).pinned(true))
                    .trailing_control(TabStripControlPresentation::new(
                        TabStripText::new("pin"),
                        TabStripText::new("pin a11y"),
                    )),
                );
        let routes = TabStripRouteTable::from_projection(&projection);

        assert!(matches!(
            routes.proposal_for("root-tab-0-trailing"),
            Some((_, TabStripProposalOperation::RequestClose(_)))
        ));
        assert!(matches!(
            routes.proposal_for("root-tab-1-trailing"),
            Some((
                _,
                TabStripProposalOperation::SetPinned { pinned: false, .. }
            ))
        ));
    }

    #[test]
    fn rename_proposal_is_only_defined_for_group_popup_rename_paths() {
        let projection =
            TabStripProjection::new(3, TabStripCorrelation::from_opaque_bytes(b"correlation"))
                .group(
                    TabStripGroupDescriptor::new(
                        TabStripGroupTarget::from_opaque_bytes(b"group"),
                        TabStripText::new("group"),
                    )
                    .popup(
                        TabStripGroupPopupPresentation::new()
                            .rename_placeholder(TabStripText::new("rename"))
                            .entry(TabStripMenuEntry::action(
                                TabStripText::new("Ungroup"),
                                TabStripText::new("Ungroup"),
                                TabStripMenuOperation::Ungroup,
                            )),
                    )
                    .capabilities(TabStripGroupCapabilities::new().collapsible(true)),
                )
                .tab(
                    TabStripTabDescriptor::new(
                        TabStripTabTarget::from_opaque_bytes(b"tab"),
                        TabStripText::new("tab"),
                    )
                    .capabilities(TabStripTabCapabilities::new().selectable(true)),
                );
        let routes = TabStripRouteTable::from_projection(&projection);

        assert!(matches!(
            routes.rename_proposal_for(
                "root-group-0-popup-rename",
                TabStripText::new("renamed-group"),
            ),
            Some((_, TabStripProposalOperation::RenameGroup { .. }))
        ));
        assert!(
            routes
                .rename_proposal_for("root-tab-0-label", TabStripText::new("ignored"))
                .is_none()
        );
        assert!(matches!(
            routes.proposal_for("root-group-0-popup-0"),
            Some((_, TabStripProposalOperation::Ungroup(_)))
        ));
    }

    #[test]
    fn nested_menu_entries_are_inserted_recursively_for_set_pinned_and_group_popups() {
        let projection =
            TabStripProjection::new(4, TabStripCorrelation::from_opaque_bytes(b"correlation"))
                .tab(
                    TabStripTabDescriptor::new(
                        TabStripTabTarget::from_opaque_bytes(b"tab"),
                        TabStripText::new("tab"),
                    )
                    .capabilities(TabStripTabCapabilities::new().selectable(true))
                    .context_menu(
                        TabStripContextMenuPresentation::new().entry(
                            TabStripMenuEntry::submenu(
                                TabStripText::new("pin-state"),
                                TabStripText::new("pin-state"),
                            )
                            .child(TabStripMenuEntry::action(
                                TabStripText::new("Pin state"),
                                TabStripText::new("Pin state"),
                                TabStripMenuOperation::SetPinned(false),
                            )),
                        ),
                    ),
                )
                .group(
                    TabStripGroupDescriptor::new(
                        TabStripGroupTarget::from_opaque_bytes(b"group"),
                        TabStripText::new("group"),
                    )
                    .popup(
                        TabStripGroupPopupPresentation::new().entry(
                            TabStripMenuEntry::submenu(
                                TabStripText::new("nested"),
                                TabStripText::new("nested"),
                            )
                            .child(
                                TabStripMenuEntry::action(
                                    TabStripText::new("Ungroup"),
                                    TabStripText::new("Ungroup"),
                                    TabStripMenuOperation::Ungroup,
                                )
                                .child(TabStripMenuEntry::action(
                                    TabStripText::new("Remove"),
                                    TabStripText::new("Remove"),
                                    TabStripMenuOperation::RemoveFromGroup,
                                )),
                            ),
                        ),
                    )
                    .capabilities(TabStripGroupCapabilities::new().collapsible(true)),
                );
        let routes = TabStripRouteTable::from_projection(&projection);

        assert!(matches!(
            routes.proposal_for("root-tab-0-menu-0-0"),
            Some((
                _,
                TabStripProposalOperation::SetPinned { pinned: false, .. }
            ))
        ));
        assert!(matches!(
            routes.proposal_for("root-group-0-popup-0-0"),
            Some((_, TabStripProposalOperation::Ungroup(_)))
        ));
        assert!(routes.proposal_for("root-group-0-popup-0-0-0").is_none());
    }

    #[test]
    #[should_panic(expected = "rename proposals require their one-shot name")]
    fn route_proposal_for_rename_is_contractual_panic() {
        let route =
            super::TabStripRoute::RenameGroup(TabStripGroupTarget::from_opaque_bytes(b"group"));
        let _ = route.proposal();
    }

    #[test]
    fn response_id_resolves_only_the_current_frame_structural_route() {
        let projection =
            TabStripProjection::new(9, TabStripCorrelation::from_opaque_bytes(b"correlation")).tab(
                TabStripTabDescriptor::new(
                    TabStripTabTarget::from_opaque_bytes(b"tab"),
                    TabStripText::new("label"),
                )
                .capabilities(TabStripTabCapabilities::new().selectable(true)),
            );
        let mut routes = TabStripRouteTable::from_projection(&projection);
        let response_id = egui::Id::new("response");
        routes.register_response(
            "root-tab-0-label",
            response_id,
            UiRect::new(0, 0, 20, 20),
            "label",
            false,
        );

        assert!(matches!(
            routes.proposal_for_response(response_id),
            Some((_, TabStripProposalOperation::SelectTab(_)))
        ));
        routes.begin_frame();
        assert!(routes.proposal_for_response(response_id).is_none());
    }

    #[test]
    fn disabled_response_route_cannot_produce_a_proposal() {
        let projection =
            TabStripProjection::new(9, TabStripCorrelation::from_opaque_bytes(b"correlation")).tab(
                TabStripTabDescriptor::new(
                    TabStripTabTarget::from_opaque_bytes(b"tab"),
                    TabStripText::new("label"),
                )
                .capabilities(TabStripTabCapabilities::new().selectable(true)),
            );
        let mut routes = TabStripRouteTable::from_projection(&projection);
        let response_id = egui::Id::new("disabled-response");
        routes.register_response(
            "root-tab-0-label",
            response_id,
            UiRect::new(0, 0, 20, 20),
            "label",
            true,
        );

        assert!(routes.proposal_for_response(response_id).is_none());
    }

    #[test]
    fn overlay_menu_routes_keep_parent_targets_opaque_and_reject_wrong_contexts() {
        let destination_group = TabStripGroupTarget::from_opaque_bytes(b"destination-group");
        let recolor_swatch = TabStripSwatchTarget::from_opaque_bytes(b"recolor-swatch");
        let tab_menu = TabStripContextMenuPresentation::new()
            .entry(TabStripMenuEntry::action(
                TabStripText::new("Close"),
                TabStripText::new("Close tab"),
                TabStripMenuOperation::RequestClose,
            ))
            .entry(TabStripMenuEntry::action(
                TabStripText::new("Close others"),
                TabStripText::new("Close other tabs"),
                TabStripMenuOperation::CloseOthers,
            ))
            .entry(TabStripMenuEntry::action(
                TabStripText::new("Close all"),
                TabStripText::new("Close all tabs"),
                TabStripMenuOperation::CloseAll,
            ))
            .entry(TabStripMenuEntry::action(
                TabStripText::new("Close left"),
                TabStripText::new("Close tabs to the left"),
                TabStripMenuOperation::CloseToLeft,
            ))
            .entry(TabStripMenuEntry::action(
                TabStripText::new("Close right"),
                TabStripText::new("Close tabs to the right"),
                TabStripMenuOperation::CloseToRight,
            ))
            .entry(TabStripMenuEntry::action(
                TabStripText::new("Restore"),
                TabStripText::new("Restore closed tab"),
                TabStripMenuOperation::RestoreClosed,
            ))
            .entry(TabStripMenuEntry::action(
                TabStripText::new("Pin"),
                TabStripText::new("Pin tab"),
                TabStripMenuOperation::SetPinned(true),
            ))
            .entry(TabStripMenuEntry::action(
                TabStripText::new("Create group"),
                TabStripText::new("Create group from tab"),
                TabStripMenuOperation::CreateGroup,
            ))
            .entry(
                TabStripMenuEntry::submenu(
                    TabStripText::new("Add to group"),
                    TabStripText::new("Add tab to group"),
                )
                .child(TabStripMenuEntry::action(
                    TabStripText::new("Target group"),
                    TabStripText::new("Move tab to target group"),
                    TabStripMenuOperation::MoveToGroup(destination_group),
                )),
            )
            .entry(TabStripMenuEntry::action(
                TabStripText::new("Remove from group"),
                TabStripText::new("Remove tab from group"),
                TabStripMenuOperation::RemoveFromGroup,
            ))
            .entry(TabStripMenuEntry::action(
                TabStripText::new("Invalid tab entry"),
                TabStripText::new("Must not route"),
                TabStripMenuOperation::Ungroup,
            ));
        let group_popup = TabStripGroupPopupPresentation::new()
            .entry(TabStripMenuEntry::action(
                TabStripText::new("Ungroup"),
                TabStripText::new("Ungroup tabs"),
                TabStripMenuOperation::Ungroup,
            ))
            .entry(TabStripMenuEntry::action(
                TabStripText::new("Close group"),
                TabStripText::new("Close group"),
                TabStripMenuOperation::CloseGroup,
            ))
            .entry(TabStripMenuEntry::action(
                TabStripText::new("Green"),
                TabStripText::new("Use green group color"),
                TabStripMenuOperation::Recolor(recolor_swatch),
            ))
            .entry(TabStripMenuEntry::action(
                TabStripText::new("Invalid group entry"),
                TabStripText::new("Must not route"),
                TabStripMenuOperation::CloseAll,
            ));
        let projection =
            TabStripProjection::new(11, TabStripCorrelation::from_opaque_bytes(b"correlation"))
                .tab(
                    TabStripTabDescriptor::new(
                        TabStripTabTarget::from_opaque_bytes(b"tab"),
                        TabStripText::new("tab"),
                    )
                    .context_menu(tab_menu),
                )
                .group(
                    TabStripGroupDescriptor::new(
                        TabStripGroupTarget::from_opaque_bytes(b"group"),
                        TabStripText::new("group"),
                    )
                    .popup(group_popup)
                    .swatch(TabStripSwatchDescriptor::new(
                        TabStripSwatchTarget::from_opaque_bytes(b"palette-swatch"),
                        katana_ui_core::molecule::RgbaColor::new(74, 144, 217, 255),
                    )),
                );
        let routes = TabStripRouteTable::from_projection(&projection);

        assert!(matches!(
            routes.proposal_for("root-tab-0-menu-0"),
            Some((_, TabStripProposalOperation::RequestClose(_)))
        ));
        assert!(matches!(
            routes.proposal_for("root-tab-0-menu-1"),
            Some((_, TabStripProposalOperation::CloseOthers(_)))
        ));
        assert!(matches!(
            routes.proposal_for("root-tab-0-menu-2"),
            Some((_, TabStripProposalOperation::CloseAll))
        ));
        assert!(matches!(
            routes.proposal_for("root-tab-0-menu-3"),
            Some((_, TabStripProposalOperation::CloseToLeft(_)))
        ));
        assert!(matches!(
            routes.proposal_for("root-tab-0-menu-4"),
            Some((_, TabStripProposalOperation::CloseToRight(_)))
        ));
        assert!(matches!(
            routes.proposal_for("root-tab-0-menu-5"),
            Some((_, TabStripProposalOperation::RestoreClosed))
        ));
        assert!(matches!(
            routes.proposal_for("root-tab-0-menu-6"),
            Some((_, TabStripProposalOperation::SetPinned { pinned: true, .. }))
        ));
        assert!(matches!(
            routes.proposal_for("root-tab-0-menu-7"),
            Some((_, TabStripProposalOperation::CreateGroup(_)))
        ));
        assert!(matches!(
            routes.proposal_for("root-tab-0-menu-8-0"),
            Some((_, TabStripProposalOperation::MoveTabToGroup { .. }))
        ));
        assert!(matches!(
            routes.proposal_for("root-tab-0-menu-9"),
            Some((_, TabStripProposalOperation::RemoveFromGroup(_)))
        ));
        assert!(routes.proposal_for("root-tab-0-menu-10").is_none());
        assert!(matches!(
            routes.proposal_for("root-group-0-popup-0"),
            Some((_, TabStripProposalOperation::Ungroup(_)))
        ));
        assert!(matches!(
            routes.proposal_for("root-group-0-popup-1"),
            Some((_, TabStripProposalOperation::CloseGroup(_)))
        ));
        assert!(matches!(
            routes.proposal_for("root-group-0-popup-2"),
            Some((_, TabStripProposalOperation::RecolorGroup { .. }))
        ));
        assert!(routes.proposal_for("root-group-0-popup-3").is_none());
        assert!(matches!(
            routes.proposal_for("root-group-0-popup-swatch-0"),
            Some((_, TabStripProposalOperation::RecolorGroup { .. }))
        ));
    }
}
