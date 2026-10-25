use super::*;
use crate::egui::text_command_surface::{
    TabStripControlPresentation, TabStripCorrelation, TabStripGroupDescriptor, TabStripGroupTarget,
    TabStripNavigationPresentation, TabStripProjection, TabStripProjectionLease,
    TabStripScrollPresentation, TabStripSurfaceCapabilities, TabStripTabCapabilities,
    TabStripTabDescriptor, TabStripTabTarget, TabStripText,
};
use std::sync::Arc;

pub(super) const RETAINED_TEST_VIEWPORT_WIDTH_PX: f32 = 260.0;
pub(super) const RETAINED_TEST_VIEWPORT_HEIGHT_PX: f32 = 80.0;

pub(super) fn retained_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        events,
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(
                RETAINED_TEST_VIEWPORT_WIDTH_PX,
                RETAINED_TEST_VIEWPORT_HEIGHT_PX,
            ),
        )),
        ..Default::default()
    }
}

pub(super) fn accesskit_click(node: egui::accesskit::NodeId) -> egui::Event {
    egui::Event::AccessKitActionRequest(egui::accesskit::ActionRequest {
        action: egui::accesskit::Action::Click,
        target_tree: egui::accesskit::TreeId::ROOT,
        target_node: node,
        data: None,
    })
}

pub(super) fn accesskit_button(output: &egui::FullOutput, label: &str) -> egui::accesskit::NodeId {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .and_then(|update| {
            update.nodes.iter().find_map(|(node_id, node)| {
                (node.role() == egui::accesskit::Role::Button && node.label() == Some(label))
                    .then_some(*node_id)
            })
        })
        .unwrap_or_else(|| panic!("AccessKit button should be published: {label}"))
}

pub(super) fn build_state_with_projection(projection: TabStripProjection) -> TabStripRetainedState {
    let lease = TabStripProjectionLease::new(projection);
    let config = crate::text_raster::PlatformTextRasterConfig::default();
    let catalog = Arc::new(crate::text_raster::PlatformFontCatalog::new(
        config.catalog_policy(),
    ));
    TabStripRetainedState::from_lease(lease, catalog, config)
        .expect("tab strip retained state should be constructible")
}

pub(super) fn build_projection_with_reveal() -> TabStripProjection {
    TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"tab"),
                TabStripText::new("tab"),
            )
            .capabilities(TabStripTabCapabilities::new().selectable(true).active(true)),
        )
        .scroll_presentation(TabStripScrollPresentation::new().request_active_reveal(true))
}

pub(super) fn build_projection_with_navigation() -> TabStripProjection {
    let previous = TabStripControlPresentation::new(
        TabStripText::new("previous"),
        TabStripText::new("previous"),
    );
    let next =
        TabStripControlPresentation::new(TabStripText::new("next"), TabStripText::new("next"));
    TabStripProjection::new(2, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"tab-nav"),
                TabStripText::new("tab"),
            )
            .capabilities(TabStripTabCapabilities::new().selectable(true)),
        )
        .group(TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"group-nav"),
            TabStripText::new("group"),
        ))
        .capabilities(
            TabStripSurfaceCapabilities::new()
                .previous_available(true)
                .next_available(true)
                .tab_drop_at_end_available(true),
        )
        .navigation(TabStripNavigationPresentation::new(previous, next))
}
