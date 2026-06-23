use super::canvas_clip::CanvasClip;
use super::text_selection::SelectableTextRun;

#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) logical_width: usize,
    pub(super) logical_height: usize,
    pub(super) scale_factor: f32,
    pub(super) raster_scale_factor: f32,
    pub(super) image_surface_extent_mode: CanvasImageSurfaceExtentMode,
    pub(super) pixels: Vec<u32>,
    pub(super) clip: Option<CanvasClip>,
    pub(super) text_runs: Vec<SelectableTextRun>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CanvasImageSurfaceExtentMode {
    LogicalDisplay,
    RasterPresentation,
}
