use super::*;
use crate::atom::{TextAreaAction, TextAreaCompositionPhase};

#[test]
fn disabled_surface_does_not_reclaim_focus_from_pointer_input() {
    let context = egui::Context::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("disabled").value("one").disabled(true),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    ));
    let layout = TextSurfaceLayout::new(
        "disabled-layout",
        UiRect::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    );
    let frame = surface.frame(&layout);
    let events = run_pointer_frame(
        &context,
        &mut surface,
        &layout,
        &frame,
        vec![egui::Event::PointerButton {
            pos: egui::pos2(10.0, 10.0),
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }],
    );

    assert!(events.is_empty());
    assert!(!surface.state().text_area.focused);
    assert_eq!(None, surface.state().pointer_anchor);
}

#[test]
fn disabled_parent_ui_surrenders_existing_surface_interaction_state() {
    let context = egui::Context::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("parent-disabled").value("one"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    ));
    let layout = TextSurfaceLayout::new(
        "parent-disabled-layout",
        UiRect::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    );
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let _ = surface.apply_layout_action(
        &layout,
        crate::text_surface::TextSurfaceLayoutAction::PointerPress {
            point: crate::text_surface::TextSurfacePoint::new(10, 10),
            extend_selection: false,
        },
    );
    let before_value = surface.state().text_area.value.clone();
    let before_selection = surface.state().text_area.selection;
    assert!(surface.state().text_area.focused);
    assert!(surface.state().pointer_anchor.is_some());
    let frame = surface.frame(&layout);
    let mut captured = Vec::new();
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        ui.add_enabled_ui(false, |ui| {
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
                &[],
            );
        });
    });
    output.textures_delta.clear();

    assert!(captured.contains(&TextSurfaceEvent::FocusChanged(false)));
    assert!(!surface.state().text_area.focused);
    assert_eq!(None, surface.state().pointer_anchor);
    assert_eq!(before_value, surface.state().text_area.value);
    assert_eq!(before_selection, surface.state().text_area.selection);
}

#[test]
fn sizing_pass_skips_input_without_clearing_surface_or_pending_interaction_state() {
    let context = egui::Context::default();
    let mut surface = TextSurface::new(TextSurfaceProps::new(
        TextArea::new("sizing-pass").value("one"),
        Vec::new(),
        TextSurfaceViewport::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    ));
    let layout = TextSurfaceLayout::new(
        "sizing-pass-layout",
        UiRect::new(0, 0, 100, TEST_SURFACE_HEIGHT),
    );
    let _ = surface.apply_action(TextSurfaceAction::SetFocus(true));
    let _ = surface.apply_action(TextSurfaceAction::TextArea(TextAreaAction::composition(
        TextAreaCompositionPhase::Update,
        "かな",
        "かな".len(),
    )));
    let _ = surface.apply_layout_action(
        &layout,
        crate::text_surface::TextSurfaceLayoutAction::PointerPress {
            point: crate::text_surface::TextSurfacePoint::new(10, 10),
            extend_selection: false,
        },
    );
    let before_state = surface.state().clone();
    let frame = surface.frame(&layout);
    let mut captured = Vec::new();
    let mut output = context.run_ui(
        egui::RawInput {
            events: vec![egui::Event::Text("typed during sizing".to_string())],
            ..Default::default()
        },
        |ui| {
            ui.scope_builder(egui::UiBuilder::new().sizing_pass().invisible(), |ui| {
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
                    &[],
                );
            });
        },
    );
    output.textures_delta.clear();

    assert!(captured.is_empty());
    assert_eq!(before_state, *surface.state());
}
