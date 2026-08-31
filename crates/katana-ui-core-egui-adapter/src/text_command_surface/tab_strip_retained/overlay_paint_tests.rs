use super::*;
use crate::text_command_surface::{
    TabStripCorrelation, TabStripProjection, TabStripProjectionLease, TabStripText,
};
use std::sync::Arc;

fn build_state() -> TabStripRetainedState {
    let config = katana_ui_core_text_raster::PlatformTextRasterConfig::default();
    let catalog = Arc::new(katana_ui_core_text_raster::PlatformFontCatalog::new(
        config.catalog_policy(),
    ));
    TabStripRetainedState::from_lease(
        TabStripProjectionLease::new(TabStripProjection::new(
            1,
            TabStripCorrelation::from_opaque_bytes(b"corr"),
        )),
        catalog,
        config,
    )
    .expect("tab strip retained state should be constructible")
}

#[test]
fn paint_overlay_texture_adds_a_texture_operation() {
    let mut state = build_state();
    let mut operations = Vec::new();
    let context = egui::Context::default();
    let mut output_bounds = None;
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        let bounds = ui.max_rect();
        let texture = TabStripPaintTexture {
            identity: "test-texture".to_string(),
            width: 4,
            height: 4,
            rgba_pixels: vec![255; 4 * 4 * 4],
        };
        output_bounds = Some(bounds);
        state.paint_overlay_texture(ui, &mut operations, bounds, &texture, bounds.shrink(10.0));
    });
    platform_output.textures_delta.clear();

    assert_eq!(operations.len(), 1);
    assert_eq!(
        operations[0].clip_bounds,
        super::ui_rect(output_bounds.expect("ui max rect"))
    );
    assert!(matches!(
        operations[0].kind,
        TabStripPaintOperationKind::Texture { .. }
    ));
}

#[test]
fn paint_overlay_label_records_text_raster_texture_plan() {
    let mut state = build_state();
    let mut operations = Vec::new();
    let context = egui::Context::default();
    let mut platform_output = context.run_ui(egui::RawInput::default(), |ui| {
        let clip = ui.max_rect();
        let row = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(40.0, 20.0));
        state
            .paint_overlay_label(
                ui,
                &mut operations,
                clip,
                &TabStripText::new("overlay"),
                row,
                "label",
                0,
            )
            .expect("paint_overlay_label should record texture operation");
    });
    platform_output.textures_delta.clear();

    assert_eq!(operations.len(), 1);
    assert!(matches!(
        operations[0].kind,
        TabStripPaintOperationKind::Texture { .. }
    ));
}

#[test]
fn overlay_label_replaces_retained_texture_when_same_entry_is_renamed() {
    let mut state = build_state();
    let context = egui::Context::default();
    let row = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(120.0, 24.0));
    let mut first_operations = Vec::new();
    let mut first = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .paint_overlay_label(
                ui,
                &mut first_operations,
                ui.max_rect(),
                &TabStripText::new("Before"),
                row,
                "entry",
                0,
            )
            .expect("initial overlay label should render");
    });
    let first_identity = overlay_texture_identity(&first_operations).to_owned();
    first.textures_delta.clear();

    let mut second_operations = Vec::new();
    let mut second = context.run_ui(egui::RawInput::default(), |ui| {
        state
            .paint_overlay_label(
                ui,
                &mut second_operations,
                ui.max_rect(),
                &TabStripText::new("Renamed entry"),
                row,
                "entry",
                0,
            )
            .expect("renamed overlay label should render");
    });

    assert_ne!(first_identity, overlay_texture_identity(&second_operations));
    assert!(!second.textures_delta.set.is_empty());
    second.textures_delta.clear();
}

#[test]
fn overlay_texture_live_paint_uses_supplied_clip_bounds() {
    let mut state = build_state();
    let context = egui::Context::default();
    let clip = egui::Rect::from_min_size(egui::pos2(8.0, 8.0), egui::vec2(12.0, 12.0));
    let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(32.0, 32.0));
    let texture = TabStripPaintTexture {
        identity: "clipped-overlay-texture".to_owned(),
        width: 4,
        height: 4,
        rgba_pixels: vec![255; 4 * 4 * 4],
    };
    let mut operations = Vec::new();
    let mut output = context.run_ui(egui::RawInput::default(), |ui| {
        state.paint_overlay_texture(ui, &mut operations, clip, &texture, bounds);
    });

    assert!(output.shapes.iter().any(|shape| shape.clip_rect == clip));
    assert_eq!(operations[0].clip_bounds, super::ui_rect(clip));
    output.textures_delta.clear();
}

fn overlay_texture_identity(operations: &[TabStripPaintOperation]) -> &str {
    operations
        .iter()
        .find_map(|operation| match &operation.kind {
            TabStripPaintOperationKind::Texture { texture, .. } => Some(texture.identity.as_str()),
            TabStripPaintOperationKind::Fill { .. } => None,
        })
        .expect("overlay frame should contain a texture")
}
