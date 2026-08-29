use crate::texture_cache::RgbaTextureCache;
use katana_ui_core_svg_raster::UiSvgRasterizer;
use katana_ui_core_text_raster::PlatformTextRasterizer;

pub struct EguiTextSurfaceAdapter {
    pub(crate) rasterizer: PlatformTextRasterizer,
    pub(crate) svg_rasterizer: UiSvgRasterizer,
    pub(crate) textures: RgbaTextureCache,
    pub(crate) pending_focus_request: Option<bool>,
}
