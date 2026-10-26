use super::*;
use crate::egui::text_command_surface::tab_strip_retained::{
    TabStripDragState, TabStripGroupDragState, TabStripRenameDraft,
};
use crate::egui::text_command_surface::{
    TabStripGroupCapabilities, TabStripGroupPopupPresentation, TabStripGroupTarget,
    TabStripProposal, TabStripProposalPort, TabStripProposalPortError, TabStripProposalPortHandle,
    TabStripTabCapabilities, TabStripTabTarget,
};

struct AcceptingPort;

impl TabStripProposalPort for AcceptingPort {
    fn forward_proposal(
        &mut self,
        _proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        Ok(())
    }
}

struct RejectingPort;

impl TabStripProposalPort for RejectingPort {
    fn forward_proposal(
        &mut self,
        _proposal: TabStripProposal,
    ) -> Result<(), TabStripProposalPortError> {
        Err(TabStripProposalPortError::Rejected)
    }
}

#[test]
fn synchronize_lease_clears_consumed_rename_and_invalid_group_drag() {
    let target = TabStripGroupTarget::from_opaque_bytes(b"stale-group");
    let initial = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(
            TabStripGroupDescriptor::new(target.clone(), TabStripText::new("G"))
                .capabilities(TabStripGroupCapabilities::new().draggable(true)),
        );
    let mut state = build_state_with_projection(initial);
    state.inline_rename_consumed.push(target.copy_for_route());
    state.overlay = TabStripOverlayState::GroupPopup {
        path: "root-group-0".to_owned(),
        anchor: egui::Pos2::ZERO,
        submenu_path: Vec::new(),
        rename: Some(Box::new(TabStripRenameDraft::new("G", "Rename"))),
    };
    state.group_drag = Some(TabStripGroupDragState {
        source: target.copy_for_route(),
        label: TabStripText::new("G"),
        pointer: egui::Pos2::ZERO,
    });
    state.group_drag_release_pending = true;
    let next = TabStripProjection::new(2, TabStripCorrelation::from_opaque_bytes(b"corr"));
    state
        .synchronize_lease(TabStripProjectionLease::new(next))
        .expect("stale group lease should synchronize");
    assert!(matches!(state.overlay, TabStripOverlayState::Closed));
    assert!(state.inline_rename_consumed.is_empty());
    assert!(state.group_drag.is_none());
    assert!(!state.group_drag_release_pending);
}

#[test]
fn synchronize_lease_cancels_drag_when_group_becomes_non_draggable() {
    let target = TabStripGroupTarget::from_opaque_bytes(b"disabled-group");
    let initial = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(
            TabStripGroupDescriptor::new(target.clone(), TabStripText::new("G"))
                .capabilities(TabStripGroupCapabilities::new().draggable(true)),
        );
    let mut state = build_state_with_projection(initial);
    state.group_drag = Some(TabStripGroupDragState {
        source: target.copy_for_route(),
        label: TabStripText::new("G"),
        pointer: egui::Pos2::ZERO,
    });
    state.group_drag_release_pending = true;
    let next = TabStripProjection::new(2, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .group(TabStripGroupDescriptor::new(target, TabStripText::new("G")));
    state
        .synchronize_lease(TabStripProjectionLease::new(next))
        .expect("disabled group lease should synchronize");
    assert!(state.group_drag.is_none());
    assert!(!state.group_drag_release_pending);
}

#[test]
fn synchronize_lease_closes_rename_editor_when_rename_contract_is_revoked() {
    let target = TabStripGroupTarget::from_opaque_bytes(b"rename-contract");
    let initial = || {
        TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr")).group(
            TabStripGroupDescriptor::new(target.clone(), TabStripText::new("G"))
                .capabilities(TabStripGroupCapabilities::new().renamable(true))
                .popup(
                    TabStripGroupPopupPresentation::new()
                        .rename_placeholder(TabStripText::new("Rename")),
                ),
        )
    };
    let mut state = build_state_with_projection(initial());
    state.inline_rename_consumed.push(target.copy_for_route());
    state.overlay = TabStripOverlayState::GroupPopup {
        path: "root-group-0".to_owned(),
        anchor: egui::Pos2::ZERO,
        submenu_path: Vec::new(),
        rename: Some(Box::new(TabStripRenameDraft::new("G", "Rename"))),
    };
    state
        .synchronize_lease(TabStripProjectionLease::new(
            TabStripProjection::new(2, TabStripCorrelation::from_opaque_bytes(b"corr")).group(
                TabStripGroupDescriptor::new(target.clone(), TabStripText::new("G")),
            ),
        ))
        .expect("capability revocation should synchronize");
    assert!(matches!(state.overlay, TabStripOverlayState::Closed));
    assert!(state.inline_rename_consumed.is_empty());

    let mut state = build_state_with_projection(initial());
    state.inline_rename_consumed.push(target.copy_for_route());
    state.overlay = TabStripOverlayState::GroupPopup {
        path: "root-group-0".to_owned(),
        anchor: egui::Pos2::ZERO,
        submenu_path: Vec::new(),
        rename: Some(Box::new(TabStripRenameDraft::new("G", "Rename"))),
    };
    state
        .synchronize_lease(TabStripProjectionLease::new(
            TabStripProjection::new(3, TabStripCorrelation::from_opaque_bytes(b"corr")).group(
                TabStripGroupDescriptor::new(target, TabStripText::new("G"))
                    .capabilities(TabStripGroupCapabilities::new().renamable(true))
                    .popup(TabStripGroupPopupPresentation::new()),
            ),
        ))
        .expect("placeholder revocation should synchronize");
    assert!(matches!(state.overlay, TabStripOverlayState::Closed));
    assert!(state.inline_rename_consumed.is_empty());
}

