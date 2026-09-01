#![cfg(feature = "egui")]
use katana_ui_core::egui::text_command_surface::{TabStripText, TabStripTextRasterizer};
use katana_ui_core::text_raster::{PlatformFontCatalog, PlatformTextRasterConfig};
use std::sync::Arc;

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
