use super::*;

#[test]
fn tab_strip_retained_error_display_covers_all_variants() {
    assert!(
        TabStripRetainedError::MissingPort
            .to_string()
            .contains("tab operation requires a proposal port")
    );
    assert!(
        TabStripRetainedError::MissingRoute
            .to_string()
            .contains("tab interaction route is unavailable")
    );
    assert!(
        TabStripRetainedError::MissingOverlayBounds
            .to_string()
            .contains("tab overlay did not produce combined bounds")
    );

    let raster_error =
        TabStripRetainedError::Raster(crate::text_raster::PlatformTextRasterError::EmptyText);
    assert!(
        raster_error
            .to_string()
            .contains("tab label rasterization failed")
    );

    let svg_error =
        TabStripRetainedError::Svg(crate::svg_raster::UiSvgRasterError::InvalidDimensions {
            width_px: 0,
            height_px: 0,
        });
    assert!(
        svg_error
            .to_string()
            .contains("tab icon rasterization failed")
    );

    let surface_error = TabStripRetainedError::TextSurface(
        crate::egui::text_surface::EguiTextSurfaceError::FrameNotProduced,
    );
    assert!(
        surface_error
            .to_string()
            .contains("tab rename input failed")
    );
}
