use super::super::tab_strip_proposal_port::{
    TabStripProposal, TabStripProposalPort, TabStripProposalPortError, TabStripProposalPortHandle,
};
use super::*;
use crate::egui::text_command_surface::{
    TabStripCorrelation, TabStripGroupCapabilities, TabStripGroupDescriptor,
    TabStripGroupPlacement, TabStripProjection, TabStripProjectionLease, TabStripTabDescriptor,
    TabStripTabTarget, TabStripText,
};
use crate::text_raster::{PlatformFontCatalog, PlatformTextRasterConfig};
use std::cell::RefCell;
use std::sync::Arc;

struct RecordingPort {
    proposals: std::rc::Rc<RefCell<usize>>,
}

impl TabStripProposalPort for RecordingPort {
    fn forward_proposal(
        &mut self,
        _proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        *self.proposals.borrow_mut() += 1;
        Ok(())
    }
}

#[test]
fn ongoing_group_drag_skips_self_and_tab_candidates_and_keeps_feedback() {
    let source = TabStripGroupTarget::from_opaque_bytes(b"source");
    let target = TabStripGroupTarget::from_opaque_bytes(b"target");
    let tab = TabStripTabTarget::from_opaque_bytes(b"tab");
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(
            TabStripGroupDescriptor::new(source.copy_for_route(), TabStripText::new("Source"))
                .capabilities(TabStripGroupCapabilities::new().draggable(true)),
        )
        .group(TabStripGroupDescriptor::new(
            target.copy_for_route(),
            TabStripText::new("Target"),
        ))
        .tab(TabStripTabDescriptor::new(
            tab.copy_for_route(),
            TabStripText::new("Tab"),
        ));
    let config = PlatformTextRasterConfig::default();
    let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
    let mut state = TabStripRetainedState::from_lease(
        TabStripProjectionLease::new(projection),
        catalog,
        config,
    )
    .expect("retained state");
    let candidate_bounds = egui::Rect::from_min_size(egui::pos2(20.0, 0.0), egui::vec2(40.0, 20.0));
    state.group_drag_candidates = [
        TabStripDropCandidateKind::Group(source.copy_for_route()),
        TabStripDropCandidateKind::Tab(tab),
        TabStripDropCandidateKind::Group(target.copy_for_route()),
    ]
    .into_iter()
    .map(|kind| super::super::TabStripDropCandidate {
        bounds: candidate_bounds,
        kind,
    })
    .collect();
    let pointer = egui::pos2(58.0, 10.0);
    let strip = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(120.0, 20.0));
    state.group_drag = Some(super::super::TabStripGroupDragState {
        source: source.copy_for_route(),
        label: TabStripText::new("Source"),
        pointer,
    });
    let context = egui::Context::default();
    let mut operations = Vec::new();
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .resolve_group_drag(ui, strip, &mut operations)
            .expect("ongoing drag");
    });
    output.textures_delta.clear();
    assert!(!operations.is_empty());
    assert_eq!(
        state.group_drag.as_ref().map(|drag| &drag.source),
        Some(&source)
    );
    let drop = state
        .resolve_group_drop(pointer, &source, strip)
        .expect("destination");
    assert!(matches!(drop.destination, TabStripGroupPlacement::After(value) if value == target));
}

#[test]
fn ongoing_group_drag_propagates_real_ghost_raster_failure_without_forwarding() {
    let source = TabStripGroupTarget::from_opaque_bytes(b"source-empty-label");
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(
            TabStripGroupDescriptor::new(source.copy_for_route(), TabStripText::new(""))
                .capabilities(TabStripGroupCapabilities::new().draggable(true)),
        );
    let config = PlatformTextRasterConfig::default();
    let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
    let mut state = TabStripRetainedState::from_lease(
        TabStripProjectionLease::new(projection),
        catalog,
        config,
    )
    .expect("retained state");
    let proposals = std::rc::Rc::new(RefCell::new(0));
    state.port = Some(TabStripProposalPortHandle::new(RecordingPort {
        proposals: std::rc::Rc::clone(&proposals),
    }));
    state.group_drag = Some(super::super::TabStripGroupDragState {
        source,
        label: TabStripText::new(""),
        pointer: egui::pos2(20.0, 10.0),
    });
    let context = egui::Context::default();
    let strip = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(120.0, 24.0));
    let mut operations = Vec::new();
    let mut result = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        result = Some(state.resolve_group_drag(ui, strip, &mut operations));
    });
    output.textures_delta.clear();
    assert!(matches!(
        result,
        Some(Err(TabStripRetainedError::Raster(_)))
    ));
    assert!(state.group_drag.is_none());
    assert_eq!(
        *proposals.borrow(),
        0,
        "failed feedback must not forward a proposal"
    );
    assert!(operations.is_empty());
}

#[test]
fn ongoing_group_drag_finishes_on_pointer_release_when_response_disappears() {
    let source = TabStripGroupTarget::from_opaque_bytes(b"source-release");
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(
            TabStripGroupDescriptor::new(source.copy_for_route(), TabStripText::new("Source"))
                .capabilities(TabStripGroupCapabilities::new().draggable(true)),
        );
    let config = PlatformTextRasterConfig::default();
    let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
    let mut state = TabStripRetainedState::from_lease(
        TabStripProjectionLease::new(projection),
        catalog,
        config,
    )
    .expect("retained state");
    let proposals = std::rc::Rc::new(RefCell::new(0));
    state.port = Some(TabStripProposalPortHandle::new(RecordingPort {
        proposals: std::rc::Rc::clone(&proposals),
    }));
    state.group_drag = Some(super::super::TabStripGroupDragState {
        source,
        label: TabStripText::new("Source"),
        pointer: egui::pos2(20.0, 10.0),
    });
    let context = egui::Context::default();
    let strip = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(120.0, 24.0));
    let mut operations = Vec::new();
    let mut output = context.run_ui(
        egui::RawInput {
            events: vec![egui::Event::PointerButton {
                pos: egui::pos2(20.0, 10.0),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        },
        |ui| {
            state
                .resolve_group_drag(ui, strip, &mut operations)
                .expect("pointer release should cancel stale group drag");
        },
    );
    output.textures_delta.clear();
    assert_eq!(*proposals.borrow(), 1);
    assert!(state.group_drag.is_none());
}

#[test]
fn middle_of_non_source_group_candidate_has_no_drop_destination() {
    let source = TabStripGroupTarget::from_opaque_bytes(b"source");
    let target = TabStripGroupTarget::from_opaque_bytes(b"target");
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(TabStripGroupDescriptor::new(
            source.copy_for_route(),
            TabStripText::new("Source"),
        ))
        .group(TabStripGroupDescriptor::new(
            target.copy_for_route(),
            TabStripText::new("Target"),
        ));
    let config = PlatformTextRasterConfig::default();
    let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
    let mut state = TabStripRetainedState::from_lease(
        TabStripProjectionLease::new(projection),
        catalog,
        config,
    )
    .expect("retained state");
    state.group_drag_candidates = vec![super::super::TabStripDropCandidate {
        bounds: egui::Rect::from_min_max(egui::pos2(20.0, 0.0), egui::pos2(60.0, 20.0)),
        kind: TabStripDropCandidateKind::Group(target.copy_for_route()),
    }];
    let strip = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(120.0, 20.0));
    assert!(
        state
            .resolve_group_drop(egui::pos2(40.0, 10.0), &source, strip)
            .is_none()
    );
}
