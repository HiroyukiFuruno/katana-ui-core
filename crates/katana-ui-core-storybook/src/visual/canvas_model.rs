use super::canvas_clip::CanvasClip;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Canvas {
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) pixels: Vec<u32>,
    pub(super) clip: Option<CanvasClip>,
}
