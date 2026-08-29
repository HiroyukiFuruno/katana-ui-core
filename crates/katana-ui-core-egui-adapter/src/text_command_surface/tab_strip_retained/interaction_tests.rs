use super::*;
use crate::text_command_surface::tab_strip_retained::TabStripDropCandidate;
use crate::text_command_surface::{
    TabStripCorrelation, TabStripGroupDescriptor, TabStripGroupTarget, TabStripProjection,
    TabStripProjectionLease, TabStripProposalPort, TabStripProposalPortError,
    TabStripProposalPortHandle, TabStripTabCapabilities, TabStripTabDescriptor,
    TabStripTabPlacement, TabStripTabTarget, TabStripText,
};
use std::sync::Arc;

fn build_state_with_projection(projection: TabStripProjection) -> TabStripRetainedState {
    let lease = TabStripProjectionLease::new(projection);
    let config = katana_ui_core_text_raster::PlatformTextRasterConfig::default();
    let catalog = Arc::new(katana_ui_core_text_raster::PlatformFontCatalog::new(
        config.catalog_policy(),
    ));
    TabStripRetainedState::from_lease(lease, catalog, config)
        .expect("tab strip retained state should be constructible")
}

struct CountingPort;

impl TabStripProposalPort for CountingPort {
    fn forward_proposal(
        &mut self,
        proposal: crate::text_command_surface::tab_strip_proposal_port::TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        let _ = proposal.nonce();
        Ok(())
    }
}

#[test]
fn forward_route_is_fail_closed_when_response_is_stale_or_missing() {
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .tab(
            TabStripTabDescriptor::new(
                TabStripTabTarget::from_opaque_bytes(b"tab"),
                TabStripText::new("tab"),
            )
            .capabilities(TabStripTabCapabilities::new().selectable(true)),
        );
    let mut state = build_state_with_projection(projection);
    state.port = Some(TabStripProposalPortHandle::new(CountingPort));
    let response_id = egui::Id::new("tab-response");
    state.routes.register_response(
        "root-tab-0-label",
        response_id,
        ui_rect(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(10.0, 10.0),
        )),
        "tab",
        false,
    );
    assert!(state.forward_response_route(response_id).is_ok());
    state.routes.begin_frame();
    assert!(matches!(
        state.forward_response_route(response_id),
        Err(TabStripRetainedError::MissingRoute)
    ));
}

#[test]
fn start_tab_drag_is_fail_closed_without_port_and_reuses_existing_drag_state() {
    let tab = TabStripTabDescriptor::new(
        TabStripTabTarget::from_opaque_bytes(b"drag"),
        TabStripText::new("drag"),
    );
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    let context = egui::Context::default();
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(10.0, 10.0));
        assert!(matches!(
            state.start_tab_drag(&tab, bounds, ui),
            Err(TabStripRetainedError::MissingPort)
        ));
        state.drag = Some(TabStripDragState {
            source: tab.target.copy_for_route(),
            label: TabStripText::new("drag"),
            pointer: bounds.center(),
        });
        assert!(state.start_tab_drag(&tab, bounds, ui).is_ok());
    });
    platform_output.textures_delta.clear();
}

#[test]
fn forward_rename_route_is_fail_closed_without_group_popup_path() {
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(TabStripGroupDescriptor::new(
            TabStripGroupTarget::from_opaque_bytes(b"no-popup"),
            TabStripText::new("No popup"),
        ));
    let mut state = build_state_with_projection(projection);

    assert!(matches!(
        state.forward_rename_route("root-group-0-popup-rename", TabStripText::new("renamed")),
        Err(TabStripRetainedError::MissingRoute)
    ));
    assert!(
        state
            .forward_rename_route("missing", TabStripText::new("renamed"))
            .is_err()
    );
}

#[test]
fn resolve_tab_drop_chooses_none_for_same_target_and_end_drop_when_available() {
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    let source = TabStripTabTarget::from_opaque_bytes(b"source");

    state.drag_candidates = vec![TabStripDropCandidate {
        bounds: egui::Rect::from_min_size(egui::pos2(10.0, 0.0), egui::vec2(20.0, 10.0)),
        kind: TabStripDropCandidateKind::Tab(source.copy_for_route()),
    }];
    assert!(
        state
            .resolve_tab_drop(
                egui::pos2(20.0, 5.0),
                &source,
                egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(100.0, 20.0)),
                false,
            )
            .is_none()
    );

    state.drag_candidates.push(TabStripDropCandidate {
        bounds: egui::Rect::from_min_size(egui::pos2(60.0, 0.0), egui::vec2(10.0, 10.0)),
        kind: TabStripDropCandidateKind::Group(TabStripGroupTarget::from_opaque_bytes(
            b"target-group",
        )),
    });
    assert!(matches!(
        state.resolve_tab_drop(
            egui::pos2(60.0, 0.0),
            &source,
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(120.0, 20.0)),
            true,
        ),
        Some(value) if matches!(
            value.destination,
            TabStripTabPlacement::InGroup(_)
        )
    ));

    let end_drop = state
        .resolve_tab_drop(
            egui::pos2(110.0, 5.0),
            &source,
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(120.0, 20.0)),
            true,
        )
        .expect("end-of-strip drop should be available");
    assert!(matches!(
        end_drop.destination,
        TabStripTabPlacement::EndOfStrip
    ));
}
