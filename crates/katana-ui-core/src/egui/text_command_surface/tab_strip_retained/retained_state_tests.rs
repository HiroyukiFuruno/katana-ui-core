use super::*;
use crate::egui::text_command_surface::{
    TabStripControlPresentation, TabStripCorrelation, TabStripGroupDescriptor, TabStripGroupTarget,
    TabStripNavigationPresentation, TabStripProjection, TabStripProjectionLease,
    TabStripScrollPresentation, TabStripSurfaceCapabilities, TabStripTabCapabilities,
    TabStripTabDescriptor, TabStripTabTarget, TabStripText,
};
use std::sync::Arc;

const RETAINED_TEST_VIEWPORT_WIDTH_PX: f32 = 260.0;
const RETAINED_TEST_VIEWPORT_HEIGHT_PX: f32 = 80.0;

#[path = "retained_state_tests/navigation.rs"]
mod navigation;

fn retained_input(events: Vec<egui::Event>) -> egui::RawInput {
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

fn accesskit_click(node: egui::accesskit::NodeId) -> egui::Event {
    egui::Event::AccessKitActionRequest(egui::accesskit::ActionRequest {
        action: egui::accesskit::Action::Click,
        target_tree: egui::accesskit::TreeId::ROOT,
        target_node: node,
        data: None,
    })
}

fn accesskit_button(output: &egui::FullOutput, label: &str) -> egui::accesskit::NodeId {
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

fn build_state_with_projection(projection: TabStripProjection) -> TabStripRetainedState {
    let lease = TabStripProjectionLease::new(projection);
    let config = crate::text_raster::PlatformTextRasterConfig::default();
    let catalog = Arc::new(crate::text_raster::PlatformFontCatalog::new(
        config.catalog_policy(),
    ));
    TabStripRetainedState::from_lease(lease, catalog, config)
        .expect("tab strip retained state should be constructible")
}

fn build_projection_with_reveal() -> TabStripProjection {
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

fn build_projection_with_navigation() -> TabStripProjection {
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

#[test]
fn show_updates_projection_and_clears_active_reveal_when_rendered() {
    let projection = build_projection_with_reveal();
    let mut state = build_state_with_projection(projection);

    let context = egui::Context::default();
    let mut output = None;
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        output = Some(
            state
                .show(ui)
                .expect("projection should render without proposal port"),
        );
    });
    platform_output.textures_delta.clear();

    let output = output.expect("show should emit output");
    assert!(!state.active_reveal_pending);
    assert!(output.paint_plan.operations.len() >= 2);
    assert!(matches!(state.overlay, TabStripOverlayState::Closed));
    assert_eq!(state.projection.revision, 1);
}

#[test]
fn show_projection_renders_navigation_and_preserves_scroll_state() {
    let projection = build_projection_with_navigation();
    let mut state = build_state_with_projection(projection);

    let context = egui::Context::default();
    let mut output = None;
    let mut active_reveal_pending = false;
    let mut platform_output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(
                    RETAINED_TEST_VIEWPORT_WIDTH_PX,
                    RETAINED_TEST_VIEWPORT_HEIGHT_PX,
                ),
            )),
            ..Default::default()
        },
        |ui| {
            output = Some(
                state
                    .show_projection(
                        ui,
                        &build_projection_with_navigation(),
                        &mut active_reveal_pending,
                        0.0,
                    )
                    .expect("projection render path should not fail"),
            );
        },
    );
    platform_output.textures_delta.clear();

    let output = output.expect("show_projection should emit output");
    assert!(matches!(state.overlay, TabStripOverlayState::Closed));
    assert!(!active_reveal_pending);
    assert!(!output.paint_plan.operations.is_empty());
    assert!(output.overlay_paint_plan.is_none());
    assert_eq!(output.horizontal_scroll_offset, 0.0);
}

#[test]
fn show_preserves_retained_projection_and_scroll_when_real_tab_raster_fails() {
    let mut state = build_state_with_projection(build_projection_with_reveal());
    let context = egui::Context::default();
    let mut output = context.run_ui(retained_input(Vec::new()), |ui| {
        state
            .show(ui)
            .expect("valid projection should warm the retained renderer");
    });
    output.textures_delta.clear();

    state.horizontal_scroll_offset = 7.0;
    state.projection =
        TabStripProjection::new(9, TabStripCorrelation::from_opaque_bytes(b"invalid-corr")).tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"empty-tab"),
                TabStripText::new(""),
            ),
        );
    let mut observed = None;
    let mut output = context.run_ui(retained_input(Vec::new()), |ui| {
        observed = Some(state.show(ui));
    });
    output.textures_delta.clear();

    assert!(matches!(
        observed.expect("invalid projection frame should execute"),
        Err(TabStripRetainedError::Raster(_))
    ));
    assert_eq!(state.projection.revision, 9);
    assert_eq!(state.horizontal_scroll_offset, 7.0);
}

#[test]
fn show_projection_propagates_real_tab_and_group_label_raster_failures() {
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    let context = egui::Context::default();
    let valid = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"valid"));
    let mut output = context.run_ui(retained_input(Vec::new()), |ui| {
        state
            .show_projection(ui, &valid, &mut false, 0.0)
            .expect("valid projection should warm the retained renderer");
    });
    output.textures_delta.clear();

    let invalid_tab =
        TabStripProjection::new(2, TabStripCorrelation::from_opaque_bytes(b"invalid-tab")).tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"empty-tab"),
                TabStripText::new(""),
            ),
        );
    let mut tab_result = None;
    let mut output = context.run_ui(retained_input(Vec::new()), |ui| {
        tab_result = Some(state.show_projection(ui, &invalid_tab, &mut false, 0.0));
    });
    output.textures_delta.clear();
    assert!(matches!(
        tab_result.expect("invalid tab frame should execute"),
        Err(TabStripRetainedError::Raster(_))
    ));

    let invalid_group =
        TabStripProjection::new(3, TabStripCorrelation::from_opaque_bytes(b"invalid-group")).group(
            TabStripGroupDescriptor::new(
                TabStripGroupTarget::from_opaque_bytes(b"empty-group"),
                TabStripText::new(""),
            ),
        );
    let mut group_result = None;
    let mut output = context.run_ui(retained_input(Vec::new()), |ui| {
        group_result = Some(state.show_projection(ui, &invalid_group, &mut false, 0.0));
    });
    output.textures_delta.clear();
    assert!(matches!(
        group_result.expect("invalid group frame should execute"),
        Err(TabStripRetainedError::Raster(_))
    ));
}

#[test]
fn show_projection_propagates_real_drag_release_without_a_port() {
    let projection =
        TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"drag-corr"));
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"drag-corr"),
    ));
    let context = egui::Context::default();
    let mut output = context.run_ui(retained_input(Vec::new()), |ui| {
        state
            .show_projection(ui, &projection, &mut false, 0.0)
            .expect("idle drag frame should render");
    });
    output.textures_delta.clear();

    state.drag = Some(super::super::TabStripDragState {
        source: TabStripTabTarget::from_opaque_bytes(b"drag-source"),
        label: TabStripText::new("drag"),
        pointer: egui::pos2(20.0, 10.0),
    });
    state.drag_release_pending = true;
    let mut observed = None;
    let mut output = context.run_ui(retained_input(Vec::new()), |ui| {
        observed = Some(state.show_projection(ui, &projection, &mut false, 0.0));
    });
    output.textures_delta.clear();

    assert!(matches!(
        observed.expect("drag release frame should execute"),
        Err(TabStripRetainedError::MissingPort)
    ));
}
