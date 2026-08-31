use super::{RGBA_CHANNEL_COUNT, TabStripTextRasterizer};
use crate::text_command_surface::TabStripText;

#[test]
fn rasterizes_japanese_and_color_emoji_without_egui_text_widgets() -> Result<(), String> {
    let mut rasterizer = TabStripTextRasterizer::new().map_err(|error| error.to_string())?;
    let raster = rasterizer
        .rasterize(&TabStripText::new("日本語 ⭐️"), 1.0)
        .map_err(|error| error.to_string())?;

    assert!(raster.width > 0);
    assert!(raster.height > 0);
    assert!(
        raster
            .rgba_pixels
            .chunks_exact(RGBA_CHANNEL_COUNT)
            .any(|rgba| {
                let [red, green, blue, alpha] = [rgba[0], rgba[1], rgba[2], rgba[3]];
                alpha > 0 && (red != green || green != blue)
            })
    );
    Ok(())
}

#[test]
fn raster_debug_shape_is_opaque() -> Result<(), String> {
    let mut rasterizer = TabStripTextRasterizer::new().map_err(|error| error.to_string())?;
    let raster = rasterizer
        .rasterize(&TabStripText::new("debug"), 1.0)
        .map_err(|error| error.to_string())?;

    assert_eq!(format!("{:?}", raster), "TabStripTextRaster(..)");
    assert!(raster.width > 0);
    assert!(raster.height > 0);
    Ok(())
}

#[test]
fn raster_empty_text_is_treated_as_validation_error() {
    let mut rasterizer =
        TabStripTextRasterizer::new().expect("tab strip rasterizer should initialize");
    let error = rasterizer
        .rasterize(&TabStripText::new(""), 1.0)
        .expect_err("empty tab strip text must fail");
    assert!(error.to_string().contains("must not be empty"));
}

#[test]
fn rasterize_with_high_scale_preserves_non_zero_extent() -> Result<(), String> {
    let mut rasterizer = TabStripTextRasterizer::new().map_err(|error| error.to_string())?;
    let raster = rasterizer
        .rasterize(&TabStripText::new("高解像度"), 2.0)
        .map_err(|error| error.to_string())?;

    assert!(raster.width > 0);
    assert!(raster.height > 0);
    assert!(raster.rgba_pixels.len().is_multiple_of(RGBA_CHANNEL_COUNT));
    Ok(())
}