#[test]
fn synchronize_lease_rearms_consumed_inline_rename_after_request_is_cleared() {
    let target = TabStripGroupTarget::from_opaque_bytes(b"rearm-group");
    let projection = |revision, request_inline_rename| {
        TabStripProjection::new(revision, TabStripCorrelation::from_opaque_bytes(b"corr")).group(
            TabStripGroupDescriptor::new(target.clone(), TabStripText::new("G"))
                .capabilities(TabStripGroupCapabilities::new().renamable(true))
                .popup(
                    TabStripGroupPopupPresentation::new()
                        .request_inline_rename(request_inline_rename)
                        .rename_placeholder(TabStripText::new("Rename")),
                ),
        )
    };
    let mut state = build_state_with_projection(projection(1, true));
    state.inline_rename_consumed.push(target.copy_for_route());

    state
        .synchronize_lease(TabStripProjectionLease::new(projection(2, false)))
        .expect("clearing the inline request should synchronize");
    assert!(state.inline_rename_consumed.is_empty());

    state
        .synchronize_lease(TabStripProjectionLease::new(projection(3, true)))
        .expect("rearming the inline request should synchronize");
    assert!(state.inline_rename_consumed.is_empty());
}

#[test]
fn synchronize_lease_cancels_tab_drag_when_source_is_removed_or_disabled() {
    let source = TabStripTabTarget::from_opaque_bytes(b"tab-drag-source");
    let initial = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr")).tab(
        TabStripTabDescriptor::new(source.clone(), TabStripText::new("source"))
            .capabilities(TabStripTabCapabilities::new().draggable(true)),
    );
    let mut state = build_state_with_projection(initial);
    state.port = Some(TabStripProposalPortHandle::new(AcceptingPort));
    state.drag = Some(TabStripDragState {
        source: source.copy_for_route(),
        origin: None,
        label: TabStripText::new("source"),
        pointer: egui::Pos2::ZERO,
    });
    state.drag_release_pending = true;
    state
        .synchronize_lease(TabStripProjectionLease::new(TabStripProjection::new(
            2,
            TabStripCorrelation::from_opaque_bytes(b"corr"),
        )))
        .expect("removed tab lease should synchronize");
    assert!(state.drag.is_none());
    assert!(!state.drag_release_pending);

    let mut state = build_state_with_projection(
        TabStripProjection::new(3, TabStripCorrelation::from_opaque_bytes(b"corr")).tab(
            TabStripTabDescriptor::new(source.clone(), TabStripText::new("source"))
                .capabilities(TabStripTabCapabilities::new().draggable(true)),
        ),
    );
    state.port = Some(TabStripProposalPortHandle::new(AcceptingPort));
    state.drag = Some(TabStripDragState {
        source,
        origin: None,
        label: TabStripText::new("source"),
        pointer: egui::Pos2::ZERO,
    });
    state.drag_release_pending = true;
    state
        .synchronize_lease(TabStripProjectionLease::new(
            TabStripProjection::new(4, TabStripCorrelation::from_opaque_bytes(b"corr")).tab(
                TabStripTabDescriptor::new(
                    TabStripTabTarget::from_opaque_bytes(b"tab-drag-source"),
                    TabStripText::new("source"),
                )
                .capabilities(TabStripTabCapabilities::new().draggable(false)),
            ),
        ))
        .expect("disabled tab lease should synchronize");
    assert!(state.drag.is_none());
    assert!(!state.drag_release_pending);
}

#[test]
fn synchronize_lease_keeps_old_tab_drag_when_cancel_is_rejected() {
    let source = TabStripTabTarget::from_opaque_bytes(b"tab-drag-rejected");
    let mut state = build_state_with_projection(
        TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"old-corr")).tab(
            TabStripTabDescriptor::new(source.clone(), TabStripText::new("source"))
                .capabilities(TabStripTabCapabilities::new().draggable(true)),
        ),
    );
    state.port = Some(TabStripProposalPortHandle::new(RejectingPort));
    state.drag = Some(TabStripDragState {
        source,
        origin: None,
        label: TabStripText::new("source"),
        pointer: egui::Pos2::ZERO,
    });
    let result = state.synchronize_lease(TabStripProjectionLease::new(TabStripProjection::new(
        2,
        TabStripCorrelation::from_opaque_bytes(b"new-corr"),
    )));
    assert!(matches!(
        result,
        Err(TabStripRetainedError::Port(
            TabStripProposalPortError::Rejected
        ))
    ));
    assert_eq!(state.projection.revision, 1);
    assert_eq!(state.projection.correlation.payload.as_ref(), b"old-corr");
    assert!(state.drag.is_some());
}
