use super::*;

#[test]
fn command_chrome_error_conversions_preserve_specific_failure_context() {
    let text: EguiCommandChromeError = PlatformTextRasterError::EmptyText.into();
    assert_eq!(
        text.to_string(),
        "command chrome text raster failed: platform text raster request must not be empty"
    );

    let svg: EguiCommandChromeError = UiSvgRasterError::EmptySource.into();
    assert!(svg.to_string().contains("command chrome SVG raster failed"));

    let surface: EguiCommandChromeError = EguiTextSurfaceError::FrameNotProduced.into();
    assert_eq!(
        surface.to_string(),
        "command chrome text surface failed: egui did not produce a text surface frame"
    );

    let serialization = EguiCommandChromeError::ArtifactSerialization("invalid frame".to_string());
    assert_eq!(
        serialization.to_string(),
        "command chrome artifact serialization failed: invalid frame"
    );
}

#[test]
fn rendered_raster_reserves_and_paints_the_common_logical_extent_at_hidpi_scales() {
    for scale in [1.0, 1.5, 2.0] {
        let raster = RenderedRaster::new("label".to_string(), 37, 23, Vec::new(), scale);
        assert_eq!(
            raster.width,
            crate::egui::raster_extent::LogicalRasterExtent::from_physical(37, scale)
        );
        assert_eq!(
            raster.height,
            crate::egui::raster_extent::LogicalRasterExtent::from_physical(23, scale)
        );
    }
}
