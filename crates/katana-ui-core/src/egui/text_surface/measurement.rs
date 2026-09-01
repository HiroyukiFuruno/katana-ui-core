use super::artifact_model::EguiTextSurfaceError;
use super::model::SharedTextMetrics;
use super::model::TextSurfaceRasterStyle;
use super::raster::{RasterFrame, rasterize_gutter_label};
use crate::render_model::UiRect;
use crate::text_raster::PlatformTextRasterizer;
use crate::text_surface::{TextSurface, TextSurfaceLayout, TextSurfaceViewportSizing};
use egui::Ui;

pub(super) const AUTOMATIC_GUTTER_MIN_WIDTH: u32 = 52;
const AUTOMATIC_GUTTER_LABEL_PADDING: u32 = 12;

pub(super) fn placeholder_raster_identity(
    surface: &TextSurface,
    raster_style: &TextSurfaceRasterStyle,
) -> String {
    format!(
        "{}:placeholder:{}:{raster_style:?}",
        surface.state().text_area.state_id.as_str(),
        surface.props().text_area.options().placeholder,
    )
}

pub(super) fn placeholder_bounds(
    content_bounds: &UiRect,
    placeholder: Option<&RasterFrame>,
    scale_factor: f32,
) -> Option<UiRect> {
    placeholder.map(|value| {
        let width = logical_extent(value.raster.width, scale_factor).min(content_bounds.width);
        let height = logical_extent(value.raster.height, scale_factor).min(content_bounds.height);
        UiRect::new(content_bounds.x, content_bounds.y, width, height)
    })
}

pub(super) fn controlled_gutter_width(
    rasterizer: &mut PlatformTextRasterizer,
    layout: &TextSurfaceLayout,
    style: &TextSurfaceRasterStyle,
    scale_factor: f32,
    metrics: &SharedTextMetrics,
) -> Result<u32, EguiTextSurfaceError> {
    let label = layout
        .lines
        .iter()
        .map(|line| line.logical_row.saturating_add(1))
        .max()
        .unwrap_or(1)
        .to_string();
    let raster = rasterize_gutter_label(rasterizer, &label, style, scale_factor, metrics)?;
    Ok(logical_extent(raster.width, scale_factor)
        .saturating_add(AUTOMATIC_GUTTER_LABEL_PADDING)
        .max(AUTOMATIC_GUTTER_MIN_WIDTH))
}

pub(super) fn surface_extent_for_ui(ui: &Ui, surface: &mut TextSurface) -> (f32, f32) {
    match surface.props().viewport_sizing {
        TextSurfaceViewportSizing::Fixed => (
            ui.available_width()
                .min(surface.props().viewport.width.max(1) as f32),
            surface.props().viewport.height.max(1) as f32,
        ),
        TextSurfaceViewportSizing::AdapterMeasured => {
            let measured_viewport_width = ui.available_width().max(1.0).round() as u32;
            let measured_viewport_height = ui.available_height().max(1.0).round() as u32;
            let _ = surface.synchronize_measured_viewport_size(
                measured_viewport_width,
                measured_viewport_height,
            );
            (
                measured_viewport_width as f32,
                measured_viewport_height as f32,
            )
        }
    }
}

fn logical_extent(value: usize, scale_factor: f32) -> u32 {
    ((value as f32 / scale_factor.max(1.0)).ceil().max(1.0)) as u32
}
