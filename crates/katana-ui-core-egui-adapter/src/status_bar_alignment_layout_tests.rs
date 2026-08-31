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
fn long_alignment_groups_render_and_hit_test_in_non_overlapping_root_columns() {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-bar-alignment-columns")
        .expect("status bar adapter should retain its platform rasterizer");
    let long_label = "This status segment must remain inside its own alignment column";
    let mut status = StatusBar::new("alignment-columns")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new("leading", long_label)
                .alignment(StatusBarSegmentAlignment::Leading)
                .interactive(true),
        )
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
    let bounds = ["leading", "center", "trailing"].map(|id| {
        *adapter
            .segment_bounds
            .get(id)
            .unwrap_or_else(|| panic!("{id} segment keeps an interaction target"))
    });
    for bounds in bounds {
        assert!(
            bounds.left() >= root.x as f32 && bounds.right() <= root.x as f32 + root.width as f32,
            "segment interaction bounds escaped root {root:?}: {bounds:?}"
        );
    }
    for (left_index, left) in bounds.iter().enumerate() {
        for right in &bounds[left_index + 1..] {
            assert!(
                left.right() <= right.left() || right.right() <= left.left(),
                "alignment targets overlap: {left:?} and {right:?}"
            );
        }
    }

    for (id, position) in ["leading", "center", "trailing"]
        .into_iter()
        .zip(bounds.map(|rect| rect.center()))
    {
        let click = vec![
            egui::Event::PointerMoved(position),
            egui::Event::PointerButton {
                pos: position,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos: position,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ];
        let mut output = None;
        crate::run_ui_discard(&context, narrow_input(click), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                output = Some(adapter.show(ui, &mut status).expect("status bar renders"));
            });
        });
        assert_eq!(
            output.expect("click frame runs").events(),
            &[StatusBarEvent::SegmentPressed { id: id.to_owned() }]
        );
    }
}

#[test]
fn fitting_alignment_groups_preserve_their_root_anchors() {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-bar-root-anchors")
        .expect("status bar adapter should retain its platform rasterizer");
    let mut status = StatusBar::new("root-anchors")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new("leading", "Leading")
                .alignment(StatusBarSegmentAlignment::Leading),
        )
        .segment(
            StatusBarSegment::new("center", "Center").alignment(StatusBarSegmentAlignment::Center),
        )
        .segment(
            StatusBarSegment::new("trailing", "Trailing")
                .alignment(StatusBarSegmentAlignment::Trailing),
        );

    crate::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(700.0, NARROW_VIEWPORT_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                adapter.show(ui, &mut status).expect("status bar renders");
            });
        },
    );

    let root = adapter
        .artifact_paint_plan()
        .expect("status bar produces a paint plan")
        .surface_bounds;
    let leading = adapter.segment_bounds["leading"];
    let center = adapter.segment_bounds["center"];
    let trailing = adapter.segment_bounds["trailing"];
    assert_eq!(leading.left(), root.x as f32);
    assert_eq!(center.center().x, root.x as f32 + root.width as f32 / 2.0);
    assert_eq!(trailing.right(), root.x as f32 + root.width as f32);
}

#[test]
fn single_long_alignment_keeps_the_full_root_width() {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-bar-single-long-alignment")
        .expect("status bar adapter should retain its platform rasterizer");
    let mut status = StatusBar::new("single-long-alignment")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new(
                "leading",
                "A single alignment group may use the complete status bar width",
            )
            .alignment(StatusBarSegmentAlignment::Leading),
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
    let leading = adapter.segment_bounds["leading"];
    assert_eq!(leading.left(), root.x as f32);
    assert_eq!(leading.right(), root.x as f32 + root.width as f32);
}

#[test]
fn asymmetric_fitting_groups_partition_when_their_desired_intervals_overlap() {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-bar-asymmetric-columns")
        .expect("status bar adapter should retain its platform rasterizer");
    let mut status = StatusBar::new("asymmetric-columns")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new("leading", "WWWWWWWWWWWWWWWWWW")
                .alignment(StatusBarSegmentAlignment::Leading)
                .interactive(true),
        )
        .segment(
            StatusBarSegment::new("center", "C")
                .alignment(StatusBarSegmentAlignment::Center)
                .interactive(true),
        );

    crate::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(400.0, NARROW_VIEWPORT_HEIGHT),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                adapter.show(ui, &mut status).expect("status bar renders");
            });
        },
    );

    let root = adapter
        .artifact_paint_plan()
        .expect("status bar produces a paint plan")
        .surface_bounds;
    let leading = adapter.segment_bounds["leading"];
    let center = adapter.segment_bounds["center"];
    assert!(
        leading.left() >= root.x as f32 && leading.right() <= root.x as f32 + root.width as f32
    );
    assert!(center.left() >= root.x as f32 && center.right() <= root.x as f32 + root.width as f32);
    assert!(
        leading.right() <= center.left() || center.right() <= leading.left(),
        "fitting desired intervals must not overlap: {leading:?} and {center:?}"
    );
}
