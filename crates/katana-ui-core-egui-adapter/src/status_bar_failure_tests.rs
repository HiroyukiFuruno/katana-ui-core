use super::{EguiStatusBarAdapter, StatusBarRenderStyle};
use katana_ui_core::molecule::{
    StatusBar, StatusBarMode, StatusBarSegment, StatusBarSegmentAlignment,
};

#[test]
fn fresh_adapter_exposes_empty_artifact_and_raster_evidence() {
    let adapter = EguiStatusBarAdapter::new("status-bar-default-evidence")
        .expect("status bar adapter should retain its platform rasterizer");

    assert!(adapter.artifact_paint_plan().is_none());
    assert!(adapter.raster_evidence().is_empty());
}

#[test]
fn unit_adapter_rejects_invalid_rasters_without_retaining_a_partial_plan() {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-bar-empty-segment")
        .expect("status bar adapter should retain its platform rasterizer");
    let mut status = StatusBar::new("empty-segment")
        .mode(StatusBarMode::MultiSegment)
        .segment(StatusBarSegment::new("empty", ""));
    let mut result = None;

    crate::run_ui_discard(&context, egui::RawInput::default(), |ui| {
        result = Some(adapter.show(ui, &mut status));
    });

    let error = match result.expect("empty-segment frame runs") {
        Ok(_) => panic!("an empty segment must fail closed"),
        Err(error) => error,
    };
    assert_eq!(
        error.to_string(),
        "status-bar raster failed: platform text raster request must not be empty"
    );
    assert!(adapter.artifact_paint_plan().is_none());
    assert!(adapter.raster_evidence().is_empty());

    let mut partial = StatusBar::new("partial-failure")
        .mode(StatusBarMode::MultiSegment)
        .segment(StatusBarSegment::new("valid", "Rendered first"))
        .segment(StatusBarSegment::new("invalid", "").alignment(StatusBarSegmentAlignment::Center));
    let mut partial_result = None;
    crate::run_ui_discard(&context, egui::RawInput::default(), |ui| {
        partial_result = Some(adapter.show(ui, &mut partial));
    });
    let partial_error = match partial_result.expect("partial-failure frame runs") {
        Ok(_) => panic!("a later invalid segment must fail closed"),
        Err(error) => error,
    };
    assert_eq!(partial_error.to_string(), error.to_string());
    assert!(adapter.artifact_paint_plan().is_none());
    assert!(
        adapter.raster_evidence().is_empty(),
        "a failed frame must not expose evidence from an earlier valid segment"
    );

    let mut empty_message = StatusBar::new("empty-message")
        .mode(StatusBarMode::SingleMessage)
        .message("");
    let mut message_result = None;
    crate::run_ui_discard(&context, egui::RawInput::default(), |ui| {
        message_result = Some(adapter.show(ui, &mut empty_message));
    });
    let message_error = match message_result.expect("empty-message frame runs") {
        Ok(_) => panic!("an empty single message must fail closed"),
        Err(error) => error,
    };
    assert_eq!(message_error.to_string(), error.to_string());
    assert!(adapter.artifact_paint_plan().is_none());
    assert!(adapter.raster_evidence().is_empty());

    let oversized_context = egui::Context::default();
    oversized_context.set_pixels_per_point(2.0);
    let mut oversized = StatusBar::new("oversized-label")
        .mode(StatusBarMode::MultiSegment)
        .segment(StatusBarSegment::new("oversized", "W"));
    let mut oversized_style = StatusBarRenderStyle::standard();
    oversized_style.font.size = 2_900.0;
    let mut oversized_result = None;
    crate::run_ui_discard(
        &oversized_context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(10_000.0, 80.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            oversized_result = Some(adapter.show_with_style(ui, &mut oversized, &oversized_style));
        },
    );
    let oversized_error = match oversized_result.expect("oversized-label frame runs") {
        Ok(_) => panic!("a high-DPI oversized label must fail closed"),
        Err(error) => error,
    };
    assert!(
        oversized_error
            .to_string()
            .contains("exceeds 16777216 pixel limit")
    );
    assert!(adapter.artifact_paint_plan().is_none());
    assert!(adapter.raster_evidence().is_empty());
}

#[test]
fn unit_adapter_omits_an_empty_elided_label_without_a_raster_error() {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-bar-empty-elision")
        .expect("status bar adapter should retain its platform rasterizer");
    let mut status = StatusBar::new("empty-elision")
        .mode(StatusBarMode::SingleMessage)
        .message("A label that cannot fit into a one-pixel status bar");
    crate::run_ui_discard(
        &context,
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1.0, 60.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            adapter
                .show(ui, &mut status)
                .expect("empty elision must render");
        },
    );
    assert!(adapter.raster_evidence().is_empty());
}
