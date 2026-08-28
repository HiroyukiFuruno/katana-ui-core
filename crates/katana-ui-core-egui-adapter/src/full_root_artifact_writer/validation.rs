use super::metadata::FullRootArtifactError;
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use sha2::{Digest, Sha256};
use std::path::Path;

const RGBA_CHANNEL_COUNT: usize = 4;

pub(super) fn validate_output_dir(path: &Path) -> Result<(), FullRootArtifactError> {
    if path.as_os_str().is_empty() {
        return Err(FullRootArtifactError::InvalidPath("path is empty"));
    }
    if path.as_os_str() == "." || path.as_os_str() == ".." {
        return Err(FullRootArtifactError::InvalidPath(
            "path names a directory marker",
        ));
    }
    Ok(())
}

pub(super) fn validate_stage_id(stage_id: &str) -> Result<(), FullRootArtifactError> {
    if stage_id.is_empty()
        || stage_id == "."
        || stage_id == ".."
        || stage_id.contains('/')
        || stage_id.contains('\\')
        || stage_id.contains('\0')
    {
        return Err(FullRootArtifactError::InvalidStageId);
    }
    Ok(())
}

pub(super) fn checked_rgba_len(width: u32, height: u32) -> Result<usize, FullRootArtifactError> {
    if width == 0 || height == 0 {
        return Err(FullRootArtifactError::ZeroDimensions);
    }
    (width as usize)
        .checked_mul(height as usize)
        .and_then(|pixels| pixels.checked_mul(RGBA_CHANNEL_COUNT))
        .ok_or(FullRootArtifactError::DimensionOverflow)
}

pub(super) fn encode_png(
    rgba: &[u8],
    width: u32,
    height: u32,
) -> Result<Vec<u8>, FullRootArtifactError> {
    let mut bytes = Vec::new();
    PngEncoder::new(&mut bytes)
        .write_image(rgba, width, height, ColorType::Rgba8.into())
        .map_err(FullRootArtifactError::Encode)?;
    Ok(bytes)
}

pub(super) fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::super::metadata::FullRootArtifactError;
    use super::{checked_rgba_len, encode_png, validate_output_dir, validate_stage_id};

    #[test]
    fn checks_output_dir_and_stage_id_inputs() {
        assert!(matches!(
            validate_output_dir(std::path::Path::new("")),
            Err(FullRootArtifactError::InvalidPath(_))
        ));
        assert!(matches!(
            validate_output_dir(std::path::Path::new("frame")),
            Ok(())
        ));
        assert!(validate_output_dir(std::path::Path::new(".")).is_err());
        assert!(validate_output_dir(std::path::Path::new("..")).is_err());

        assert!(validate_stage_id("abc").is_ok());
        assert!(validate_stage_id("").is_err());
        assert!(validate_stage_id("../bad").is_err());
        assert!(validate_stage_id("a\\b").is_err());
    }

    #[test]
    fn checks_rgba_len_and_png_encoding() {
        assert!(checked_rgba_len(2, 1).is_ok());
        assert!(checked_rgba_len(0, 1).is_err());
        assert!(checked_rgba_len(u32::MAX, u32::MAX,).is_err());

        let png = encode_png(&[0, 0, 0, 0, 255, 255, 255, 255], 2, 1).expect("small png encodes");
        assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
    }
}
