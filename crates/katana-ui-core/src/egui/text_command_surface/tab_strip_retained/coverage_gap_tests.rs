use super::tab_strip_projection_lease::{
    TabStripCorrelation, TabStripGroupCapabilities, TabStripGroupPopupPresentation,
};
use super::tab_strip_proposal_port::{TabStripInlineRenameStarted, TabStripProposalPort};
use super::*;
use std::num::NonZeroUsize;

#[derive(Default)]
struct Seen {
    starts: Vec<TabStripGroupTarget>,
    proposals: usize,
}
struct Port(Rc<RefCell<Seen>>);
impl TabStripProposalPort for Port {
    fn inline_rename_started(
        &mut self,
        receipt: TabStripInlineRenameStarted,
    ) -> Result<(), TabStripProposalPortError> {
        self.0.borrow_mut().starts.push(receipt.group());
        Ok(())
    }
    fn forward_proposal(
        &mut self,
        _proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        self.0.borrow_mut().proposals += 1;
        Ok(())
    }
}
fn group(target: &TabStripGroupTarget, rename: bool) -> TabStripGroupDescriptor {
    let group = TabStripGroupDescriptor::new(target.clone(), TabStripText::new("Group"))
        .capabilities(TabStripGroupCapabilities::new().renamable(rename));
    if rename {
        group.popup(
            TabStripGroupPopupPresentation::new().rename_placeholder(TabStripText::new("Name")),
        )
    } else {
        group
    }
}
fn state(projection: TabStripProjection) -> TabStripRetainedState {
    let config = PlatformTextRasterConfig::default();
    let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy()));
    TabStripRetainedState::from_lease(TabStripProjectionLease::new(projection), catalog, config)
        .expect("state")
}
fn popup() -> TabStripOverlayState {
    TabStripOverlayState::GroupPopup {
        path: "root-group-0".into(),
        anchor: egui::Pos2::ZERO,
        submenu_path: Vec::new(),
        rename: None,
    }
}

#[test]
fn pending_receipts_wait_for_pass_and_invalidated_start_preserves_other_group() {
    let target = TabStripGroupTarget::from_opaque_bytes(b"target");
    let other = TabStripGroupTarget::from_opaque_bytes(b"other");
    let corr = TabStripCorrelation::from_opaque_bytes(b"correlation");
    let projection = TabStripProjection::new(1, corr.clone())
        .group(group(&target, true))
        .group(group(&other, true));
    let mut state = state(projection);
    let seen = Rc::new(RefCell::new(Seen::default()));
    let port = TabStripProposalPortHandle::new(Port(Rc::clone(&seen)));
    state.inline_rename_consumed = vec![target.clone(), other.clone()];
    state.overlay = popup();
    let context = egui::Context::default();
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        let commit = pass_commit::TabStripPassCommit::committed();
        commit.watch(ui.ctx());
        state.pending_rename_start = Some(TabStripPendingRenameStart {
            target: target.clone(),
            correlation: corr.clone(),
            port: port.clone(),
            commit: commit.clone(),
        });
        state.pending_rename_changes = Some(TabStripPendingRenameChanges {
            target: target.clone(),
            correlation: corr.clone(),
            port: port.clone(),
            commit,
            names: vec![TabStripText::new("changed")],
            before: Some(popup()),
        });
        state.flush_rename_start().expect("pending start");
        state.flush_rename_changes().expect("pending change");
        assert!(state.pending_rename_start.is_some());
        assert!(state.pending_rename_changes.is_some());
        assert!(seen.borrow().starts.is_empty());
        assert_eq!(seen.borrow().proposals, 0);
    });
    output.textures_delta.clear();
    state.projection = TabStripProjection::new(
        2,
        TabStripCorrelation::from_opaque_bytes(b"new correlation"),
    )
    .group(group(&other, true));
    state.flush_rename_start().expect("invalid start");
    state.flush_rename_changes().expect("invalid change");
    assert_eq!(state.inline_rename_consumed, vec![other]);
    assert!(matches!(state.overlay, TabStripOverlayState::Closed));
    assert!(seen.borrow().starts.is_empty());
    assert_eq!(seen.borrow().proposals, 0);
}

