use super::super::{
    TabStripCorrelation, TabStripGroupDescriptor, TabStripGroupTarget, TabStripProjection,
    TabStripTabCapabilities, TabStripTabDescriptor, TabStripTabTarget, TabStripText,
};
use super::TabStripRouteTable;

#[test]
fn interaction_ids_follow_opaque_owner_and_route_identity() {
    let tab = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"tab-owner"),
        TabStripText::new("tab"),
    )
    .capabilities(TabStripTabCapabilities::new().selectable(true));
    let group = TabStripGroupDescriptor::new(
        TabStripGroupTarget::from_opaque_bytes(b"group-owner"),
        TabStripText::new("group"),
    )
    .tab(tab);
    let projection =
        TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr")).group(group);
    let routes = TabStripRouteTable::from_projection(&projection);
    let parent = egui::Id::new("tab-strip");

    let group_id = routes.interaction_id(parent, "root-group-0");
    let child_id = routes.interaction_id(parent, "root-group-0-tab-0-label");
    let unknown_id = routes.interaction_id(parent, "unresolved-path");
    assert_ne!(group_id, child_id);
    assert_ne!(child_id, unknown_id);
    assert_eq!(
        child_id,
        routes.interaction_id(parent, "root-group-0-tab-0-label")
    );
}
