use super::*;

#[test]
fn resolve_tab_drag_propagates_real_ghost_raster_error_without_proposal() {
    let mut state = build_state_with_projection(TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"corr"),
    ));
    let proposals = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    state.port = Some(TabStripProposalPortHandle::new(RecordingPort::new(
        proposals.clone(),
    )));
    let label = "x".repeat(16_000);
    state.drag = Some(TabStripDragState {
        center_offset_x: 0.0,
        ghost_width: 0.0,
        source: TabStripTabTarget::from_opaque_bytes(b"tab-drag"),
        origin: None,
        label: TabStripText::new(&label),
        pointer: egui::pos2(10.0, 10.0),
    });
    let context = egui::Context::default();
    let mut operations = Vec::new();
    let mut result = None;
    let mut frame = context.run_ui(egui::RawInput::default(), |ui| {
        result = Some(state.resolve_tab_drag(
            ui,
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(100.0, 20.0)),
            false,
            &mut operations,
        ));
    });
    frame.textures_delta.clear();

    assert!(matches!(
        result.expect("drag frame should execute"),
        Err(TabStripRetainedError::Raster(_))
    ));
    assert!(
        state.drag.is_none(),
        "failed feedback must not retain stale drag"
    );
    assert!(
        operations.is_empty(),
        "failed feedback must not paint a ghost"
    );
    assert!(
        proposals.borrow().is_empty(),
        "failed feedback must not forward a proposal"
    );
}
