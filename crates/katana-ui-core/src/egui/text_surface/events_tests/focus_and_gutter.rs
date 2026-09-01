use super::*;

#[test]
fn pointer_exclusion_keeps_a_focused_surface_focused_from_actual_raw_input() {
    let context = egui::Context::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("excluded").value("one"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    ));
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let layout = TextSurfaceLayout::new(
        "excluded-layout",
        UiRect::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    );
    let frame = surface.frame(&layout);
    let mut captured = Vec::new();
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(100.0, TEST_SURFACE_HEIGHT as f32),
            )),
            events: vec![egui::Event::PointerButton {
                pos: egui::pos2(10.0, 10.0),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        },
        |ui| {
            let (_, response) = ui.allocate_exact_size(
                egui::vec2(100.0, TEST_SURFACE_HEIGHT as f32),
                egui::Sense::click_and_drag(),
            );
            captured = TextSurfaceInteraction::apply_interactions(
                ui,
                &response,
                &mut surface,
                &layout,
                &frame,
                &EguiTextSurfaceInputPolicy::default(),
                None,
                &[UiRect::new(0, 0, 20, TEST_SURFACE_HEIGHT)],
            );
        },
    );
    output.textures_delta.clear();

    assert!(captured.is_empty());
    assert!(surface.state().text_area.focused);
}

#[test]
fn retained_pointer_focus_reclaims_focus_after_unrelated_physical_input() {
    let context = egui::Context::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("retained-focus").value("one"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    ));
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let layout = TextSurfaceLayout::new(
        "retained-focus-layout",
        UiRect::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    );
    let frame = surface.frame(&layout);
    let mut response_id = None;
    let mut captured = Vec::new();
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(240.0, 80.0),
            )),
            events: vec![egui::Event::PointerButton {
                pos: egui::pos2(220.0, 60.0),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        },
        |ui| {
            let (_, response) = ui.allocate_exact_size(
                egui::vec2(100.0, TEST_SURFACE_HEIGHT as f32),
                egui::Sense::click_and_drag(),
            );
            ui.add(egui::Button::new("other focus owner"))
                .request_focus();
            response_id = Some(response.id);
            captured = TextSurfaceInteraction::apply_interactions(
                ui,
                &response,
                &mut surface,
                &layout,
                &frame,
                &EguiTextSurfaceInputPolicy::default().with_retained_pointer_focus(),
                None,
                &[],
            );
        },
    );
    output.textures_delta.clear();

    assert!(captured.is_empty());
    assert_eq!(context.memory(|memory| memory.focused()), response_id);
    assert!(surface.state().text_area.focused);
}

#[test]
fn focus_routes_release_pending_focus_and_reclaim_core_focus_without_pointer_input() {
    let context = egui::Context::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("pending-release").value("one"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    ));
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let layout = TextSurfaceLayout::new(
        "pending-release-layout",
        UiRect::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    );
    let frame = surface.frame(&layout);
    let mut released = Vec::new();
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        let (_, response) = ui.allocate_exact_size(
            egui::vec2(100.0, TEST_SURFACE_HEIGHT as f32),
            egui::Sense::click_and_drag(),
        );
        response.request_focus();
        released = TextSurfaceInteraction::apply_interactions(
            ui,
            &response,
            &mut surface,
            &layout,
            &frame,
            &EguiTextSurfaceInputPolicy::default(),
            Some(false),
            &[],
        );
    });
    output.textures_delta.clear();
    assert!(released.contains(&TextSurfaceEvent::FocusChanged(false)));
    assert!(!surface.state().text_area.focused);

    let context = egui::Context::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("core-refocus").value("one"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    ));
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let layout = TextSurfaceLayout::new(
        "core-refocus-layout",
        UiRect::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    );
    let frame = surface.frame(&layout);
    let mut response_id = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        let (_, response) = ui.allocate_exact_size(
            egui::vec2(100.0, TEST_SURFACE_HEIGHT as f32),
            egui::Sense::click_and_drag(),
        );
        response_id = Some(response.id);
        let events = TextSurfaceInteraction::apply_interactions(
            ui,
            &response,
            &mut surface,
            &layout,
            &frame,
            &EguiTextSurfaceInputPolicy::default(),
            None,
            &[],
        );
        assert!(events.is_empty());
    });
    output.textures_delta.clear();
    assert_eq!(context.memory(|memory| memory.focused()), response_id);
}

#[test]
fn excluded_pointer_on_unfocused_surface_is_ignored_without_claiming_focus() {
    let context = egui::Context::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("unfocused-exclusion").value("one"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    ));
    let layout = TextSurfaceLayout::new(
        "unfocused-exclusion-layout",
        UiRect::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    );
    let frame = surface.frame(&layout);
    let mut captured = Vec::new();
    let mut output = context.run_ui(
        egui::RawInput {
            events: vec![egui::Event::PointerButton {
                pos: egui::pos2(10.0, 10.0),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        },
        |ui| {
            let (_, response) = ui.allocate_exact_size(
                egui::vec2(100.0, TEST_SURFACE_HEIGHT as f32),
                egui::Sense::click_and_drag(),
            );
            captured = TextSurfaceInteraction::apply_interactions(
                ui,
                &response,
                &mut surface,
                &layout,
                &frame,
                &EguiTextSurfaceInputPolicy::default(),
                None,
                &[UiRect::new(0, 0, 20, TEST_SURFACE_HEIGHT)],
            );
        },
    );
    output.textures_delta.clear();
    assert!(captured.is_empty());
    assert!(!surface.state().text_area.focused);
}

#[test]
fn primary_pointer_on_gutter_returns_the_real_marker_event_before_text_dragging() {
    let context = egui::Context::default();
    let mut surface = TextSurface::new(
        TextSurfaceProps::new(
            TextArea::new("gutter-event").value("line"),
            Vec::new(),
            TextSurfaceViewport::new(0, 0, 100, TEST_SURFACE_HEIGHT),
        )
        .gutter(
            TextSurfaceGutter::new(24).row(TextSurfaceGutterRow::new(0, "1").marker_id("marker")),
        ),
    );
    let layout = TextSurfaceLayout::from_grapheme_boxes(
        "gutter-event-layout",
        UiRect::new(24, 0, 76, TEST_SURFACE_HEIGHT),
        "line",
        vec![TextSurfaceGraphemeBox {
            grapheme_index: 0,
            byte_start: 0,
            byte_end: 4,
            bounds: UiRect::new(24, 0, 30, TEST_SURFACE_HEIGHT),
        }],
    );
    let frame = surface.frame(&layout);
    let gutter_bounds = frame.gutter.first().expect("gutter frame exists").bounds;
    let pointer = egui::pos2(gutter_bounds.x as f32 + 1.0, gutter_bounds.y as f32 + 1.0);
    let _ = run_pointer_frame(
        &context,
        &mut surface,
        &layout,
        &frame,
        vec![egui::Event::PointerMoved(pointer)],
    );
    let captured = run_pointer_frame(
        &context,
        &mut surface,
        &layout,
        &frame,
        vec![egui::Event::PointerButton {
            pos: pointer,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    assert!(captured.iter().any(|event| matches!(
        event,
        TextSurfaceEvent::GutterMarkerActivated {
            logical_row: 0,
            marker_id,
        } if marker_id == "marker"
    )));
}
