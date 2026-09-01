use image::codecs::gif::GifEncoder;
use image::{Delay, Frame, RgbaImage};
use katana_ui_core::egui::artifact_compositor::{
    ArtifactCanvasBounds, ArtifactCompositeRequest, ArtifactCompositor, ArtifactPaintPlanRef,
};
use katana_ui_core::egui::text_surface::TextSurfaceArtifactFrame;
use katana_ui_core::render_model::UiRect;
use std::fs::File;
use std::path::Path;

pub(super) const RGBA_CHANNELS: usize = 4;
const GIF_FRAME_DELAY_MS: u32 = 160;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TextSurfacePlanPixels {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) rgba: Vec<u8>,
    pub(super) paint_plan_hash: String,
    pub(super) pixel_hash: String,
}

pub(super) fn render_artifact_frame(
    frame: &TextSurfaceArtifactFrame,
    canvas: UiRect,
) -> Result<TextSurfacePlanPixels, String> {
    let plan = &frame.paint_plan;
    let plans = [ArtifactPaintPlanRef::TextSurface(plan)];
    let composite = match ArtifactCompositor::compose(ArtifactCompositeRequest {
        canvas: ArtifactCanvasBounds::new(canvas),
        plans: &plans,
    }) {
        Ok(composite) => composite,
        Err(error) => return Err(error.to_string()),
    };
    Ok(TextSurfacePlanPixels {
        width: canvas.width,
        height: canvas.height,
        pixel_hash: composite.pixel_hash,
        paint_plan_hash: composite.paint_plan_hash,
        rgba: composite.rgba_pixels,
    })
}

pub(super) fn write_png(pixels: &TextSurfacePlanPixels, path: &Path) -> image::ImageResult<()> {
    image_for_pixels(pixels)?.save(path)
}

pub(super) fn write_gif(frames: &[TextSurfacePlanPixels], path: &Path) -> image::ImageResult<()> {
    let file = File::create(path)?;
    let mut encoder = GifEncoder::new(file);
    let animation = frames
        .iter()
        .map(|pixels| {
            Ok(Frame::from_parts(
                image_for_pixels(pixels)?,
                0,
                0,
                Delay::from_numer_denom_ms(GIF_FRAME_DELAY_MS, 1),
            ))
        })
        .collect::<image::ImageResult<Vec<_>>>()?;
    encoder.encode_frames(animation)
}

fn image_for_pixels(pixels: &TextSurfacePlanPixels) -> image::ImageResult<RgbaImage> {
    RgbaImage::from_raw(pixels.width, pixels.height, pixels.rgba.clone()).ok_or_else(|| {
        image::ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::DimensionMismatch,
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::text_surface_script::run_scripted_sequence;

    #[test]
    fn invalid_pixel_dimensions_fail_closed_before_encoding() {
        let pixels = TextSurfacePlanPixels {
            width: 2,
            height: 2,
            rgba: vec![255],
            paint_plan_hash: "paint".to_string(),
            pixel_hash: "pixel".to_string(),
        };
        assert!(image_for_pixels(&pixels).is_err());
    }

    #[test]
    fn empty_composite_canvas_fails_closed()
    -> Result<(), crate::visual::text_surface_script_types::TextSurfaceArtifactError> {
        let sequence = run_scripted_sequence()?;
        assert!(
            render_artifact_frame(&sequence.steps[0].artifact, UiRect::new(0, 0, 0, 0),).is_err()
        );
        Ok(())
    }
}
