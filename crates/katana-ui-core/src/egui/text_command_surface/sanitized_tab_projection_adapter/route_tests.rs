use super::SanitizedTabProjectionRouteTable;
use crate::egui::text_command_surface::sanitized_document_root::sanitized_tab_projection::{
    SanitizedTabGroupTarget, SanitizedTabTarget,
};
use crate::molecule::structured::{
    CloseableTabGroupId, CloseableTabId, CloseableTabStripEvent,
};

#[test]
fn tab_events_do_not_route_through_a_group_entry() {
    let mut routes = SanitizedTabProjectionRouteTable::default();
    routes.insert_group(
        "group".to_owned(),
        &SanitizedTabGroupTarget::from_opaque_bytes([1]),
    );

    assert!(routes
        .route_event(&CloseableTabStripEvent::TabSelected {
            tab_id: CloseableTabId::new("group"),
        })
        .is_none());
    assert!(routes
        .route_event(&CloseableTabStripEvent::TabCloseRequested {
            tab_id: CloseableTabId::new("group"),
        })
        .is_none());
}

#[test]
fn group_events_do_not_route_through_a_tab_entry() {
    let mut routes = SanitizedTabProjectionRouteTable::default();
    routes.insert_tab(
        "tab".to_owned(),
        &SanitizedTabTarget::from_opaque_bytes([2]),
    );

    assert!(routes
        .route_event(&CloseableTabStripEvent::GroupCollapseChanged {
            group_id: CloseableTabGroupId::new("tab"),
            collapsed: true,
        })
        .is_none());
}
