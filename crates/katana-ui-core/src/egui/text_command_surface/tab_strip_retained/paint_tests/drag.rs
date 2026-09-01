use super::*;

const DRAG_TEST_VIEWPORT_WIDTH_PX: f32 = 200.0;
const DRAG_TEST_VIEWPORT_HEIGHT_PX: f32 = 40.0;

#[test]
fn render_tab_records_drag_release_pending_on_drag_stopped() {
    let draggable_tab = || {
        TabStripTabDescriptor::new(
            TabStripTabTarget::from_opaque_bytes(b"drag-stop-tab"),
            TabStripText::new("drag-stop"),
        )
        .capabilities(
            TabStripTabCapabilities::new()
                .draggable(true)
                .selectable(false),
        )
    };
    let projection = TabStripProjection::new(1, TabStripCorrelation::from_opaque_bytes(b"corr"))
        .tab(draggable_tab());
    let mut state = build_state_from_lease(
        TabStripProjectionLease::new(projection).with_proposal_port(NullPort),
    );
    let tab = draggable_tab();

    let context = egui::Context::default();
    let mut operations = Vec::new();
    let pointer = egui::pos2(12.0, 12.0);
    let drag_pointer = egui::pos2(40.0, 12.0);
    let frames = [
        vec![egui::Event::PointerMoved(pointer)],
        vec![
            egui::Event::PointerMoved(pointer),
            egui::Event::PointerButton {
                pos: pointer,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
        vec![egui::Event::PointerMoved(drag_pointer)],
        vec![egui::Event::PointerButton {
            pos: drag_pointer,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    ];
    for (index, events) in frames.into_iter().enumerate() {
        let mut active_reveal_pending = false;
        let mut x = 0.0;
        operations.clear();
        let mut platform_output = context.run_ui(
            egui::RawInput {
                events,
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(DRAG_TEST_VIEWPORT_WIDTH_PX, DRAG_TEST_VIEWPORT_HEIGHT_PX),
                )),
                ..Default::default()
            },
            |ui| {
                state
                    .render_tab(
                        ui,
                        &tab,
                        "root-tab-0".to_string(),
                        &mut x,
                        ui.available_rect_before_wrap(),
                        &mut operations,
                        &mut active_reveal_pending,
                    )
                    .expect("drag interaction frame should render");
            },
        );
        platform_output.textures_delta.clear();
        if index == 2 {
            assert!(
                state.drag.is_some(),
                "pointer movement should start the drag"
            );
        }
    }

    assert!(state.drag_release_pending);
    assert!(
        !operations.is_empty(),
        "rendering should emit operations even when drag state updates"
    );
    assert_eq!(
        state.drag.as_ref().map(|drag| drag.label.value.as_str()),
        Some("drag-stop")
    );
}
