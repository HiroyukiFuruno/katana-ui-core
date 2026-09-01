use super::EguiStatusBarAdapter;
use crate::molecule::{
    ProgressMeterShape, ProgressMeterSpec, StatusBar, StatusBarAction, StatusBarEvent,
    StatusBarMode, StatusBarPopoverSpec, StatusBarSegment,
};
use crate::render_model::UiTone;

use crate::egui::status_bar::StatusBarPaintOperationKind;

#[test]
fn unit_adapter_renders_single_message_and_closes_an_open_popover_from_escape() {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-bar-unit-render")
        .expect("status bar adapter should retain its platform rasterizer");
    let mut single = StatusBar::new("single")
        .mode(StatusBarMode::SingleMessage)
        .message("ready");
    let mut single_output = None;
    crate::egui::run_ui_discard(&context, egui::RawInput::default(), |ui| {
        single_output = Some(adapter.show(ui, &mut single));
    });
    assert!(
        single_output
            .expect("single-message frame runs")
            .expect("single-message frame renders")
            .events()
            .is_empty()
    );

    let mut with_popover = StatusBar::new("popover")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new("details", "Details")
                .popover(StatusBarPopoverSpec::new("Status detail", "Current status")),
        );
    let opened = with_popover.apply_action(&StatusBarAction::PressSegment {
        id: "details".to_owned(),
    });
    assert!(opened.contains(&StatusBarEvent::SegmentPopoverOpened {
        id: "details".to_owned(),
    }));

    let mut escaped_output = None;
    crate::egui::run_ui_discard(
        &context,
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        },
        |ui| escaped_output = Some(adapter.show(ui, &mut with_popover)),
    );
    let escaped = escaped_output
        .expect("escape frame runs")
        .expect("escape frame renders");
    assert!(
        escaped
            .events()
            .contains(&StatusBarEvent::SegmentPopoverClosed {
                id: "details".to_owned(),
            })
    );
    assert!(with_popover.state().open_popover().is_none());
}

#[test]
fn unit_adapter_only_activates_interactive_segment_when_it_has_focus() {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-bar-unit-focus-gating")
        .expect("status bar adapter should retain its platform rasterizer");
    let mut status = StatusBar::new("segments")
        .mode(StatusBarMode::MultiSegment)
        .segment(
            StatusBarSegment::new("segment-a", "Interactive A")
                .interactive(true)
                .tooltip("A"),
        )
        .segment(
            StatusBarSegment::new("segment-b", "Interactive B")
                .interactive(true)
                .tooltip("B"),
        );
    let enter_and_space = [
        egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        },
        egui::Event::Key {
            key: egui::Key::Space,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        },
    ];
    context.memory_mut(|memory| memory.request_focus(egui::Id::new("editor-control")));

    for event in &enter_and_space {
        let event = event.clone();
        let mut output = None;
        crate::egui::run_ui_discard(
            &context,
            egui::RawInput {
                events: vec![event],
                ..egui::RawInput::default()
            },
            |ui| {
                output = Some(adapter.show(ui, &mut status).expect("status bar renders"));
            },
        );
        assert!(
            output.expect("status-bar frame runs").events().is_empty(),
            "global keypress should not activate any segment without focus"
        );
    }

    context.memory_mut(|memory| {
        memory.request_focus(adapter.id.with("segment-a".to_owned()));
    });

    for event in &enter_and_space {
        let event = event.clone();
        let mut output = None;
        crate::egui::run_ui_discard(
            &context,
            egui::RawInput {
                events: vec![event],
                ..egui::RawInput::default()
            },
            |ui| {
                output = Some(adapter.show(ui, &mut status).expect("status bar renders"));
            },
        );
        assert_eq!(
            output.expect("status-bar frame runs").events(),
            &[StatusBarEvent::SegmentPressed {
                id: "segment-a".to_owned()
            }]
        );
    }
}

#[test]
fn unit_adapter_records_all_progress_shape_paint_contracts() {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-bar-progress-shapes")
        .expect("status bar adapter should retain its platform rasterizer");
    let mut status =
        StatusBar::new("progress-shapes")
            .mode(StatusBarMode::MultiSegment)
            .segment(StatusBarSegment::new("linear", "Linear").progress(
                ProgressMeterSpec::new(ProgressMeterShape::Linear, 50).tone(UiTone::Accent),
            ))
            .segment(StatusBarSegment::new("ring", "Ring").progress(
                ProgressMeterSpec::new(ProgressMeterShape::Ring, 75).tone(UiTone::Success),
            ))
            .segment(
                StatusBarSegment::new("ring-warning", "Ring warning").progress(
                    ProgressMeterSpec::new(ProgressMeterShape::Ring, 75).tone(UiTone::Warning),
                ),
            )
            .segment(StatusBarSegment::new("pie", "Pie").progress(
                ProgressMeterSpec::new(ProgressMeterShape::Pie, 40).tone(UiTone::Warning),
            ));

    crate::egui::run_ui_discard(&context, egui::RawInput::default(), |ui| {
        adapter.show(ui, &mut status).expect("status bar renders");
    });

    let operations = &adapter
        .artifact_paint_plan()
        .expect("status bar produces a paint plan")
        .operations;
    let texture = |identity_fragment| {
        operations
            .iter()
            .find_map(|operation| match &operation.kind {
                StatusBarPaintOperationKind::Texture { texture, .. }
                    if texture.identity.contains(identity_fragment) =>
                {
                    Some(texture)
                }
                _ => None,
            })
            .expect("shape-specific progress texture is recorded")
    };
    let ring = texture("progress:ring:75:14x14:80-80-80-255:100-210-145-255");
    let ring_warning = texture("progress:ring:75:14x14:80-80-80-255:240-190-75-255");
    let pie = texture("progress:pie:40");
    assert_eq!(
        ring.identity,
        "status-bar-progress:ring:75:14x14:80-80-80-255:100-210-145-255"
    );
    assert_eq!(
        pie.identity,
        "status-bar-progress:pie:40:14x14:80-80-80-255:240-190-75-255"
    );
    assert_ne!(ring.identity, ring_warning.identity);
    let center = |texture: &crate::egui::status_bar::StatusBarPaintTexture| {
        let pixel = (texture.height / 2 * texture.width + texture.width / 2) as usize;
        let start = pixel * 4;
        [
            texture.rgba_pixels[start],
            texture.rgba_pixels[start + 1],
            texture.rgba_pixels[start + 2],
            texture.rgba_pixels[start + 3],
        ]
    };
    assert_eq!(center(ring), [0, 0, 0, 0]);
    assert_eq!(center(pie), [240, 190, 75, 255]);
    for (texture, foreground) in [
        (ring, [100, 210, 145, 255]),
        (ring_warning, [240, 190, 75, 255]),
        (pie, [240, 190, 75, 255]),
    ] {
        let (pixel_chunks, remainder) = texture.rgba_pixels.as_chunks::<4>();
        assert!(remainder.is_empty());
        assert!(pixel_chunks.iter().any(|pixel| pixel == &foreground));
        assert!(pixel_chunks.iter().any(|pixel| pixel == &[80, 80, 80, 255]));
    }
    assert_eq!(
        operations
            .iter()
            .filter(|operation| matches!(
                operation.kind,
                StatusBarPaintOperationKind::Fill { bounds, .. }
                    if bounds.height == 3 && bounds.width > 3
            ))
            .count(),
        2,
        "linear progress records its background and foreground fills"
    );
}
