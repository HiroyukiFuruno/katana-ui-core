#![cfg(feature = "egui")]
#[path = "support/text_raster_tab_host.rs"]
mod text_raster_tab_host;

use katana_ui_core::egui::text_command_surface::{
    EguiTextCommandSurfaceHostRoot, TabStripCorrelation, TabStripProjection,
    TabStripTabCapabilities, TabStripTabDescriptor, TabStripTabTarget, TabStripText,
    TabStripTextRasterizer,
};
use katana_ui_core::text_raster::{PlatformFontCatalog, PlatformTextRasterConfig};
use std::sync::Arc;

fn frame_with_texture_limit(
    context: &egui::Context,
    root: &mut EguiTextCommandSurfaceHostRoot,
    max_texture_side: usize,
) -> egui::FullOutput {
    context.run_ui(
        egui::RawInput {
            max_texture_side: Some(max_texture_side),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 720.0),
            )),
            ..egui::RawInput::default()
        },
        |ui| {
            root.show(ui).expect("tab-strip frame");
        },
    )
}

#[test]
fn public_tab_strip_text_raster_route_executes_real_rasterization() -> Result<(), String> {
    let mut default_rasterizer =
        TabStripTextRasterizer::new().map_err(|error| error.to_string())?;
    let default_raster = default_rasterizer
        .rasterize(&TabStripText::new("default route"), 1.0)
        .map_err(|error| error.to_string())?;
    assert_eq!(format!("{default_raster:?}"), "TabStripTextRaster(..)");

    let config = PlatformTextRasterConfig::default();
    let catalog = Arc::new(PlatformFontCatalog::new(config.catalog_policy().clone()));
    let mut rasterizer =
        TabStripTextRasterizer::with_catalog(catalog, config).map_err(|error| error.to_string())?;

    let text = TabStripText::new("日本語 ⭐️");
    let raster = rasterizer
        .rasterize(&text, 1.0)
        .map_err(|error| error.to_string())?;

    assert_eq!(format!("{raster:?}"), "TabStripTextRaster(..)");
    Ok(())
}

#[test]
fn public_tab_strip_long_label_uploads_only_backend_sized_tiles() {
    let target = TabStripTabTarget::from_opaque_bytes(b"long-label");
    let projection = TabStripProjection::new(
        1,
        TabStripCorrelation::from_opaque_bytes(b"long-label-correlation"),
    )
    .tab(
        TabStripTabDescriptor::new(target, TabStripText::new("長いラベル ".repeat(40)))
            .capabilities(TabStripTabCapabilities::new().selectable(true)),
    );
    let mut root = text_raster_tab_host::root_with_projection(projection);
    let context = egui::Context::default();
    let mut output = frame_with_texture_limit(&context, &mut root, 1024);
    let uploaded = output
        .textures_delta
        .set
        .values()
        .flat_map(|deltas| deltas.iter().map(|delta| delta.image.size()))
        .collect::<Vec<_>>();
    output.textures_delta.clear();
    assert!(!uploaded.is_empty(), "tab-strip emitted no textures");
    assert!(
        uploaded
            .iter()
            .all(|size| size.iter().all(|side| *side <= 1024)),
        "tab-strip texture upload exceeded backend limit"
    );
}
