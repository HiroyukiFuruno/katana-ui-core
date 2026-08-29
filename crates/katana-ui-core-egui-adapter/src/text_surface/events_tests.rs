use super::{TextSurfaceInteraction, secondary_pointer_hit};
use crate::text_surface::model::EguiTextSurfaceInputPolicy;
use katana_ui_core::atom::TextArea;
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAction, TextSurfaceLayout, TextSurfaceProps, TextSurfaceViewport,
};

const TEST_SURFACE_HEIGHT: u32 = 20;

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
fn focused_hovered_surface_applies_smooth_wheel_scroll() {
    let context = egui::Context::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("scroll").value("one\ntwo\nthree\nfour\nfive"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 100, 20),
    ));
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let layout = TextSurfaceLayout::new("scroll-layout", UiRect::new(0, 0, 100, 100));
    let frame = surface.frame(&layout);
    let raw = egui::RawInput {
        events: vec![
            egui::Event::PointerMoved(egui::pos2(10.0, 10.0)),
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, 12.0),
                modifiers: egui::Modifiers::NONE,
                phase: egui::TouchPhase::Move,
            },
        ],
        ..Default::default()
    };
    let mut events = Vec::new();
    let mut output = context.run_ui(raw, |ui| {
        let (_, response) =
            ui.allocate_exact_size(egui::vec2(100.0, 20.0), egui::Sense::click_and_drag());
        response.request_focus();
        events = TextSurfaceInteraction::apply_interactions(
            ui,
            &response,
            &mut surface,
            &layout,
            &frame,
            &EguiTextSurfaceInputPolicy::default(),
            None,
            &[],
        );
    });
    output.textures_delta.clear();
    assert!(events.iter().any(|event| matches!(
        event,
        katana_ui_core::text_surface::TextSurfaceEvent::Scrolled { .. }
    )));
}

#[test]
fn dragging_then_releasing_emits_selection_event_from_pointer_release() {
    let context = egui::Context::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("drag").value("one\ntwo"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 100, 40),
    ));
    let layout = TextSurfaceLayout::new("drag-layout", UiRect::new(0, 0, 100, 40));
    let frame = surface.frame(&layout);

    let run = |events: Vec<egui::Event>, surface: &mut TextSurface| {
        let mut captured = Vec::new();
        let mut output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(100.0, 40.0),
                )),
                events,
                ..Default::default()
            },
            |ui| {
                let (_, response) =
                    ui.allocate_exact_size(egui::vec2(100.0, 40.0), egui::Sense::click_and_drag());
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

    let _ = run(Vec::new(), &mut surface);
    let _ = run(
        vec![egui::Event::PointerButton {
            pos: egui::pos2(10.0, 10.0),
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }],
        &mut surface,
    );
    let _ = run(
        vec![egui::Event::PointerMoved(egui::pos2(30.0, 10.0))],
        &mut surface,
    );
    let events = run(
        vec![egui::Event::PointerButton {
            pos: egui::pos2(30.0, 10.0),
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
        &mut surface,
    );

    match events.as_slice() {
        [
            katana_ui_core::text_surface::TextSurfaceEvent::SelectionChanged {
                selection_start: 0,
                selection_end: 0,
            },
        ] => {}
        other => panic!("release frame must emit exactly one current selection: {other:?}"),
    }

    /* WHY: PointerRelease consumes the anchor on the actual drag_stopped branch, so a
    subsequent move must not continue the previous drag. */
    let after_release = run(
        vec![egui::Event::PointerMoved(egui::pos2(40.0, 10.0))],
        &mut surface,
    );
    assert!(after_release.is_empty());
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
            katana_ui_core::text_surface::TextSurfaceEvent::ContextTargetRequested {
                selection
            } if *selection == frame.selection.range
        )),
        "captured events: {captured:?}"
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
        katana_ui_core::text_surface::TextSurfaceEvent::ContextTargetRequested { .. }
    )));
}

fn run_pointer_frame(
    context: &egui::Context,
    surface: &mut TextSurface,
    layout: &TextSurfaceLayout,
    frame: &katana_ui_core::text_surface::TextSurfaceFrameRecord,
    events: Vec<egui::Event>,
) -> Vec<katana_ui_core::text_surface::TextSurfaceEvent> {
    let mut captured = Vec::new();
    let mut output = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(100.0, TEST_SURFACE_HEIGHT as f32),
            )),
            events,
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
                surface,
                layout,
                frame,
                &EguiTextSurfaceInputPolicy::default(),
                None,
                &[],
            );
        },
    );
    output.textures_delta.clear();
    captured
}
