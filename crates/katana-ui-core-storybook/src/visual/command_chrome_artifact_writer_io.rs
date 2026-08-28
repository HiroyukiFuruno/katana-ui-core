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
    if name.len() < FRAME_NAME_PREFIX_LENGTH {
        return false;
    }
    let prefix = &name.as_bytes()[..FRAME_NAME_PREFIX_LENGTH];
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    #[cfg(target_os = "linux")]
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};

    #[test]
    fn frame_prefix_predicate_matches_only_expected_names() {
        assert!(has_frame_prefix("12-frame.png"));
        assert!(has_frame_prefix("00-foo.png"));
        assert!(!has_frame_prefix("1x-foo.png"));
        assert!(!has_frame_prefix("12_foo"));
        assert!(!has_frame_prefix("foo"));
        assert!(!has_frame_prefix("x"));
    }

    #[test]
    fn clear_previous_artifact_files_keeps_only_managed_artifact_files()
    -> Result<(), CommandChromeArtifactError> {
        let output_dir = temp_dir("command-chrome-artifact-writer-io")?;
        let files = [
            ARTIFACT_GIF_FILE,
            ARTIFACT_MANIFEST_FILE,
            "00-frame.png",
            "01-frame.png",
            "note.txt",
            "00-keep.png.tmp",
        ];
        for file in files {
            let path = output_dir.join(file);
            fs::write(&path, b"data")?;
        }

        clear_previous_artifact_files(&output_dir)?;

        assert!(!output_dir.join(ARTIFACT_GIF_FILE).exists());
        assert!(!output_dir.join(ARTIFACT_MANIFEST_FILE).exists());
        assert!(!output_dir.join("00-frame.png").exists());
        assert!(!output_dir.join("01-frame.png").exists());
        assert!(output_dir.join("note.txt").exists());
        assert!(output_dir.join("00-keep.png.tmp").exists());
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn clear_previous_artifact_files_ignores_non_utf8_names()
    -> Result<(), CommandChromeArtifactError> {
        let output_dir = temp_dir("command-chrome-artifact-writer-io-non-utf8")?;
        let path = output_dir.join(OsString::from_vec(vec![0xff]));
        fs::write(&path, b"data")?;
        clear_previous_artifact_files(&output_dir)?;
        assert!(path.exists());
        Ok(())
    }

    #[test]
    fn write_png_fails_for_invalid_pixel_dimensions() -> Result<(), CommandChromeArtifactError> {
        let output_dir = temp_dir("command-chrome-artifact-writer-io-invalid")?;
        let path = output_dir.join("invalid.png");
        let pixels = CommandChromePlanPixels {
            width: 2,
            height: 2,
            rgba: vec![255, 255, 255, 255, 255],
            paint_plan_hash: "hash".to_string(),
            pixel_hash: "hash".to_string(),
        };
        let result = write_png(&pixels, &path);
        assert!(result.is_err());
        Ok(())
    }

    fn temp_dir(prefix: &str) -> Result<PathBuf, CommandChromeArtifactError> {
        let mut path = env::temp_dir();
        path.push("katana-storybook-command-chrome-artifact-writer");
        path.push(format!(
            "{prefix}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        fs::create_dir_all(&path).map_err(CommandChromeArtifactError::Io)?;
        Ok(path)
    }
}
