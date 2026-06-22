use katana_ui_core::render_model::UiImageSurfaceProps;

#[derive(Clone, PartialEq)]
pub(super) struct CachedImageKey {
    fingerprint: String,
    width: u32,
    height: u32,
    rgba_len: usize,
    content_scale: u32,
    target_width: usize,
    target_height: usize,
    canvas_scale_bits: u32,
}

impl CachedImageKey {
    pub(super) fn placeholder() -> Self {
        Self {
            fingerprint: String::new(),
            width: 0,
            height: 0,
            rgba_len: 0,
            content_scale: 0,
            target_width: 0,
            target_height: 0,
            canvas_scale_bits: 0,
        }
    }

    pub(super) fn new(
        image: &UiImageSurfaceProps,
        canvas_scale: f32,
        target_width: usize,
        target_height: usize,
    ) -> Option<Self> {
        if canvas_scale.to_bits() != 1.0f32.to_bits() && canvas_scale.to_bits() != 2.0f32.to_bits()
        {
            return None;
        }
        if image.transform.zoom_percent != 100
            || image.transform.pan_x != 0
            || image.transform.pan_y != 0
        {
            return None;
        }
        Some(Self {
            fingerprint: image.fingerprint.clone(),
            width: image.width,
            height: image.height,
            rgba_len: image.rgba.len(),
            content_scale: image.content_scale,
            target_width,
            target_height,
            canvas_scale_bits: canvas_scale.to_bits(),
        })
    }
}
