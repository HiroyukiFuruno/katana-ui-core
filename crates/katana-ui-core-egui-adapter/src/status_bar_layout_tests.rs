use super::EguiStatusBarAdapter;
use katana_ui_core::molecule::{
    StatusBar, StatusBarEvent, StatusBarMode, StatusBarSegment, StatusBarSegmentAlignment,
};

const NARROW_VIEWPORT_WIDTH: f32 = 120.0;
const NARROW_VIEWPORT_HEIGHT: f32 = 60.0;

fn narrow_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(NARROW_VIEWPORT_WIDTH, NARROW_VIEWPORT_HEIGHT),
        )),
        events,
        ..egui::RawInput::default()
    }
}

#[test]
fn long_center_and_trailing_segments_are_elided_and_cannot_hit_outside_the_root() {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-bar-clipped-segments")
        .expect("status bar adapter should retain its platform rasterizer");
    let long_label = "This status segment must never extend beyond its allocated status bar root";
    let mut status = StatusBar::new("clipped-segments")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new("center", long_label)
                .alignment(StatusBarSegmentAlignment::Center)
                .interactive(true),
        )
        .segment(
            StatusBarSegment::new("trailing", long_label)
                .alignment(StatusBarSegmentAlignment::Trailing)
                .interactive(true),
        );

    crate::run_ui_discard(&context, narrow_input(Vec::new()), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            adapter.show(ui, &mut status).expect("status bar renders");
        });
    });
    let root = adapter
        .artifact_paint_plan()
        .expect("status bar produces a paint plan")
        .surface_bounds;
    for bounds in adapter.segment_bounds.values() {
        assert!(
            bounds.left() >= root.x as f32 && bounds.right() <= root.x as f32 + root.width as f32,
            "segment interaction bounds escaped root {root:?}: {bounds:?}"
        );
    }
    let outside_root = egui::pos2(root.x as f32 - 1.0, root.y as f32 + 10.0);
    let pointer_click = vec![
        egui::Event::PointerMoved(outside_root),
        egui::Event::PointerButton {
            pos: outside_root,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        },
        egui::Event::PointerButton {
            pos: outside_root,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        },
    ];
    let mut output = None;
    crate::run_ui_discard(&context, narrow_input(pointer_click), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            output = Some(adapter.show(ui, &mut status).expect("status bar renders"));
        });
    });

    let output = output.expect("status bar frame runs");
    assert!(
        output.events().is_empty(),
        "outside-root pointer dispatched events: {:?}",
        output.events()
    );
    let inside_trailing = adapter
        .segment_bounds
        .get("trailing")
        .expect("trailing segment keeps clipped interaction bounds")
        .center();
    let inside_click = vec![
        egui::Event::PointerMoved(inside_trailing),
        egui::Event::PointerButton {
            pos: inside_trailing,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        },
        egui::Event::PointerButton {
            pos: inside_trailing,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        },
    ];
    let mut inside_output = None;
    crate::run_ui_discard(&context, narrow_input(inside_click), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            inside_output = Some(adapter.show(ui, &mut status).expect("status bar renders"));
        });
    });
    assert_eq!(
        inside_output.expect("inside click frame runs").events(),
        &[StatusBarEvent::SegmentPressed {
            id: "trailing".to_owned(),
        }]
    );
    let plan = adapter
        .artifact_paint_plan()
        .expect("status bar produces a paint plan");
    for operation in &plan.operations {
        let clip = operation.clip_bounds;
        assert!(clip.x >= plan.surface_bounds.x);
        assert!(clip.y >= plan.surface_bounds.y);
        assert!(
            clip.x.saturating_add_unsigned(clip.width)
                <= plan
                    .surface_bounds
                    .x
                    .saturating_add_unsigned(plan.surface_bounds.width)
        );
        assert!(
            clip.y.saturating_add_unsigned(clip.height)
                <= plan
                    .surface_bounds
                    .y
                    .saturating_add_unsigned(plan.surface_bounds.height)
        );
    }
    assert!(
        adapter
            .raster_evidence()
            .iter()
            .all(|raster| raster.width <= plan.surface_bounds.width)
    );
}
