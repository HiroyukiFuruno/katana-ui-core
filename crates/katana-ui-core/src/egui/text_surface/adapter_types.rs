use crate::egui::texture_cache::RgbaTextureCache;
use crate::svg_raster::UiSvgRasterizer;
use crate::text_raster::PlatformTextRasterizer;

pub struct EguiTextSurfaceAdapter {
    pub(crate) rasterizer: PlatformTextRasterizer,
    pub(crate) svg_rasterizer: UiSvgRasterizer,
    pub(crate) textures: RgbaTextureCache,
    pub(crate) pending_focus_request: Option<bool>,
}
