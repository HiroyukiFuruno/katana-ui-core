use super::canvas_clip::CanvasClip;

#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) logical_width: usize,
    pub(super) logical_height: usize,
    pub(super) scale_factor: f32,
    pub(super) pixels: Vec<u32>,
    pub(super) clip: Option<CanvasClip>,
}