#[test]
fn discarded_change_never_restores_a_replaced_or_nonrenamable_target() {
    let target = TabStripGroupTarget::from_opaque_bytes(b"target");
    let corr = TabStripCorrelation::from_opaque_bytes(b"correlation");
    let mut state = state(TabStripProjection::new(1, corr.clone()).group(group(&target, true)));
    let seen = Rc::new(RefCell::new(Seen::default()));
    let port = TabStripProposalPortHandle::new(Port(Rc::clone(&seen)));
    let context = egui::Context::default();
    context.options_mut(|options| options.max_passes = NonZeroUsize::new(2).expect("passes"));
    let mut first = true;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        if first {
            first = false;
            let commit = pass_commit::TabStripPassCommit::committed();
            commit.watch(ui.ctx());
            state.pending_rename_changes = Some(TabStripPendingRenameChanges {
                target: target.clone(),
                correlation: corr.clone(),
                port: port.clone(),
                commit,
                names: vec![TabStripText::new("changed")],
                before: Some(popup()),
            });
            state.projection =
                TabStripProjection::new(2, corr.clone()).group(group(&target, false));
            ui.ctx().request_discard("replace group capability");
        } else {
            state.flush_rename_changes().expect("discarded change");
            assert!(matches!(state.overlay, TabStripOverlayState::Closed));
        }
    });
    output.textures_delta.clear();
    assert!(!first);
    assert!(state.pending_rename_changes.is_none());
    assert_eq!(seen.borrow().proposals, 0);
}

#[test]
fn requested_rename_preserves_another_groups_consumed_receipt() {
    let first = TabStripGroupTarget::from_opaque_bytes(b"first");
    let other = TabStripGroupTarget::from_opaque_bytes(b"other");
    let mut state = state(
        TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
            .group(group(&first, true)),
    );
    let seen = Rc::new(RefCell::new(Seen::default()));
    state.port = Some(TabStripProposalPortHandle::new(Port(Rc::clone(&seen))));
    state.inline_rename_consumed.push(other.clone());
    let context = egui::Context::default();
    for _ in 0..2 {
        let mut draft = TabStripRenameDraft::new("Group", "Name");
        draft.requested_start = true;
        let mut output = context.run_ui(egui::RawInput::default(), |ui| {
            state
                .render_group_popup_prefix(
                    ui,
                    "root-group-0",
                    egui::Pos2::ZERO,
                    &group(&first, true),
                    Some(Box::new(draft.clone())),
                )
                .expect("popup");
        });
        output.textures_delta.clear();
    }
    assert!(state.inline_rename_consumed.contains(&first));
    assert!(state.inline_rename_consumed.contains(&other));
    assert_eq!(state.inline_rename_consumed.len(), 2);
    assert!(state.pending_rename_start.is_some());
    state.flush_rename_start().expect("accepted start");
    assert_eq!(seen.borrow().starts, vec![first]);
}

#[test]
fn nearest_drop_filters_group_candidates_and_preserves_visible_partial_boundary() {
    let first = TabStripTabTarget::from_opaque_bytes(b"first");
    let hidden = TabStripTabTarget::from_opaque_bytes(b"hidden");
    let mut state = state(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    state.drag_candidates = vec![
        TabStripDropCandidate {
            kind: TabStripDropCandidateKind::Group(TabStripGroupTarget::from_opaque_bytes(
                b"group",
            )),
            bounds: egui::Rect::from_min_max(egui::pos2(-20.0, 0.0), egui::pos2(0.0, 20.0)),
        },
        TabStripDropCandidate {
            kind: TabStripDropCandidateKind::Tab(first.clone()),
            bounds: egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(40.0, 20.0)),
        },
        TabStripDropCandidate {
            kind: TabStripDropCandidateKind::Tab(hidden),
            bounds: egui::Rect::from_min_max(egui::pos2(80.0, 0.0), egui::pos2(120.0, 20.0)),
        },
    ];
    let viewport = egui::Rect::from_min_max(egui::pos2(10.0, 0.0), egui::pos2(35.0, 20.0));
    let before = state
        .resolve_nearest_tab_drop(10.0, true, viewport)
        .expect("visible left edge");
    assert!(matches!(before.destination, TabStripTabPlacement::Before(value) if value == first));
    let after = state
        .resolve_nearest_tab_drop(50.0, true, viewport)
        .expect("visible right edge");
    assert!(matches!(after.destination, TabStripTabPlacement::After(value) if value == first));
    assert_eq!(after.indicator.center().x, viewport.max.x);
}
