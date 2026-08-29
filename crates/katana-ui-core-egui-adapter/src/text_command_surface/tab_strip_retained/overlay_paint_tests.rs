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
