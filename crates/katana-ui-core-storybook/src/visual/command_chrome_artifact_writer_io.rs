use super::super::command_chrome_artifact::CommandChromePlanPixels;
use super::super::command_chrome_script_types::CommandChromeArtifactError;
use super::{ARTIFACT_GIF_FILE, ARTIFACT_MANIFEST_FILE};
use image::codecs::gif::GifEncoder;
use image::{Delay, Frame, RgbaImage};
use std::fs;
use std::path::Path;

const GIF_FRAME_DELAY_MS: u32 = 160;
const FRAME_NAME_PREFIX_LENGTH: usize = 3;
const FRAME_TENS_INDEX: usize = 0;
const FRAME_ONES_INDEX: usize = 1;
const FRAME_SEPARATOR_INDEX: usize = 2;

pub(super) fn clear_previous_artifact_files(
    output_dir: &Path,
) -> Result<(), CommandChromeArtifactError> {
    for entry in fs::read_dir(output_dir).map_err(CommandChromeArtifactError::Io)? {
        let entry = entry.map_err(CommandChromeArtifactError::Io)?;
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if is_managed_artifact_file(&path, name) {
            fs::remove_file(path).map_err(CommandChromeArtifactError::Io)?;
        }
    }
    Ok(())
}

fn is_managed_artifact_file(path: &Path, name: &str) -> bool {
    name == ARTIFACT_GIF_FILE
        || name == ARTIFACT_MANIFEST_FILE
        || (has_frame_prefix(name) && path.extension().is_some_and(|extension| extension == "png"))
}

fn has_frame_prefix(name: &str) -> bool {
    let Some(prefix) = name.as_bytes().get(..FRAME_NAME_PREFIX_LENGTH) else {
        return false;
    };
    prefix[FRAME_TENS_INDEX].is_ascii_digit()
        && prefix[FRAME_ONES_INDEX].is_ascii_digit()
        && prefix[FRAME_SEPARATOR_INDEX] == b'-'
}

pub(super) fn write_png(pixels: &CommandChromePlanPixels, path: &Path) -> image::ImageResult<()> {
    image_for_pixels(pixels)?.save(path)
}

pub(super) fn write_gif(frames: &[CommandChromePlanPixels], path: &Path) -> image::ImageResult<()> {
    let file = std::fs::File::create(path)?;
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

fn image_for_pixels(pixels: &CommandChromePlanPixels) -> image::ImageResult<RgbaImage> {
    RgbaImage::from_raw(pixels.width, pixels.height, pixels.rgba.clone()).ok_or_else(|| {
        image::ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::DimensionMismatch,
        ))
    })
}
