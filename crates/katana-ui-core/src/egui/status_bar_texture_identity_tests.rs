use super::EguiStatusBarAdapter;
use crate::molecule::{StatusBar, StatusBarMode, StatusBarSegment};

#[test]
fn unit_adapter_replaces_retained_label_texture_when_tone_changes() {
    let context = egui::Context::default();
    let mut adapter = EguiStatusBarAdapter::new("status-bar-retained-tone")
        .expect("status bar adapter should retain its platform rasterizer");
    let neutral = StatusBar::new("neutral")
        .mode(StatusBarMode::MultiSegment)
        .segment(StatusBarSegment::new("state", "Ready"));
    let mut neutral = serde_json::to_value(neutral).expect("neutral status serializes");
    let mut warning = serde_json::to_value(
        StatusBar::new("warning")
            .mode(StatusBarMode::MultiSegment)
            .segment(StatusBarSegment::new("state", "Ready")),
    )
    .expect("warning status serializes");
    neutral["segments"][0]["tone"] = serde_json::json!("Neutral");
    warning["segments"][0]["tone"] = serde_json::json!("Warning");
    let mut neutral: StatusBar = serde_json::from_value(neutral).expect("neutral status decodes");
    let mut warning: StatusBar = serde_json::from_value(warning).expect("warning status decodes");

    let first = context.run_ui(egui::RawInput::default(), |ui| {
        adapter
            .show(ui, &mut neutral)
            .expect("neutral status renders");
    });
    let first_texture = first
        .textures_delta
        .set
        .keys()
        .next()
        .copied()
        .expect("neutral label uploads a texture");
    let mut first = first;
    first.textures_delta.clear();

    let second = context.run_ui(egui::RawInput::default(), |ui| {
        adapter
            .show(ui, &mut warning)
            .expect("warning status renders");
    });
    let second_texture = second
        .textures_delta
        .set
        .keys()
        .next()
        .copied()
        .expect("warning label uploads a replacement texture");

    assert_ne!(first_texture, second_texture);
    let mut second = second;
    second.textures_delta.clear();
}
