use super::ui_tree_canvas_image_cache_key::CachedImageKey;

#[derive(Clone)]
pub(super) struct CachedImageSurface {
    pub(super) key: CachedImageKey,
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) pixels: Vec<u32>,
    pub(super) alpha: Vec<u8>,
    pub(super) opaque_rows: Vec<bool>,
    pub(super) opaque_spans: Vec<Vec<(usize, usize)>>,
    pub(super) translucent_spans: Vec<Vec<(usize, usize)>>,
}

impl CachedImageSurface {
    pub(super) fn row_opaque(&self, row: usize, start: usize, end: usize) -> bool {
        self.opaque_rows.get(row).copied().unwrap_or(false)
            || self.alpha[start..end].iter().all(|alpha| *alpha == u8::MAX)
    }
}
