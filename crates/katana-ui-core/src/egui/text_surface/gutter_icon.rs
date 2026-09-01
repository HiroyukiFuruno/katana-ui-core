use super::artifact_model::EguiTextSurfaceError;
use super::model::{
    EguiTextSurfaceDrawLayer, TextSurfacePaintOperation, TextSurfacePaintOperationKind,
    TextSurfacePaintTexture,
};
use crate::molecule::RgbaColor;
use crate::render_model::{RGBA_CHANNEL_COUNT, UiRect};
use crate::svg_raster::{UiSvgRasterRequest, UiSvgRasterizer};
use crate::text_surface::TextSurfaceGutterFrame;

pub(super) fn marker_texture_operation(
    rasterizer: &mut UiSvgRasterizer,
    gutter: &TextSurfaceGutterFrame,
    clip_bounds: UiRect,
    foreground: [u8; RGBA_CHANNEL_COUNT],
) -> Result<Option<TextSurfacePaintOperation>, EguiTextSurfaceError> {
    let (Some(icon), Some(bounds)) = (gutter.icon.as_ref(), gutter.marker_bounds) else {
        return Ok(None);
    };
    let [red, green, blue, alpha] = foreground;
    let raster = rasterizer.rasterize(&UiSvgRasterRequest {
        icon: icon.clone(),
        width_px: bounds.width,
        height_px: bounds.height,
        color: RgbaColor::new(red, green, blue, alpha),
    })?;
    Ok(Some(TextSurfacePaintOperation {
        layer: EguiTextSurfaceDrawLayer::Gutter,
        clip_bounds,
        kind: TextSurfacePaintOperationKind::Texture {
            bounds,
            texture: TextSurfacePaintTexture {
                identity: raster.metadata.cache_key,
                width: raster.width_px,
                height: raster.height_px,
                rgba_pixels: raster.rgba_unmultiplied,
            },
        },
    }))
}
