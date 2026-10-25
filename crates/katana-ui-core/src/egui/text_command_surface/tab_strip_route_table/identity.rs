use super::super::tab_strip_projection_lease::{TabStripProjection, TabStripRootItem};
use super::{TabStripGroupDescriptor, TabStripRoute, TabStripRouteTable};
use std::collections::BTreeMap;

pub(super) fn projection_identities(projection: &TabStripProjection) -> BTreeMap<String, egui::Id> {
    let mut identities = BTreeMap::new();
    for (index, tab) in projection.tabs.iter().enumerate() {
        identities.insert(
            format!("root-tab-{index}"),
            egui::Id::new(("tab", &tab.target.payload)),
        );
    }
    for (index, group) in projection.groups.iter().enumerate() {
        group_identities(&mut identities, group, format!("root-group-{index}"));
    }
    if !projection.root_items.is_empty() {
        let mut tab_occurrence = 0;
        let mut group_occurrence = 0;
        for item in &projection.root_items {
            match item {
                TabStripRootItem::Tab(index) => {
                    if let Some(tab) = projection.tabs.get(*index) {
                        identities.insert(
                            format!("root-tab-{tab_occurrence}"),
                            egui::Id::new(("tab", &tab.target.payload)),
                        );
                    }
                    tab_occurrence += 1;
                }
                TabStripRootItem::Group(index) => {
                    if let Some(group) = projection.groups.get(*index) {
                        group_identities(
                            &mut identities,
                            group,
                            format!("root-group-{group_occurrence}"),
                        );
                    }
                    group_occurrence += 1;
                }
            }
        }
    }
    identities
}

fn group_identities(
    identities: &mut BTreeMap<String, egui::Id>,
    group: &TabStripGroupDescriptor,
    path: String,
) {
    identities.insert(
        path.clone(),
        egui::Id::new(("group", &group.target.payload)),
    );
    for (index, tab) in group.tabs.iter().enumerate() {
        identities.insert(
            format!("{path}-tab-{index}"),
            egui::Id::new(("tab", &tab.target.payload)),
        );
    }
    for (index, child) in group.groups.iter().enumerate() {
        group_identities(identities, child, format!("{path}-group-{index}"));
    }
}

impl TabStripRouteTable {
    pub(in super::super) fn interaction_id(&self, parent: egui::Id, path: &str) -> egui::Id {
        /* WHY: occurrenceが再利用されても、失効targetへのAX入力を別targetへ転用しない。 */
        let mut prefix = path;
        let owner = loop {
            if let Some(target) = self.identities.get(prefix) {
                break Some((prefix, target));
            }
            let Some((ancestor, _)) = prefix.rsplit_once('-') else {
                break None;
            };
            prefix = ancestor;
        };
        let identity = owner.map_or_else(
            || parent.with(path),
            |(prefix, target)| parent.with((target, &path[prefix.len()..])),
        );
        self.routes
            .get(path)
            .map_or(identity, |route| identity.with(route.identity()))
    }
}

impl TabStripRoute {
    fn identity(&self) -> egui::Id {
        let kind = std::mem::discriminant(self);
        match self {
            Self::SelectTab(target)
            | Self::RequestClose(target)
            | Self::CloseOthers(target)
            | Self::CloseToLeft(target)
            | Self::CloseToRight(target)
            | Self::Unpin(target)
            | Self::Pin(target)
            | Self::CreateGroup(target)
            | Self::RemoveFromGroup(target) => egui::Id::new((kind, &target.payload)),
            Self::SetGroupCollapsed { group, .. }
            | Self::RenameGroup(group)
            | Self::Ungroup(group)
            | Self::CloseGroup(group) => egui::Id::new((kind, &group.payload)),
            Self::MoveTabToGroup { tab, group } => {
                egui::Id::new((kind, &tab.payload, &group.payload))
            }
            Self::RecolorGroup { group, swatch } => {
                egui::Id::new((kind, &group.payload, &swatch.payload))
            }
            Self::Previous
            | Self::Next
            | Self::OpenOverflow
            | Self::CloseAll
            | Self::RestoreClosed => egui::Id::new(kind),
        }
    }
}
