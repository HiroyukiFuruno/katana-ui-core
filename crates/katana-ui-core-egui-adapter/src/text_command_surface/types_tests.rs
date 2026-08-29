use super::EguiTextCommandSurfaceAdapter;
use katana_ui_core_text_raster::PlatformTextRasterConfig;

#[test]
fn with_text_raster_config_accepts_default_settings() {
    assert!(
        EguiTextCommandSurfaceAdapter::with_text_raster_config(PlatformTextRasterConfig::default())
            .is_ok()
    );
}
