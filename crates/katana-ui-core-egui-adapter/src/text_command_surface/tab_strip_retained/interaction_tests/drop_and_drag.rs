use super::*;

#[test]
fn resolve_tab_drop_uses_right_ratio_for_after_destination() {
    let source = TabStripTabTarget::from_opaque_bytes(b"source");
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    state.drag_candidates.push(TabStripDropCandidate {
        bounds: egui::Rect::from_min_size(
            egui::pos2(10.0, 0.0),
            egui::vec2(DROP_CANDIDATE_WIDTH_PX, DROP_CANDIDATE_HEIGHT_PX),
        ),
        kind: TabStripDropCandidateKind::Tab(TabStripTabTarget::from_opaque_bytes(b"target")),
    });

    let drop = state.resolve_tab_drop(
        egui::pos2(27.5, DROP_POINTER_Y_PX),
        &source,
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(DROP_STRIP_WIDTH_PX, DROP_STRIP_HEIGHT_PX),
        ),
        true,
    );
    let Some(resolved) = drop else {
        panic!("expected right-side drop target");
    };
    assert!(matches!(
        resolved.destination,
        TabStripTabPlacement::After(_)
    ));
}

#[test]
fn resolve_tab_drop_maps_left_ratio_to_before_placement() {
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    let source = TabStripTabTarget::from_opaque_bytes(b"source");
    let destination = TabStripTabTarget::from_opaque_bytes(b"destination");

    state.drag_candidates = vec![TabStripDropCandidate {
        bounds: egui::Rect::from_min_size(egui::pos2(10.0, 0.0), egui::vec2(80.0, 20.0)),
        kind: TabStripDropCandidateKind::Tab(destination.copy_for_route()),
    }];

    let Some(drop) = state.resolve_tab_drop(
        egui::pos2(24.0, DROP_POINTER_Y_PX),
        &source,
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(DROP_WIDE_STRIP_WIDTH_PX, 30.0)),
        false,
    ) else {
        panic!("left ratio should map to before placement");
    };
    assert!(matches!(
        drop.destination,
        TabStripTabPlacement::Before(target) if target.payload == destination.payload
    ));
}

#[test]
fn resolve_tab_drop_maps_right_ratio_to_after_placement() {
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    let source = TabStripTabTarget::from_opaque_bytes(b"source");
    let destination = TabStripTabTarget::from_opaque_bytes(b"destination");

    state.drag_candidates = vec![TabStripDropCandidate {
        bounds: egui::Rect::from_min_size(egui::pos2(10.0, 0.0), egui::vec2(80.0, 20.0)),
        kind: TabStripDropCandidateKind::Tab(destination.copy_for_route()),
    }];

    let Some(drop) = state.resolve_tab_drop(
        egui::pos2(90.0, DROP_POINTER_Y_PX),
        &source,
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(DROP_WIDE_STRIP_WIDTH_PX, 30.0)),
        false,
    ) else {
        panic!("right ratio should map to after placement");
    };
    assert!(matches!(
        drop.destination,
        TabStripTabPlacement::After(target) if target.payload == destination.payload
    ));
}

#[test]
fn resolve_tab_drop_maps_middle_ratio_to_no_placement() {
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    let source = TabStripTabTarget::from_opaque_bytes(b"source");
    let destination = TabStripTabTarget::from_opaque_bytes(b"destination");

    state.drag_candidates = vec![TabStripDropCandidate {
        bounds: egui::Rect::from_min_size(egui::pos2(10.0, 0.0), egui::vec2(100.0, 20.0)),
        kind: TabStripDropCandidateKind::Tab(destination.copy_for_route()),
    }];

    let strip_bounds = egui::Rect::from_min_size(
        egui::Pos2::ZERO,
        egui::vec2(DROP_WIDE_STRIP_WIDTH_PX, DROP_STRIP_HEIGHT_PX),
    );
    assert!(
        state
            .resolve_tab_drop(egui::pos2(60.0, 10.0), &source, strip_bounds, false)
            .is_none()
    );
}

#[test]
fn resolve_tab_drag_cancel_when_release_requested_and_no_destination() {
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    let source = TabStripTabTarget::from_opaque_bytes(b"source");
    let destination = TabStripTabTarget::from_opaque_bytes(b"destination");
    let recorded_operations = std::rc::Rc::new(RefCell::new(Vec::new()));
    let recorder = RecordingPort::new(std::rc::Rc::clone(&recorded_operations));
    state.port = Some(TabStripProposalPortHandle::new(recorder));
    state.drag = Some(TabStripDragState {
        source: source.copy_for_route(),
        label: TabStripText::new("label"),
        pointer: egui::pos2(20.0, 10.0),
    });
    state.drag_candidates = vec![TabStripDropCandidate {
        bounds: egui::Rect::from_min_size(egui::pos2(10.0, 0.0), egui::vec2(40.0, 20.0)),
        kind: TabStripDropCandidateKind::Tab(destination.copy_for_route()),
    }];
    state.drag_release_pending = true;

    let context = egui::Context::default();
    let mut operations = Vec::new();
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .resolve_tab_drag(
                ui,
                egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(DROP_WIDE_STRIP_WIDTH_PX, DROP_STRIP_HEIGHT_PX),
                ),
                true,
                &mut operations,
            )
            .expect("release should forward a finish/cancel proposal");
    });
    platform_output.textures_delta.clear();

    assert_eq!(
        1,
        recorded_operations.borrow().len(),
        "one proposal should be forwarded on release"
    );
    assert!(
        recorded_operations
            .borrow()
            .contains(&TabStripProposalOperationClass::FinishDragBefore),
        "release should forward a finish drag operation"
    );
    assert!(!state.drag_release_pending);
}

#[test]
fn resolve_tab_drag_escapes_on_escape_keypress() {
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    let source = TabStripTabTarget::from_opaque_bytes(b"source");
    let operations = std::rc::Rc::new(RefCell::new(Vec::new()));
    let recorder = RecordingPort::new(std::rc::Rc::clone(&operations));
    state.port = Some(TabStripProposalPortHandle::new(recorder));
    state.drag = Some(TabStripDragState {
        source: source.copy_for_route(),
        label: TabStripText::new("label"),
        pointer: egui::pos2(20.0, 10.0),
    });

    let context = egui::Context::default();
    let mut render_operations = Vec::new();
    let mut platform_output = context.run_ui(
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        },
        |ui| {
            state
                .resolve_tab_drag(
                    ui,
                    egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(DROP_WIDE_STRIP_WIDTH_PX, DROP_STRIP_HEIGHT_PX),
                    ),
                    true,
                    &mut render_operations,
                )
                .expect("escape should cancel drag proposal");
        },
    );
    platform_output.textures_delta.clear();

    assert!(
        operations
            .borrow()
            .contains(&TabStripProposalOperationClass::CancelDrag)
    );
    assert!(state.drag.is_none());
}
