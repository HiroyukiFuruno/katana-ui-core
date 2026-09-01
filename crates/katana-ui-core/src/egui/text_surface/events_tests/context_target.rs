use super::*;

#[test]
fn secondary_pointer_hit_requires_secondary_activation() {
    let context = egui::Context::default();
    let mut captured = None;
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        let (_, response) = ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::click());
        captured = Some(secondary_pointer_hit(ui, &response, egui::pos2(10.0, 10.0)));
    });
    output.textures_delta.clear();
    assert_eq!(captured, Some(false));
}

#[test]
fn secondary_pointer_inside_surface_requests_context_target() {
    let context = egui::Context::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("context").value("one"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    ));
    let layout = TextSurfaceLayout::new(
        "context-layout",
        UiRect::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    );
    let frame = surface.frame(&layout);
    let run = |events: Vec<egui::Event>, surface: &mut TextSurface| {
        let mut captured = Vec::new();
        let mut output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(100.0, 20.0),
                )),
                events,
                ..Default::default()
            },
            |ui| {
                let (_, response) =
                    ui.allocate_exact_size(egui::vec2(100.0, 20.0), egui::Sense::click_and_drag());
                captured = TextSurfaceInteraction::apply_interactions(
                    ui,
                    &response,
                    surface,
                    &layout,
                    &frame,
                    &EguiTextSurfaceInputPolicy::default(),
                    None,
                    &[],
                );
            },
        );
        output.textures_delta.clear();
        captured
    };

    let _ = run(
        vec![egui::Event::PointerMoved(egui::pos2(10.0, 10.0))],
        &mut surface,
    );
    let captured = run(
        vec![egui::Event::PointerButton {
            pos: egui::pos2(10.0, 10.0),
            button: egui::PointerButton::Secondary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }],
        &mut surface,
    );

    assert!(
        captured.iter().any(|event| matches!(
            event,
            crate::text_surface::TextSurfaceEvent::ContextTargetRequested {
                selection
            } if *selection == frame.selection.range
        )),
        "captured events: {captured:?}"
    );
}

#[test]
fn secondary_pointer_release_also_requests_context_target_from_actual_frame() {
    let context = egui::Context::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("context-release").value("one"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    ));
    let layout = TextSurfaceLayout::new(
        "context-release-layout",
        UiRect::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    );
    let frame = surface.frame(&layout);
    let run = |events: Vec<egui::Event>, surface: &mut TextSurface| {
        let mut captured = Vec::new();
        let mut output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(100.0, 20.0),
                )),
                events,
                ..Default::default()
            },
            |ui| {
                let (_, response) =
                    ui.allocate_exact_size(egui::vec2(100.0, 20.0), egui::Sense::click_and_drag());
                captured = TextSurfaceInteraction::apply_interactions(
                    ui,
                    &response,
                    surface,
                    &layout,
                    &frame,
                    &EguiTextSurfaceInputPolicy::default(),
                    None,
                    &[],
                );
            },
        );
        output.textures_delta.clear();
        captured
    };

    let _ = run(
        vec![egui::Event::PointerMoved(egui::pos2(10.0, 10.0))],
        &mut surface,
    );
    let _ = run(
        vec![egui::Event::PointerButton {
            pos: egui::pos2(10.0, 10.0),
            button: egui::PointerButton::Secondary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }],
        &mut surface,
    );
    let released = run(
        vec![egui::Event::PointerButton {
            pos: egui::pos2(10.0, 10.0),
            button: egui::PointerButton::Secondary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
        &mut surface,
    );

    assert!(
        released.iter().any(|event| matches!(
            event,
            crate::text_surface::TextSurfaceEvent::ContextTargetRequested {
                selection
            } if *selection == frame.selection.range
        )),
        "captured events: {released:?}"
    );
}

#[test]
fn pointer_position_without_secondary_activation_resolves_as_ordinary_pointer_input() {
    let context = egui::Context::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("pointer").value("one"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 100, 20),
    ));
    let layout = TextSurfaceLayout::new("pointer-layout", UiRect::new(0, 0, 100, 20));
    let frame = surface.frame(&layout);
    let _ = run_pointer_frame(&context, &mut surface, &layout, &frame, Vec::new());
    let captured = run_pointer_frame(
        &context,
        &mut surface,
        &layout,
        &frame,
        vec![egui::Event::PointerMoved(egui::pos2(10.0, 10.0))],
    );

    assert!(!captured.iter().any(|event| matches!(
        event,
        crate::text_surface::TextSurfaceEvent::ContextTargetRequested { .. }
    )));
}
