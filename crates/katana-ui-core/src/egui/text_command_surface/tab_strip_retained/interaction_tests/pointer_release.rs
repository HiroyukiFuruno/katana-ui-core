use super::*;

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
fn resolve_tab_drag_finishes_on_pointer_release_when_response_disappears() {
    let source = TabStripTabTarget::from_opaque_bytes(b"source");
    let operations = std::rc::Rc::new(RefCell::new(Vec::new()));
    let recorder = RecordingPort::new(std::rc::Rc::clone(&operations));
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    state.port = Some(TabStripProposalPortHandle::new(recorder));
    state.drag = Some(TabStripDragState {
        center_offset_x: 0.0,
        ghost_width: 0.0,
        source,
        origin: None,
        label: TabStripText::new("label"),
        pointer: egui::pos2(20.0, 10.0),
    });

    let context = egui::Context::default();
    let mut render_operations = Vec::new();
    let mut platform_output = context.run_ui(
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
                .resolve_tab_drag(
                    ui,
                    egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(DROP_WIDE_STRIP_WIDTH_PX, DROP_STRIP_HEIGHT_PX),
                    ),
                    false,
                    &mut render_operations,
                )
                .expect("pointer release should cancel the stale drag");
        },
    );
    platform_output.textures_delta.clear();

    assert_eq!(
        operations.borrow().as_slice(),
        &[TabStripProposalOperationClass::CancelDrag]
    );
    assert!(state.drag.is_none());
}

#[test]
fn suspend_interaction_is_idempotent_and_preserves_drag_on_cancel_failure() {
    let source = TabStripTabTarget::from_opaque_bytes(b"source");
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    state.port = Some(TabStripProposalPortHandle::new(RejectingPort));
    state.drag = Some(TabStripDragState {
        center_offset_x: 0.0,
        ghost_width: 0.0,
        source,
        origin: None,
        label: TabStripText::new("label"),
        pointer: egui::Pos2::ZERO,
    });
    state.drag_release_pending = true;
    assert!(matches!(
        state.suspend_interaction(),
        Err(TabStripRetainedError::Port(
            TabStripProposalPortError::Rejected
        ))
    ));
    assert!(state.drag.is_some());

    let mut idle = build_state_with_projection(TabStripProjection::new(
        2,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    idle.overlay_input.primary_press = Some(egui::Id::new("press"));
    idle.group_drag_release_pending = true;
    idle.suspend_interaction().expect("idle suspend");
    idle.suspend_interaction().expect("repeat suspend");
    assert!(idle.drag.is_none());
    assert!(idle.group_drag.is_none());
    assert!(!idle.drag_release_pending);
    assert!(!idle.group_drag_release_pending);
    assert!(idle.overlay_input.primary_press.is_none());
}
