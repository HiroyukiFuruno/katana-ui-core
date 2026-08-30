use super::{TextSurfaceInteraction, secondary_pointer_hit};
use crate::text_surface::model::EguiTextSurfaceInputPolicy;
use katana_ui_core::atom::TextArea;
use katana_ui_core::render_model::UiRect;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAction, TextSurfaceEvent, TextSurfaceGraphemeBox, TextSurfaceGutter,
    TextSurfaceGutterRow, TextSurfaceLayout, TextSurfaceProps, TextSurfaceViewport,
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
            katana_ui_core::text_surface::TextSurfaceEvent::ContextTargetRequested {
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
        katana_ui_core::text_surface::TextSurfaceEvent::ContextTargetRequested { .. }
    )));
}

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
