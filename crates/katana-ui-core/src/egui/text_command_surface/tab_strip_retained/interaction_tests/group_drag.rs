use super::*;
use crate::egui::text_command_surface::tab_strip_retained::TabStripGroupDragState;

#[test]
fn resolve_group_drag_forwards_before_after_and_end_placements() {
    let source = TabStripGroupTarget::from_opaque_bytes(b"source-group");
    let target = TabStripGroupTarget::from_opaque_bytes(b"target-group");
    let operations = std::rc::Rc::new(RefCell::new(Vec::new()));
    let recorder = RecordingPort::new(std::rc::Rc::clone(&operations));
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    state.port = Some(TabStripProposalPortHandle::new(recorder));
    state.group_drag_candidates = vec![TabStripDropCandidate {
        bounds: egui::Rect::from_min_size(egui::pos2(20.0, 0.0), egui::vec2(40.0, 20.0)),
        kind: TabStripDropCandidateKind::Group(target.copy_for_route()),
    }];
    let strip = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(120.0, 20.0));
    for pointer in [
        egui::pos2(22.0, 10.0),
        egui::pos2(58.0, 10.0),
        egui::pos2(110.0, 10.0),
    ] {
        state.group_drag = Some(TabStripGroupDragState {
            source: source.copy_for_route(),
            label: TabStripText::new("group"),
            pointer,
        });
        state.group_drag_release_pending = true;
        let context = egui::Context::default();
        let mut output = context.run_ui(egui::RawInput::default(), |ui| {
            state
                .resolve_group_drag(ui, strip, &mut Vec::new())
                .expect("group drop should forward");
        });
        output.textures_delta.clear();
        assert_eq!(
            operations.borrow().last().copied(),
            Some(TabStripProposalOperationClass::Other)
        );
        assert!(state.group_drag.is_none());
    }
}

#[test]
fn group_drag_skips_when_another_drag_is_active_and_cancels_without_destination() {
    let source = TabStripGroupTarget::from_opaque_bytes(b"source-group");
    let group = TabStripGroupDescriptor::new(source.copy_for_route(), TabStripText::new("group"));
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    state.drag = Some(TabStripDragState {
        source: TabStripTabTarget::from_opaque_bytes(b"tab-drag"),
        origin: None,
        label: TabStripText::new("tab"),
        pointer: egui::Pos2::ZERO,
    });
    let context = egui::Context::default();
    let mut frame = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .start_group_drag(&group, egui::Rect::EVERYTHING, ui)
            .expect("an existing tab drag must not reject group drag");
    });
    frame.textures_delta.clear();
    assert!(state.group_drag.is_none());

    let operations = std::rc::Rc::new(RefCell::new(Vec::new()));
    state.port = Some(TabStripProposalPortHandle::new(RecordingPort::new(
        std::rc::Rc::clone(&operations),
    )));
    state.drag = None;
    state.group_drag = Some(TabStripGroupDragState {
        source,
        label: TabStripText::new("group"),
        pointer: egui::pos2(60.0, 10.0),
    });
    state.group_drag_candidates = vec![TabStripDropCandidate {
        bounds: egui::Rect::from_min_size(egui::pos2(20.0, 0.0), egui::vec2(40.0, 20.0)),
        kind: TabStripDropCandidateKind::Tab(TabStripTabTarget::from_opaque_bytes(b"tab")),
    }];
    state.group_drag_release_pending = true;
    let mut frame = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .resolve_group_drag(
                ui,
                egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(100.0, 20.0)),
                &mut Vec::new(),
            )
            .expect("group release without destination should cancel");
    });
    frame.textures_delta.clear();
    assert_eq!(
        operations.borrow().last().copied(),
        Some(TabStripProposalOperationClass::CancelDrag)
    );
    assert!(state.group_drag.is_none());
}
