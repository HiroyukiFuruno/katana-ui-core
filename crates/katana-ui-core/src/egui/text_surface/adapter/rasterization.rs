use super::super::artifact_model::{EguiTextSurfaceError, raster_identity};
use super::super::measurement::controlled_gutter_width;
use super::super::model::{EguiTextSurfaceAdapter, TextSurfaceRasterStyle};
use super::super::raster::{RasterFrame, layout_for_surface, rasterize_surface};
use crate::text_surface::{TextSurface, TextSurfacePoint};

impl EguiTextSurfaceAdapter {
    pub(super) fn raster(
        &mut self,
        surface: &TextSurface,
        style: &TextSurfaceRasterStyle,
        content_width: f32,
        scale_factor: f32,
    ) -> Result<RasterFrame, EguiTextSurfaceError> {
        rasterize_surface(
            &mut self.rasterizer,
            surface,
            style,
            content_width,
            scale_factor,
            raster_identity(surface, style),
            &self.metrics,
        )
    }

    pub(super) fn gutter(
        &mut self,
        frame: &RasterFrame,
        surface: &TextSurface,
        style: &TextSurfaceRasterStyle,
        scale_factor: f32,
    ) -> Result<u32, EguiTextSurfaceError> {
        let layout = layout_for_surface(frame, surface, TextSurfacePoint::new(0, 0));
        controlled_gutter_width(
            &mut self.rasterizer,
            &layout,
            style,
            scale_factor,
            &self.metrics,
        )
    }
}
